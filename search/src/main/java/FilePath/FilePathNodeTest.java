package FilePath;

public class FilePathNodeTest {

    public  static void main(String[] args) {
        String slist[] = new String[] {
                "/mnt/sdcard/folder1/a/b/file1.file",
                "/mnt/sdcard/folder1/a/b/file2.file",
                "/mnt/sdcard/folder1/a/b/file3.file",
                "/mnt/sdcard/folder1/a/b/file4.file",
                "/mnt/sdcard/folder1/a/b/file5.file",
                "/mnt/sdcard/folder1/e/c/file6.file",
                "/mnt/sdcard/folder2/d/file7.file",
                "/mnt/sdcard/folder2/d/file8.file",
                "/mnt/sdcard/file9.file"
        };

        FilePathTree tree = new FilePathTree(new FilePathNode("root", "root"));
        for (String data: slist) {
            tree.addElement(data);
        }
        tree.printTree();
    }


}
