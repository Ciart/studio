import 'package:ciart_studio/stores/tool_store.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:provider/provider.dart';

class StatusBar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final toolStore = context.read<ToolContainer>();

    return Container(
      color: const Color(0xffd1d8dc),
      width: double.infinity,
      child: Padding(
        padding: const EdgeInsets.all(4.0),
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
