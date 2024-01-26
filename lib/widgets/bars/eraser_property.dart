import 'package:ciart_studio/stores/tool_store.dart';
import 'package:ciart_studio/tools/eraser.dart';
import 'package:ciart_studio/tools/pen.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:provider/provider.dart';

class EraserProperty extends StatelessWidget {
  const EraserProperty({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final tool = context.read<ToolContainer>().focusTool;

    if (tool is! Eraser) {
      return Container();
    }

    return Observer(
      builder: (context) {
        return Row(
          children: [
            Padding(
              padding: const EdgeInsets.only(left: 4.0, right: 16.0),
              child: Text('Size'),
            ),
            SizedBox(
              width: 120,
              child: Slider(
                label: tool.size.toString(),
                min: 1,
                max: 32,
                value: tool.size.toDouble(),
                onChanged: (value) => tool.size = value.toInt(),
              ),
            ),
          ],
        );
      },
    );
  }
}
