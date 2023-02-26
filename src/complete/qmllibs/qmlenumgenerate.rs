use super::QmlEnum;
pub(super) fn get_enum_from_source<S: ToString>(content: S, node: tree_sitter::Node) -> QmlEnum {
    let mut name = String::new();
    let mut values = Vec::new();
    let source = &content.to_string();
    let newsource: Vec<&str> = source.lines().collect();
    let mut course = node.walk();
    for child in node.children(&mut course) {
        if child.kind() == "ui_binding" {
            let id = child.child_by_field_name("name").unwrap();
            let row = id.start_position().row;
            let start_x = id.start_position().column;
            let start_y = id.end_position().column;
            let idname = newsource[row][start_x..start_y].to_string();
            match idname.as_str() {
                "name" => {
                    let value = child
                        .child_by_field_name("value")
                        .unwrap()
                        .child(0)
                        .unwrap()
                        .child(1)
                        .unwrap();
                    let row = value.start_position().row;
                    let start_x = value.start_position().column;
                    let start_y = value.end_position().column;
                    name = newsource[row][start_x..start_y].to_string();
                }
                "values" => {
                    let value = child
                        .child_by_field_name("value")
                        .unwrap()
                        .child(0)
                        .unwrap();
                    values = get_enum_values_from_source(content.to_string(), value);
                }
                _ => {}
            }
        }
    }
    QmlEnum { name, values }
}

fn get_enum_values_from_source<S: ToString>(
    content: S,
    node: tree_sitter::Node,
) -> Vec<(String, i32)> {
    let source = &content.to_string();
    let newsource: Vec<&str> = source.lines().collect();
    let mut course = node.walk();
    let mut enumvalues = Vec::new();
    for child in node.children(&mut course) {
        if child.kind() == "pair" {
            let key = child.child_by_field_name("key").unwrap().child(1).unwrap();
            let row = key.start_position().row;
            let start_x = key.start_position().column;
            let start_y = key.end_position().column;
            let key_name = newsource[row][start_x..start_y].to_string();

            let value = child.child_by_field_name("value").unwrap();
            let row = value.start_position().row;
            let start_x = value.start_position().column;
            let start_y = value.end_position().column;
            let value_name = newsource[row][start_x..start_y].to_string();

            enumvalues.push((key_name, value_name.parse().unwrap()))
        }
    }
    enumvalues
}
