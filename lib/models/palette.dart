import 'dart:ui';

import 'package:ciart_studio/utilities/event.dart';

class ChangeEventArgs extends EventArgs {
  List<Color> colors;

  ChangeEventArgs(this.colors);
}

class Palette {
  Palette(this._colors);

  List<Color> _colors;

  Event change = Event();

  void addColor(Color color) {
    _colors.add(color);
    change(ChangeEventArgs(_colors.toList()));
  }
}
