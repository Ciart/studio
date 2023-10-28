import 'package:ciart_studio/stores/color_store.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:provider/provider.dart';

class PalettePanel extends StatefulWidget {
  const PalettePanel({super.key});

  @override
  _PalettePanelState createState() => _PalettePanelState();
}

class _PalettePanelState extends State<PalettePanel> {
  @override
  Widget build(BuildContext context) {
    final colorStore = context.read<ColorStore>();

    return Column(
      children: [
        Row(
          children: [
            IconButton(
              icon: const Icon(FluentIcons.add, size: 12.0),
              onPressed: () {
                colorStore.addColor(colorStore.primaryColor);
              },
            )
          ],
        ),
        Expanded(
          child: Observer(
            builder: (context) {
              return GridView.extent(
                maxCrossAxisExtent: 20,
                children: colorStore.palette
                    .map(
                      (color) => Draggable<Color>(
                        data: color,
                        feedback: Container(
                          constraints:
                              BoxConstraints.expand(width: 20, height: 20),
                          color: color,
                        ),
                        childWhenDragging: Container(
                          constraints:
                              BoxConstraints.expand(width: 20, height: 20),
                        ),
                        child: GestureDetector(
                          onTap: () =>
                              colorStore.setPrimary(HSVColor.fromColor(color)),
                          child: DragTarget<Color>(
                            builder: (
                              BuildContext context,
                              List<Color?> candidateData,
                              List<dynamic> rejectedData,
                            ) {
                              return Container(
                                constraints: BoxConstraints.expand(
                                  width: 20,
                                  height: 20,
                                ),
                                color: color,
                              );
                            },
                            onAccept: (data) =>
                                colorStore.swapColor(color, data),
                          ),
                        ),
                      ),
                    )
                    .toList(),
              );
            },
          ),
        )
      ],
    );
  }
}
