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

    public class DashboardStats
    {
        [JsonPropertyName("totalAnalyzed")]
        public int TotalAnalyzed { get; set; }

        [JsonPropertyName("threatsBlocked")]
        public int ThreatsBlocked { get; set; }

        [JsonPropertyName("avgAnalysisTime")]
        public string AvgAnalysisTime { get; set; }

        [JsonPropertyName("recentActivity")]
        public System.Collections.Generic.List<Analysis> RecentActivity { get; set; }
    }

    public class ManualScanResponse
    {
        [JsonPropertyName("id")]
        public string Id { get; set; }

        [JsonPropertyName("status")]
        public string Status { get; set; }

        [JsonPropertyName("message")]
        public string Message { get; set; }
    }
}
