import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ciart_studio/providers/color.dart';

class PalettePanel extends ConsumerStatefulWidget {
  const PalettePanel({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() => _PalettePanelState();
}

class _PalettePanelState extends ConsumerState<PalettePanel> {
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
