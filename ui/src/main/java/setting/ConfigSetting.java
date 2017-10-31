package setting;

import java.util.List;

public class ConfigSetting {

    private String searchMethod;

    private List<String> excludeFilePathList;

    private List<String> fileList;

    private List<String> compressFileList;

    private boolean hasCreateIndex;

    public String getSearchMethod() {
        return searchMethod;
    }

    public void setSearchMethod(String searchMethod) {
        this.searchMethod = searchMethod;
    }

    public List<String> getExcludeFilePathList() {
        return excludeFilePathList;
    }

    public void setExcludeFilePathList(List<String> excludeFilePathList) {
        this.excludeFilePathList = excludeFilePathList;
    }

    public List<String> getFileList() {
        return fileList;
    }

    public void setFileList(List<String> fileList) {
        this.fileList = fileList;
    }

    public List<String> getCompressFileList() {
        return compressFileList;
    }

    public void setCompressFileList(List<String> compressFileList) {
        this.compressFileList = compressFileList;
    }

    public boolean getHasCreateIndex() {
        return hasCreateIndex;
    }

    public void setHasCreateIndex(boolean hasCreateIndex) {
        this.hasCreateIndex = hasCreateIndex;
    }
}
