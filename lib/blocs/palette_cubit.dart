import 'dart:ui';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stellon/models/palette.dart';
import 'package:stellon/repositories/document_repository.dart';

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
