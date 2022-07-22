/// about the gammers
use crate::CompletionResponse;
use lsp_types::{CompletionItem, CompletionItemKind};
/// checkerror the gammer error
/// if there is error , it will return the position of the error
pub fn checkerror(
    input: tree_sitter::Node,
) -> Option<Vec<(tree_sitter::Point, tree_sitter::Point)>> {
    if input.has_error() {
        if input.is_error() {
            Some(vec![(input.start_position(), input.end_position())])
        } else {
            let mut course = input.walk();
            {
                let mut output = vec![];
                for node in input.children(&mut course) {
                    if let Some(mut tran) = checkerror(node) {
                        output.append(&mut tran);
                    }
                }
                if output.is_empty() {
                    None
                } else {
                    Some(output)
                }
            }
        }
    } else {
        None
    }
}
/// get the complet messages
#[allow(dead_code)]
pub fn getcoplete(input: tree_sitter::Node, source: &str) -> Option<CompletionResponse> {
    let newsource: Vec<&str> = source.lines().collect();
    let mut course = input.walk();
    //let mut course2 = course.clone();
    //let mut hasid = false;
    let mut complete: Vec<CompletionItem> = vec![];
    for child in input.children(&mut course) {
        match child.kind() {
            "ui_object_definition" => {
                let child = child.child_by_field_name("initializer").unwrap();
                if let Some(mut completes) = getsubcoplete(child, source) {
                    complete.append(&mut completes);
                };
                let mut course = child.walk();
                for neochild in child.children(&mut course) {
                    if let "ui_binding" = neochild.kind() {
                        let temp = neochild.child_by_field_name("name").unwrap();
                        let h = temp.start_position().row;
                        let x = temp.start_position().column;
                        let y = temp.end_position().column;
                        let identerfy = &newsource[h][x..y];
                        if identerfy == "id" {
                            let name = neochild.child_by_field_name("value").unwrap();
                            let h = name.start_position().row;
                            let x = name.start_position().column;
                            let y = name.end_position().column;
                            let idname = &newsource[h][x..y];
                            complete.push(CompletionItem {
                                label: format!("{}", idname),
                                kind: Some(CompletionItemKind::VARIABLE),
                                detail: Some("defined variable".to_string()),
                                ..Default::default()
                            });
                        }
                    }
                }
            }

            _ => {}
        }
    }

    if complete.is_empty() {
        None
    } else {
        Some(CompletionResponse::Array(complete))
    }
}
/// get the variable from the loop
#[allow(dead_code)]
fn getsubcoplete(input: tree_sitter::Node, source: &str) -> Option<Vec<CompletionItem>> {
    let newsource: Vec<&str> = source.lines().collect();
    let mut course = input.walk();
    //let mut course2 = course.clone();
    //let mut hasid = false;
    let mut complete: Vec<CompletionItem> = vec![];
    for child in input.children(&mut course) {
        match child.kind() {
            "ui_object_definition" => {
                let child = child.child_by_field_name("initializer").unwrap();
                if let Some(mut completes) = getsubcoplete(child, source) {
                    complete.append(&mut completes);
                };
                let mut course = child.walk();
                for neochild in child.children(&mut course) {
                    if let "ui_binding" = neochild.kind() {
                        let temp = neochild.child_by_field_name("name").unwrap();
                        let h = temp.start_position().row;
                        let x = temp.start_position().column;
                        let y = temp.end_position().column;
                        let identerfy = &newsource[h][x..y];
                        if identerfy == "id" {
                            let name = neochild.child_by_field_name("value").unwrap();
                            let h = name.start_position().row;
                            let x = name.start_position().column;
                            let y = name.end_position().column;
                            let idname = &newsource[h][x..y];
                            complete.push(CompletionItem {
                                label: format!("{}", idname),
                                kind: Some(CompletionItemKind::VARIABLE),
                                detail: Some("defined variable".to_string()),
                                ..Default::default()
                            });
                        }
                    }
                }
            }

            _ => {}
        }
    }

    if complete.is_empty() {
        None
    } else {
        Some(complete)
    }
}
#[allow(dead_code)]
pub fn get_id_complete(
    input: tree_sitter::Node,
    source: &str,
    tosearch: &str,
) -> Option<CompletionResponse> {
    //let mut course2 = course.clone();
    //let mut hasid = false;
    match input.child_by_field_name("root") {
        Some(child) => get_id_sub_complete(child, source, tosearch),
        None => None,
    }
}
#[allow(dead_code)]
pub fn get_id_sub_complete(
    input: tree_sitter::Node,
    source: &str,
    tosearch: &str,
) -> Option<CompletionResponse> {
    let newsource: Vec<&str> = source.lines().collect();
    //let mut course2 = course.clone();
    //let mut hasid = false;
    let mut complete: Vec<CompletionItem> = vec![];
    match input.kind() {
        "ui_object_definition" => {
            let child = input.child_by_field_name("initializer").unwrap();
            //if let Some(completes) = get_id_complete(child, source, tosearch) {
            //    return Some(completes);
            //};
            let mut neocursor = child.walk();
            //let neochild = child.children_by_field_name("ui_binding", &mut neocursor);
            let mut finded = false;

            for neochild in child.children(&mut neocursor) {
                if let "ui_binding" = neochild.kind() {
                    let temp = neochild.child_by_field_name("name").unwrap();
                    let h = temp.start_position().row;
                    let x = temp.start_position().column;
                    let y = temp.end_position().column;
                    let identerfy = &newsource[h][x..y];
                    if identerfy == "id" {
                        let value = neochild.child_by_field_name("value").unwrap();
                        let h = value.start_position().row;
                        let x = value.start_position().column;
                        let y = value.end_position().column;
                        let name = &newsource[h][x..y];
                        if name == tosearch {
                            finded = true;
                        } else {
                            break;
                        }
                    } else {
                        complete.push(CompletionItem {
                            label: identerfy.to_string(),
                            kind: Some(CompletionItemKind::VARIABLE),
                            detail: Some("defined variable".to_string()),
                            ..Default::default()
                        });
                    }
                }
            }
            if finded {
                return Some(CompletionResponse::Array(complete));
            } else {
                let mut course = child.walk();
                let children = child.children(&mut course);
                for achild in children.into_iter() {
                    if achild.child_count() != 0 && child.kind() != "ERROR"{
                        let output = get_id_sub_complete(achild, source, tosearch);
                        if output.is_some() {
                            return output;
                        }
                    }
                }
            }
        }

        _ => {}
    }
    None
}
