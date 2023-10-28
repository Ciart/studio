import 'package:ciart_studio/stores/tool_store.dart';
import 'package:ciart_studio/tools/pen.dart';
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

    return Container(
      color: Color.fromARGB(255, 204, 207, 212),
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Observer(
          builder: (context) {
            final tool = toolContainer.focusTool;

            if (tool is Pen) {
              return PenProperty();
            } else {
              return Container();
            }
          },
        ),
      ),
    );
  }
}
