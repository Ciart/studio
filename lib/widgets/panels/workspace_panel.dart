import 'package:ciart_studio/stores/document_container.dart';
import 'package:ciart_studio/widgets/atoms/workspace.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:nabi/nabi.dart';
import 'package:provider/provider.dart';

class WorkspacePanel extends StatelessWidget {
  const WorkspacePanel({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final index = NabiWidget.of(context).layout.arguments as String?;

    assert(index != null, "WorkspacePanel's layout arguments is required");

    final documentContainer = context.read<DocumentContainer>();
    final document = documentContainer.documents[index];

    assert(document != null, "Document is not found");

    return Container(
        color: Colors.grey[200], child: Workspace(document: document!));
  }
}
