import 'package:ciart_studio/stores/tool_store.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:provider/provider.dart';

class ToolBar extends StatelessWidget {
  const ToolBar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final toolStore = context.read<ToolStore>();

    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: ListView.builder(
        itemCount: toolStore.tools.length,
        itemBuilder: (context, index) {
          final tool = toolStore.tools[index];

          return Observer(
            builder: (context) => ToggleButton(
              child: Text(tool.name),
              checked: toolStore.focusIndex == index,
              onChanged: (value) {
                if (!value) {
                  return;
                }

                toolStore.focus(index);
              },
            ),
          );
        },
      ),
    );
  }
}
