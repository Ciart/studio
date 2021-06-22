import 'package:flutter/widgets.dart';

class App extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return WidgetsApp(
      title: 'doter',
      color: const Color(0xFFFFFFFF),
      debugShowCheckedModeBanner: false,
      builder: (context, int) {
        return Center(
          child: Text('Hello, World!'),
        );
      },
    );
  }
}
