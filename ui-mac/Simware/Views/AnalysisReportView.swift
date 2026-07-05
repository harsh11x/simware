import SwiftUI

struct AnalysisReportView: View {
    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 32) {
                // Header
                HStack {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Analysis Report")
                            .font(SimwareTypography.display())
                            .foregroundColor(.simwareTextPrimary)
                        Text("invoice_update.pdf.exe")
                            .font(SimwareTypography.codeMd())
                            .foregroundColor(.simwareTextSecondary)
                    }
                    
                    Spacer()
                    
                    StatusChip(text: "MALICIOUS", color: .simwareDanger)
                        .scaleEffect(1.2)
                }
                
                // Explainer Card
                SimwareCard(accentColor: .simwareDanger) {
                    VStack(alignment: .leading, spacing: 16) {
                        Text("AI Summary")
                            .font(SimwareTypography.h2())
                            .foregroundColor(.simwareTextPrimary)
                        
                        Text("This file is highly malicious. It masquerades as a PDF document but is an executable. Upon execution, it attempts to inject code into a system process (lsass.exe) to dump credentials, and modifies registry keys to ensure persistence. It exhibits behaviors consistent with the AgentTesla malware family.")
                            .font(SimwareTypography.bodyLg())
                            .foregroundColor(.simwareTextSecondary)
                            .lineSpacing(4)
                    }
                }
                
                // MITRE ATT&CK Mapping
                VStack(alignment: .leading, spacing: 16) {
                    Text("MITRE ATT&CK Tactics")
                        .font(SimwareTypography.h2())
                        .foregroundColor(.simwareTextPrimary)
                    
                    HStack {
                        MitreTag(tactic: "T1055: Process Injection")
                        MitreTag(tactic: "T1547: Registry Run Keys")
                        MitreTag(tactic: "T1003: OS Credential Dumping")
                    }
                }
                
                Spacer()
            }
            .padding(32)
        }
    }
}

struct MitreTag: View {
    let tactic: String
    
    var body: some View {
        Text(tactic)
            .font(SimwareTypography.labelMd())
            .foregroundColor(.simwareDanger)
            .padding(.horizontal, 12)
            .padding(.vertical, 6)
            .overlay(
                RoundedRectangle(cornerRadius: 16)
                    .stroke(Color.simwareDanger.opacity(0.3), lineWidth: 1)
            )
            .background(Color.simwareDanger.opacity(0.1))
            .cornerRadius(16)
    }
}
