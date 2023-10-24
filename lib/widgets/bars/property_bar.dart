import 'package:ciart_studio/stores/tool_store.dart';
import 'package:ciart_studio/tools/pen.dart';
import 'package:ciart_studio/widgets/bars/pen_property.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:provider/provider.dart';

class PropertyBar extends StatelessWidget {
  const PropertyBar({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final tool = context.read<ToolStore>().focusTool;

    Widget child;

    if (tool is Pen) {
      child = PenProperty();
    } else {
      child = Container();
    }

    return Container(
        color: Color.fromARGB(255, 204, 207, 212),
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: child,
        ));
  }
}
