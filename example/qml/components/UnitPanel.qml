import QtQuick

Rectangle {
    id: unit
    default property bool selected: false
    property int _width: 200
    readonly property int _height : 80
    signal clicked()
    width: unit._width
    height: unit._height
    color: "grey"
    Rectangle {
        id: left
        color: unit.selected ? "green": "white"
        height: unit._height
        width: 10
        anchors {
            left: right.left
        }
    }
    Column {
        id : right
        leftPadding: 10
        Text {
            text: "beta"
        }
        Text {
            text: "beta"
        }
    }
    MouseArea {
        anchors.fill: parent
        cursorShape: Qt.PointingHandCursor
        onClicked: {
            unit.clicked()
        }
    }
}
