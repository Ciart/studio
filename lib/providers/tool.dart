import 'package:ciart_studio/tools/eraser.dart';
import 'package:ciart_studio/tools/line.dart';
import 'package:ciart_studio/tools/pen.dart';
import 'package:ciart_studio/tools/rectangle.dart';
import 'package:ciart_studio/tools/tool.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

final toolListProvider =
    StateNotifierProvider<ToolList, List<Tool>>((ref) => ToolList());

final selectedToolIndexProvider = StateProvider<int>((ref) => 0);

final toolProvider = ChangeNotifierProvider<Tool>((ref) {
  return ref.watch(toolListProvider)[ref.watch(selectedToolIndexProvider)];
});

class ToolList extends StateNotifier<List<Tool>> {
  ToolList()
      : super([
          Pen(),
          Eraser(),
          Rectangle(),
          Line(),
        ]);
}
