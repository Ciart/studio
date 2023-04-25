import 'package:stellon/providers/color.dart';
import 'package:stellon/widgets/atoms/color_picker.dart';
import 'package:fluent_ui/fluent_ui.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';

class ColorPickerPanel extends ConsumerStatefulWidget {
  const ColorPickerPanel({Key? key}) : super(key: key);

  @override
  ConsumerState<ConsumerStatefulWidget> createState() =>
      _ColorPickerPanelState();
}

class _ColorPickerPanelState extends ConsumerState<ColorPickerPanel> {
  late HSVColor primaryHSVColor;

  @override
  void initState() {
    super.initState();

    primaryHSVColor = HSVColor.fromColor(ref.read(primaryColorProvider));
  }

  @override
  Widget build(BuildContext context) {
    ref.listen<Color>(primaryColorProvider, (previous, next) {
      setState(() {
        primaryHSVColor = HSVColor.fromColor(next);
      });
    });

    return ColorPicker(
      color: primaryHSVColor,
      onChange: (color) {
        var primaryColor = ref.read(primaryColorProvider.notifier);
        primaryColor.state = color.toColor();

        setState(() {
          primaryHSVColor = color;
        });
      },
    );
  }
}
