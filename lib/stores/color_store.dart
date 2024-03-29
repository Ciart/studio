import 'package:flutter/painting.dart';
import 'package:mobx/mobx.dart';

part 'color_store.g.dart';

class ColorStore = _ColorStore with _$ColorStore;

abstract class _ColorStore with Store {
  @observable
  HSVColor primary = HSVColor.fromColor(Color(0xff000000));

  @computed
  Color get primaryColor => primary.toColor();

  @observable
  HSVColor secondary = HSVColor.fromColor(Color(0xffffffff));

  @computed
  Color get secondaryColor => secondary.toColor();

  @observable
  ObservableList<Color> palette = ObservableList.of([]);

  @action
  void setPrimary(HSVColor color) {
    primary = color;
  }

  @action
  void setSecondary(HSVColor color) {
    secondary = color;
  }

  @action
  void addColor(Color color) {
    if (palette.contains(color)) {
      return;
    }

    palette.add(color);
  }

  @action
  void swapColor(Color a, Color b) {
    final aIndex = palette.indexOf(a);
    final bIndex = palette.indexOf(b);

    if (aIndex == -1 || bIndex == -1) {
      return;
    }

    palette[aIndex] = b;
    palette[bIndex] = a;
  }
}
