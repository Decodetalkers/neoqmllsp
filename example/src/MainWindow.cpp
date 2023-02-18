#include "MainWindow.h"

MainWindow::MainWindow(QWindow *parent)
  : QQuickView(parent)
{
    qmlRegisterSingletonInstance("im.test", 1, 0, "MainWindow", this);
    setSource(QUrl(QStringLiteral("qrc:///main.qml")));
}

void
MainWindow::onReload()
{
}
