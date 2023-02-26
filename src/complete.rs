mod qmllibs;
pub use qmllibs::{update_modules, update_root_dir};
use crate::cppregister;
use crate::treehelper;
use lsp_types::CompletionResponse;
use lsp_types::MessageType;
use lsp_types::Position;
use lsp_types::{CompletionItem, CompletionItemKind};

pub async fn getcomplete(
    source: &str,
    location: Position,
    client: &tower_lsp::Client,
    triggertype: Option<String>,
) -> Option<CompletionResponse> {
    client.log_message(MessageType::INFO, "Complete").await;
    let mut parse = tree_sitter::Parser::new();
    parse.set_language(tree_sitter_qmljs::language()).unwrap();
    let thetree = parse.parse(source, None);
    let tree = thetree.unwrap();
    match triggertype {
        Some(_) => {
            if location.character > 1 {
                let character = location.character - 2;
                let line = location.line;
                if let Some(tomatch) = treehelper::get_positon_string(
                    Position { line, character },
                    tree.root_node(),
                    source,
                ) {
                    return get_id_complete(tree.root_node(), source, &tomatch);
                }
            }
            None
        }
        None => {
            let mut completes = vec![];
            let data = cppregister::GLOBAL_DATA.lock().await;
            for da in data.iter() {
                completes.push(CompletionItem {
                    label: da.name.clone(),
                    kind: Some(CompletionItemKind::VARIABLE),
                    detail: Some("defined variable".to_string()),
                    ..Default::default()
                });
            }
            if let Some(mut subbasecomplete) = getsubbasecomplete(tree.root_node(), source) {
                completes.append(&mut subbasecomplete);
            }
            Some(CompletionResponse::Array(completes))
            //getbasecoplete(tree.root_node(), source),
        }
    }
}

/// get the variable from the loop
#[allow(dead_code)]
fn getsubbasecomplete(input: tree_sitter::Node, source: &str) -> Option<Vec<CompletionItem>> {
    let newsource: Vec<&str> = source.lines().collect();
    let mut course = input.walk();
    //let mut course2 = course.clone();
    //let mut hasid = false;
    let mut complete: Vec<CompletionItem> = vec![];
    for child in input.children(&mut course) {
        if child.kind() == "ui_object_definition" {
            let child = child.child_by_field_name("initializer").unwrap();
            if let Some(mut completes) = getsubbasecomplete(child, source) {
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
                            label: idname.to_string(),
                            kind: Some(CompletionItemKind::VARIABLE),
                            detail: Some("defined variable".to_string()),
                            ..Default::default()
                        });
                    }
                }
            }
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
    if input.kind() == "ui_object_definition" {
        let child = input.child_by_field_name("initializer").unwrap();
        //if let Some(completes) = get_id_complete(child, source, tosearch) {
        //    return Some(completes);
        //};
        let mut neocursor = child.walk();
        //let neochild = child.children_by_field_name("ui_binding", &mut neocursor);
        let mut finded = false;

        for neochild in child.children(&mut neocursor) {
            //if let "ui_binding" = neochild.kind() {
            match neochild.kind() {
                "ui_binding" => {
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
                "function_declaration" => {
                    let temp = neochild.child_by_field_name("name").unwrap();
                    let h = temp.start_position().row;
                    let x = temp.start_position().column;
                    let y = temp.end_position().column;
                    let identerfy = &newsource[h][x..y];
                    complete.push(CompletionItem {
                        label: format!("{}()", identerfy),
                        kind: Some(CompletionItemKind::FUNCTION),
                        detail: Some("defined function".to_string()),
                        ..Default::default()
                    });
                }
                _ => {}
            }
        }
        if finded {
            return Some(CompletionResponse::Array(complete));
        } else {
            let mut course = child.walk();
            let children = child.children(&mut course);
            for achild in children {
                if achild.child_count() != 0 && child.kind() != "ERROR" {
                    let output = get_id_sub_complete(achild, source, tosearch);
                    if output.is_some() {
                        return output;
                    }
                }
            }
        }
    }
    None
}
