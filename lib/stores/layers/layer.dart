import 'dart:ui';

import 'package:mobx/mobx.dart';

part 'layer.g.dart';

abstract class Layer = _Layer with _$Layer;

abstract class _Layer with Store {
  @observable
  String name;

  @observable
  bool isVisible;

  Function? onInvalidate;

  bool _isInvalidated = true;

  Image? _image;

  // 축소된 이미지로 변경될 것을 대비
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

  void dispose() {
    _reactionDisposer();
  }

  Future<Image> renderImage() async {
    if (_isInvalidated) {
      _isInvalidated = false;

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
