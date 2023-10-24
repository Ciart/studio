import 'dart:ui' as ui;

import 'package:ciart_studio/stores/document_container.dart';
import 'package:ciart_studio/widgets/atoms/ui_image.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:provider/provider.dart';

class LayerPanel extends StatefulWidget {
  const LayerPanel({Key? key}) : super(key: key);

  @override
  _LayerPanelState createState() => _LayerPanelState();
}

class _LayerPanelState extends State<LayerPanel> {
  final ScrollController controller = ScrollController();

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (context) {
        final document = context.read<DocumentContainer>().focusDocument;

        if (document == null) {
          return Container();
        }

        return Observer(
          builder: (context) {
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
                    child: Observer(
                      builder: (context) {
                        final layer = document.layers[index];

                        return Padding(
                          padding: const EdgeInsets.all(8.0),
                          child: Row(
                            children: [
                              Checkbox(
                                checked: layer.isVisible,
                                onChanged: (value) {
                                  if (value != null) {
                                    layer.isVisible = value;
                                  }
                                },
                              ),
                              FutureBuilder<ui.Image?>(
                                future: layer.image,
                                builder: (
                                  context,
                                  AsyncSnapshot<ui.Image?> snapshot,
                                ) {
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
                                child: Text(layer.name),
                              )
                            ],
                          ),
                        );
                      },
                    ),
                  ),
                ),
              ),
              onReorder: document.updateLayerIndex,
            );
          },
        );
      },
    );
  }
}
