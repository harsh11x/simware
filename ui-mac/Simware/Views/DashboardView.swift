import SwiftUI

struct DashboardView: View {
    @State private var selectedTab: String? = "Dashboard"
    @State private var activeAnalysisId: String? = nil
    
    var body: some View {
        NavigationView {
            // Custom Sidebar
            VStack(alignment: .leading, spacing: 0) {
                // Header
                HStack {
                    Image(systemName: "shield.checkered")
                        .foregroundColor(.simwarePrimary)
                        .font(.system(size: 24, weight: .bold))
                    Text("Simware")
                        .font(SimwareTypography.h2())
                        .foregroundColor(.simwareTextPrimary)
                }
                .padding(.horizontal, 24)
                .padding(.vertical, 32)
                
                // Navigation Links
                VStack(spacing: 8) {
                    SidebarItem(icon: "chart.bar.fill", title: "Dashboard", isSelected: selectedTab == "Dashboard") {
                        selectedTab = "Dashboard"
                    }
                    SidebarItem(icon: "magnifyingglass", title: "Analysis Workspace", isSelected: selectedTab == "Workspace") {
                        selectedTab = "Workspace"
                    }
                    SidebarItem(icon: "doc.text.magnifyingglass", title: "Global Search", isSelected: selectedTab == "Search") {
                        selectedTab = "Search"
                    }
                    SidebarItem(icon: "lock.shield", title: "Quarantine", isSelected: selectedTab == "Quarantine") {
                        selectedTab = "Quarantine"
                    }
                }
                .padding(.horizontal, 16)
                
                Spacer()
                
                // Agent Status
                HStack(spacing: 12) {
                    Circle()
                        .fill(Color.simwareSuccess)
                        .frame(width: 8, height: 8)
                    Text("Agent Active • v1.0.4")
                        .font(SimwareTypography.bodySm())
                        .foregroundColor(.simwareTextSecondary)
                }
                .padding(24)
            }
            .frame(width: 240, alignment: .leading)
            .background(Color.simwareSurface)
            
            // Main Content Area
            ZStack {
                Color.simwareBackground.ignoresSafeArea()
                
                if selectedTab == "Dashboard" {
                    DashboardContent(selectedTab: $selectedTab, activeAnalysisId: $activeAnalysisId)
                } else if selectedTab == "Workspace" {
                    AnalysisWorkspaceView(activeAnalysisId: activeAnalysisId ?? "")
                } else if selectedTab == "Search" {
                    GlobalSearchView(selectedTab: $selectedTab, activeAnalysisId: $activeAnalysisId)
                } else {
                    Text("Coming Soon")
                        .font(SimwareTypography.h1())
                        .foregroundColor(.simwareTextSecondary)
                }
            }
        }
        .navigationViewStyle(DoubleColumnNavigationViewStyle())
    }
}

struct SidebarItem: View {
    let icon: String
    let title: String
    let isSelected: Bool
    let action: () -> Void
    
    var body: some View {
        Button(action: action) {
            HStack(spacing: 12) {
                Image(systemName: icon)
                    .font(.system(size: 16))
                    .foregroundColor(isSelected ? .simwarePrimary : .simwareTextSecondary)
                    .frame(width: 24)
                
                Text(title)
                    .font(SimwareTypography.bodyMd())
                    .foregroundColor(isSelected ? .simwareTextPrimary : .simwareTextSecondary)
                
                Spacer()
            }
            .padding(.horizontal, 12)
            .padding(.vertical, 10)
            .background(isSelected ? Color.simwareSurfaceLighter : Color.clear)
            .cornerRadius(8)
        }
        .buttonStyle(PlainButtonStyle())
    }
}

struct DashboardContent: View {
    @StateObject private var apiService = ApiService()
    @State private var stats: DashboardStats?
    @Binding var selectedTab: String?
    @Binding var activeAnalysisId: String?

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 32) {
                // Header
                HStack {
                    Text("Overview")
                        .font(SimwareTypography.display())
                        .foregroundColor(.simwareTextPrimary)
                    
                    Spacer()
                    
                    SimwareButton(title: "Manual Scan", action: {
                        Task {
                            if let id = try? await apiService.triggerManualScan(fileName: "manual_upload.exe") {
                                activeAnalysisId = id
                                selectedTab = "Workspace"
                            }
                        }
                    })
                }
                
                if let stats = stats {
                    // Metrics
                    HStack(spacing: 24) {
                        MetricCard(title: "Files Analyzed", value: "\\(stats.totalAnalyzed)", trend: "All time", trendColor: .simwareTextSecondary)
                        MetricCard(title: "Threats Blocked", value: "\\(stats.threatsBlocked)", trend: "All time", trendColor: .simwareDanger, isAlert: stats.threatsBlocked > 0)
                        MetricCard(title: "Avg Analysis Time", value: stats.avgAnalysisTime, trend: "Optimal", trendColor: .simwareSuccess)
                    }
                    
                    // Recent Activity
                    VStack(alignment: .leading, spacing: 16) {
                        Text("Recent Simulations")
                            .font(SimwareTypography.h2())
                            .foregroundColor(.simwareTextPrimary)
                        
                        SimwareCard {
                            VStack(spacing: 0) {
                                ForEach(Array(stats.recentActivity.enumerated()), id: \\.element.id) { index, activity in
                                    ActivityRow(
                                        filename: activity.fileName ?? "unknown",
                                        hash: String((activity.fileHash ?? "").prefix(12)) + "...",
                                        status: activity.finalDecision ?? activity.status,
                                        color: (activity.finalDecision == "BLOCK") ? .simwareDanger : (activity.status == "completed" ? .simwareSuccess : .simwareWarning),
                                        action: {
                                            activeAnalysisId = activity.id
                                            selectedTab = "Workspace"
                                        },
                                        exportAction: {
                                            apiService.exportReport(id: activity.id)
                                        }
                                    )
                                    if index < stats.recentActivity.count - 1 {
                                        Divider().background(Color.simwareBorder)
                                    }
                                }
                                if stats.recentActivity.isEmpty {
                                    Text("No recent activity.")
                                        .padding()
                                        .foregroundColor(.simwareTextSecondary)
                                }
                            }
                        }
                    }
                } else {
                    ProgressView("Loading Stats...")
                        .padding(32)
                }
                Spacer()
            }
            .padding(32)
            .onAppear {
                Task {
                    stats = try? await apiService.getStats()
                }
            }
        }
    }
}

struct ActivityRow: View {
    let filename: String
    let hash: String
    let status: String
    let color: Color
    
    var body: some View {
        HStack {
            Image(systemName: "doc.fill")
                .foregroundColor(.simwareTextSecondary)
            
            VStack(alignment: .leading, spacing: 4) {
                Text(filename)
                    .font(SimwareTypography.bodyMd())
                    .foregroundColor(.simwareTextPrimary)
                Text(hash)
                    .font(SimwareTypography.codeMd())
                    .foregroundColor(.simwareTextSecondary)
            }
            
            Spacer()
            
            StatusChip(text: status, color: color)
            
            SimwareButton(title: "View Details", action: {}, isPrimary: false)
                .padding(.leading, 16)
        }
        .padding(.vertical, 12)
    }
}
