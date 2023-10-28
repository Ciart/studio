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

  @computed
  Layer get selectedLayer => layers[selectLayerIndex];

  @observable
  Picture? picture;

  bool _isInvalidated = true;

  bool _isRendering = false;

  late ReactionDisposer _reactionDisposer;

  _Document({required this.name, required this.width, required this.height}) {
    layers.add(
      BitmapLayer(
        name: 'Layer 1',
        width: width,
        height: height,
        onInvalidate: _invalidate,
      ),
    );
    layers.add(
      BitmapLayer(
        name: 'Layer 2',
        width: width,
        height: height,
        onInvalidate: _invalidate,
      ),
    );
    layers.add(
      BitmapLayer(
        name: 'Layer 3',
        width: width,
        height: height,
        onInvalidate: _invalidate,
      ),
    );
  }

  void update() async {
    if (_isInvalidated && !_isRendering) {
      _isInvalidated = false;
      _isRendering = true;

      await render();

      _isRendering = false;
    }
  }

  void dispose() {
    _reactionDisposer();

    for (final layer in layers) {
      layer.dispose();
    }
  }

  void _invalidate() {
    _isInvalidated = true;
  }

  @action
  void createBitmapLayer() {
    layers.insert(
      selectLayerIndex,
      BitmapLayer(
        name: 'Layer',
        width: width,
        height: height,
        onInvalidate: _invalidate,
      ),
    );

    // _invalidate();
  }

  @action
  void updateLayerIndex(int oldIndex, int newIndex) {
    if (oldIndex < newIndex) {
      newIndex -= 1;
    }

    final layer = layers.removeAt(oldIndex);
    layers.insert(newIndex, layer);

    _invalidate();
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

      final layerImage = await layer.renderImage();

      canvas.drawImage(layerImage, Offset.zero, Paint());
    }

    picture = recorder.endRecording();
  }

  @action
  void undo() {}

  @action
  void redo() {}
}
