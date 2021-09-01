import 'package:doter/controllers/core_controller.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';

class StatusBar extends StatelessWidget {
  final CoreController coreController = Get.find();

  @override
  Widget build(BuildContext context) {
    return Container(
      color: const Color(0xff888888),
      width: double.infinity,
      child: Center(
        child: Obx(() => Text(coreController.documentPosition.toString())),
      ),
    );
  }
}
