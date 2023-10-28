import 'dart:ui';

import 'package:ciart_studio/stores/document.dart';
import 'package:ciart_studio/stores/layers/bitmap_layer.dart';
import 'package:ciart_studio/tools/tool.dart';
import 'package:ciart_studio/utilities/plot.dart';

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
    final layer = target.selectedLayer;

    if (!(layer is BitmapLayer)) {
      return;
    }

    Plot.line(
      _startPosition.dx.toInt(),
      _startPosition.dy.toInt(),
      data.position.dx.toInt(),
      data.position.dy.toInt(),
      (x, y) {
        layer.setPixel(data.primaryColor, x, y);
      },
    );

    layer.invalidate();
  }
}
