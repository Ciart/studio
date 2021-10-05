import 'dart:math';

import 'package:fluent_ui/fluent_ui.dart';

typedef ColorChangeEventListener = void Function(HSVColor color);

class ColorPicker extends StatelessWidget {
  const ColorPicker({
    Key? key,
    required this.color,
    required this.onChange,
  }) : super(key: key);

  final HSVColor color;
  final ColorChangeEventListener onChange;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        var size = min(constraints.maxWidth, constraints.maxHeight);
        var rectSize = sqrt(pow(size - 16 * 2, 2) / 2) - 8;

        return Stack(
          children: [
            Center(
              child: SizedBox(
                width: size,
                height: size,
                child: HueRing(
                  color: color,
                  onChange: onChange,
                ),
              ),
            ),
            Center(
              child: SizedBox(
                width: rectSize,
                height: rectSize,
                child: GradientBox(
                  color: color,
                  onChange: onChange,
                ),
              ),
            ),
          ],
        );
      },
    );
  }
}

class HueRing extends StatelessWidget {
  const HueRing({
    Key? key,
    required this.color,
    required this.onChange,
  }) : super(key: key);

  final HSVColor color;
  final ColorChangeEventListener onChange;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        final center =
            Offset(constraints.maxWidth / 2, constraints.maxHeight / 2);

        return GestureDetector(
          onPanStart: (details) {
            final position = details.localPosition - center;
            final angle = atan2(position.dy, position.dx) * 180 / pi;

            onChange(
              color.withHue(
                angle >= 0 ? angle : 360 + angle,
              ),
            );
          },
          onPanUpdate: (details) {
            final position = details.localPosition - center;
            final angle = atan2(position.dy, position.dx) * 180 / pi;

            onChange(
              color.withHue(
                angle >= 0 ? angle : 360 + angle,
              ),
            );
          },
          child: CustomPaint(
            painter: _HueRingPainter(color: color),
          ),
        );
      },
    );
  }
}

class _HueRingPainter extends CustomPainter {
  const _HueRingPainter({required this.color}) : super();

  final HSVColor color;
  final double ringWidth = 16;

  @override
  void paint(Canvas canvas, Size size) {
    var center = Offset(size.width / 2, size.height / 2);
    var paint = Paint()
      ..shader = SweepGradient(
        colors: [
          const HSVColor.fromAHSV(1, 0, 1, 1).toColor(),
          const HSVColor.fromAHSV(1, 60, 1, 1).toColor(),
          const HSVColor.fromAHSV(1, 120, 1, 1).toColor(),
          const HSVColor.fromAHSV(1, 180, 1, 1).toColor(),
          const HSVColor.fromAHSV(1, 240, 1, 1).toColor(),
          const HSVColor.fromAHSV(1, 300, 1, 1).toColor(),
          const HSVColor.fromAHSV(1, 360, 1, 1).toColor(),
        ],
      ).createShader(
        Rect.fromCenter(
          center: center,
          width: size.width,
          height: size.height,
        ),
      )
      ..style = PaintingStyle.stroke
      ..strokeWidth = ringWidth;

    var radius = (size.width - ringWidth) / 2;
    var angle = color.hue / 360 * 2 * pi;

    canvas.drawCircle(center, radius, paint);

    canvas.translate(center.dx, center.dy);

    canvas.drawCircle(
      Offset(
        cos(angle) * radius,
        sin(angle) * radius,
      ),
      ringWidth / 2,
      Paint()
        ..style = PaintingStyle.stroke
        ..strokeWidth = 1,
    );

    canvas.drawCircle(
      Offset(
        cos(angle) * radius,
        sin(angle) * radius,
      ),
      ringWidth / 2 - 1,
      Paint()
        ..color = const Color(0xffffffff)
        ..style = PaintingStyle.stroke
        ..strokeWidth = 1,
    );
  }

  @override
  bool shouldRepaint(_HueRingPainter oldDelegate) => oldDelegate.color != color;
}

class GradientBox extends StatelessWidget {
  const GradientBox({
    Key? key,
    required this.color,
    required this.onChange,
  }) : super(key: key);

  final HSVColor color;
  final ColorChangeEventListener onChange;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        return GestureDetector(
          onPanStart: (details) {
            var localPosition = details.localPosition;

            onChange(
              HSVColor.fromAHSV(
                1,
                color.hue,
                localPosition.dx.clamp(0, constraints.maxWidth) /
                    constraints.maxWidth,
                1 -
                    (localPosition.dy.clamp(0, constraints.maxHeight) /
                        constraints.maxHeight),
              ),
            );
          },
          onPanUpdate: (details) {
            var localPosition = details.localPosition;

            onChange(
              HSVColor.fromAHSV(
                1,
                color.hue,
                localPosition.dx.clamp(0, constraints.maxWidth) /
                    constraints.maxWidth,
                1 -
                    (localPosition.dy.clamp(0, constraints.maxHeight) /
                        constraints.maxHeight),
              ),
            );
          },
          child: CustomPaint(
            painter: _GradientBox(color: color),
          ),
        );
      },
    );
  }
}

class _GradientBox extends CustomPainter {
  const _GradientBox({required this.color}) : super();

  final HSVColor color;

  @override
  void paint(Canvas canvas, Size size) {
    var rect = Offset.zero & size;

    canvas.drawRect(
      rect,
      Paint()
        ..shader = LinearGradient(
          colors: [
            HSVColor.fromAHSV(1, color.hue, 0, 1).toColor(),
            HSVColor.fromAHSV(1, color.hue, 1, 1).toColor()
          ],
        ).createShader(rect),
    );

    canvas.drawRect(
      rect,
      Paint()
        ..blendMode = BlendMode.multiply
        ..shader = LinearGradient(
          begin: Alignment.topCenter,
          end: Alignment.bottomCenter,
          colors: [
            const HSVColor.fromAHSV(1, 0, 0, 1).toColor(),
            const HSVColor.fromAHSV(1, 0, 0, 0).toColor()
          ],
        ).createShader(rect),
    );

    canvas.drawCircle(
      Offset(
        color.saturation * rect.width,
        (1 - color.value) * rect.height,
      ),
      5,
      Paint()
        ..style = PaintingStyle.stroke
        ..strokeWidth = 1,
    );

    canvas.drawCircle(
      Offset(
        color.saturation * rect.width,
        (1 - color.value) * rect.height,
      ),
      4,
      Paint()
        ..color = const Color(0xffffffff)
        ..style = PaintingStyle.stroke
        ..strokeWidth = 1,
    );
  }

  @override
  bool shouldRepaint(_GradientBox oldDelegate) => oldDelegate.color != color;
}
