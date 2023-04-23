import 'package:stellon/providers/ui.dart';
import 'package:stellon/widgets/bars/menu_bar.dart';
import 'package:stellon/widgets/bars/property_bar.dart';
import 'package:stellon/widgets/bars/tool_bar.dart';
import 'package:stellon/widgets/panels/color_picker_panel.dart';
import 'package:stellon/widgets/panels/layer_panel.dart';
import 'package:stellon/widgets/panels/palette_panel.dart';
import 'package:stellon/widgets/panels/workspace_panel.dart';
import 'package:stellon/widgets/bars/title_bar.dart';
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
      theme: FluentThemeData(
        brightness: Brightness.light,
        accentColor: Colors.teal,
      ),
      home: NavigationView(
        // appBar: NavigationAppBar(title: const Text(title), actions: TitleBar()),
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
                              'layerPanel': (context) => LayerPanel(),
                              'palettePanel': (context) => PalettePanel(),
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
