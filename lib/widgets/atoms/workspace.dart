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
    final toolStore = context.read<ToolContainer>();

    toolStore.setPosition(_computeDocumentPosition(localPosition));
  }

  void _onPointerHover(PointerHoverEvent event) {
    _updateDocumentPosition(event.localPosition);
  }

  void _onPointerDown(PointerDownEvent event) {
    final documentContainer = context.read<DocumentContainer>();
    final toolStore = context.read<ToolContainer>();
    final colorStore = context.read<ColorStore>();

    final tool = toolStore.focusTool;
    final primaryColor = colorStore.primaryColor;

    if (event.kind == PointerDeviceKind.touch) {
      return;
    }

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
    final toolStore = context.read<ToolContainer>();
    final colorStore = context.read<ColorStore>();

    final tool = toolStore.focusTool;
    final primaryColor = colorStore.primaryColor;

    if (event.kind == PointerDeviceKind.touch) {
      return;
    }

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
    final toolStore = context.read<ToolContainer>();
    final colorStore = context.read<ColorStore>();

    final tool = toolStore.focusTool;
    final primaryColor = colorStore.primaryColor;

    if (event.kind == PointerDeviceKind.touch) {
      return;
    }

    var position = _computeDocumentPosition(event.localPosition);

    tool.onRelease(
      widget.document,
      ToolData(primaryColor: primaryColor, position: position),
    );
  }

  void _onPointerSignal(PointerSignalEvent event) {
    if (event is PointerScrollEvent && event.kind == PointerDeviceKind.mouse) {
      setState(() {
        const minScale = 0.5;
        const maxScale = 5.0;

        _rawScale -= event.scrollDelta.dy / 500;

        if (_rawScale < minScale) {
          _rawScale = minScale;
        } else if (_rawScale > maxScale) {
          _rawScale = maxScale;
        }

        _scale = _rawScale * _rawScale * _rawScale;
      });
    }
  }

  var _trackpadZoomOffset = 1.0;

  void _onPointerPanZoomStart(ScaleStartDetails event) {
    setState(() {
      _trackpadZoomOffset = 1.0;
    });
  }

  void _onPointerPanZoomUpdate(ScaleUpdateDetails event) {
    // if (event.pointerCount != 2) {
    //   return;
    // }

    setState(() {
      _offset += event.focalPointDelta;

      const minScale = 0.5;
      const maxScale = 1000.0;

      _trackpadZoomOffset = event.scale;

      var trackpadZoomScale = _rawScale * _trackpadZoomOffset;

      if (trackpadZoomScale < minScale) {
        trackpadZoomScale = minScale;
      } else if (trackpadZoomScale > maxScale) {
        trackpadZoomScale = maxScale;
      }

      _scale = trackpadZoomScale;
    });
  }

  void _onPointerPanZoomEnd(ScaleEndDetails event) {
    setState(() {
      const minScale = 0.5;
      const maxScale = 1000.0;

      _rawScale *= _trackpadZoomOffset;

      if (_rawScale < minScale) {
        _rawScale = minScale;
      } else if (_rawScale > maxScale) {
        _rawScale = maxScale;
      }

      _scale = _rawScale;
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
        // onPointerPanZoomStart: _onPointerPanZoomStart,
        // onPointerPanZoomUpdate: _onPointerPanZoomUpdate,
        // onPointerPanZoomEnd: _onPointerPanZoomEnd,
        child: RawGestureDetector(
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
                    gridSize: 16,
                    updateOrigin: _updateOrigin,
                  ),
                ),
              ),
            ),
          ),
          gestures: {
            ScaleGestureRecognizer:
                GestureRecognizerFactoryWithHandlers<ScaleGestureRecognizer>(
              () => ScaleGestureRecognizer(
                supportedDevices: {
                  PointerDeviceKind.touch,
                  PointerDeviceKind.trackpad
                },
              ),
              (instance) {
                instance.onStart = _onPointerPanZoomStart;
                instance.onUpdate = _onPointerPanZoomUpdate;
                instance.onEnd = _onPointerPanZoomEnd;
              },
            ),
          },
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
    required this.gridSize,
    required this.updateOrigin,
  }) : super();

  final Picture? picture;
  final int width;
  final int height;
  final Offset offset;
  final double scale;
  final int gridSize;

  final void Function(Offset origin) updateOrigin;

  void drawCheckPattern(Canvas canvas, Rect rect, double gridSize) {
    final paint = Paint()
      ..color = Colors.white
      ..style = PaintingStyle.fill;

    canvas.drawRect(rect, paint);

    paint.color = Colors.grey[20];

    for (int i = 0; i < rect.width / gridSize; i++) {
      for (int j = 0; j < rect.height / gridSize; j++) {
        if ((i + j) % 2 == 0) {
          continue;
        }

        final cellRect = Rect.fromLTWH(
          rect.left + i * gridSize,
          rect.top + j * gridSize,
          gridSize,
          gridSize,
        );

        canvas.drawRect(cellRect, paint);
      }
    }
  }

  void drawPicture(Canvas canvas, Picture picture) {
    canvas.save();
    canvas.scale(scale);
    canvas.translate(width / -2, height / -2);
    canvas.drawPicture(picture);
    canvas.restore();
  }

  void drawPixelGird(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = Colors.grey[60]
      ..strokeWidth = 0.5;

    canvas.save();

    canvas.translate(-width / 2 * scale, -height / 2 * scale);

    // TODO: size 변수에 따라 선이 선명하게 수정
    for (int i = 1; i < width; i++) {
      var x = (i * scale) + 0.5;

      canvas.drawLine(Offset(x, 0), Offset(x, height * scale), paint);
    }

    for (int i = 1; i < height; i++) {
      var y = (i * scale) + 0.5;

      canvas.drawLine(Offset(0, y), Offset(width * scale, y), paint);
    }

    canvas.restore();
  }

  @override
  void paint(Canvas canvas, Size size) {
    final picture = this.picture;

    if (picture == null) {
      return;
    }

    final center = size.center(offset);
    final origin =
        Offset(center.dx - width / 2 * scale, center.dy - height / 2 * scale);

    updateOrigin(origin);

    canvas.save();
    canvas.translate(center.dx, center.dy);

    drawCheckPattern(
      canvas,
      Rect.fromLTWH(
        -width / 2 * scale,
        -height / 2 * scale,
        width.toDouble() * scale,
        height.toDouble() * scale,
      ),
      this.gridSize * scale,
    );

    drawPicture(canvas, picture);

    if (scale > 10) {
      drawPixelGird(canvas, size);
    }

    canvas.restore();
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) {
    return true;
  }
}
