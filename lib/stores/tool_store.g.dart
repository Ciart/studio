// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'tool_store.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic, no_leading_underscores_for_local_identifiers

mixin _$ToolContainer on _ToolContainer, Store {
  Computed<Tool>? _$focusToolComputed;

  @override
  Tool get focusTool =>
      (_$focusToolComputed ??= Computed<Tool>(() => super.focusTool,
              name: '_ToolContainer.focusTool'))
          .value;

  late final _$focusIndexAtom =
      Atom(name: '_ToolContainer.focusIndex', context: context);

  @override
  int get focusIndex {
    _$focusIndexAtom.reportRead();
    return super.focusIndex;
  }

  @override
  set focusIndex(int value) {
    _$focusIndexAtom.reportWrite(value, super.focusIndex, () {
      super.focusIndex = value;
    });
  }

  late final _$positionAtom =
      Atom(name: '_ToolContainer.position', context: context);

  @override
  Offset get position {
    _$positionAtom.reportRead();
    return super.position;
  }

  @override
  set position(Offset value) {
    _$positionAtom.reportWrite(value, super.position, () {
      super.position = value;
    });
  }

  late final _$toolsAtom = Atom(name: '_ToolContainer.tools', context: context);

  @override
  ObservableList<Tool> get tools {
    _$toolsAtom.reportRead();
    return super.tools;
  }

  @override
  set tools(ObservableList<Tool> value) {
    _$toolsAtom.reportWrite(value, super.tools, () {
      super.tools = value;
    });
  }

  late final _$_ToolContainerActionController =
      ActionController(name: '_ToolContainer', context: context);

  @override
  void focus(int index) {
    final _$actionInfo = _$_ToolContainerActionController.startAction(
        name: '_ToolContainer.focus');
    try {
      return super.focus(index);
    } finally {
      _$_ToolContainerActionController.endAction(_$actionInfo);
    }
  }

  @override
  void setPosition(Offset position) {
    final _$actionInfo = _$_ToolContainerActionController.startAction(
        name: '_ToolContainer.setPosition');
    try {
      return super.setPosition(position);
    } finally {
      _$_ToolContainerActionController.endAction(_$actionInfo);
    }
  }

  @override
  String toString() {
    return '''
focusIndex: ${focusIndex},
position: ${position},
tools: ${tools},
focusTool: ${focusTool}
    ''';
  }
}
