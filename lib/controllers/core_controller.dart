import 'dart:ui';

import 'package:doter/models/document.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';

class CoreController extends GetxController {
  var documentPosition = Offset.zero.obs;

  var primaryColor = Color(0xffffffff).obs;

  var document = Document(width: 100, height: 100).obs;
}
