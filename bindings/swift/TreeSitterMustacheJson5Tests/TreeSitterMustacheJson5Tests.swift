import XCTest
import SwiftTreeSitter
import TreeSitterMustacheJson5

final class TreeSitterMustacheJson5Tests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_mustache_json5())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading MustacheJson5 grammar")
    }
}
