using System;
using System.Text.Json.Serialization;

namespace Simware.Models
{
    public class Analysis
    {
        [JsonPropertyName("id")]
        public string Id { get; set; }

        [JsonPropertyName("file_hash")]
        public string FileHash { get; set; }

        [JsonPropertyName("file_name")]
        public string FileName { get; set; }

        [JsonPropertyName("status")]
        public string Status { get; set; }

        [JsonPropertyName("ai_risk_score")]
        public double? AiRiskScore { get; set; }

        [JsonPropertyName("final_decision")]
        public string FinalDecision { get; set; }
    }
}
