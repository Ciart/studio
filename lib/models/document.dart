import 'dart:typed_data';
import 'dart:ui';

import 'layer.dart';

class Document {
  int _width;
  int get width => _width;

  int _height;
  int get height => _height;

  List<Layer> layers = [];

  Image? image;

  var pixels = Uint8List(400 * 300 * 4);

  Document({required int width, required int height})
      : _width = width,
        _height = height {
    for (int i = 0; i < pixels.length; i++) {
      pixels[i] = 255;

      if (i % 4 == 2) {
        pixels[i] = 0;
      }
    }

    decodeImageFromPixels(pixels, 100, 100, PixelFormat.rgba8888, callback);
  }

  void callback(image) {
    this.image = image;
  }
}
