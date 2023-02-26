use super::QmlProperty;
pub fn get_property_from_source<S: ToString>(content: S, node: tree_sitter::Node) -> QmlProperty {
    let source = &content.to_string();
    let newsource: Vec<&str> = source.lines().collect();
    let mut course = node.walk();

    let mut name = String::new();
    let mut qmltype = String::new();
    let mut isreadonly = false;
    let mut ispointer = false;

    for child in node.children(&mut course) {
        if child.kind() == "ui_binding" {
            let id = child.child_by_field_name("name").unwrap();
            let row = id.start_position().row;
            let start_x = id.start_position().column;
            let start_y = id.end_position().column;
            let idname = newsource[row][start_x..start_y].to_string();
            let value = get_value_from_statement(source.clone(), child);
            match idname.as_str() {
                "name" => {
                    name = value.split('"').collect::<Vec<&str>>()[1].to_string();
                }
                "type" => {
                    qmltype = value.split('"').collect::<Vec<&str>>()[1].to_string();
                }
                "isPointer" => {
                    ispointer = value.as_str() == "true";
                }
                "isReadonly" => {
                    isreadonly = value.as_str() == "true";
                }
                _ => {}
            }
        }
    }
    QmlProperty {
        name,
        qmltype,
        ispointer,
        isreadonly,
    }
}

fn get_value_from_statement(content: String, node: tree_sitter::Node) -> String {
    let newsource: Vec<&str> = content.lines().collect();
    let value = node.child_by_field_name("value").unwrap().child(0).unwrap();
    let row = value.start_position().row;
    let start_x = value.start_position().column;
    let start_y = value.end_position().column;
    newsource[row][start_x..start_y].to_string()
}
