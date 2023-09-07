package src;

public final class LinkedList<T> {
  private static class Node<T> {
    private T item;
    private Node<T> next;

    private static Node<Object> EMPTY_NODE = new Node<>(null, null);

    private Node(T item, Node<T> next) {
      this.item = item;
      this.next = next;
    }

    public static <T> Node<T> empty() {
      @SuppressWarnings("unchecked")
      Node<T> node = (Node<T>) EMPTY_NODE;
      return node;
    }

    public static <T> Node<T> build(T item, Node<T> next) {
      return new Node<>(item, next);
    }

    public T getItem() {
      return this.item;
    }

    public Node<T> getNext() {
      return this.next;
    }
  }

  private Node<T> head;

  private static LinkedList<Object> EMPTY_LINKED_LIST = new LinkedList<>(Node.empty());

  private LinkedList(Node<T> head) {
    this.head = head;
  }

  public static <T> LinkedList<T> empty() {
    @SuppressWarnings("unchecked")
    LinkedList<T> list = (LinkedList<T>) EMPTY_LINKED_LIST;
    return list;
  }

  public LinkedList<T> prepend(T item) {
    return new LinkedList<>(Node.build(item, this.head));
  }
}