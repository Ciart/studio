import 'dart:ui' as ui;

import 'package:ciart_studio/providers/document.dart';
import 'package:ciart_studio/widgets/atoms/ui_image.dart';
import 'package:fluent_ui/fluent_ui.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';

class LayerPanel extends ConsumerStatefulWidget {
  const LayerPanel({Key? key}) : super(key: key);

  @override
  _LayerPanelState createState() => _LayerPanelState();
}

class _LayerPanelState extends ConsumerState<LayerPanel> {
  final ScrollController controller = ScrollController();

  @override
  Widget build(BuildContext context) {
    final document = ref.watch(focusDocumentProvider);

    if (document == null) {
      return Container();
    }

    return ReorderableListView.builder(
      scrollController: controller,
      buildDefaultDragHandles: false,
      itemCount: document.layers.length,
      itemBuilder: (context, index) => ReorderableDragStartListener(
        key: ValueKey(document.layers[index]),
        index: index,
        child: GestureDetector(
          onTap: () {
            document.selectLayerIndex = index;
          },
          child: Container(
            color: index == document.selectLayerIndex
                ? Colors.white
                : Colors.transparent,
            child: Padding(
              padding: const EdgeInsets.all(8.0),
              child: Row(
                children: [
                  Checkbox(
                    checked: document.layers[index].isVisible,
                    onChanged: (value) {
                      if (value != null) {
                        document.layers[index].isVisible = value;
                      }
                    },
                  ),
                  FutureBuilder<ui.Image?>(
                    future: document.layers[index].image,
                    builder: (context, AsyncSnapshot<ui.Image?> snapshot) {
                      if (!snapshot.hasData ||
                          snapshot.hasError ||
                          snapshot.data == null) {
                        return SizedBox(
                          width: 32,
                          height: 32,
                        );
                      }

                      return UiImage(
                        image: snapshot.data!,
                        width: 32,
                        height: 32,
                      );
                    },
                  ),
                  Expanded(
                    child: Text(document.layers[index].name),
                  )
                ],
              ),
            ),
          ),
        ),
      ),
      onReorder: document.updateLayerIndex,
    );
  }
}
