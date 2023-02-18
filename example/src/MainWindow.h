#pragma once

#include <QQuickView>

class MainWindow final : public QQuickView
{
    Q_OBJECT
public:
    explicit MainWindow(QWindow *parent = nullptr);
public slots:
    void onReload();
signals:
    void reload();

};
