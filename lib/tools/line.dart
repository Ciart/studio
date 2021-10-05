import 'dart:ui';

import 'package:doter/models/document.dart';
import 'package:doter/tools/tool.dart';
import 'package:doter/utilities/plot.dart';

class Line extends Tool {
  Line() : super(ToolId.line, 'Line');

  Offset _startPosition = Offset.zero;

  @override
  void onPress(Document target, ToolData data) {
    _startPosition = data.position;
  }

  @override
  void onMove(Document target, ToolData data) {}

  @override
  void onRelease(Document target, ToolData data) {
    Plot.line(
      _startPosition.dx.toInt(),
      _startPosition.dy.toInt(),
      data.position.dx.toInt(),
      data.position.dy.toInt(),
      (x, y) {
        target.setPixel(data.primaryColor, x, y);
      },
    );
  }
}
