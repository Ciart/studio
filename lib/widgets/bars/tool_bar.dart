import 'package:ciart_studio/stores/tool_store.dart';
import 'package:ciart_studio/widgets/atoms/tool_button.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:provider/provider.dart';

class ToolBar extends StatelessWidget {
  const ToolBar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final toolStore = context.read<ToolContainer>();

    return Align(
      alignment: Alignment.topCenter,
      child: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Wrap(
          children: toolStore.tools
              .map(
                (tool) => Observer(
                  builder: (context) {
                    return ToolButton(
                      icon: tool.icon,
                      checked: toolStore.focusTool == tool,
                      onChanged: (value) {
                        if (value) {
                          toolStore.focus(toolStore.getToolIndex(tool));
                        }
                      },
                    );
                  },
                ),
              )
              .toList(),
        ),
      ),
    );
  }
}
