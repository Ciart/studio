import 'package:ciart_studio/stores/document.dart';
import 'package:flutter/scheduler.dart';
import 'package:mobx/mobx.dart';

part 'document_container.g.dart';

class DocumentContainer = _DocumentContainer with _$DocumentContainer;

abstract class _DocumentContainer with Store {
  late Ticker _ticker;

  _DocumentContainer() {
    _ticker = Ticker(
      (duration) async {
        // TODO: 노출된 Document만 업데이트하도록 변경해야 함.
        for (final document in documents.values) {
          document.update();
        }
      },
    )..start();
  }

  // 호출되지 않음
  void dispose() {
    _ticker.dispose();
  }

  @observable
  ObservableMap<String, Document> documents = ObservableMap();

  @observable
  String? focusDocumentId;

  @computed
  Document? get focusDocument => documents[focusDocumentId];

  @action
  void add(Document document) {
    documents[document.id] = document;
  }

  @action
  void remove(String id) {
    final document = documents.remove(id);

    document?.dispose();
  }

  @action
  void focus(String id) {
    focusDocumentId = id;
  }
}
