// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'layer.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic, no_leading_underscores_for_local_identifiers

mixin _$Layer on _Layer, Store {
  Computed<Image?>? _$thumbnailComputed;

  @override
  Image? get thumbnail => (_$thumbnailComputed ??=
          Computed<Image?>(() => super.thumbnail, name: '_Layer.thumbnail'))
      .value;

  late final _$nameAtom = Atom(name: '_Layer.name', context: context);

  @override
  String get name {
    _$nameAtom.reportRead();
    return super.name;
  }

  @override
  set name(String value) {
    _$nameAtom.reportWrite(value, super.name, () {
      super.name = value;
    });
  }

  late final _$isVisibleAtom = Atom(name: '_Layer.isVisible', context: context);

  @override
  bool get isVisible {
    _$isVisibleAtom.reportRead();
    return super.isVisible;
  }

  @override
  set isVisible(bool value) {
    _$isVisibleAtom.reportWrite(value, super.isVisible, () {
      super.isVisible = value;
    });
  }

  late final _$_imageAtom = Atom(name: '_Layer._image', context: context);

  @override
  Image? get _image {
    _$_imageAtom.reportRead();
    return super._image;
  }

  @override
  set _image(Image? value) {
    _$_imageAtom.reportWrite(value, super._image, () {
      super._image = value;
    });
  }

  @override
  String toString() {
    return '''
name: ${name},
isVisible: ${isVisible},
thumbnail: ${thumbnail}
    ''';
  }
}
