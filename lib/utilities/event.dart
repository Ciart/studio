class EventArgs {}

typedef EventListener<T extends EventArgs> = void Function(T args);

class Event<T extends EventArgs> {
  final List<EventListener<T>> _listeners = [];

  void addListener(EventListener<T> listener) {
    _listeners.add(listener);
  }

  void removeListener(EventListener<T> listener) {
    _listeners.remove(listener);
  }

  void call(T args) {
    for (final listener in _listeners) {
      listener(args);
    }
  }
}
