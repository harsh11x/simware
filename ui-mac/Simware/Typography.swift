import SwiftUI

struct SimwareTypography {
    static func display() -> Font { .system(size: 32, weight: .semibold, design: .default) }
    static func h1() -> Font { .system(size: 24, weight: .semibold, design: .default) }
    static func h2() -> Font { .system(size: 20, weight: .semibold, design: .default) }
    
    static func bodyLg() -> Font { .system(size: 16, weight: .regular, design: .default) }
    static func bodyMd() -> Font { .system(size: 14, weight: .regular, design: .default) }
    static func bodySm() -> Font { .system(size: 12, weight: .regular, design: .default) }
    
    static func labelMd() -> Font { .system(size: 12, weight: .medium, design: .default) }
    
    static func codeMd() -> Font { .system(size: 13, weight: .regular, design: .monospaced) }
}
