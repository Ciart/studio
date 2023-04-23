import 'package:stellon/providers/tool.dart';
import 'package:stellon/tools/pen.dart';
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
        Slider(
          label: '${tool.size}',
          style: SliderThemeData(labelBackgroundColor: Colors.black),
          value: tool.size.toDouble(),
          min: 1,
          max: 10,
          onChanged: (value) => tool.size = value.toInt(),
        ),
      ],
    );
  }
}
