import 'dart:ui' as ui;

import 'package:doter/controllers/core_controller.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';

class Workspace extends StatefulWidget {
  const Workspace({Key? key}) : super(key: key);

  @override
  _WorkspaceState createState() => _WorkspaceState();
}

class _WorkspaceState extends State<Workspace> {
  var _offset = Offset.zero;
  var _scale = 1.0;
  var _localFocalPoint = Offset.zero;
  var _center = Offset.zero;

  CoreController coreController = Get.find();

  void _updateDocumentPosition(Offset localPosition) {
    var document = coreController.document.value;
    var documentSize =
        Offset(document.width.toDouble(), document.height.toDouble());

    coreController.documentPosition.value =
        (localPosition - _offset - _center + documentSize * _scale / 2) /
            _scale;
  }

  void _onPointerMove(PointerMoveEvent event) {
    _updateDocumentPosition(event.localPosition);
  }

  void _onPointerSignal(PointerSignalEvent event) {
    if (event is PointerScrollEvent) {
      setState(() {
        var prevScale = _scale;

        _scale -= event.scrollDelta.dy / 100;

        if (_scale < 0.1) {
          _scale = 0.1;
        } else if (_scale > 20) {
          _scale = 20;
        }
      });
    }
  }

  void _onHover(PointerHoverEvent event) {
    _updateDocumentPosition(event.localPosition);
  }

  void _onScaleStart(ScaleStartDetails details) {
    setState(() {
      _localFocalPoint = details.localFocalPoint;
    });
  }

  void _onScaleUpdate(ScaleUpdateDetails details) {
    setState(() {
      _offset += details.localFocalPoint - _localFocalPoint;
      _localFocalPoint = details.localFocalPoint;

      _scale -= details.scale / 100;

      if (_scale < 0.1) {
        _scale = 0.1;
      } else if (_scale > 20) {
        _scale = 20;
      }
    });
  }

  void _updateCenter(Offset center) {
    _center = center;
  }

  @override
  Widget build(BuildContext context) {
    return Listener(
        onPointerMove: _onPointerMove,
        onPointerSignal: _onPointerSignal,
        child: MouseRegion(
          onHover: _onHover,
          child: GestureDetector(
              onScaleStart: _onScaleStart,
              onScaleUpdate: _onScaleUpdate,
              child: ClipRect(
                  child: Obx(
                () => CustomPaint(
                  painter: _WorkspacePainter(
                      coreController.document.value.image,
                      _offset,
                      _scale,
                      _updateCenter),
                ),
              ))),
        ));
  }
}

class _WorkspacePainter extends CustomPainter {
  const _WorkspacePainter(
      this.image, this.offset, this.scale, this.updateCenter)
      : super();

  final ui.Image? image;
  final Offset offset;
  final double scale;

  final void Function(Offset center) updateCenter;

  @override
  void paint(Canvas canvas, Size size) {
    var paint = Paint();

    Offset center = Offset(size.width / 2, size.height / 2);
    updateCenter(center);

    if (image != null) {
      canvas.drawImageRect(
          image!,
          Rect.fromLTWH(0, 0, 100, 100),
          Rect.fromCenter(
              center: center + offset,
              width: (image?.width ?? 0) * scale,
              height: (image?.height ?? 0) * scale),
          paint);
    }
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) {
    return true;
  }
}
