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

        return Column(
          children: [
            Row(
              children: [
                IconButton(
                  icon: const Icon(FluentIcons.add, size: 12.0),
                  onPressed: document != null
                      ? () {
                          document.createBitmapLayer();
                        }
                      : null,
                ),
              ],
            ),
            Expanded(
              child: Observer(
                builder: (context) {
                  if (document == null) {
                    return Container();
                  }

                  return ReorderableListView.builder(
                    scrollController: controller,
                    buildDefaultDragHandles: false,
                    itemCount: document.layers.length,
                    itemBuilder: (context, index) =>
                        ReorderableDragStartListener(
                      key: ValueKey(document.layers[index]),
                      index: index,
                      child: Observer(
                        builder: (context) {
                          final layer = document.layers[index];

                          return GestureDetector(
                            onTap: () {
                              document.selectLayerIndex = index;
                            },
                            child: Container(
                              color: index == document.selectLayerIndex
                                  ? Colors.grey[160]
                                  : Colors.transparent,
                              child: Padding(
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
                                    // layer.thumbnail != null
                                    false
                                        ? UiImage(
                                            image: layer.thumbnail!,
                                            width: 32,
                                            height: 32,
                                          )
                                        : Container(),
                                    Expanded(
                                      child: Text(layer.name),
                                    ),
                                  ],
                                ),
                              ),
                            ),
                          );
                        },
                      ),
                    ),
                    onReorder: document.updateLayerIndex,
                  );
                },
              ),
            ),
          ],
        );
      },
    );
  }
}
