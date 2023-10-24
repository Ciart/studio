import 'package:ciart_studio/stores/tool_store.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:provider/provider.dart';

class StatusBar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    // var documentPosition = ref.watch(documentPositionProvider);

    final toolStore = context.read<ToolStore>();

    return Container(
      color: const Color(0xff888888),
      width: double.infinity,
      child: Row(
        children: [
          Observer(
            builder: (context) => Text(
              "(${toolStore.position.dx.floor()}, ${toolStore.position.dy.floor()})",
            ),
          )
        ],
      ),
    );
  }
}
