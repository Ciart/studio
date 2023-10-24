import 'package:ciart_studio/stores/document.dart';
import 'package:mobx/mobx.dart';

part 'document_container.g.dart';

class DocumentContainer = _DocumentContainer with _$DocumentContainer;

abstract class _DocumentContainer with Store {
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
    documents.remove(id);
  }

  @action
  void focus(String id) {
    focusDocumentId = id;
  }
}
