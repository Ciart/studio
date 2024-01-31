import 'package:flutter/widgets.dart';
import 'package:mobx/mobx.dart';

import '../stores/document.dart';

part 'tool.g.dart';

enum ToolId {
  pen,
  eraser,
  line,
  rectangle,
  ellipse,
  eyedropper,
}

class ToolData {
  ToolData({required this.primaryColor, required this.position});

  final Color primaryColor;
  final Offset position;
}

abstract class Tool = _Tool with _$Tool;

abstract class _Tool with Store {
  _Tool(this.id, this.name, this.icon);

  final ToolId id;
  final String name;
  final IconData icon;

  void onPress(Document target, ToolData data);
  void onMove(Document target, ToolData data);
  void onRelease(Document target, ToolData data);
}
