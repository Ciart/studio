import 'dart:ui';

import 'package:ciart_studio/models/document.dart';

class DocumentRepository {
  final Document document = Document('test');

  void addPaletteColor(Color color) {
    document.palette.add(color);
  }
}
