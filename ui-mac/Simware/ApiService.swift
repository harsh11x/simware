import Foundation

struct Analysis: Codable {
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

class ApiService: ObservableObject {
    private let baseURL = "http://localhost:8000/api/v1"
    
    func getAnalysis(id: String) async throws -> Analysis {
        guard let url = URL(string: "\(baseURL)/analysis/\(id)") else {
            throw URLError(.badURL)
        }
        
        let (data, response) = try await URLSession.shared.data(from: url)
        
        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
            throw URLError(.badServerResponse)
        }
        
        return try JSONDecoder().decode(Analysis.self, from: data)
    }
}
