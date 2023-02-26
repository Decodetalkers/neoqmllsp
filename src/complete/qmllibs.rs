mod qmlenumgenerate;
mod qmlmethodgenerate;
mod qmlparmatergenerate;
mod qmlpropertygenerate;
mod qmlsignalgenerate;

use once_cell::sync::Lazy;

use std::{io::Read, sync::Arc};
use tokio::sync::Mutex;
pub static ROOT_LIB_DIR: Lazy<Arc<Mutex<String>>> =
    Lazy::new(|| Arc::new(Mutex::new("/usr/lib/qt/qml".to_string())));

async fn buildinpath() -> String {
    let dir = ROOT_LIB_DIR.lock().await;
    format!("{}/builtins.qmltypes", dir)
}

#[allow(dead_code)]
pub async fn update_root_dir<S: ToString>(path: S) {
    let mut data = ROOT_LIB_DIR.lock().await;
    *data = path.to_string()
}

pub static ALL_MODULES: Lazy<Arc<Mutex<Vec<QmlModule>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

pub async fn update_modules() -> anyhow::Result<()> {
    let mut modules = ALL_MODULES.lock().await;
    let rootdir = ROOT_LIB_DIR.lock().await;
    let mut new_modules = Vec::new();
    let pattern = format!("{}/**/*.qmltypes", rootdir);
    drop(rootdir);
    for entrys in glob::glob(&pattern)?.flatten() {
        dbg!(&entrys);
        if let Ok(module) = QmlModule::new(entrys).await {
            new_modules.push(module);
        };
    }
    if let Ok(module) = QmlModule::buildintypes().await {
        new_modules.push(module);
    }
    *modules = new_modules;
    Ok(())
}
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
    pub prototype: Option<String>,
    pub exports: Option<Vec<ModuleExport>>,
    pub qmlenums: Vec<QmlEnum>,
    pub iscreatable: bool,
    pub issingleton: bool,
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
    pub values: Vec<(String, i32)>,
}

#[derive(Debug)]
pub struct QmlProperty {
    pub name: String,
    pub qmltype: String,
    pub isreadonly: bool,
    pub ispointer: bool,
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
    pub ispointer: bool,
}

use std::{fs::File, path::Path};

async fn get_namespace<P: AsRef<Path>>(path: P) -> String {
    let dir = ROOT_LIB_DIR.lock().await;
    let relative = pathdiff::diff_paths(&path, dir.clone())
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    relative.replace('/', ".")
}

impl QmlModule {
    pub async fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let mut file = File::open(&path)?;
        let mut context = String::new();
        file.read_to_string(&mut context)?;
        let namespace = get_namespace(&path.as_ref().parent().unwrap()).await;
        get_module_from_source(
            context,
            Some(namespace),
            path.as_ref().to_str().unwrap().to_string(),
        )
    }
    pub async fn buildintypes() -> anyhow::Result<Self> {
        let path = buildinpath().await;
        let mut file = File::open(&path)?;
        let mut context = String::new();
        file.read_to_string(&mut context)?;
        get_module_from_source(context, None, path)
    }
}

