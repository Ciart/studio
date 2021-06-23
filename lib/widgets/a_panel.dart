import 'package:flutter/widgets.dart';

class APanel extends StatelessWidget {
  APanel() {
    print("test");
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      child: Center(
        child: Text("A"),
      ),
      color: const Color(0xff883333),
    );
  }
}
