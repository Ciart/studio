import 'dart:io';

import 'package:ciart_studio/widgets/dialogs/new_document_dialog.dart';
import 'package:fluent_ui/fluent_ui.dart';

class TitleBar extends StatelessWidget {
  const TitleBar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Padding(
          padding: EdgeInsets.only(left: Platform.isMacOS ? 68 : 0),
          child: Row(
            children: [
              Button(
                child: Text('New File'),
                onPressed: () async {
                  await showDialog(
                    context: context,
                    builder: (context) => NewDocumentDialog(),
                  );
                },
              ),
              Button(
                child: Text('Undo'),
                onPressed: () {
                  // document?.undo();
                },
              ),
              Button(
                child: Text('Redo'),
                onPressed: () {
                  // document?.redo();
                },
              ),
            ],
          ),
        ),
      ],
    );
  }
}
