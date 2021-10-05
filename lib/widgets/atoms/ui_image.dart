import 'dart:ui' as ui;

import 'package:fluent_ui/fluent_ui.dart';

class UiImage extends StatelessWidget {
  const UiImage({
    Key? key,
    required this.image,
    this.width,
    this.height,
  }) : super(key: key);

  final ui.Image image;

  final double? width;

  final double? height;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: width ?? image.width.toDouble(),
      height: height ?? image.height.toDouble(),
      child: CustomPaint(painter: _UiImagePainter(image)),
    );
  }
}

class _UiImagePainter extends CustomPainter {
  _UiImagePainter(this.image);

  final ui.Image image;

  @override
  void paint(Canvas canvas, Size size) {
    canvas.drawImageRect(
      image,
      Rect.fromLTWH(0, 0, image.width.toDouble(), image.height.toDouble()),
      Rect.fromLTWH(0, 0, size.width, size.height),
      Paint(),
    );
  }

  @override
  bool shouldRepaint(_UiImagePainter oldDelegate) {
    return image != oldDelegate.image;
  }
}
