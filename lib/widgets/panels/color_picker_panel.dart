import 'package:ciart_studio/stores/color_store.dart';
import 'package:ciart_studio/widgets/atoms/color_picker.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:provider/provider.dart';

class ColorPickerPanel extends StatefulWidget {
  const ColorPickerPanel({Key? key}) : super(key: key);

  @override
  _ColorPickerPanelState createState() => _ColorPickerPanelState();
}

class _ColorPickerPanelState extends State<ColorPickerPanel> {
  @override
  Widget build(BuildContext context) {
    final colorStore = context.read<ColorStore>();

    return Observer(
      builder: (context) => ColorPicker(
        color: colorStore.primary,
        onChange: (color) {
          colorStore.setPrimary(color);
        },
      ),
    );
  }
}
