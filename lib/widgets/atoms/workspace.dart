import 'dart:ui';

import 'package:ciart_studio/stores/color_store.dart';
import 'package:ciart_studio/stores/document.dart';
import 'package:ciart_studio/stores/document_container.dart';
import 'package:ciart_studio/stores/tool_store.dart';
import 'package:ciart_studio/tools/tool.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:provider/provider.dart';

class Workspace extends StatefulWidget {
  const Workspace({Key? key, required this.document}) : super(key: key);

  final Document document;

  @override
  _WorkspaceState createState() => _WorkspaceState();
}

class _WorkspaceState extends State<Workspace> {
  var _offset = Offset.zero;
  var _rawScale = 1.0;
  var _scale = 1.0;
  var _prevLocalPosition = Offset.zero;
  var _origin = Offset.zero;

  @override
  void initState() {
    super.initState();
  }

  Offset _computeDocumentPosition(Offset localPosition) {
    return ((localPosition - _origin) / _scale);
  }

  void _updateDocumentPosition(Offset localPosition) {
    final toolStore = context.read<ToolStore>();

    toolStore.setPosition(_computeDocumentPosition(localPosition));
  }

  void _onPointerHover(PointerHoverEvent event) {
    _updateDocumentPosition(event.localPosition);
  }

  void _onPointerDown(PointerDownEvent event) {
    final documentContainer = context.read<DocumentContainer>();
    final toolStore = context.read<ToolStore>();
    final colorStore = context.read<ColorStore>();

    final tool = toolStore.focusTool;
    final primaryColor = colorStore.primaryColor;

    if (event.buttons & kSecondaryButton != 0) {
      setState(() {
        _prevLocalPosition = event.localPosition;
      });
    }

    var position = _computeDocumentPosition(event.localPosition);

    if (event.buttons & kPrimaryButton != 0) {
      tool.onPress(
        widget.document,
        ToolData(primaryColor: primaryColor, position: position),
      );
    }

    // focus (나중에 Nabi에서 해야 함)
    documentContainer.focus(widget.document.id);
  }

  void _onPointerMove(PointerMoveEvent event) {
    final toolStore = context.read<ToolStore>();
    final colorStore = context.read<ColorStore>();

    final tool = toolStore.focusTool;
    final primaryColor = colorStore.primaryColor;

    if (event.buttons & kSecondaryButton != 0) {
      setState(() {
        _offset += event.localPosition - _prevLocalPosition;
        _prevLocalPosition = event.localPosition;
      });
    }

    var position = _computeDocumentPosition(event.localPosition);

    if (event.buttons & kPrimaryButton != 0) {
      tool.onMove(
        widget.document,
        ToolData(primaryColor: primaryColor, position: position),
      );
    }
  }

  void _onPointerUp(PointerUpEvent event) {
    final toolStore = context.read<ToolStore>();
    final colorStore = context.read<ColorStore>();

    final tool = toolStore.focusTool;
    final primaryColor = colorStore.primaryColor;

    var position = _computeDocumentPosition(event.localPosition);

    tool.onRelease(
      widget.document,
      ToolData(primaryColor: primaryColor, position: position),
    );
  }

  void _onPointerSignal(PointerSignalEvent event) {
    // if (event is PointerScrollEvent) {
    //   setState(() {
    //     const minScale = 0.5;
    //     const maxScale = 5.0;

    //     _rawScale -= event.scrollDelta.dy / 100;

    //     if (_rawScale < minScale) {
    //       _rawScale = minScale;
    //     } else if (_rawScale > maxScale) {
    //       _rawScale = maxScale;
    //     }

    //     _scale = _rawScale * _rawScale * _rawScale;
    //   });
    // }
  }

  var _zoom = 1.0;

  void _onPointerPanZoomStart(PointerPanZoomStartEvent event) {
    setState(() {
      _zoom = 1.0;
    });
  }

