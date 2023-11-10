import 'dart:async';
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
      // children = ObservableList.of(snapshot.children);
    }
  }

  @override
  Future<Image> render() async {
    final recorder = PictureRecorder();
    final canvas = Canvas(recorder);

    for (final child in children) {
      final image = await child.render();
      canvas.drawImage(image, Offset.zero, Paint());
    }

    final picture = recorder.endRecording();
    final image = await picture.toImage(100, 100);
    return image;
  }
}
