import 'package:doter/models/document.dart';
import 'package:doter/tools/tool.dart';

class Eyedropper extends Tool {
  Eyedropper() : super(ToolId.eyedropper, 'Eyedropper');

  @override
  void onPress(Document target, ToolData data) {}

  @override
  void onMove(Document target, ToolData data) {}

  @override
  void onRelease(Document target, ToolData data) {}
}
