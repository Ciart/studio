import 'dart:io';

import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter/services.dart';
// import 'package:menubar/menubar.dart';

class MenuBar extends StatelessWidget {
  const MenuBar({Key? key, this.child}) : super(key: key);

  final Widget? child;

  @override
  Widget build(BuildContext context) {
    // if (Platform.isMacOS || Platform.isLinux) {
    //   setApplicationMenu([
    //     Submenu(
    //       label: 'File',
    //       children: [
    //         MenuItem(
    //           label: 'New File',
    //           enabled: true,
    //           shortcut: LogicalKeySet(
    //             LogicalKeyboardKey.meta,
    //             LogicalKeyboardKey.keyN,
    //           ),
    //           onClicked: () async {
    //             await showDialog(
    //               context: context,
    //               builder: (context) => ContentDialog(
    //                 title: const Text('New File'),
    //                 content: TextBox(),
    //                 actions: [
    //                   Button(
    //                     onPressed: () {
    //                       Navigator.pop(context, '');
    //                     },
    //                     child: const Text('확인'),
    //                   )
    //                 ],
    //               ),
    //             );
    //           },
    //         ),
    //         MenuDivider(),
    //         Submenu(
    //           label: 'Presets',
    //           children: [
    //             MenuItem(
    //               label: 'Red',
    //               enabled: true,
    //               shortcut: LogicalKeySet(
    //                 LogicalKeyboardKey.meta,
    //                 LogicalKeyboardKey.shift,
    //                 LogicalKeyboardKey.keyR,
    //               ),
    //               onClicked: () {},
    //             ),
    //           ],
    //         )
    //       ],
    //     ),
    //   ]);
    // }

    return Container(
      child: child,
    );
  }
}
