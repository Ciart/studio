import 'package:doter/providers/tool.dart';
import 'package:doter/tools/pen.dart';
import 'package:fluent_ui/fluent_ui.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';

class PenProperty extends ConsumerWidget {
  const PenProperty({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    var tool = ref.watch(toolProvider);

    if (tool is! Pen) {
      return Container();
    }

    return Row(
      children: [
        Slider(value: tool.size.toDouble(), max: 100, onChanged: (value) {})
      ],
    );
  }
}
