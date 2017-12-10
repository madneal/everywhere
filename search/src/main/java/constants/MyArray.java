package constants;

import java.util.Arrays;
import java.util.Collection;

public class MyArray<E> {

    private static final int DEFAULT_CAPACITY = 100;

    private static final int MAX_ARRAY_SIZE = Integer.MAX_VALUE - 8;

    private static final Object[] EMPTY_ELMENT_DATA = {};

    private int size;

    private Object[] elementData;

    private int modCount = 0;

    public MyArray(int capacity) {
        if (capacity > 0) {
            this.elementData = new Object[capacity];
        } else if (capacity == 0) {
            this.elementData = EMPTY_ELMENT_DATA;
        } else {
            throw new IllegalArgumentException("Illegal capacity " + capacity);
        }
    }

    public boolean add(E e) {
        ensureCapacityInternal(size + 1);
        elementData[size++] = e;
        return true;
    }

    public boolean addAll(Collection<? extends E> c) {
        Object[] a = c.toArray();
        int numNew = a.length;
        ensureCapacityInternal(size + numNew);
        System.arraycopy(a, 0, elementData, size, numNew);
        size += numNew;
        return numNew != 0;
    }

    private void ensureCapacityInternal(int minCapcity) {
        if (elementData == EMPTY_ELMENT_DATA) {
            minCapcity = Math.max(DEFAULT_CAPACITY, minCapcity);
        }
        ensureExplicitCapacity(minCapcity);
    }

    private void ensureExplicitCapacity(int minCapcity) {
        modCount++;

        if (minCapcity - elementData.length > 0) {
            grow(minCapcity);
        }
    }

    private void grow(int minCapacity) {
        int oldCapcity = elementData.length;
        int newCapacity = oldCapcity + 10;
        if (newCapacity < minCapacity) {
            newCapacity = minCapacity;
        }
        if (newCapacity > MAX_ARRAY_SIZE) {
            newCapacity = hugeCapacity(minCapacity);
        }
        elementData = Arrays.copyOf(elementData, newCapacity);
    }

    private static int hugeCapacity(int minCapacity) {
        if (minCapacity < 0) {
            throw new OutOfMemoryError();
        }
        return (minCapacity > MAX_ARRAY_SIZE) ? Integer.MAX_VALUE : MAX_ARRAY_SIZE;
    }


}
