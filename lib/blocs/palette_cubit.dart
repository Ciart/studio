import 'dart:ui';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:ciart_studio/models/palette.dart';
import 'package:ciart_studio/repositories/document_repository.dart';

class PaletteCubit extends Cubit<Palette> {
  PaletteCubit(this.documentRepository) : super([]) {
    documentRepository.paletteChange.addListener((args) {
      emit(args.palette);
    });
  }

  DocumentRepository documentRepository;

  void addColor(Color color) {
    documentRepository.addPaletteColor(color);
  }
}
