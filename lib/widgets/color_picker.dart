import 'package:doter/controllers/core_controller.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';

class ColorPicker extends StatefulWidget {
  const ColorPicker({Key? key}) : super(key: key);

  @override
  _ColorPickerState createState() => _ColorPickerState();
}

class _ColorPickerState extends State<ColorPicker> {
  var r = 255.0;

  final CoreController coreController = Get.find();

  @override
  Widget build(BuildContext context) {
    return Container(
      child: Obx(() {
        var primaryColor = coreController.primaryColor.value;

        return Column(
          children: [
            Container(
              height: 100,
              width: 100,
              color: primaryColor,
            ),
            Text(primaryColor.toString()),
            Slider(
              min: 0,
              max: 255,
              value: primaryColor.red.toDouble(),
              onChanged: _onChangedRedSlider,
            ),
            Slider(
              min: 0,
              max: 255,
              value: primaryColor.green.toDouble(),
              onChanged: _onChangedGreenSlider,
            ),
            Slider(
              min: 0,
              max: 255,
              value: primaryColor.blue.toDouble(),
              onChanged: _onChangedBlueSlider,
            )
          ],
        );
      }),
    );
  }

  void _onChangedRedSlider(double value) {
    coreController.primaryColor.value =
        coreController.primaryColor.value.withRed(value.toInt());
  }

  void _onChangedGreenSlider(double value) {
    coreController.primaryColor.value =
        coreController.primaryColor.value.withGreen(value.toInt());
  }

  void _onChangedBlueSlider(double value) {
    coreController.primaryColor.value =
        coreController.primaryColor.value.withBlue(value.toInt());
  }
}
