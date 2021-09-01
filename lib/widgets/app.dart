import 'package:doter/controllers/core_controller.dart';
import 'package:doter/controllers/document_controller.dart';
import 'package:doter/widgets/color_picker.dart';
import 'package:doter/widgets/menu_bar.dart';
import 'package:doter/widgets/status_bar.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';

import 'workspace.dart';

class App extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    Get.put(CoreController());
    Get.put(DocumentController());

    return MaterialApp(
        title: 'doter',
        color: const Color(0xFFFFFFFF),
        debugShowCheckedModeBanner: false,
        home: Scaffold(
            body: MenuBar(
          child: Column(
            children: [
              Expanded(
                child: Row(
                  children: [
                    Flexible(
                        flex: 4,
                        child: ConstrainedBox(
                            constraints: BoxConstraints.expand(),
                            child: Workspace())),
                    Flexible(flex: 1, child: ColorPicker())
                  ],
                ),
              ),
              StatusBar()
            ],
          ),
        )));
  }
}
