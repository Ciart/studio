import 'package:ciart_studio/providers/ui.dart';
import 'package:fluent_ui/fluent_ui.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';

class StatusBar extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    var documentPosition = ref.watch(documentPositionProvider);

    return Container(
      color: const Color(0xff888888),
      width: double.infinity,
      child: Row(
        children: [
          () {
            return Text(
              "(${documentPosition.dx.floor()}, ${documentPosition.dy.floor()})",
            );
          }()
        ],
      ),
    );
  }
}
