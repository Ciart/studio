import 'package:doter/tools/eraser.dart';
import 'package:doter/tools/line.dart';
import 'package:doter/tools/pen.dart';
import 'package:doter/tools/rectangle.dart';
import 'package:doter/tools/tool.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

final toolListProvider =
    StateNotifierProvider<ToolList, List<Tool>>((ref) => ToolList());

final selectedToolIndexProvider = StateProvider<int>((ref) => 0);

final toolProvider = Provider<Tool>((ref) {
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
