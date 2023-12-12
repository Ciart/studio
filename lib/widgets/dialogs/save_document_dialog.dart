import 'package:ciart_studio/stores/document.dart';
import 'package:ciart_studio/stores/document_container.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:nabi/nabi.dart';
import 'package:path/path.dart';
import 'package:path_provider/path_provider.dart';
import 'package:provider/provider.dart';
import 'package:image/image.dart' as img;

class SaveDocumentDialog extends StatefulWidget {
  final Document document;

  const SaveDocumentDialog({
    Key? key,
    required this.document,
  }) : super(key: key);

  @override
  _SaveDocumentDialogState createState() => _SaveDocumentDialogState();
}

class _SaveDocumentDialogState extends State<SaveDocumentDialog> {
  late TextEditingController _nameController;

  void saveDocument() async {
    final appDocumentsDir = await getApplicationDocumentsDirectory();

    setState(() {
      join(appDocumentsDir.path, _nameController.text);
    });
  }

  @override
  void initState() {
    super.initState();

    _nameController = TextEditingController(text: "무제");
  }

  @override
  Widget build(BuildContext context) {
    return ContentDialog(
      title: const Text('저장'),
      content: Container(),
      actions: [
        Button(
          onPressed: () {
            Navigator.pop(context);
          },
          child: const Text('저장'),
        )
      ],
    );
  }
}
