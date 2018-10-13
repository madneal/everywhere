package filepath;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class FilePathNode {
    List<FilePathNode> children;
    List<FilePathNode> leafs;
    String data;
    String incrementalPath;

    public FilePathNode(String nodeVal, String incrementalPath) {
        children = new ArrayList<>();
        leafs = new ArrayList<>();
        data = nodeVal;
        this.incrementalPath = incrementalPath;
    }

    public boolean isLeaf() {
        return children.isEmpty() && leafs.isEmpty();
    }

    public void addElement(String currentPath, String[] list) {
        if (list[0] == null || list[0].equals("")) {
            list = Arrays.copyOfRange(list, 1, list.length);
        }
        FilePathNode currentChild = new FilePathNode(list[0], currentPath + "/" + list[0]);
        if (list.length == 1) {
            leafs.add(currentChild);
            return;
        } else {
            int index = children.indexOf(currentChild);
            if (index == -1) {
                children.add(currentChild);
                currentChild.addElement(currentChild.incrementalPath, Arrays.copyOfRange(list, 1, list.length));
            } else {
                FilePathNode nextChild = children.get(index);
                nextChild.addElement(currentChild.incrementalPath, Arrays.copyOfRange(list, 1, list.length));
            }
        }
    }

    @Override
    public boolean equals(Object obj) {
        FilePathNode cmp = (FilePathNode) obj;
        return incrementalPath.equals(cmp.incrementalPath) && data.equals(cmp.data);
    }

    public void printNode(int increment) {
        for (int i = 0; i < increment; i++) {
            System.out.println(" ");
        }
        System.out.println(incrementalPath + (isLeaf() ? " -> " + data : ""));
        for (FilePathNode node: children) {
            node.printNode(increment + 2);
        }
        for (FilePathNode node: leafs) {
            node.printNode(increment + 2);
        }
    }

    @Override
    public String toString() {
        return data;
    }
}
