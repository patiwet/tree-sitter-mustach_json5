// swift-tools-version:5.3

import Foundation
import PackageDescription

var sources = ["src/parser.c"]
if FileManager.default.fileExists(atPath: "src/scanner.c") {
    sources.append("src/scanner.c")
}

let package = Package(
    name: "TreeSitterMustacheJson5",
    products: [
        .library(name: "TreeSitterMustacheJson5", targets: ["TreeSitterMustacheJson5"]),
    ],
    dependencies: [
        .package(url: "https://github.com/tree-sitter/swift-tree-sitter", from: "0.8.0"),
    ],
    targets: [
        .target(
            name: "TreeSitterMustacheJson5",
            dependencies: [],
            path: ".",
            sources: sources,
            resources: [
                .copy("queries")
            ],
            publicHeadersPath: "bindings/swift",
            cSettings: [.headerSearchPath("src")]
        ),
        .testTarget(
            name: "TreeSitterMustacheJson5Tests",
            dependencies: [
                "SwiftTreeSitter",
                "TreeSitterMustacheJson5",
            ],
            path: "bindings/swift/TreeSitterMustacheJson5Tests"
        )
    ],
    cLanguageStandard: .c11
)
