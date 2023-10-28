// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'pen.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic, no_leading_underscores_for_local_identifiers

mixin _$Pen on _Pen, Store {
  late final _$sizeAtom = Atom(name: '_Pen.size', context: context);

  @override
  int get size {
    _$sizeAtom.reportRead();
    return super.size;
  }

  @override
  set size(int value) {
    _$sizeAtom.reportWrite(value, super.size, () {
      super.size = value;
    });
  }

  @override
  String toString() {
    return '''
size: ${size}
    ''';
  }
}
