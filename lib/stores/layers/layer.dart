import 'dart:ui';

import 'package:mobx/mobx.dart';

part 'layer.g.dart';

class LayerSnapshot {
  LayerSnapshot({
    required this.name,
    required this.isVisible,
  });

  LayerSnapshot.fromLayer(_Layer layer)
      : name = layer.name,
        isVisible = layer.isVisible;

  final String name;
  final bool isVisible;
}

abstract class Layer = _Layer with _$Layer;

abstract class _Layer with Store {
  @observable
  String name;

  @observable
  bool isVisible;

  Function? onInvalidate;

  bool _isInvalidated = true;

  @observable
  Image? _image;

  // 축소된 이미지로 변경될 것을 대비
  @computed
  Image? get thumbnail => _image;

  late ReactionDisposer _reactionDisposer;

  _Layer({
    required this.name,
    this.isVisible = true,
    this.onInvalidate,
  }) {
    _reactionDisposer = reaction((_) => isVisible, (_) {
      onInvalidate?.call();
    });
  }

  LayerSnapshot makeSnapshot() {
    return LayerSnapshot.fromLayer(this);
  }

  void restoreSnapshot(LayerSnapshot snapshot) {
    name = snapshot.name;
    isVisible = snapshot.isVisible;
  }

  void dispose() {
    _image?.dispose();
    _reactionDisposer();
  }

  Future<Image> renderImage() async {
    if (_isInvalidated) {
      _isInvalidated = false;

      _image?.dispose();

      _image = await render();
    }

    return _image!;
  }

  void invalidate() {
    _isInvalidated = true;
    onInvalidate?.call();
  }

  Future<Image> render();
}
