package FilePath;

public class FilePathTree {
    FilePathNode root;
    FilePathNode commonRoot;

    public FilePathTree(FilePathNode root) {
        this.root = root;
        commonRoot = null;
    }

    public void addElement(String elementValue) {
        String[] list = elementValue.split("/");
        root.addElement(root.incrementalPath, list);
    }

    public void printTree() {
        getCommonRoot();
        commonRoot.printNode(0);
    }

    public FilePathNode getCommonRoot() {
        if (commonRoot != null) {
            return commonRoot;
        } else {
            FilePathNode current = root;
            while (current.leafs.size() <= 0) {
                current = current.children.get(0);
            }
            commonRoot = current;
            return commonRoot;
        }
    }
}
