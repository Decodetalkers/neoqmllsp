//use crate::treehelper::point_to_position;
use lsp_types::{DocumentSymbol, DocumentSymbolResponse, SymbolKind};
#[allow(deprecated)]
pub fn getast(input: tree_sitter::Node, source: &str) -> Option<DocumentSymbolResponse> {
    getsubast(input, source).map(DocumentSymbolResponse::Nested)
}
#[allow(deprecated)]
fn getsubast(input: tree_sitter::Node, source: &str) -> Option<Vec<DocumentSymbol>> {
    let newsource: Vec<&str> = source.lines().collect();
    let mut course = input.walk();
    let mut asts: Vec<DocumentSymbol> = vec![];
    for child in input.children(&mut course) {
        match child.kind() {
            "ui_object_initializer" => {
                if let Some(mut messages) = getsubast(child, source) {
                    asts.append(&mut messages);
                }
            }
            "function_declaration" => {
                let newchild = child.child_by_field_name("name").unwrap();
                let h = newchild.start_position().row;
                let x = newchild.start_position().column;
                let y = newchild.end_position().column;
                let name = (newsource[h][x..y]).to_string();
                asts.push(DocumentSymbol {
                    name,
                    detail: None,
                    kind: SymbolKind::FUNCTION,
                    tags: None,
                    deprecated: None,
                    range: lsp_types::Range {
                        start: lsp_types::Position {
                            line: child.start_position().row as u32,
                            character: child.start_position().column as u32,
                        },
                        end: lsp_types::Position {
                            line: child.end_position().row as u32,
                            character: child.end_position().column as u32,
                        },
                    },
                    selection_range: lsp_types::Range {
                        start: lsp_types::Position {
                            line: child.start_position().row as u32,
                            character: child.start_position().column as u32,
                        },
                        end: lsp_types::Position {
                            line: child.end_position().row as u32,
                            character: child.end_position().column as u32,
                        },
                    },
                    children: getsubast(child, source),
                });
            }
            "ui_object_definition" => {
                let newchild = child.child_by_field_name("type_name").unwrap();
                let h = newchild.start_position().row;
                let h2 = newchild.end_position().row;
                let x = newchild.start_position().column;
                let y = newchild.end_position().column;
                if h == h2 {
                    let name = (newsource[h][x..y]).to_string();
                    asts.push(DocumentSymbol {
                        name,
                        detail: None,
                        kind: SymbolKind::VARIABLE,
                        tags: None,
                        deprecated: None,
                        range: lsp_types::Range {
                            start: lsp_types::Position {
                                line: child.start_position().row as u32,
                                character: child.start_position().column as u32,
                            },
                            end: lsp_types::Position {
                                line: child.end_position().row as u32,
                                character: child.end_position().column as u32,
                            },
                        },
                        selection_range: lsp_types::Range {
                            start: lsp_types::Position {
                                line: child.start_position().row as u32,
                                character: child.start_position().column as u32,
                            },
                            end: lsp_types::Position {
                                line: child.end_position().row as u32,
                                character: child.end_position().column as u32,
                            },
                        },
                        children: getsubast(child, source),
                    });
                }
            }

            _ => {}
        }
    }

    if asts.is_empty() {
        None
    } else {
        Some(asts)
    }
}
