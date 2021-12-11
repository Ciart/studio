import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'package:flutter/material.dart';

final windowButtonColors =
    WindowButtonColors(iconNormal: const Color(0xff000000));

class TitleBar extends StatelessWidget {
  const TitleBar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return WindowTitleBarBox(
      child: Row(
        children: [
          Expanded(
            child: MoveWindow(),
          ),
          Row(
            children: [
              MinimizeWindowButton(colors: windowButtonColors),
              MaximizeWindowButton(colors: windowButtonColors),
              CloseWindowButton(colors: windowButtonColors),
            ],
          ),
        ],
      ),
    );
  }
}
