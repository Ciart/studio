import 'dart:ui';

import 'package:ciart_studio/stores/document.dart';
import 'package:ciart_studio/stores/layers/bitmap_layer.dart';
import 'package:ciart_studio/tools/tool.dart';
import 'package:ciart_studio/utilities/plot.dart';
import 'package:mobx/mobx.dart';

part 'pen.g.dart';

class Pen = _Pen with _$Pen;

abstract class _Pen extends Tool with Store {
  _Pen({this.size = 1}) : super(ToolId.pen, 'Pen');

  @observable
  int size;

  Offset _prevPosition = Offset.zero;

  @override
  void onPress(Document target, ToolData data) {
    final layer = target.selectedLayer;

    if (!(layer is BitmapLayer)) {
      return;
    }

    var position = data.position
        .translate((size ~/ -2).toDouble(), (size ~/ -2).toDouble());

    for (int i = 0; i < size; i++) {
      for (int j = 0; j < size; j++) {
        layer.setPixel(
          data.primaryColor,
          position.dx.toInt() + i,
          position.dy.toInt() + j,
        );
      }
    }

    layer.invalidate();

    _prevPosition = position;
  }

  @override
  void onMove(Document target, ToolData data) {
    final layer = target.selectedLayer;

    if (!(layer is BitmapLayer)) {
      return;
    }

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
            layer.setPixel(
              data.primaryColor,
              x + i,
              y + j,
            );
          }
        }
      },
    );

    layer.invalidate();

    _prevPosition = position;
  }

  @override
  void onRelease(Document target, ToolData data) {
    // TODO: implement onRelease
  }
}
