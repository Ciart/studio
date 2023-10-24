import 'package:ciart_studio/stores/color_store.dart';
import 'package:ciart_studio/stores/document_container.dart';
import 'package:ciart_studio/stores/tool_store.dart';
import 'package:ciart_studio/widgets/bars/menu_bar.dart';
import 'package:ciart_studio/widgets/bars/property_bar.dart';
import 'package:ciart_studio/widgets/bars/status_bar.dart';
import 'package:ciart_studio/widgets/bars/tool_bar.dart';
import 'package:ciart_studio/widgets/panels/color_picker_panel.dart';
import 'package:ciart_studio/widgets/panels/layer_panel.dart';
import 'package:ciart_studio/widgets/panels/palette_panel.dart';
import 'package:ciart_studio/widgets/panels/workspace_panel.dart';
import 'package:ciart_studio/widgets/bars/title_bar.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:nabi/nabi.dart';
import 'package:provider/provider.dart';

const title = 'Ciart Studio';

class App extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MultiProvider(
      providers: [
        Provider(
          create: (_) => Layout(
            LayoutFlex(
              direction: Axis.vertical,
              children: [
                LayoutWidget(name: 'propertyBar', size: 1),
                LayoutFlex(
                  size: 18,
                  direction: Axis.horizontal,
                  children: [
                    LayoutWidget(name: 'toolBar', size: 1),
                    LayoutStack(id: 'workspace', size: 4, children: []),
                    LayoutFlex(
                      children: [
                        LayoutWidget(name: 'colorPicker', size: 1),
                        LayoutWidget(name: 'palettePanel', size: 1),
                        LayoutWidget(name: 'layerPanel', size: 1)
                      ],
                      direction: Axis.vertical,
                    ),
                  ],
                ),
              ],
            ),
          ),
        ),
        Provider(
          create: (_) => ColorStore(),
        ),
        Provider(
          create: (_) => ToolStore(),
        ),
        Provider(create: (_) => DocumentContainer()),
      ],
      child: FluentApp(
        title: title,
        color: const Color(0xFFFFFFFF),
        debugShowCheckedModeBanner: false,
        theme: FluentThemeData(
          brightness: Brightness.light,
          accentColor: Colors.teal,
        ),
        home: Column(
          children: [
            TitleBar(),
            Expanded(
              child: MenuBar(
                child: Column(
                  children: [
                    Expanded(
                      child: Consumer<Layout>(
                        builder: (context, layout, child) {
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
