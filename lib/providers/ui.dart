import 'package:flutter/widgets.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:nabi/nabi.dart';

final layoutProvider = Provider(
  (ref) => Layout(
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
);

final documentPositionProvider = StateProvider<Offset>((ref) => Offset.zero);
