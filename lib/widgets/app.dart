import 'package:doter/providers/ui.dart';
import 'package:doter/widgets/bars/menu_bar.dart';
import 'package:doter/widgets/bars/property_bar.dart';
import 'package:doter/widgets/bars/tool_bar.dart';
import 'package:doter/widgets/panels/color_picker_panel.dart';
import 'package:doter/widgets/panels/layer_panel.dart';
import 'package:doter/widgets/panels/workspace_panel.dart';
import 'package:doter/widgets/bars/title_bar.dart';
import 'package:fluent_ui/fluent_ui.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:nabi/nabi.dart';

import 'bars/status_bar.dart';

const title = 'Doter';

class App extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return FluentApp(
      title: title,
      color: const Color(0xFFFFFFFF),
      debugShowCheckedModeBanner: false,
      theme: ThemeData(brightness: Brightness.light, accentColor: Colors.blue),
      home: NavigationView(
        appBar: NavigationAppBar(title: const Text(title), actions: TitleBar()),
        content: Column(
          children: [
            TitleBar(),
            Expanded(
              child: MenuBar(
                child: Column(
                  children: [
                    Expanded(
                      child: Consumer(
                        builder: (context, ref, child) {
                          final layout = ref.watch(layoutProvider);

                          return Nabi(
                            registeredWidgets: {
                              'propertyBar': (context) => PropertyBar(),
                              'toolBar': (context) => ToolBar(),
                              'workspacePanel': (context) => WorkspacePanel(),
                              'colorPicker': (context) => ColorPickerPanel(),
                              'layerPanel': (context) => LayerPanel()
                            },
                            layout: layout,
                          );
                        },
                      ),
                    ),
                    StatusBar(),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
