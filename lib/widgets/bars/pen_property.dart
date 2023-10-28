import 'package:ciart_studio/stores/tool_store.dart';
import 'package:ciart_studio/tools/pen.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:provider/provider.dart';

class PenProperty extends StatelessWidget {
  const PenProperty({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final tool = context.read<ToolContainer>().focusTool;

    if (tool is! Pen) {
      return Container();
    }

    return Observer(
      builder: (context) {
        return Row(
          children: [
            Slider(
              label: '${tool.size}',
              style: SliderThemeData(labelBackgroundColor: Colors.black),
              value: tool.size.toDouble(),
              min: 1,
              max: 10,
              onChanged: (value) => tool.size = value.toInt(),
            ),
          ],
        );
      },
    );
  }
}
