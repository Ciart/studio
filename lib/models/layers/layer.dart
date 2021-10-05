import 'dart:ui';

import '../document.dart';

abstract class Layer {
  Layer({required this.document, required this.name, bool isVisible = true})
      : _isVisible = isVisible;

  Document document;

  String name;

  Image? _image;

  Future<Image?> get image async {
    if (!_isRefreshed) {
      _isRefreshed = true;
      _image = await render();

      if (!_isRefreshed) {
        document.refresh();
      }
    }

    return _image;
  }

  bool _isVisible;

  bool get isVisible => _isVisible;

  set isVisible(bool value) {
    _isVisible = value;
    refresh();
  }

  /// 갱신이 필요한지 확인하는 변수
  bool _isRefreshed = false;

  void refresh() {
    _isRefreshed = false;
    document.refresh();
  }

  Future<Image> render();
}
