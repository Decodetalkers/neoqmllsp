import QtQuick 2.15
import QtQuick.Window 2.15
import "./components" as Components
import Qt.labs.platform 1.1 as Platform
Rectangle {
    width: 640
    height: 480
    visible: true
    //title: qsTr("Hello World")
    Components.UnitPanel {
    }
    Platform.Menu {
        id: menu
        Platform.MenuItem {
            text: qsTr("Cancle")
        }
    }
    TapHandler {
        acceptedButtons: Qt.RightButton
        gesturePolicy: TapHandler.ReleaseWithinBounds
        onSingleTapped: {
            menu.open()
        }
    }
    TapHandler {
        acceptedButtons: Qt.LeftButton
        gesturePolicy: TapHandler.ReleaseWithinBounds
        onSingleTapped: {
            menu.close()
        }
    }
}
