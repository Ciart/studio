import 'package:doter/providers/document.dart';
import 'package:doter/providers/ui.dart';
import 'package:fluent_ui/fluent_ui.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:nabi/nabi.dart';
import 'package:uuid/uuid.dart';

class NewDocumentDialog extends ConsumerStatefulWidget {
  const NewDocumentDialog({Key? key}) : super(key: key);

  @override
  _NewDocumentDialogState createState() => _NewDocumentDialogState();
}

class _NewDocumentDialogState extends ConsumerState<NewDocumentDialog> {
  String _documentName = '';

  @override
  Widget build(BuildContext context) {
    return ContentDialog(
      title: const Text('New Document'),
      content: TextBox(
        onChanged: (value) => setState(() {
          _documentName = value;
        }),
      ),
      actions: [
        Button(
          onPressed: () {
            var id = Uuid().v4();

            var document = ref.watch(documentProvider(id));
            document.init(name: _documentName, width: 32, height: 32);

            ref.watch(layoutProvider).addChild(
                  'workspace',
                  LayoutWidget(
                    name: 'workspacePanel',
                    title: _documentName,
                    arguments: id,
                  ),
                );
            Navigator.pop(context);
          },
          child: const Text('Ok'),
        )
      ],
    );
  }
}
