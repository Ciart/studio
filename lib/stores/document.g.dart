// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'document.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic, no_leading_underscores_for_local_identifiers

mixin _$DocumentHistory on _DocumentHistory, Store {
  late final _$canUndoAtom =
      Atom(name: '_DocumentHistory.canUndo', context: context);

  @override
  bool get canUndo {
    _$canUndoAtom.reportRead();
    return super.canUndo;
  }

  @override
  set canUndo(bool value) {
    _$canUndoAtom.reportWrite(value, super.canUndo, () {
      super.canUndo = value;
    });
  }

  late final _$canRedoAtom =
      Atom(name: '_DocumentHistory.canRedo', context: context);

  @override
  bool get canRedo {
    _$canRedoAtom.reportRead();
    return super.canRedo;
  }

  @override
  set canRedo(bool value) {
    _$canRedoAtom.reportWrite(value, super.canRedo, () {
      super.canRedo = value;
    });
  }

  @override
  String toString() {
    return '''
canUndo: ${canUndo},
canRedo: ${canRedo}
    ''';
  }
}

mixin _$Document on _Document, Store {
  Computed<String>? _$titleComputed;

  @override
  String get title => (_$titleComputed ??=
          Computed<String>(() => super.title, name: '_Document.title'))
      .value;
  Computed<Layer>? _$selectedLayerComputed;

  @override
  Layer get selectedLayer =>
      (_$selectedLayerComputed ??= Computed<Layer>(() => super.selectedLayer,
              name: '_Document.selectedLayer'))
          .value;

  late final _$pathAtom = Atom(name: '_Document.path', context: context);

  @override
  String? get path {
    _$pathAtom.reportRead();
    return super.path;
  }

  @override
  set path(String? value) {
    _$pathAtom.reportWrite(value, super.path, () {
      super.path = value;
    });
  }

  late final _$widthAtom = Atom(name: '_Document.width', context: context);

  @override
  int get width {
    _$widthAtom.reportRead();
    return super.width;
  }

  @override
  set width(int value) {
    _$widthAtom.reportWrite(value, super.width, () {
      super.width = value;
    });
  }

  late final _$heightAtom = Atom(name: '_Document.height', context: context);

  @override
  int get height {
    _$heightAtom.reportRead();
    return super.height;
  }

  @override
  set height(int value) {
    _$heightAtom.reportWrite(value, super.height, () {
      super.height = value;
    });
  }

  late final _$paletteAtom = Atom(name: '_Document.palette', context: context);

  @override
  ObservableList<Color> get palette {
    _$paletteAtom.reportRead();
    return super.palette;
  }

  @override
  set palette(ObservableList<Color> value) {
    _$paletteAtom.reportWrite(value, super.palette, () {
      super.palette = value;
    });
  }

  late final _$layersAtom = Atom(name: '_Document.layers', context: context);

  @override
  ObservableList<Layer> get layers {
    _$layersAtom.reportRead();
    return super.layers;
  }

  @override
  set layers(ObservableList<Layer> value) {
    _$layersAtom.reportWrite(value, super.layers, () {
      super.layers = value;
    });
  }

  late final _$selectLayerIndexAtom =
      Atom(name: '_Document.selectLayerIndex', context: context);

  @override
  int get selectLayerIndex {
    _$selectLayerIndexAtom.reportRead();
    return super.selectLayerIndex;
  }

  @override
  set selectLayerIndex(int value) {
    _$selectLayerIndexAtom.reportWrite(value, super.selectLayerIndex, () {
      super.selectLayerIndex = value;
    });
  }

  late final _$previewLayerAtom =
      Atom(name: '_Document.previewLayer', context: context);

  @override
  BitmapLayer? get previewLayer {
    _$previewLayerAtom.reportRead();
    return super.previewLayer;
  }

  @override
  set previewLayer(BitmapLayer? value) {
    _$previewLayerAtom.reportWrite(value, super.previewLayer, () {
      super.previewLayer = value;
    });
  }

  late final _$pictureAtom = Atom(name: '_Document.picture', context: context);

  @override
  Picture? get picture {
    _$pictureAtom.reportRead();
    return super.picture;
  }

  @override
  set picture(Picture? value) {
    _$pictureAtom.reportWrite(value, super.picture, () {
      super.picture = value;
    });
  }

  late final _$renderAsyncAction =
      AsyncAction('_Document.render', context: context);

  @override
  Future<void> render() {
    return _$renderAsyncAction.run(() => super.render());
  }

  late final _$_DocumentActionController =
      ActionController(name: '_Document', context: context);

  @override
  void createBitmapLayer({Uint8List? pixels}) {
    final _$actionInfo = _$_DocumentActionController.startAction(
        name: '_Document.createBitmapLayer');
    try {
      return super.createBitmapLayer(pixels: pixels);
    } finally {
      _$_DocumentActionController.endAction(_$actionInfo);
    }
  }

  @override
  void updateLayerIndex(int oldIndex, int newIndex) {
    final _$actionInfo = _$_DocumentActionController.startAction(
        name: '_Document.updateLayerIndex');
    try {
      return super.updateLayerIndex(oldIndex, newIndex);
    } finally {
      _$_DocumentActionController.endAction(_$actionInfo);
    }
  }

  @override
  String toString() {
    return '''
path: ${path},
width: ${width},
height: ${height},
palette: ${palette},
layers: ${layers},
selectLayerIndex: ${selectLayerIndex},
previewLayer: ${previewLayer},
picture: ${picture},
title: ${title},
selectedLayer: ${selectedLayer}
    ''';
  }
}
