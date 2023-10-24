import 'package:ciart_studio/stores/color_store.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:provider/provider.dart';

class PalettePanel extends StatefulWidget {
  const PalettePanel({super.key});

  @override
  _PalettePanelState createState() => _PalettePanelState();
}

class _PalettePanelState extends State<PalettePanel> {
  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Row(
          children: [
            IconButton(
              icon: const Icon(FluentIcons.add, size: 12.0),
              onPressed: () {
                final color = context.read<ColorStore>().primaryColor;
              },
            )
          ],
        ),
        Expanded(
          child: GridView.extent(
            maxCrossAxisExtent: 20,
            children: [Color(0x00)]
                .map(
                  (color) => Draggable(
                    feedback: Container(
                      constraints: BoxConstraints.expand(width: 20, height: 20),
                      color: color,
                    ),
                    childWhenDragging: Container(
                      constraints: BoxConstraints.expand(width: 20, height: 20),
                    ),
                    child: GestureDetector(
                      onTap: () => context
                          .read<ColorStore>()
                          .setPrimary(HSVColor.fromColor(color)),
                      child: Container(
                        constraints:
                            BoxConstraints.expand(width: 20, height: 20),
                        color: color,
                      ),
                    ),
                  ),
                )
                .toList(),
          ),
        )
      ],
    );
  }
}
