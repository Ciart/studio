import 'package:ciart_studio/stores/document.dart';
import 'package:ciart_studio/tools/tool.dart';
import 'package:fluent_ui/fluent_ui.dart';

class Eyedropper extends Tool {
  Eyedropper() : super(ToolId.eyedropper, 'Eyedropper', FluentIcons.eyedropper);

  @override
  void onPress(Document target, ToolData data) {}

  @override
  void onMove(Document target, ToolData data) {}

  @override
  void onRelease(Document target, ToolData data) {}
}
