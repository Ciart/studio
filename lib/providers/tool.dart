import 'package:stellon/tools/eraser.dart';
import 'package:stellon/tools/line.dart';
import 'package:stellon/tools/pen.dart';
import 'package:stellon/tools/rectangle.dart';
import 'package:stellon/tools/tool.dart';
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
