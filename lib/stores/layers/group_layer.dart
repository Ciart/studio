import 'dart:async';
import 'dart:typed_data';
import 'dart:ui';

import 'package:mobx/mobx.dart';

import 'layer.dart';

part 'group_layer.g.dart';

class GroupLayerSnapshot extends LayerSnapshot {
  GroupLayerSnapshot({
    required String name,
    required bool isVisible,
    required this.children,
  }) : super(name: name, isVisible: isVisible);

  GroupLayerSnapshot.fromGroupLayer(_GroupLayer layer)
      : children = layer.children.map((layer) => layer.makeSnapshot()).toList(),
        super.fromLayer(layer);

  final List<LayerSnapshot> children;
}

class GroupLayer = _GroupLayer with _$GroupLayer;

abstract class _GroupLayer extends Layer with Store {
  _GroupLayer({
    required String name,
    Function? onInvalidate,
  }) : super(name: name, onInvalidate: onInvalidate);

  @observable
  ObservableList<Layer> children = ObservableList<Layer>();

  @override
  GroupLayerSnapshot makeSnapshot() {
    return GroupLayerSnapshot.fromGroupLayer(this);
  }

  @override
  void restoreSnapshot(LayerSnapshot snapshot) {
    super.restoreSnapshot(snapshot);

    if (snapshot is GroupLayerSnapshot) {
      children = ObservableList.of(snapshot.children);
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
