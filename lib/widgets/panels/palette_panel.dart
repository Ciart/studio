import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:stellon/blocs/palette_cubit.dart';
import 'package:stellon/providers/color.dart';
import 'package:stellon/repositories/document_repository.dart';

class PalettePanel extends ConsumerStatefulWidget {
  const PalettePanel({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() => _PalettePanelState();
}

class _PalettePanelState extends ConsumerState<PalettePanel> {
  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (context) => PaletteCubit(context.read<DocumentRepository>()),
      child: Builder(
        builder: (context) {
          return Column(
            children: [
              Row(
                children: [
                  IconButton(
                    icon: const Icon(FluentIcons.add, size: 12.0),
                    onPressed: () {
                      final color = ref.read(primaryColorProvider);

                      context.read<PaletteCubit>().addColor(color);
                    },
                  )
                ],
              ),
              Expanded(
                child: BlocBuilder<PaletteCubit, List<Color>>(
                  builder: (context, state) {
                    return GridView.extent(
                      maxCrossAxisExtent: 20,
                      children: state
                          .map(
                            (color) => Draggable(
                              feedback: Container(
                                constraints: BoxConstraints.expand(
                                    width: 20, height: 20),
                                color: color,
                              ),
                              childWhenDragging: Container(
                                constraints: BoxConstraints.expand(
                                    width: 20, height: 20),
                              ),
                              child: GestureDetector(
                                onTap: () => ref
                                    .read(primaryColorProvider.notifier)
                                    .state = color,
                                child: Container(
                                  constraints: BoxConstraints.expand(
                                      width: 20, height: 20),
                                  color: color,
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
        },
      ),
    );
  }
}
