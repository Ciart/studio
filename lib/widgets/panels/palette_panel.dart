import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:stellon/providers/color.dart';

class PalettePanel extends ConsumerStatefulWidget {
  const PalettePanel({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() => _PalettePanelState();
}

class _PalettePanelState extends ConsumerState<PalettePanel> {
  late List<Color> _colors;

  @override
  void initState() {
    super.initState();

    _colors = [];
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Row(
          children: [
            IconButton(
              icon: const Icon(FluentIcons.add, size: 12.0),
              onPressed: () {
                final color = ref.read(primaryColorProvider);

                setState(() {
                  _colors.add(color);
                });
              },
            )
          ],
        ),
        Expanded(
          child: GridView.extent(
            maxCrossAxisExtent: 20,
            children: _colors
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
                      onTap: () =>
                          ref.read(primaryColorProvider.notifier).state = color,
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
