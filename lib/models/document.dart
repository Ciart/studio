import 'dart:ui';

import 'package:flutter/foundation.dart';
import 'package:flutter/scheduler.dart';

import 'layers/layer.dart';
import 'layers/pixel_layer.dart';

class Document extends ChangeNotifier {
  /// documentProvider에서 사용하는 식별자
  final String id;

  late String _name;
  String get name => _name;

  late int _width;
  int get width => _width;

  late int _height;
  int get height => _height;

  List<Layer> layers = [];

  Picture? picture;

  bool _isRefreshed = false;

  late Ticker _ticker;

  Document(this.id);

  void init({required String name, required int width, required int height}) {
    _name = name;
    _width = width;
    _height = height;

    layers.add(
      PixelLayer(document: this, name: 'Layer 1', width: width, height: height),
    );
    layers.add(
      PixelLayer(document: this, name: 'Layer 2', width: width, height: height),
    );
    layers.add(
      PixelLayer(document: this, name: 'Layer 3', width: width, height: height),
    );

    _ticker = Ticker(
      (duration) async {
        if (!_isRefreshed) {
          _isRefreshed = true;
          await render();
        }
      },
    )..start();
  }

  @override
  void dispose() {
    // super.dispose();

    // _ticker.dispose();
  }

  void refresh() {
    _isRefreshed = false;
  }

  setPixel(Color color, int x, int y) {
    (layers[0] as PixelLayer).setPixel(color, x, y);
  }

  void updateLayerIndex(int oldIndex, int newIndex) {
    if (oldIndex < newIndex) {
      newIndex -= 1;
    }

    final layer = layers.removeAt(oldIndex);
    layers.insert(newIndex, layer);

    notifyListeners();

    render();
  }

  Future<void> render() async {
    final recorder = PictureRecorder();
    final canvas = Canvas(recorder);

    for (int i = layers.length - 1; i >= 0; i--) {
      final layer = layers[i];

      if (!layer.isVisible) {
        continue;
      }

      final layerImage = await layer.image;

      if (layerImage == null) {
        continue;
      }

      canvas.drawImage(layerImage, Offset.zero, Paint());
    }

    picture = recorder.endRecording();

    notifyListeners();
  }
}
