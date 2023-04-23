import 'package:stellon/models/document.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

final documentProvider =
    ChangeNotifierProvider.family<Document, String>((ref, id) => Document(id));

final focusDocumentIdProvider = StateProvider<String?>((ref) => null);

final focusDocumentProvider = ChangeNotifierProvider<Document?>(
  (ref) {
    var id = ref.watch(focusDocumentIdProvider);

    if (id == null) {
      return null;
    }

    return ref.watch(documentProvider(id));
  },
);
