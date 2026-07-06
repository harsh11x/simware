import SwiftUI

struct AnalysisWorkspaceView: View {
    @StateObject private var apiService = ApiService()
    let activeAnalysisId: String
    @State private var analysis: Analysis?
    
    var codeOutput: String {
        return apiService.liveTelemetry.isEmpty ? "[Waiting for telemetry...]" : apiService.liveTelemetry.joined(separator: "\n")
    }
    
    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 24) {
                // Header
                HStack {
                    Text("Analysis Workspace")
                        .font(SimwareTypography.display())
                        .foregroundColor(.simwareTextPrimary)
                    
                    Spacer()
                    
                    SimwareButton(title: "Terminate Sandbox", action: {}, isPrimary: false)
                }
                
                // Sandbox Status Bar
                SimwareCard {
                    HStack {
                        Image(systemName: "server.rack")
                            .foregroundColor(.simwareTextSecondary)
                        Text("Active Sandbox: vm-windows-10-testbed")
                            .font(SimwareTypography.bodyMd())
                            .foregroundColor(.simwareTextPrimary)
                        
                        Spacer()
                        
                        StatusChip(text: "ISOLATED", color: .simwareSuccess)
                        StatusChip(text: "CPU: 45%", color: .simwareTextSecondary)
                        StatusChip(text: "RAM: 2.4GB", color: .simwareTextSecondary)
                    }
                }
                
                // Telemetry Streams
                HStack(alignment: .top, spacing: 24) {
                    // Left Column (Process Tree & Network)
                    VStack(spacing: 24) {
                        SimwareCard(accentColor: .simwarePrimary) {
                            VStack(alignment: .leading, spacing: 16) {
                                Text("Process Tree")
                                    .font(SimwareTypography.h2())
                                    .foregroundColor(.simwareTextPrimary)
                                
                                Text("explorer.exe (PID: 2341)\n ↳ installer.exe (PID: 4402)\n    ↳ powershell.exe (PID: 4410)")
                                    .font(SimwareTypography.codeMd())
                                    .foregroundColor(.simwareTextSecondary)
                            }
                        }
                        
                        SimwareCard(accentColor: .simwareWarning) {
                            VStack(alignment: .leading, spacing: 16) {
                                Text("Network Activity")
                                    .font(SimwareTypography.h2())
                                    .foregroundColor(.simwareTextPrimary)
                                
                                Text("TCP 10.0.0.5:54323 -> 185.12.X.X:443 (Suspicious IP)")
                                    .font(SimwareTypography.codeMd())
                                    .foregroundColor(.simwareTextSecondary)
                            }
                        }
                    }
                    .frame(maxWidth: .infinity)
                    
                    // Right Column (Live Log)
                    VStack(spacing: 8) {
                        Text("LIVE TELEMETRY")
                            .font(SimwareTypography.labelMd())
                            .foregroundColor(.simwareTextSecondary)
                            .frame(maxWidth: .infinity, alignment: .leading)
                        
                        CodeBlock(code: codeOutput)
                    }
                    .frame(maxWidth: .infinity)
                }
                
                Spacer()
            }
            .padding(32)
        }
        .onAppear {
            apiService.connectWebSocket()
            if !activeAnalysisId.isEmpty {
                Task {
                    analysis = try? await apiService.getAnalysis(id: activeAnalysisId)
                }
            }
        }
    }
}
