// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'document_container.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic, no_leading_underscores_for_local_identifiers

mixin _$DocumentContainer on _DocumentContainer, Store {
  Computed<Document?>? _$focusDocumentComputed;

  @override
  Document? get focusDocument => (_$focusDocumentComputed ??=
          Computed<Document?>(() => super.focusDocument,
              name: '_DocumentContainer.focusDocument'))
      .value;

  late final _$documentsAtom =
      Atom(name: '_DocumentContainer.documents', context: context);

  @override
  ObservableMap<String, Document> get documents {
    _$documentsAtom.reportRead();
    return super.documents;
  }

  @override
  set documents(ObservableMap<String, Document> value) {
    _$documentsAtom.reportWrite(value, super.documents, () {
      super.documents = value;
    });
  }

  late final _$focusDocumentIdAtom =
      Atom(name: '_DocumentContainer.focusDocumentId', context: context);

  @override
  String? get focusDocumentId {
    _$focusDocumentIdAtom.reportRead();
    return super.focusDocumentId;
  }

  @override
  set focusDocumentId(String? value) {
    _$focusDocumentIdAtom.reportWrite(value, super.focusDocumentId, () {
      super.focusDocumentId = value;
    });
  }

  late final _$_DocumentContainerActionController =
      ActionController(name: '_DocumentContainer', context: context);

  @override
  void add(Document document) {
    final _$actionInfo = _$_DocumentContainerActionController.startAction(
        name: '_DocumentContainer.add');
    try {
      return super.add(document);
    } finally {
      _$_DocumentContainerActionController.endAction(_$actionInfo);
    }
  }

  @override
  void remove(String id) {
    final _$actionInfo = _$_DocumentContainerActionController.startAction(
        name: '_DocumentContainer.remove');
    try {
      return super.remove(id);
    } finally {
      _$_DocumentContainerActionController.endAction(_$actionInfo);
    }
  }

  @override
  void focus(String id) {
    final _$actionInfo = _$_DocumentContainerActionController.startAction(
        name: '_DocumentContainer.focus');
    try {
      return super.focus(id);
    } finally {
      _$_DocumentContainerActionController.endAction(_$actionInfo);
    }
  }

  @override
  String toString() {
    return '''
documents: ${documents},
focusDocumentId: ${focusDocumentId},
focusDocument: ${focusDocument}
    ''';
  }
}
