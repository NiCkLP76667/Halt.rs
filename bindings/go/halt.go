// Halt.rs Go Bindings
// Provides Go language bindings for the Halt.rs multi-agent proxy system

package halt

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"sync"
	"time"

	"github.com/gorilla/websocket"
	"github.com/valyala/fasthttp"
)

// Client represents a client for the Halt.rs API
type Client struct {
	BaseURL    string
	HTTPClient *http.Client
	FastClient *fasthttp.Client
	mu         sync.RWMutex
}

// NewClient creates a new Halt.rs client
func NewClient(baseURL string) *Client {
	return &Client{
		BaseURL: baseURL,
		HTTPClient: &http.Client{
			Timeout: 30 * time.Second,
		},
		FastClient: &fasthttp.Client{
			ReadTimeout:  30 * time.Second,
			WriteTimeout: 30 * time.Second,
		},
	}
}

// Priority represents task priority levels
type Priority int

const (
	PriorityLow    Priority = 0
	PriorityMedium Priority = 1
	PriorityHigh   Priority = 2
)

// Task represents a task in the backpressure queue
type Task struct {
	ID       string   `json:"id"`
	Name     string   `json:"name"`
	Priority Priority `json:"priority"`
	Payload  string   `json:"payload"`
}

// Interaction represents an agent interaction for logging
type Interaction struct {
	FromAgent  string   `json:"from_agent"`
	ToAgent    string   `json:"to_agent"`
	TokenPath  []string `json:"token_path"`
}

// HealthResponse represents the health check response
type HealthResponse struct {
	Status  string `json:"status"`
	Version string `json:"version"`
	Uptime  int    `json:"uptime"`
}

// CircuitBreakerStatus represents circuit breaker status
type CircuitBreakerStatus struct {
	From              string `json:"from"`
	To                string `json:"to"`
	Status            string `json:"status"`
	CallsInWindow     int    `json:"calls_in_window"`
}

// TopologyMap represents the swarm topology
type TopologyMap map[string][]TopologyEdge

type TopologyEdge struct {
	From       string    `json:"from"`
	To         string    `json:"to"`
	TokenPath  []string  `json:"token_path"`
	Timestamp  time.Time `json:"timestamp"`
}

// HealthCheck performs a health check on the Halt server
func (c *Client) HealthCheck() (*HealthResponse, error) {
	var resp HealthResponse
	err := c.getJSON("/health", &resp)
	return &resp, err
}

// GetCircuitBreakerStatus gets the circuit breaker status between two agents
func (c *Client) GetCircuitBreakerStatus(from, to string) (*CircuitBreakerStatus, error) {
	var resp CircuitBreakerStatus
	err := c.getJSON(fmt.Sprintf("/api/v1/circuit-breaker/%s/%s", from, to), &resp)
	return &resp, err
}

// RegisterCall registers a call between agents
func (c *Client) RegisterCall(from, to string, isTerminal bool) (map[string]interface{}, error) {
	data := map[string]interface{}{
		"is_terminal": isTerminal,
	}
	var resp map[string]interface{}
	err := c.postJSON(fmt.Sprintf("/api/v1/circuit-breaker/%s/%s", from, to), data, &resp)
	return resp, err
}

// PushTask pushes a task to the backpressure queue
func (c *Client) PushTask(name string, priority Priority, payload string) (map[string]interface{}, error) {
	data := map[string]interface{}{
		"name":     name,
		"priority": int(priority),
		"payload":  payload,
	}
	var resp map[string]interface{}
	err := c.postJSON("/api/v1/queue/push", data, &resp)
	return resp, err
}

// PopTask pops the highest priority task from the queue
func (c *Client) PopTask() (*Task, error) {
	var resp map[string]interface{}
	err := c.postJSON("/api/v1/queue/pop", nil, &resp)
	if err != nil {
		return nil, err
	}

	// Check if response is null (empty queue)
	if resp == nil {
		return nil, nil
	}

	// Convert to Task struct
	task := &Task{
		ID:       getString(resp, "id"),
		Name:     getString(resp, "name"),
		Priority: Priority(getInt(resp, "priority")),
		Payload:  getString(resp, "payload"),
	}

	return task, nil
}

// GetTopology gets the current swarm topology map
func (c *Client) GetTopology() (TopologyMap, error) {
	var resp TopologyMap
	err := c.getJSON("/api/v1/topology", &resp)
	return resp, err
}

