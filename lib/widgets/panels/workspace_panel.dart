import 'package:doter/providers/document.dart';
import 'package:doter/widgets/atoms/workspace.dart';
import 'package:fluent_ui/fluent_ui.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:nabi/nabi.dart';

class WorkspacePanel extends ConsumerWidget {
  const WorkspacePanel({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final index = NabiWidget.of(context).layout.arguments as String?;
    assert(index != null, "WorkspacePanel's layout arguments is required");

    final document = ref.watch(documentProvider(index!));

    return Workspace(document: document);
  }
}
