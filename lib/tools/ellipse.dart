import 'package:ciart_studio/stores/document.dart';
import 'package:ciart_studio/stores/layers/bitmap_layer.dart';
import 'package:ciart_studio/tools/tool.dart';
import 'package:ciart_studio/utilities/plot.dart';
import 'package:fluent_ui/fluent_ui.dart';

class Ellipse extends Tool {
  Ellipse() : super(ToolId.ellipse, 'Ellipse', FluentIcons.ellipse);

  Offset _startPosition = Offset.zero;

  @override
  void onPress(Document target, ToolData data) {
    _startPosition = data.position;
  }

  @override
  void onMove(Document target, ToolData data) {
    final layer = target.previewLayer!;

    layer.clear();

    Plot.ellipse(
      _startPosition.dx.toInt(),
      _startPosition.dy.toInt(),
      data.position.dx.toInt(),
      data.position.dy.toInt(),
      (x, y) {
        layer.setPixel(data.primaryColor, x, y);
      },
    );

    layer.invalidate();
  }

  @override
  void onRelease(Document target, ToolData data) {
    target.previewLayer!.clear();

    final layer = target.selectedLayer;

    if (!(layer is BitmapLayer)) {
      return;
    }

    Plot.ellipse(
      _startPosition.dx.toInt(),
      _startPosition.dy.toInt(),
      data.position.dx.toInt(),
      data.position.dy.toInt(),
      (x, y) {
        layer.setPixel(data.primaryColor, x, y);
      },
    );

    layer.invalidate();
  }
}
