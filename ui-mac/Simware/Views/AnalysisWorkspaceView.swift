import SwiftUI

struct AnalysisWorkspaceView: View {
    @StateObject private var apiService = ApiService()
    @State private var activeAnalysisId = "1f2780d9-1307-46ba-b110-a79d407f4392"
    @State private var latestAnalysis: Analysis? = nil
    let timer = Timer.publish(every: 2, on: .main, in: .common).autoconnect()
    
    @State private var codeOutput = """
    [10:42:01] Process created: C:\\Windows\\System32\\cmd.exe
    [10:42:02] Network connection attempt to 192.168.1.104:443
    [10:42:05] File write blocked: C:\\Users\\Public\\malware.exe
    [10:42:08] Injection detected in lsass.exe
    """
    
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
        .onReceive(timer) { _ in
            Task {
                do {
                    let analysis = try await apiService.getAnalysis(id: activeAnalysisId)
                    self.latestAnalysis = analysis
                    // In a real implementation we would update the UI state with analysis.aiRiskScore, etc.
                } catch {
                    print("Failed to fetch analysis: \\(error)")
                }
            }
        }
    }
}
