import 'package:flutter/widgets.dart';

import 'a_panel.dart';
import 'b_panel.dart';
import 'layout.dart';

class App extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return WidgetsApp(
      title: 'doter',
      color: const Color(0xFFFFFFFF),
      debugShowCheckedModeBanner: false,
      builder: (context, int) {
        return Layout(
          registedWidgets: {
            'a': Builder(builder: (context) => APanel()),
            'b': Builder(builder: (context) => BPanel())
          },
          config: LayoutConfig(
              child: LayoutItemConfig(type: LayoutItemType.row, children: [
            LayoutItemConfig(type: LayoutItemType.widget, widgetName: 'a'),
            LayoutItemConfig(type: LayoutItemType.widget, widgetName: 'b'),
            LayoutItemConfig(type: LayoutItemType.column, children: [
              LayoutItemConfig(type: LayoutItemType.widget, widgetName: 'a'),
              LayoutItemConfig(type: LayoutItemType.widget, widgetName: 'b'),
              LayoutItemConfig(type: LayoutItemType.widget, widgetName: 'a'),
            ]),
          ])),
        );
      },
    );
  }
}
