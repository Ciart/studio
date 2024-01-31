import 'dart:async';
import 'dart:typed_data';
import 'dart:ui';

import 'package:mobx/mobx.dart';

import 'layer.dart';

part 'bitmap_layer.g.dart';

class BitmapLayerSnapshot extends LayerSnapshot {
  BitmapLayerSnapshot({
    required String name,
    required bool isVisible,
    required this.width,
    required this.height,
    required this.pixels,
  }) : super(name: name, isVisible: isVisible);

  BitmapLayerSnapshot.fromBitmapLayer(_BitmapLayer layer)
      : width = layer.width,
        height = layer.height,
        pixels = Uint8List.fromList(layer._pixels),
        super.fromLayer(layer);

  final int width;
  final int height;
  final Uint8List pixels;
}

class BitmapLayer = _BitmapLayer with _$BitmapLayer;

abstract class _BitmapLayer extends Layer with Store {
  @observable
  int width;

  @observable
  int height;

  Uint8List _pixels;

  _BitmapLayer({
    required String name,
    required this.width,
    required this.height,
    Uint8List? pixels,
    Function? onInvalidate,
  })  : _pixels = pixels ?? Uint8List(width * height * 4),
        super(name: name, onInvalidate: onInvalidate);

  @override
  BitmapLayerSnapshot makeSnapshot() {
    return BitmapLayerSnapshot.fromBitmapLayer(this);
  }

  @override
  void restoreSnapshot(LayerSnapshot snapshot) {
    super.restoreSnapshot(snapshot);

    if (snapshot is BitmapLayerSnapshot) {
      width = snapshot.width;
      height = snapshot.height;
    }
  }

  void setPixel(Color color, int x, int y) {
    if (x < 0 || y < 0 || x >= width || y >= height) {
      return;
    }

    var targetIndex = (y * width + x) * 4;

    _pixels[targetIndex] = color.red;
    _pixels[targetIndex + 1] = color.green;
    _pixels[targetIndex + 2] = color.blue;
    _pixels[targetIndex + 3] = color.alpha;
  }

  void clear() {
    _pixels = Uint8List(width * height * 4);
    invalidate();
  }

  @override
  Future<Image> render() async {
    var completer = Completer<Image>();

    decodeImageFromPixels(
      _pixels,
      width,
      height,
      PixelFormat.rgba8888,
      completer.complete,
    );

    return await completer.future;
  }
}
