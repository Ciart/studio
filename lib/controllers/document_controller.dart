import 'dart:ui';

import 'package:doter/models/document.dart';
import 'package:get/get.dart';

class DocumentController extends GetxController {
  var documents = <Document>[].obs;

  DocumentController() {
    documents.add(Document(width: 100, height: 100));
  }
}
