using System;
using System.Net.Http;
using System.Net.WebSockets;
using System.Text;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using Simware.Models;

namespace Simware.Services
{
    public class ApiService
    {
        private readonly HttpClient _client;
        private ClientWebSocket _webSocket;
        private const string BaseUrl = "http://localhost:8000/api/v1";
        private const string WsUrl = "ws://localhost:8000/stream";

        public event Action<string> OnTelemetryReceived;
        public event Action<string, double, string> OnAnalysisComplete;
        public event Action OnAnalysisStart;

        public ApiService()
        {
            _client = new HttpClient();
        }

        public async Task<Analysis> GetAnalysisAsync(string analysisId)
        {
            try
            {
                var response = await _client.GetAsync($"{BaseUrl}/analysis/{analysisId}");
                response.EnsureSuccessStatusCode();
                var content = await response.Content.ReadAsStringAsync();
                return JsonSerializer.Deserialize<Analysis>(content);
            }
            catch (Exception ex) { Console.WriteLine(ex.Message); return null; }
        }

        public async Task<DashboardStats> GetStatsAsync()
        {
            try
            {
                var response = await _client.GetAsync($"{BaseUrl}/stats");
                response.EnsureSuccessStatusCode();
                var content = await response.Content.ReadAsStringAsync();
                return JsonSerializer.Deserialize<DashboardStats>(content);
            }
            catch (Exception ex) { Console.WriteLine(ex.Message); return null; }
        }

        public async Task<System.Collections.Generic.List<Analysis>> SearchAnalysesAsync(string query)
        {
            try
            {
                var response = await _client.GetAsync($"{BaseUrl}/search?q={Uri.EscapeDataString(query)}");
                response.EnsureSuccessStatusCode();
                var content = await response.Content.ReadAsStringAsync();
                return JsonSerializer.Deserialize<System.Collections.Generic.List<Analysis>>(content);
            }
            catch (Exception ex) { Console.WriteLine(ex.Message); return new System.Collections.Generic.List<Analysis>(); }
        }

        public void ExportReport(string id)
        {
            System.Diagnostics.Process.Start(new System.Diagnostics.ProcessStartInfo
            {
                FileName = $"{BaseUrl}/reports/{id}",
                UseShellExecute = true
            });
        }

        public async Task<string> TriggerManualScanAsync(string fileName)
        {
            try
            {
                var json = JsonSerializer.Serialize(new { file_name = fileName });
                var data = new StringContent(json, Encoding.UTF8, "application/json");
                var response = await _client.PostAsync($"{BaseUrl}/scan/manual", data);
                response.EnsureSuccessStatusCode();
                var content = await response.Content.ReadAsStringAsync();
                var res = JsonSerializer.Deserialize<ManualScanResponse>(content);
                return res?.Id;
            }
            catch (Exception ex) { Console.WriteLine(ex.Message); return null; }
        }

        public async Task ConnectWebSocketAsync()
        {
            try
            {
                _webSocket = new ClientWebSocket();
                await _webSocket.ConnectAsync(new Uri(WsUrl), CancellationToken.None);
                
                _ = ReceiveLoopAsync();
            }
            catch (Exception ex)
            {
                Console.WriteLine($"WebSocket Connection Error: {ex.Message}");
            }
        }

        private async Task ReceiveLoopAsync()
        {
            var buffer = new byte[4096];
            while (_webSocket.State == WebSocketState.Open)
            {
                var result = await _webSocket.ReceiveAsync(new ArraySegment<byte>(buffer), CancellationToken.None);
                if (result.MessageType == WebSocketMessageType.Close)
                {
                    await _webSocket.CloseAsync(WebSocketCloseStatus.NormalClosure, string.Empty, CancellationToken.None);
                }
                else
                {
                    var message = Encoding.UTF8.GetString(buffer, 0, result.Count);
                    HandleWebSocketMessage(message);
                }
            }
        }

        private void HandleWebSocketMessage(string message)
        {
            try
            {
                using (JsonDocument doc = JsonDocument.Parse(message))
                {
                    var root = doc.RootElement;
                    var topic = root.GetProperty("topic").GetString();
                    var data = root.GetProperty("data");

                    if (topic == "telemetry_log")
                    {
                        var log = data.GetProperty("log").GetString();
                        OnTelemetryReceived?.Invoke(log);
                    }
                    else if (topic == "analysis_start")
                    {
                        OnAnalysisStart?.Invoke();
                    }
                    else if (topic == "analysis_complete")
                    {
                        var decision = data.GetProperty("decision").GetString();
                        var riskScore = data.GetProperty("riskScore").GetDouble();
                        var explanation = data.GetProperty("explanation").GetString();
                        OnAnalysisComplete?.Invoke(decision, riskScore, explanation);
                    }
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error parsing WS message: {ex.Message}");
            }
        }
    }
}