  void _onPointerPanZoomUpdate(PointerPanZoomUpdateEvent event) {
    setState(() {
      _offset += event.panDelta;

      const minScale = 0.5;
      const maxScale = 5.0;

      _zoom = event.scale;

      var a = _rawScale * _zoom;

      if (a < minScale) {
        a = minScale;
      } else if (a > maxScale) {
        a = maxScale;
      }

      _scale = a * a * a;
    });
  }

  void _onPointerPanZoomEnd(PointerPanZoomEndEvent event) {
    setState(() {
      const minScale = 0.5;
      const maxScale = 5.0;

      _rawScale *= _zoom;

      if (_rawScale < minScale) {
        _rawScale = minScale;
      } else if (_rawScale > maxScale) {
        _rawScale = maxScale;
      }

      _scale = _rawScale * _rawScale * _rawScale;
    });
  }

  void _updateOrigin(Offset origin) {
    _origin = origin;
  }

  @override
  Widget build(BuildContext context) {
    return ConstrainedBox(
      constraints: BoxConstraints.expand(),
      child: Listener(
        onPointerHover: _onPointerHover,
        onPointerDown: _onPointerDown,
        onPointerMove: _onPointerMove,
        onPointerUp: _onPointerUp,
        onPointerSignal: _onPointerSignal,
        onPointerPanZoomStart: _onPointerPanZoomStart,
        onPointerPanZoomUpdate: _onPointerPanZoomUpdate,
        onPointerPanZoomEnd: _onPointerPanZoomEnd,
        child: RepaintBoundary(
          child: ClipRect(
            child: Observer(
              builder: (context) => CustomPaint(
                painter: _WorkspacePainter(
                  picture: widget.document.picture,
                  width: widget.document.width,
                  height: widget.document.height,
                  offset: _offset,
                  scale: _scale,
                  updateOrigin: _updateOrigin,
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class _WorkspacePainter extends CustomPainter {
  const _WorkspacePainter({
    required this.picture,
    required this.width,
    required this.height,
    required this.offset,
    required this.scale,
    required this.updateOrigin,
  }) : super();

  final Picture? picture;
  final int width;
  final int height;
  final Offset offset;
  final double scale;

  final void Function(Offset origin) updateOrigin;

  @override
  void paint(Canvas canvas, Size size) {
    final picture = this.picture;

    if (picture == null) {
      return;
    }

    // TODO: rect 이름 변경 + 최적화?
    var paint = Paint();
    var rect = Rect.fromCenter(
      center: size.center(offset),
      width: width * scale,
      height: height * scale,
    );
    var center = rect.center;

    updateOrigin(rect.topLeft);

    canvas.save();
    canvas.translate(center.dx, center.dy);
    canvas.drawColor(
      const Color.fromARGB(64, 0, 0, 0),
      BlendMode.srcOver,
    );

    canvas.drawRect(
      Rect.fromLTWH(
        -width / 2 * scale,
        -height / 2 * scale,
        width.toDouble() * scale,
        height.toDouble() * scale,
      ),
      Paint()..color = Color.fromARGB(255, 219, 219, 219),
    );

    canvas.save();
    canvas.scale(scale);
    canvas.translate(width / -2, height / -2);
    canvas.drawPicture(picture);
    canvas.restore();

    //// TODO: Picture로 최적화 필요
    // if (scale > 5) {
    //   canvas.translate(-width / 2 * scale, -height / 2 * scale);
    //   paint.color = Colors.black;
    //   paint.strokeWidth = 1;

    //   // TODO: size 변수에 따라 선이 선명하게 수정
    //   for (int i = 1; i < width; i++) {
    //     var x = (i * scale).floorToDouble() + 0.5;

    //     canvas.drawLine(Offset(x, 0), Offset(x, height * scale), paint);
    //   }

    //   for (int i = 1; i < height; i++) {
    //     var y = (i * scale).floorToDouble() + 0.5;

    //     canvas.drawLine(Offset(0, y), Offset(width * scale, y), paint);
    //   }
    // }

    canvas.restore();
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) {
    return true;
  }
}