// LogInteraction logs an interaction between agents
func (c *Client) LogInteraction(from, to string, tokenPath []string) (map[string]interface{}, error) {
	data := Interaction{
		FromAgent: from,
		ToAgent:   to,
		TokenPath: tokenPath,
	}
	var resp map[string]interface{}
	err := c.postJSON("/api/v1/interactions", data, &resp)
	return resp, err
}

// WebSocketClient handles WebSocket connections for real-time updates
type WebSocketClient struct {
	URL      string
	conn     *websocket.Conn
	mu       sync.Mutex
	onMessage func(string)
}

// NewWebSocketClient creates a new WebSocket client
func NewWebSocketClient(url string) *WebSocketClient {
	return &WebSocketClient{
		URL: url,
	}
}

// Connect establishes the WebSocket connection
func (wsc *WebSocketClient) Connect() error {
	wsc.mu.Lock()
	defer wsc.mu.Unlock()

	conn, _, err := websocket.DefaultDialer.Dial(wsc.URL, nil)
	if err != nil {
		return err
	}

	wsc.conn = conn

	// Start message handler
	go wsc.handleMessages()

	return nil
}

// Disconnect closes the WebSocket connection
func (wsc *WebSocketClient) Disconnect() error {
	wsc.mu.Lock()
	defer wsc.mu.Unlock()

	if wsc.conn != nil {
		return wsc.conn.Close()
	}
	return nil
}

// Send sends a message through the WebSocket
func (wsc *WebSocketClient) Send(message string) error {
	wsc.mu.Lock()
	defer wsc.mu.Unlock()

	if wsc.conn == nil {
		return fmt.Errorf("not connected")
	}

	return wsc.conn.WriteMessage(websocket.TextMessage, []byte(message))
}

// OnMessage sets the message handler callback
func (wsc *WebSocketClient) OnMessage(callback func(string)) {
	wsc.onMessage = callback
}

func (wsc *WebSocketClient) handleMessages() {
	for {
		wsc.mu.Lock()
		conn := wsc.conn
		wsc.mu.Unlock()

		if conn == nil {
			break
		}

		_, message, err := conn.ReadMessage()
		if err != nil {
			break
		}

		if wsc.onMessage != nil {
			wsc.onMessage(string(message))
		}
	}
}

// Helper functions for JSON handling
func (c *Client) getJSON(endpoint string, v interface{}) error {
	req := fasthttp.AcquireRequest()
	resp := fasthttp.AcquireResponse()
	defer fasthttp.ReleaseRequest(req)
	defer fasthttp.ReleaseResponse(resp)

	req.SetRequestURI(c.BaseURL + endpoint)
	req.Header.SetMethod("GET")
	req.Header.Set("Content-Type", "application/json")

	err := c.FastClient.Do(req, resp)
	if err != nil {
		return err
	}

	if resp.StatusCode() != 200 {
		return fmt.Errorf("HTTP %d: %s", resp.StatusCode(), resp.Body())
	}

	return json.Unmarshal(resp.Body(), v)
}

func (c *Client) postJSON(endpoint string, data interface{}, v interface{}) error {
	var body []byte
	var err error

	if data != nil {
		body, err = json.Marshal(data)
		if err != nil {
			return err
		}
	}

	req := fasthttp.AcquireRequest()
	resp := fasthttp.AcquireResponse()
	defer fasthttp.ReleaseRequest(req)
	defer fasthttp.ReleaseResponse(resp)

	req.SetRequestURI(c.BaseURL + endpoint)
	req.Header.SetMethod("POST")
	req.Header.Set("Content-Type", "application/json")
	req.SetBody(body)

	err = c.FastClient.Do(req, resp)
	if err != nil {
		return err
	}

	if resp.StatusCode() != 200 {
		return fmt.Errorf("HTTP %d: %s", resp.StatusCode(), resp.Body())
	}

	if v != nil {
		return json.Unmarshal(resp.Body(), v)
	}

	return nil
}

func getString(m map[string]interface{}, key string) string {
	if val, ok := m[key]; ok {
		if str, ok := val.(string); ok {
			return str
		}
	}
	return ""
}

func getInt(m map[string]interface{}, key string) int {
	if val, ok := m[key]; ok {
		if num, ok := val.(float64); ok {
			return int(num)
		}
	}
	return 0
}
