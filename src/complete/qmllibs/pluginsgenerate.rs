#[derive(Debug)]
pub struct QmlModule {
    pub plugin_location: String,
    pub module_namespace: Option<String>,
    pub dependencies: Vec<String>,
    pub components: Vec<QmlComponent>,
}

#[derive(Debug)]
pub struct QmlComponent {
    pub name: String,
    pub prototypes: Option<String>,
    pub exports: Option<Vec<ModuleExport>>,
    pub qmlenums: Vec<QmlEnum>,
    pub creatable: bool,
    pub properties: Vec<QmlProperty>,
    pub signals: Vec<QmlSignal>,
    pub methods: Vec<QmlMethod>,
}

#[derive(Debug)]
pub struct ModuleExport {
    pub uri: String,
    pub name: String,
    pub version: String,
}

#[derive(Debug)]
pub struct QmlEnum {
    pub name: String,
    pub values: Vec<(String, u32)>,
}

#[derive(Debug)]
pub struct QmlProperty {
    pub name: String,
    pub qmltype: String,
    pub readonly: bool,
}

#[derive(Debug)]
pub struct QmlMethod {
    pub name: String,
    pub parmaters: Vec<QmlParmater>,
}

#[derive(Debug)]
pub struct QmlSignal {
    pub name: String,
    pub parmaters: Vec<QmlParmater>,
}

#[derive(Debug)]
pub struct QmlParmater {
    pub name: String,
    pub qmltype: String,
}

use std::{fs::File, io::BufReader, path::Path};

impl QmlModule {
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Option<Self>> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let content = String::from_utf8_lossy(reader.buffer()).to_string();
        get_module_from_source(content, None, path)
    }
}

fn get_module_from_source<S, P>(
    content: S,
    namespace: Option<String>,
    path: P,
) -> anyhow::Result<Option<QmlModule>>
where
    S: ToString + std::convert::AsRef<[u8]>,
    P: AsRef<Path>,
{
    let source = content.to_string();
    let newsource: Vec<&str> = source.lines().collect();
    let mut parse = tree_sitter::Parser::new();
    parse.set_language(tree_sitter_qmljs::language())?;
    let thetree = parse.parse(content, None).unwrap();
    let root = thetree.root_node();
    let mut course = root.walk();
    let root = root.child_by_field_name("root").unwrap();
    let root = root.child_by_field_name("initializer").unwrap();
    let mut dependencies = vec![];
    for child in root.children(&mut course) {
        match child.kind() {
            "ui_binding" => {
                let id = child.child_by_field_name("name").unwrap();
                let row = id.start_position().row;
                let start_x = id.start_position().column;
                let start_y = id.end_position().column;
                let idname = (&newsource[row][start_x..start_y]).to_string();

                if idname == "dependencies" {
                    let value = child.child_by_field_name("value").unwrap();
                    let array = value.child(0).unwrap();
                    let mut arraycourse = array.walk();
                    for depend in array.children(&mut arraycourse) {
                        if depend.kind() == "string" {
                            let dep = depend.child(1).unwrap();
                            let row = dep.start_position().row;
                            let start_x = dep.start_position().column;
                            let start_y = dep.end_position().column;
                            let dependname = (&newsource[row][start_x..start_y]).to_string();
                            dependencies.push(dependname);
                        }
                    }
                }
            }
            "ui_object_definition" => {
                let ui_root = child.child_by_field_name("initializer").unwrap();
                let mut prototypes: Option<String> = None;
                let mut exports: Vec<ModuleExport> = vec![];
                let mut name = String::new();
                let mut ui_course = ui_root.walk();
                for child in ui_root.children(&mut ui_course) {
                    match child.kind() {
                        "ui_binding" => {
                            let id = child.child_by_field_name("name").unwrap();
                            let row = id.start_position().row;
                            let start_x = id.start_position().column;
                            let start_y = id.end_position().column;
                            let idname = (&newsource[row][start_x..start_y]).to_string();
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
                                name = (&newsource[row][start_x..start_y]).to_string();
                            } else if idname == "exports" {
                                let value = child.child_by_field_name("value").unwrap();
                                let array = value.child(0).unwrap();
                                let mut arraycourse = array.walk();
                                for export in array.children(&mut arraycourse) {
                                    if export.kind() == "string" {
                                        let exp = export.child(1).unwrap();
                                        let row = exp.start_position().row;
                                        let start_x = exp.start_position().column;
                                        let start_y = exp.end_position().column;
                                        let exportname =
                                            (&newsource[row][start_x..start_y]).to_string();
                                        let colone: Vec<&str> = exportname.split(" ").collect();
                                        let version = colone[1];
                                        let exuir: &str =
                                            colone[0].split("/").collect::<Vec<&str>>()[0];
                                        let exname: &str =
                                            colone[0].split("/").collect::<Vec<&str>>()[1];
                                        exports.push(ModuleExport {
                                            uri: exuir.to_string(),
                                            name: exname.to_string(),
                                            version: version.to_string(),
                                        });
                                        //exports.push(exportname);
                                    }
                                }
                            }
                        }
                        "ui_object_definition" => {
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    Ok(None)
}

#[test]
fn tst_qmltypes_message() {
    let source = include_str!("../../../misc/builtins.qmltypes");
    get_module_from_source(source, None, "ss").unwrap();
}
