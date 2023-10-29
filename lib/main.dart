import 'dart:io';

import 'package:flutter/widgets.dart';
import 'package:flutter_acrylic/flutter_acrylic.dart';

import 'widgets/app.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await Window.initialize();
  runApp(App());

  await Window.setEffect(effect: WindowEffect.mica, dark: false);

  if (Platform.isMacOS) {
    Window.overrideMacOSBrightness(
      dark: false,
    );
  }
}
