import 'dart:typed_data';
import 'dart:ui';

import 'package:mobx/mobx.dart';
import 'package:path/path.dart' as p;
import 'package:uuid/uuid.dart';
import 'package:image/image.dart' as img;

import 'layers/layer.dart';
import 'layers/bitmap_layer.dart';

part 'document.g.dart';

class DocumentSnapshot {
  DocumentSnapshot({
    required this.width,
    required this.height,
    required this.layers,
  });

  DocumentSnapshot.fromDocument(Document document)
      : width = document.width,
        height = document.height,
        layers = document.layers.map((layer) => layer.makeSnapshot()).toList();

  final int width;
  final int height;
  final List<LayerSnapshot> layers;
}

class Command {
  Command();

  void execute() {}
}

class DocumentHistory = _DocumentHistory with _$DocumentHistory;

abstract class _DocumentHistory with Store {
  _DocumentHistory(this._document);

  _Document _document;

  @observable
  bool canUndo = false;

  @observable
  bool canRedo = false;

  void makeSnapshot() {}

  void undo() {}

  void redo() {}
}

class Document = _Document with _$Document;

abstract class _Document with Store {
  final String id = Uuid().v4();

  late final DocumentHistory history;

  @observable
  String? path;

  @computed
  String get title => path != null ? p.basename(path!) : 'Untitled';

  @observable
  int width;

  @observable
  late int height;

  @observable
  ObservableList<Color> palette = ObservableList();

  @observable
  ObservableList<Layer> layers = ObservableList();

  @observable
  int selectLayerIndex = 0;

  @computed
  Layer get selectedLayer => layers[selectLayerIndex];

  @observable
  Picture? picture;

  bool _isInvalidated = true;

  bool _isRendering = false;

  late ReactionDisposer _reactionDisposer;

  _Document({this.path, required this.width, required this.height}) {
    history = DocumentHistory(this);
  }

  void update() async {
    if (_isInvalidated && !_isRendering) {
      _isInvalidated = false;
      _isRendering = true;

      await render();

      _isRendering = false;
    }
  }

  void dispose() {
    _reactionDisposer();

    for (final layer in layers) {
      layer.dispose();
    }
  }

  void _invalidate() {
    _isInvalidated = true;
  }

  @action
  void createBitmapLayer({Uint8List? pixels}) {
    layers.insert(
      selectLayerIndex,
      BitmapLayer(
        name: '레이어',
        width: width,
        height: height,
        pixels: pixels,
        onInvalidate: _invalidate,
      ),
    );

    _invalidate();
  }

  @action
  void updateLayerIndex(int oldIndex, int newIndex) {
    if (oldIndex < newIndex) {
      newIndex -= 1;
    }

    final layer = layers.removeAt(oldIndex);
    layers.insert(newIndex, layer);

    _invalidate();
  }

  void save() async {
    if (path == null) {
      throw Exception('Path is null');
    }

    final uiImage = await picture?.toImage(width, height);

    if (uiImage == null) {
      throw Exception('Failed to convert picture to image');
    }

    img.encodePngFile(
      path!,
      img.Image.fromBytes(
        width: width,
        height: height,
        bytes: (await uiImage.toByteData())!.buffer,
        numChannels: 4,
      ),
    );
  }

  @action
  Future<void> render() async {
    final recorder = PictureRecorder();
    final canvas = Canvas(recorder);

    for (int i = layers.length - 1; i >= 0; i--) {
      final layer = layers[i];

      if (!layer.isVisible) {
        continue;
      }

      final layerImage = await layer.renderImage();

      canvas.drawImage(layerImage, Offset.zero, Paint());
    }

    picture = recorder.endRecording();
  }
}
