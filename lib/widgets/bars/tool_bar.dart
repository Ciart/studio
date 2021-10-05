import 'package:doter/providers/tool.dart';
import 'package:fluent_ui/fluent_ui.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';

class ToolBar extends ConsumerWidget {
  const ToolBar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    var toolList = ref.watch(toolListProvider);
    var selectedToolIndex = ref.watch(selectedToolIndexProvider.state);

    return ListView.builder(
      itemCount: toolList.length,
      itemBuilder: (context, index) {
        final tool = toolList[index];

        return RadioButton(
          content: Text(tool.name),
          checked: selectedToolIndex.state == index,
          onChanged: (value) {
            if (!value) {
              return;
            }

            selectedToolIndex.state = index;
          },
        );
      },
    );
  }
}
