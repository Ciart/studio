import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class PalettePanel extends ConsumerStatefulWidget {
  const PalettePanel({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() => _PalettePanelState();
}

class _PalettePanelState extends ConsumerState<PalettePanel> {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Text("palette"),
    );
  }
}
