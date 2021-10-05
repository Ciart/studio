import 'dart:ui';

import 'package:doter/models/document.dart';
import 'package:doter/providers/color.dart';
import 'package:doter/providers/document.dart';
import 'package:doter/providers/ui.dart';
import 'package:doter/providers/tool.dart';
import 'package:doter/tools/tool.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter/gestures.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';

class Workspace extends ConsumerStatefulWidget {
  const Workspace({Key? key, required this.document}) : super(key: key);

  final Document document;

  @override
  _WorkspaceState createState() => _WorkspaceState();
}

class _WorkspaceState extends ConsumerState<Workspace> {
  var _offset = Offset.zero;
  var _rawScale = 1.0;
  var _scale = 1.0;
  var _prevLocalPosition = Offset.zero;
  var _origin = Offset.zero;

  Offset _computeDocumentPosition(Offset localPosition) {
    return ((localPosition - _origin) / _scale);
  }

  void _updateDocumentPosition(Offset localPosition) {
    var documentPosition = ref.watch(documentPositionProvider.state);
    documentPosition.state = _computeDocumentPosition(localPosition);
  }

  void _onPointerHover(PointerHoverEvent event) {
    _updateDocumentPosition(event.localPosition);
  }

  void _onPointerDown(PointerDownEvent event) {
    var primaryColor = ref.read(primaryColorProvider);
    var tool = ref.read(toolProvider);

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
    ref.watch(focusDocumentIdProvider.notifier).state = widget.document.id;
  }

  void _onPointerMove(PointerMoveEvent event) {
    var primaryColor = ref.read(primaryColorProvider);
    var tool = ref.read(toolProvider);

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
    var primaryColor = ref.read(primaryColorProvider);
    var tool = ref.read(toolProvider);

    var position = _computeDocumentPosition(event.localPosition);

    tool.onRelease(
      widget.document,
      ToolData(primaryColor: primaryColor, position: position),
    );
  }

  void _onPointerSignal(PointerSignalEvent event) {
    if (event is PointerScrollEvent) {
      setState(() {
        const minScale = 0.5;
        const maxScale = 5.0;

        _rawScale -= event.scrollDelta.dy / 100;

        if (_rawScale < minScale) {
          _rawScale = minScale;
        } else if (_rawScale > maxScale) {
          _rawScale = maxScale;
        }

        _scale = _rawScale * _rawScale * _rawScale;
      });
    }
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
        child: RepaintBoundary(
          child: ClipRect(
            child: CustomPaint(
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
