import SwiftUI

struct GlobalSearchView: View {
    @State private var searchText: String = ""
    
    var body: some View {
        VStack(alignment: .leading, spacing: 24) {
            Text("Global Search")
                .font(SimwareTypography.display())
                .foregroundColor(.simwareTextPrimary)
            
            // Search Input
            HStack {
                Image(systemName: "magnifyingglass")
                    .foregroundColor(.simwareTextSecondary)
                
                TextField("Search by SHA-256 hash, filename, or IP address...", text: $searchText)
                    .font(SimwareTypography.bodyLg())
                    .textFieldStyle(PlainTextFieldStyle())
                    .foregroundColor(.simwareTextPrimary)
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
                        ActivityRow(filename: "search_result.exe", hash: "a1b2c3d4", status: "Clean", color: .simwareSuccess)
                            .padding(16)
                            .background(Color.simwareSurface)
                            .cornerRadius(12)
                            .overlay(RoundedRectangle(cornerRadius: 12).stroke(Color.simwareBorder, lineWidth: 1))
                    }
                }
            }
        }
        .padding(32)
    }
}
