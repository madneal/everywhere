package search;

import javafx.scene.control.Hyperlink;

public class SearchedResult {

    private String filepath;

    private String lastModified;

    private String context;

    private Hyperlink hyperlink;

    public String getFilepath() {
        return filepath;
    }

    public void setFilepath(String filepath) {
        this.filepath = filepath;
    }

    public String getLastModified() {
        return lastModified;
    }

    public void setLastModified(String lastModified) {
        this.lastModified = lastModified;
    }

    public String getContext() {
        return context;
    }

    public void setContext(String context) {
        this.context = context;
    }

    public Hyperlink getHyperlink() {
        return hyperlink;
    }

    public void setHyperlink(Hyperlink hyperlink) {
        this.hyperlink = hyperlink;
    }
}
