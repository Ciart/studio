import 'dart:ui';

import 'package:flutter/scheduler.dart';
import 'package:mobx/mobx.dart';
import 'package:uuid/uuid.dart';

import 'layers/layer.dart';
import 'layers/bitmap_layer.dart';

part 'document.g.dart';

class Document = _Document with _$Document;

abstract class _Document with Store {
  final String id = Uuid().v4();

  @observable
  String name;

  @observable
  int width;

  @observable
  late int height;

  @observable
  ObservableList<Color> palette = ObservableList();

  @observable
  ObservableList<Layer> layers = ObservableList();

  @observable
  int selectLayerIndex = 0;

  // set selectLayerIndex(int selectLayerIndex) {
  //   _selectLayerIndex = selectLayerIndex;
  //   notifyListeners();
  // }

  @observable
  Picture? picture;

  bool _isRefreshed = false;
  bool _isDoing = false;

  late Ticker _ticker;

  _Document({required this.name, required this.width, required this.height}) {
    layers.add(
      BitmapLayer(name: 'Layer 1', width: width, height: height),
    );
    layers.add(
      BitmapLayer(name: 'Layer 2', width: width, height: height),
    );
    layers.add(
      BitmapLayer(name: 'Layer 3', width: width, height: height),
    );

    // TODO: 외부로 옮기기
    _ticker = Ticker(
      (duration) async {
        if (!_isRefreshed) {
          _isRefreshed = true;

          if (!_isDoing) {
            _isDoing = true;
            await render();
            _isDoing = false;
          }
        }
      },
    )..start();
  }

  void refresh() {
    _isRefreshed = false;
  }

  setPixel(Color color, int x, int y) {
    (layers[this.selectLayerIndex] as BitmapLayer).setPixel(color, x, y);
  }

  @action
  void updateLayerIndex(int oldIndex, int newIndex) {
    if (oldIndex < newIndex) {
      newIndex -= 1;
    }

    final layer = layers.removeAt(oldIndex);
    layers.insert(newIndex, layer);

    refresh();
  }

  @action
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
  }

  @action
  void undo() {}

  @action
  void redo() {}
}
