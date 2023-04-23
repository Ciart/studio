import 'package:stellon/providers/tool.dart';
import 'package:stellon/tools/pen.dart';
import 'package:stellon/widgets/bars/pen_property.dart';
import 'package:fluent_ui/fluent_ui.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';

class PropertyBar extends ConsumerWidget {
  const PropertyBar({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    var tool = ref.watch(toolProvider);

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
