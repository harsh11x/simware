import SwiftUI

struct GlobalSearchView: View {
    @Binding var selectedTab: String?
    @Binding var activeAnalysisId: String?
    
    @State private var searchText: String = ""
    @State private var searchResults: [Analysis] = []
    @State private var isSearching = false
    @StateObject private var apiService = ApiService()
    
    var body: some View {
        VStack(alignment: .leading, spacing: 24) {
            Text("Global Search")
                .font(SimwareTypography.display())
                .foregroundColor(.simwareTextPrimary)
            
            // Search Input
            HStack {
                Image(systemName: "magnifyingglass")
                    .foregroundColor(.simwareTextSecondary)
                
                TextField("Search by SHA-256 hash or filename...", text: $searchText)
                    .font(SimwareTypography.bodyLg())
                    .textFieldStyle(PlainTextFieldStyle())
                    .foregroundColor(.simwareTextPrimary)
                    .onChange(of: searchText) { newValue in
                        performSearch(query: newValue)
                    }
                
                if isSearching {
                    ProgressView().scaleEffect(0.5)
                }
            }
            .padding(16)
            .background(Color.simwareBackground)
            .cornerRadius(8)
            .overlay(
                RoundedRectangle(cornerRadius: 8)
                    .stroke(Color.simwarePrimary, lineWidth: 1)
            )
            
            // Results Area
            if searchText.isEmpty {
                VStack(spacing: 16) {
                    Image(systemName: "magnifyingglass.circle")
                        .font(.system(size: 48))
                        .foregroundColor(.simwareTextSecondary)
                    Text("Enter a search term to find previous analyses.")
                        .font(SimwareTypography.bodyMd())
                        .foregroundColor(.simwareTextSecondary)
                }
                .frame(maxWidth: .infinity, maxHeight: .infinity)
            } else {
                ScrollView {
                    VStack(spacing: 16) {
                        ForEach(searchResults) { result in
                            ActivityRow(
                                filename: result.fileName ?? "unknown",
                                hash: result.fileHash ?? "",
                                status: result.finalDecision ?? result.status,
                                color: (result.finalDecision == "BLOCK") ? .simwareDanger : (result.status == "completed" ? .simwareSuccess : .simwareWarning),
                                action: {
                                    activeAnalysisId = result.id
                                    selectedTab = "Workspace"
                                },
                                exportAction: {
                                    apiService.exportReport(id: result.id)
                                }
                            )
                            .padding(16)
                            .background(Color.simwareSurface)
                            .cornerRadius(12)
                            .overlay(RoundedRectangle(cornerRadius: 12).stroke(Color.simwareBorder, lineWidth: 1))
                        }
                        
                        if searchResults.isEmpty && !isSearching {
                            Text("No results found.")
                                .foregroundColor(.simwareTextSecondary)
                                .padding()
                        }
                    }
                }
            }
        }
        .padding(32)
    }
    
    private func performSearch(query: String) {
        guard !query.isEmpty else {
            searchResults = []
            return
        }
        isSearching = true
        Task {
            do {
                searchResults = try await apiService.searchAnalyses(query: query)
            } catch {
                print("Search failed: \\(error)")
            }
            isSearching = false
    }
}
