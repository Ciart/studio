import 'package:flutter/material.dart';

class Layout extends StatefulWidget {
  const Layout({Key? key, required this.registedWidgets, required this.config})
      : super(key: key);

  final Map<String, Widget> registedWidgets;
  final LayoutConfig config;

  @override
  _LayoutState createState() => _LayoutState();
}

class LayoutConfig {
  const LayoutConfig({required this.child});

  final LayoutItemConfig child;
}

enum LayoutItemType { widget, row, column }

class LayoutItemConfig {
  const LayoutItemConfig({required this.type, this.widgetName, this.children});

  final LayoutItemType type;
  final String? widgetName;
  final List<LayoutItemConfig>? children;
}

class _LayoutState extends State<Layout> {
  @override
  Widget build(BuildContext context) {
    return convertConfigToWidget(this.widget.config.child);
  }

  Widget convertConfigToWidget(LayoutItemConfig config) {
    switch (config.type) {
      case LayoutItemType.widget:
        return this.widget.registedWidgets[config.widgetName] ?? Container();
      case LayoutItemType.row:
        if (config.children == null) {
          return Container();
        }
        return Row(
            children: config.children!
                .map((child) => Expanded(child: convertConfigToWidget(child)))
                .toList());
      case LayoutItemType.column:
        if (config.children == null) {
          return Container();
        }
        return Column(
            children: config.children!
                .map((child) => Expanded(child: convertConfigToWidget(child)))
                .toList());
    }
  }
}
