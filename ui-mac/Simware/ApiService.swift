import Foundation

struct Analysis: Codable, Identifiable {
    let id: String
    let fileHash: String?
    let fileName: String?
    let status: String
    let aiRiskScore: Double?
    let finalDecision: String?

    enum CodingKeys: String, CodingKey {
        case id
        case fileHash = "file_hash"
        case fileName = "file_name"
        case status
        case aiRiskScore = "ai_risk_score"
        case finalDecision = "final_decision"
    }
}

struct DashboardStats: Codable {
    let totalAnalyzed: Int
    let threatsBlocked: Int
    let avgAnalysisTime: String
    let recentActivity: [Analysis]
}

struct ManualScanResponse: Codable {
    let id: String
    let status: String
    let message: String
}

class ApiService: ObservableObject {
    private let baseURL = "http://localhost:8000/api/v1"
    private var webSocketTask: URLSessionWebSocketTask?
    
    @Published var liveTelemetry: [String] = []
    @Published var liveStatus: String = "Waiting..."
    @Published var liveRiskScore: Double = 0.0
    @Published var finalDecision: String = ""

    func getAnalysis(id: String) async throws -> Analysis {
        guard let url = URL(string: "\\(baseURL)/analysis/\\(id)") else { throw URLError(.badURL) }
        let (data, response) = try await URLSession.shared.data(from: url)
        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else { throw URLError(.badServerResponse) }
        return try JSONDecoder().decode(Analysis.self, from: data)
    }

    func getStats() async throws -> DashboardStats {
        guard let url = URL(string: "\\(baseURL)/stats") else { throw URLError(.badURL) }
        let (data, response) = try await URLSession.shared.data(from: url)
        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else { throw URLError(.badServerResponse) }
        return try JSONDecoder().decode(DashboardStats.self, from: data)
    }

    func searchAnalyses(query: String) async throws -> [Analysis] {
        guard let encodedQuery = query.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed),
              let url = URL(string: "\\(baseURL)/search?q=\\(encodedQuery)") else { throw URLError(.badURL) }
        let (data, response) = try await URLSession.shared.data(from: url)
        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else { throw URLError(.badServerResponse) }
        return try JSONDecoder().decode([Analysis].self, from: data)
    }
    
    func exportReport(id: String) {
        guard let url = URL(string: "\\(baseURL)/reports/\\(id)") else { return }
        NSWorkspace.shared.open(url)
    }

    func triggerManualScan(fileName: String) async throws -> String {
        guard let url = URL(string: "\\(baseURL)/scan/manual") else { throw URLError(.badURL) }
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        let body = ["file_name": fileName]
        request.httpBody = try JSONSerialization.data(withJSONObject: body)
        
        let (data, response) = try await URLSession.shared.data(for: request)
        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else { throw URLError(.badServerResponse) }
        
        let res = try JSONDecoder().decode(ManualScanResponse.self, from: data)
        return res.id
    }

    func connectWebSocket() {
        guard let url = URL(string: "ws://localhost:8000/stream") else { return }
        webSocketTask = URLSession.shared.webSocketTask(with: url)
        webSocketTask?.resume()
        receiveMessage()
    }

    private func receiveMessage() {
        webSocketTask?.receive { [weak self] result in
            switch result {
            case .failure(let error):
                print("WebSocket error: \\(error)")
            case .success(let message):
                switch message {
                case .string(let text):
                    self?.handleWebSocketMessage(text)
                case .data(_):
                    break
                @unknown default:
                    break
                }
                self?.receiveMessage()
            }
        }
    }

    private func handleWebSocketMessage(_ jsonString: String) {
        guard let data = jsonString.data(using: .utf8),
              let json = try? JSONSerialization.jsonObject(with: data, options: []) as? [String: Any],
              let topic = json["topic"] as? String,
              let payload = json["data"] as? [String: Any] else { return }

        DispatchQueue.main.async {
            switch topic {
            case "telemetry_log":
                if let log = payload["log"] as? String {
                    self.liveTelemetry.append(log)
                }
            case "analysis_start":
                self.liveStatus = "Analyzing..."
                self.liveTelemetry.removeAll()
            case "analysis_complete":
                self.liveStatus = "Complete"
                if let riskScore = payload["riskScore"] as? Double {
                    self.liveRiskScore = riskScore
                }
                if let decision = payload["decision"] as? String {
                    self.finalDecision = decision
                }
            default:
                break
            }
        }
    }
}
