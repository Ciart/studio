import 'dart:ui';

import 'package:ciart_studio/tools/ellipse.dart';
import 'package:ciart_studio/tools/eraser.dart';
import 'package:ciart_studio/tools/line.dart';
import 'package:ciart_studio/tools/pen.dart';
import 'package:ciart_studio/tools/rectangle.dart';
import 'package:ciart_studio/tools/tool.dart';
import 'package:mobx/mobx.dart';

part 'tool_store.g.dart';

class ToolContainer = _ToolContainer with _$ToolContainer;

abstract class _ToolContainer with Store {
  @observable
  int focusIndex = 0;

  @observable
  Offset position = Offset.zero;

  @computed
  Tool get focusTool => tools[focusIndex];

  @observable
  ObservableList<Tool> tools = ObservableList.of([
    Pen(),
    Eraser(),
    Line(),
    Rectangle(),
    Ellipse(),
  ]);

  @action
  void focus(int index) {
    focusIndex = index;
  }

  int getToolIndex(Tool tool) {
    return tools.indexWhere((element) => element == tool);
  }

  @action
  void setPosition(Offset position) {
    this.position = position;
  }
}
