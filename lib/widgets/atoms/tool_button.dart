import 'package:fluent_ui/fluent_ui.dart';

class ToolButton extends StatelessWidget {
  const ToolButton({
    super.key,
    required this.icon,
    required this.checked,
    required this.onChanged,
  });

  final IconData icon;

  final bool checked;

  final ValueChanged<bool> onChanged;

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: () {
        onChanged(!checked);
      },
      child: Container(
        width: 40,
        height: 40,
        decoration: BoxDecoration(
          color: checked ? Colors.blue : Colors.transparent,
          borderRadius: BorderRadius.circular(8),
        ),
        child: Center(
          child: Icon(icon, color: Colors.white),
        ),
      ),
    );
  }
}
