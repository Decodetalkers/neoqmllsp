use super::{qmlparmatergenerate::get_parmater_from_source, QmlSignal};
pub(super) fn get_signal_from_source<S: ToString>(
    content: S,
    node: tree_sitter::Node,
) -> QmlSignal {
    let source = &content.to_string();
    let newsource: Vec<&str> = source.lines().collect();
    let mut course = node.walk();

    let mut name = String::new();
    let mut parmaters = Vec::new();
    for child in node.children(&mut course) {
        if child.kind() == "ui_binding" {
            let id = child.child_by_field_name("name").unwrap();
            let row = id.start_position().row;
            let start_x = id.start_position().column;
            let start_y = id.end_position().column;
            let idname = newsource[row][start_x..start_y].to_string();
            if idname == "name" {
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
        } else if child.kind() == "ui_object_definition" {
            let object_init = child.child_by_field_name("initializer").unwrap();
            parmaters.push(get_parmater_from_source(content.to_string(), object_init));
        }
    }
    QmlSignal { name, parmaters }
}
