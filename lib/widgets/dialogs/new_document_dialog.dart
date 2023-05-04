import 'dart:ffi';

import 'package:ciart_studio/providers/document.dart';
import 'package:ciart_studio/providers/ui.dart';
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
  late TextEditingController _nameController;
  late TextEditingController _widthController;
  late TextEditingController _heightController;

  @override
  void initState() {
    super.initState();

    _nameController = TextEditingController(text: "무제");
    _widthController = TextEditingController(text: "128");
    _heightController = TextEditingController(text: "128");
  }

  @override
  Widget build(BuildContext context) {
    return ContentDialog(
      title: const Text('New Document'),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          InfoLabel(
            label: '이름',
            child: TextBox(controller: _nameController),
          ),
          InfoLabel(
            label: '너비',
            child: TextBox(controller: _widthController),
          ),
          InfoLabel(
            label: '높이',
            child: TextBox(controller: _heightController),
          ),
        ],
      ),
      actions: [
        Button(
          onPressed: () {
            var id = Uuid().v4();

            var document = ref.watch(documentProvider(id));
            document.init(
              name: _nameController.text,
              width: int.parse(_widthController.text),
              height: int.parse(_heightController.text),
            );

            ref.watch(layoutProvider).addChild(
                  'workspace',
                  LayoutWidget(
                    name: 'workspacePanel',
                    title: _nameController.text,
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
