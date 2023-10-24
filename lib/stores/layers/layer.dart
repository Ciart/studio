import 'dart:ui';

import 'package:mobx/mobx.dart';

part 'layer.g.dart';

abstract class Layer = _Layer with _$Layer;

abstract class _Layer with Store {
  _Layer({required this.name, bool this.isVisible = true});

  @observable
  String name;

  @observable
  bool isRefreshed = false;

  @observable
  bool isVisible;

  Image? _image;
}
