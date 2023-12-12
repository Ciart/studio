import 'package:ciart_studio/stores/document.dart';
import 'package:ciart_studio/stores/document_container.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:nabi/nabi.dart';
import 'package:path/path.dart';
import 'package:path_provider/path_provider.dart';
import 'package:provider/provider.dart';
import 'package:image/image.dart' as img;

class OpenDocumentDialog extends StatefulWidget {
  const OpenDocumentDialog({Key? key}) : super(key: key);

  @override
  _OpenDocumentDialogState createState() => _OpenDocumentDialogState();
}

class _OpenDocumentDialogState extends State<OpenDocumentDialog> {
  List<String>? documentPaths;

  void refreshDocuments() async {
    final appDocumentsDir = await getApplicationDocumentsDirectory();

    setState(() {
      documentPaths =
          appDocumentsDir.listSync().map((entity) => entity.path).toList();
    });
  }

  @override
  void initState() {
    super.initState();

    refreshDocuments();
  }

  @override
  Widget build(BuildContext context) {
    return ContentDialog(
      title: const Text('열기'),
      content: documentPaths != null
          ? ListView.builder(
              itemCount: documentPaths!.length,
              itemBuilder: (context, index) {
                final path = documentPaths![index];

                return ListTile(
                  title: Text(basename(path)),
                  onPressed: () async {
                    final image = await img.decodeImageFile(path);

                    if (image == null) {
                      throw Exception('Failed to load image');
                    }

                    final pixels = image
                        .getFrame(0)
                        .getBytes(order: img.ChannelOrder.rgba);

                    final document = Document(
                      path: path,
                      width: image.width,
                      height: image.height,
                    );

                    document.createBitmapLayer(pixels: pixels);

                    context.read<DocumentContainer>().add(document);

                    final layout = context.read<Layout>();

                    layout.addChild(
                      'workspace',
                      LayoutWidget(
                        name: 'workspacePanel',
                        title: document.title,
                        arguments: document.id,
                      ),
                    );
                    Navigator.pop(context);
                  },
                );
              },
            )
          : null,
      actions: [
        Button(
          onPressed: () {
            Navigator.pop(context);
          },
          child: const Text('닫기'),
        )
      ],
    );
  }
}
