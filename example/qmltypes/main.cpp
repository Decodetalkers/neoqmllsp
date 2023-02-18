#include "MainWindow.h"

#include <QApplication>
#include <generate_signals.hpp>

int
main(int argc, char *argv[])
{
    QCoreApplication app(argc, argv);
    auto qmlstorage = generate_signals::QmlMessageStroage();
    qmlstorage.get_message<MainWindow>(
      "im.test", generate_signals::RegisterType::SingleTon, 1, 0, "MainWindow");
    QObject::connect(
      &qmlstorage,
      &generate_signals::QmlMessageStroage::quit,
      &app,
      [&app](int code) {
          if (code == 0) {
              qDebug() << "Cannot generate";
          }
          app.quit();
      },
      Qt::QueuedConnection);
    qmlstorage.writeToFile();
    return app.exec();
}
