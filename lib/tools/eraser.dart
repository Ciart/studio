import 'dart:ui';

import 'package:ciart_studio/stores/document.dart';
import 'package:ciart_studio/stores/layers/bitmap_layer.dart';
import 'package:ciart_studio/tools/tool.dart';

class Eraser extends Tool {
  Eraser() : super(ToolId.eraser, 'Eraser');

  @override
  void onPress(Document target, ToolData data) {
    final layer = target.selectedLayer;

    if (!(layer is BitmapLayer)) {
      return;
    }

    layer.setPixel(
      const Color(0x00000000),
      data.position.dx.toInt(),
      data.position.dy.toInt(),
    );

    layer.invalidate();
  }

  @override
  void onMove(Document target, ToolData data) {
    final layer = target.selectedLayer;

    if (!(layer is BitmapLayer)) {
      return;
    }

    layer.setPixel(
      const Color(0x00000000),
      data.position.dx.toInt(),
      data.position.dy.toInt(),
    );

    layer.invalidate();
  }

  @override
  void onRelease(Document target, ToolData data) {
    // TODO: implement onRelease
  }
}
