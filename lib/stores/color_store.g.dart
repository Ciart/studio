// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'color_store.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic, no_leading_underscores_for_local_identifiers

mixin _$ColorStore on _ColorStore, Store {
  Computed<Color>? _$primaryColorComputed;

  @override
  Color get primaryColor =>
      (_$primaryColorComputed ??= Computed<Color>(() => super.primaryColor,
              name: '_ColorStore.primaryColor'))
          .value;
  Computed<Color>? _$secondaryColorComputed;

  @override
  Color get secondaryColor =>
      (_$secondaryColorComputed ??= Computed<Color>(() => super.secondaryColor,
              name: '_ColorStore.secondaryColor'))
          .value;

  late final _$primaryAtom =
      Atom(name: '_ColorStore.primary', context: context);

  @override
  HSVColor get primary {
    _$primaryAtom.reportRead();
    return super.primary;
  }

  @override
  set primary(HSVColor value) {
    _$primaryAtom.reportWrite(value, super.primary, () {
      super.primary = value;
    });
  }

  late final _$secondaryAtom =
      Atom(name: '_ColorStore.secondary', context: context);

  @override
  HSVColor get secondary {
    _$secondaryAtom.reportRead();
    return super.secondary;
  }

  @override
  set secondary(HSVColor value) {
    _$secondaryAtom.reportWrite(value, super.secondary, () {
      super.secondary = value;
    });
  }

  late final _$paletteAtom =
      Atom(name: '_ColorStore.palette', context: context);

  @override
  ObservableList<Color> get palette {
    _$paletteAtom.reportRead();
    return super.palette;
  }

  @override
  set palette(ObservableList<Color> value) {
    _$paletteAtom.reportWrite(value, super.palette, () {
      super.palette = value;
    });
  }

  late final _$_ColorStoreActionController =
      ActionController(name: '_ColorStore', context: context);

  @override
  void setPrimary(HSVColor color) {
    final _$actionInfo = _$_ColorStoreActionController.startAction(
        name: '_ColorStore.setPrimary');
    try {
      return super.setPrimary(color);
    } finally {
      _$_ColorStoreActionController.endAction(_$actionInfo);
    }
  }

  @override
  void setSecondary(HSVColor color) {
    final _$actionInfo = _$_ColorStoreActionController.startAction(
        name: '_ColorStore.setSecondary');
    try {
      return super.setSecondary(color);
    } finally {
      _$_ColorStoreActionController.endAction(_$actionInfo);
    }
  }

  @override
  void addColor(Color color) {
    final _$actionInfo =
        _$_ColorStoreActionController.startAction(name: '_ColorStore.addColor');
    try {
      return super.addColor(color);
    } finally {
      _$_ColorStoreActionController.endAction(_$actionInfo);
    }
  }

  @override
  void swapColor(Color a, Color b) {
    final _$actionInfo = _$_ColorStoreActionController.startAction(
        name: '_ColorStore.swapColor');
    try {
      return super.swapColor(a, b);
    } finally {
      _$_ColorStoreActionController.endAction(_$actionInfo);
    }
  }

  @override
  String toString() {
    return '''
primary: ${primary},
secondary: ${secondary},
palette: ${palette},
primaryColor: ${primaryColor},
secondaryColor: ${secondaryColor}
    ''';
  }
}
