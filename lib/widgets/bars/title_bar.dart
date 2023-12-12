import 'dart:io';

import 'package:ciart_studio/stores/document_container.dart';
import 'package:ciart_studio/widgets/dialogs/new_document_dialog.dart';
import 'package:ciart_studio/widgets/dialogs/open_document_dialog%20copy.dart';
import 'package:ciart_studio/widgets/dialogs/save_document_dialog.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:provider/provider.dart';

class TitleBar extends StatelessWidget {
  const TitleBar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (context) {
        final document = context.read<DocumentContainer>().focusDocument;

        return Column(
          children: [
            Padding(
              padding: EdgeInsets.only(left: Platform.isMacOS ? 68 : 0),
              child: Row(
                children: [
                  Button(
                    child: Text('새 파일'),
                    onPressed: () async {
                      await showDialog(
                        context: context,
                        builder: (context) => NewDocumentDialog(),
                      );
                    },
                  ),
                  Button(
                    child: Text('열기'),
                    onPressed: () async {
                      await showDialog(
                        context: context,
                        builder: (context) => OpenDocumentDialog(),
                      );
                    },
                  ),
                  Button(
                    child: Text('저장'),
                    onPressed: document != null
                        ? () async {
                            await showDialog(
                              context: context,
                              builder: (context) => SaveDocumentDialog(
                                document: document,
                              ),
                            );
                          }
                        : null,
                  ),
                  Button(
                    child: Text('취소'),
                    onPressed: document?.history.canUndo != null
                        ? document?.history.undo
                        : null,
                  ),
                  Button(
                    child: Text('재실행'),
                    onPressed: document?.history.canRedo != null
                        ? document?.history.redo
                        : null,
                  ),
                ],
              ),
            ),
          ],
        );
      },
    );
  }
}
