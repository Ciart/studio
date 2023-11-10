import 'dart:io';

import 'package:flutter/widgets.dart';
import 'package:flutter_acrylic/flutter_acrylic.dart';

import 'widgets/app.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  if (Platform.isWindows || Platform.isMacOS) {
    await Window.initialize();

    runApp(App());

    await Window.setEffect(effect: WindowEffect.mica, dark: false);
  } else {
    runApp(App());
  }

  if (Platform.isMacOS) {
    Window.overrideMacOSBrightness(
      dark: false,
    );
  }
}
