import 'dart:async';
import 'dart:typed_data';
import 'dart:ui';

import '../document.dart';
import 'layer.dart';

class PixelLayer extends Layer {
  int _width;
  int get width => _width;

  int _height;
  int get height => _height;

  Uint8List _pixels;

  PixelLayer({
    required Document document,
    required String name,
    required int width,
    required int height,
  })  : _width = width,
        _height = height,
        _pixels = Uint8List(width * height * 4),
        super(document: document, name: name);

  void setPixel(Color color, int x, int y) {
    if (x < 0 || y < 0 || x >= width || y >= height) {
      return;
    }

    var targetIndex = (y * width + x) * 4;

    if (_pixels[targetIndex] == color.red &&
        _pixels[targetIndex + 1] == color.green &&
        _pixels[targetIndex + 2] == color.blue &&
        _pixels[targetIndex + 3] == color.alpha) {
      return;
    }

    _pixels[targetIndex] = color.red;
    _pixels[targetIndex + 1] = color.green;
    _pixels[targetIndex + 2] = color.blue;
    _pixels[targetIndex + 3] = color.alpha;

    refresh();
  }

  @override
  Future<Image> render() async {
    // if (!_isOutdated) {
    //   return;
    // }
    // _isOutdated = false;

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
