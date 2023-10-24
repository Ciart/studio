import 'dart:async';
import 'dart:typed_data';
import 'dart:ui';

import 'package:mobx/mobx.dart';

import 'layer.dart';

part 'bitmap_layer.g.dart';

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
  })  : _pixels = Uint8List(width * height * 4),
        super(name: name);

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
