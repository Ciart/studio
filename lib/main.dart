import 'dart:io';

import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_acrylic/flutter_acrylic.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'widgets/app.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await Window.initialize();
  runApp(ProviderScope(child: App()));

  await Window.setEffect(effect: WindowEffect.mica, dark: false);

  if (Platform.isMacOS) {
    Window.overrideMacOSBrightness(
      dark: false,
    );
  }

  doWhenWindowReady(() {
    final initialSize = Size(1024, 768);
    appWindow.minSize = initialSize;
    appWindow.size = initialSize;
    appWindow.alignment = Alignment.center;
    appWindow.show();
  });
}
