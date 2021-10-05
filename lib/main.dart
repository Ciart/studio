import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'widgets/app.dart';

void main() {
  runApp(ProviderScope(child: App()));

  doWhenWindowReady(() {
    final initialSize = Size(1024, 768);
    appWindow.minSize = initialSize;
    appWindow.size = initialSize;
    appWindow.alignment = Alignment.center;
    appWindow.show();
  });
}
