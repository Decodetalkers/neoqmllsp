project('qtquickexample', 'cpp',
  version : '0.1',
  default_options : ['warning_level=3','cpp_std=c++20'])
qt6 = import('qt6')
qt6_dep = dependency('qt6', modules: ['Core', 'Widgets', 'Quick'])

generate_signas = dependency('generate_signals')

qml_res = qt6.compile_resources(sources: ['qml/qml.qrc'])

moc_source = [ 'src/MainWindow.h' ]

moc_files = qt6.compile_moc(
  headers: files(moc_source),
  dependencies: qt6_dep
)

source_files = [
  'src/MainWindow.cpp',
]


exe = executable(meson.project_name(),
  moc_files,
  source_files,
  'src/main.cpp',
  qml_res,
  dependencies: qt6_dep,
  install: true
)

typegenerate = executable('qmltypes',
  moc_files,
  source_files,
  'qmltypes/main.cpp',
  qml_res,
  include_directories: 'src',
  dependencies: [qt6_dep,generate_signas],
)

