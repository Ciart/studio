import 'dart:ui';

import 'package:mobx/mobx.dart';

import '../stores/document.dart';

part 'tool.g.dart';

enum ToolId {
  pen,
  eraser,
  rectangle,
  line,
  eyedropper,
}

class ToolData {
  ToolData({required this.primaryColor, required this.position});

  final Color primaryColor;
  final Offset position;
}

abstract class Tool = _Tool with _$Tool;

abstract class _Tool with Store {
  _Tool(this.id, this.name);

  final ToolId id;
  final String name;

  void onPress(Document target, ToolData data);
  void onMove(Document target, ToolData data);
  void onRelease(Document target, ToolData data);
}
