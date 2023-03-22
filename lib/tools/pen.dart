import 'dart:ui';

import 'package:doter/models/document.dart';
import 'package:doter/tools/tool.dart';
import 'package:doter/utilities/plot.dart';

class Pen extends Tool {
  Pen({int size = 1})
      : this.size = size,
        super(ToolId.pen, 'Pen');

  int size;

  Offset _prevPosition = Offset.zero;

  @override
  void onPress(Document target, ToolData data) {
    var position = data.position
        .translate((size ~/ -2).toDouble(), (size ~/ -2).toDouble());

    for (int i = 0; i < size; i++) {
      for (int j = 0; j < size; j++) {
        target.setPixel(
          data.primaryColor,
          position.dx.toInt() + i,
          position.dy.toInt() + j,
        );
      }
    }

    _prevPosition = position;
  }

  @override
  void onMove(Document target, ToolData data) {
    var position = data.position
        .translate((size ~/ -2).toDouble(), (size ~/ -2).toDouble());

    Plot.line(
      _prevPosition.dx.toInt(),
      _prevPosition.dy.toInt(),
      position.dx.toInt(),
      position.dy.toInt(),
      (x, y) {
        for (int i = 0; i < size; i++) {
          for (int j = 0; j < size; j++) {
            target.setPixel(
              data.primaryColor,
              x + i,
              y + j,
            );
          }
        }
      },
    );

    _prevPosition = position;
  }

  @override
  void onRelease(Document target, ToolData data) {
    // TODO: implement onRelease
  }
}
