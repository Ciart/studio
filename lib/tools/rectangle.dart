import 'dart:ui';

import 'package:stellon/models/document.dart';
import 'package:stellon/tools/tool.dart';

class Rectangle extends Tool {
  Rectangle() : super(ToolId.rectangle, 'Rectangle');

  Offset _startPosition = Offset.zero;

  @override
  void onPress(Document target, ToolData data) {
    _startPosition = data.position;
  }

  @override
  void onMove(Document target, ToolData data) {}

  @override
  void onRelease(Document target, ToolData data) {
    for (int i = 0; i < data.position.dx - _startPosition.dx; i++) {
      target.setPixel(
        data.primaryColor,
        _startPosition.dx.toInt() + i,
        _startPosition.dy.toInt(),
      );
      target.setPixel(
        data.primaryColor,
        _startPosition.dx.toInt() + i,
        data.position.dy.toInt(),
      );
    }
  }
}
