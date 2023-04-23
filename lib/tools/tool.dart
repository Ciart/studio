import 'dart:ui';

import 'package:stellon/models/document.dart';
import 'package:flutter/foundation.dart';

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

abstract class Tool extends ChangeNotifier {
  Tool(this.id, this.name);

  final ToolId id;
  final String name;

  void onPress(Document target, ToolData data);
  void onMove(Document target, ToolData data);
  void onRelease(Document target, ToolData data);

  // ignore: must_call_super
  @override
  void dispose() {}
}
