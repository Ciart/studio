import 'package:ciart_studio/stores/document.dart';
import 'package:ciart_studio/stores/document_container.dart';
import 'package:fluent_ui/fluent_ui.dart';

import 'package:nabi/nabi.dart';
import 'package:provider/provider.dart';

class NewDocumentDialog extends StatefulWidget {
  const NewDocumentDialog({Key? key}) : super(key: key);

  @override
  _NewDocumentDialogState createState() => _NewDocumentDialogState();
}

class _NewDocumentDialogState extends State<NewDocumentDialog> {
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
            final document = Document(
              name: _nameController.text,
              width: int.parse(_widthController.text),
              height: int.parse(_heightController.text),
            );

            context.read<DocumentContainer>().add(document);

            final layout = context.read<Layout>();

            layout.addChild(
              'workspace',
              LayoutWidget(
                name: 'workspacePanel',
                title: _nameController.text,
                arguments: document.id,
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
