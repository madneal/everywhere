package constants;

public class MyArray<E> {

    private static final int DEFAULT_CAPACITY = 100;

    private static final Object[] EMPTY_ELMENT_DATA = {};

    private int size;

    private Object[] elementData;

    public MyArray(int capacity) {
        if (capacity > 0) {
            this.elementData = new Object[capacity];
        } else if (capacity == 0) {
            this.elementData = EMPTY_ELMENT_DATA;
        } else {
            throw new IllegalArgumentException("Illegal capacity " + capacity);
        }
    }


}
