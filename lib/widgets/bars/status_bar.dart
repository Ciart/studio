import 'package:ciart_studio/stores/tool_store.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:provider/provider.dart';

class StatusBar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final toolStore = context.read<ToolContainer>();

    return Container(
      color: Colors.grey[160],
      width: double.infinity,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 8.0, vertical: 4.0),
        child: Row(
          children: [
            Observer(
              builder: (context) => Text(
                "${toolStore.position.dx.floor()}, ${toolStore.position.dy.floor()}",
              ),
            ),
          ],
        ),
      ),
    );
  }
}
