class Plot {
  static void line(
    int startX,
    int startY,
    int endX,
    int endY,
    void Function(int x, int y) onPlot,
  ) {
    int dx = (endX - startX).abs(), sx = startX < endX ? 1 : -1;
    int dy = -(endY - startY).abs(), sy = startY < endY ? 1 : -1;
    int err = dx + dy, e2;
    int x = startX, y = startY;

    while (true) {
      onPlot(x, y);

      if (x == endX && y == endY) {
        break;
      }

      e2 = 2 * err;

      if (e2 >= dy) {
        err += dy;
        x += sx;
      }

      if (e2 <= dx) {
        err += dx;
        y += sy;
      }
    }
  }

  static void ellipse(
    int x0,
    int y0,
    int x1,
    int y1,
    void Function(int x, int y) onPlot,
  ) {
    int a = (x1 - x0).abs();
    int b = (y1 - y0).abs();
    int b1 = b & 1;
    double dx = 4 * (1.0 - a) * b * b;
    double dy = 4 * (b1.toDouble() + 1) * a * a;
    double err = dx + dy + b1 * a * a;
    double e2;

    if (x0 > x1) {
      x0 = x1;
      x1 += a;
    }

    if (y0 > y1) {
      y0 = y1;
    }

    y0 += (b + 1) ~/ 2;
    y1 = y0 - b1;
    a = 8 * a * a;
    b1 = 8 * b * b;

    do {
      onPlot(x1, y0);
      onPlot(x0, y0);
      onPlot(x0, y1);
      onPlot(x1, y1);

      e2 = 2 * err;

      if (e2 <= dy) {
        y0++;
        y1--;
        err += dy += a;
      }

      if (e2 >= dx || 2 * err > dy) {
        x0++;
        x1--;
        err += dx += b1;
      }
    } while (x0 <= x1);

    while (y0 - y1 <= b) {
      onPlot(x0 - 1, y0);
      onPlot(x1 + 1, y0++);
      onPlot(x0 - 1, y1);
      onPlot(x1 + 1, y1--);
    }
  }

  static void rectangle(
    int startX,
    int startY,
    int endX,
    int endY,
    void Function(int x, int y) onPlot,
  ) {
    int minX = startX < endX ? startX : endX;
    int maxX = startX > endX ? startX : endX;
    int minY = startY < endY ? startY : endY;
    int maxY = startY > endY ? startY : endY;

    for (int x = minX; x <= maxX; x++) {
      onPlot(x, minY);
      onPlot(x, maxY);
    }

    for (int y = minY; y <= maxY; y++) {
      onPlot(minX, y);
      onPlot(maxX, y);
    }
  }
}