fn get_module_from_source<S>(
    content: S,
    namespace: Option<String>,
    path: S,
) -> anyhow::Result<QmlModule>
where
    S: ToString + std::convert::AsRef<[u8]>,
{
    let source = &content.to_string();
    let newsource: Vec<&str> = source.lines().collect();
    let mut parse = tree_sitter::Parser::new();
    parse.set_language(tree_sitter_qmljs::language())?;
    let thetree = parse.parse(content, None).unwrap();
    let root = thetree.root_node();
    let mut course = root.walk();
    let Some(root) = root.child_by_field_name("root") else {
        return Err(anyhow::anyhow!("Nothing here"));
    };
    let root = root.child_by_field_name("initializer").unwrap();

    let mut dependencies = vec![];
    let mut components = Vec::new();

    for child in root.children(&mut course) {
        match child.kind() {
            "ui_binding" => {
                let id = child.child_by_field_name("name").unwrap();
                let row = id.start_position().row;
                let start_x = id.start_position().column;
                let start_y = id.end_position().column;
                let idname = newsource[row][start_x..start_y].to_string();

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
                            let dependname = newsource[row][start_x..start_y].to_string();
                            dependencies.push(dependname);
                        }
                    }
                }
            }
            "ui_object_definition" => {
                let ui_root = child.child_by_field_name("initializer").unwrap();

                let mut ui_course = ui_root.walk();

                let mut name = String::new();
                let mut prototype: Option<String> = None;
                let mut iscreatable = false;
                let mut issingleton = false;
                let mut exports: Vec<ModuleExport> = vec![];
                let mut qmlenums = Vec::new();
                let mut qmlproperties = Vec::new();
                let mut qmlsignals = Vec::new();
                let mut qmlmethods = Vec::new();

                for child in ui_root.children(&mut ui_course) {
                    match child.kind() {
                        "ui_binding" => {
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
                            } else if idname == "prototype" {
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
                                prototype = Some(newsource[row][start_x..start_y].to_string());
                            } else if idname == "isCreateable" {
                                let value = child
                                    .child_by_field_name("value")
                                    .unwrap()
                                    .child(0)
                                    .unwrap();
                                let row = value.start_position().row;
                                let start_x = value.start_position().column;
                                let start_y = value.end_position().column;
                                iscreatable = &newsource[row][start_x..start_y] == "true";
                            } else if idname == "isSingleton" {
                                let value = child
                                    .child_by_field_name("value")
                                    .unwrap()
                                    .child(0)
                                    .unwrap();
                                let row = value.start_position().row;
                                let start_x = value.start_position().column;
                                let start_y = value.end_position().column;
                                issingleton = &newsource[row][start_x..start_y] == "true";
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
                                            newsource[row][start_x..start_y].to_string();
                                        let colone: Vec<&str> = exportname.split(' ').collect();
                                        let version = colone[1];
                                        let exporturis =
                                            colone[0].split('/').collect::<Vec<&str>>();
                                        if exporturis.len() == 2 {
                                            let exuir: &str = exporturis[0];
                                            let exname: &str = exporturis[1];
                                            exports.push(ModuleExport {
                                                uri: exuir.to_string(),
                                                name: exname.to_string(),
                                                version: version.to_string(),
                                            });
                                        } else {
                                            let exname: &str = exporturis[0];
                                            exports.push(ModuleExport {
                                                uri: match namespace {
                                                    Some(ref uri) => uri.clone(),
                                                    None => "".to_string(),
                                                },
                                                name: exname.to_string(),
                                                version: version.to_string(),
                                            })
                                        }

                                        //exports.push(exportname);
                                    }
                                }
                            }
                        }
                        "ui_object_definition" => {
                            let typename = child.child_by_field_name("type_name").unwrap();
                            let row = typename.start_position().row;
                            let start_x = typename.start_position().column;
                            let start_y = typename.end_position().column;
                            let type_name = newsource[row][start_x..start_y].to_string();
                            let object_init = child.child_by_field_name("initializer").unwrap();
                            match type_name.as_str() {
                                "Enum" => {
                                    qmlenums.push(qmlenumgenerate::get_enum_from_source(
                                        source,
                                        object_init,
                                    ));
                                }
                                "Property" => {
                                    qmlproperties.push(
                                        qmlpropertygenerate::get_property_from_source(
                                            source,
                                            object_init,
                                        ),
                                    );
                                }
                                "Signal" => {
                                    qmlsignals.push(qmlsignalgenerate::get_signal_from_source(
                                        source,
                                        object_init,
                                    ));
                                }
                                "Method" => {
                                    qmlmethods.push(qmlmethodgenerate::get_method_from_source(
                                        source,
                                        object_init,
                                    ));
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                components.push(QmlComponent {
                    name,
                    prototype,
                    exports: {
                        if exports.is_empty() {
                            None
                        } else {
                            Some(exports)
                        }
                    },
                    qmlenums,
                    iscreatable,
                    issingleton,
                    properties: qmlproperties,
                    signals: qmlsignals,
                    methods: qmlmethods,
                })
            }
            _ => {}
        }
    }
    Ok(QmlModule {
        plugin_location: path.to_string(),
        module_namespace: namespace,
        dependencies,
        components,
    })
}

#[test]
fn tst_qmltypes_message() {
    let source = include_str!("../../misc/builtins.qmltypes");
    get_module_from_source(source, None, "ss").unwrap();
    let source = include_str!("../../misc/plugins.qmltypes");
    get_module_from_source(source, None, "ss").unwrap();
}

#[tokio::test]
async fn tst_path() {
    assert_eq!(
        get_namespace("/usr/lib/qt/qml/QtQml/StateMachine").await,
        "QtQml.StateMachine".to_string()
    );
}

#[tokio::test]
async fn tst_init_modules() {
    update_modules().await.unwrap();
}
