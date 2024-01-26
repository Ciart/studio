import 'package:ciart_studio/stores/tool_store.dart';
import 'package:ciart_studio/tools/eraser.dart';
import 'package:ciart_studio/tools/pen.dart';
import 'package:ciart_studio/widgets/bars/eraser_property.dart';
import 'package:ciart_studio/widgets/bars/pen_property.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:provider/provider.dart';

class PropertyBar extends StatelessWidget {
  const PropertyBar({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final toolContainer = context.read<ToolContainer>();

    return SizedBox.expand(
      child: Container(
        color: Colors.grey[160],
        child: Padding(
          padding: const EdgeInsets.all(8.0),
          child: Observer(
            builder: (context) {
              final tool = toolContainer.focusTool;

              print(tool.id);

              if (tool is Pen) {
                return PenProperty();
              } else if (tool is Eraser) {
                return EraserProperty();
              } else {
                return Container();
              }
            },
          ),
        ),
      ),
    );
  }
}
