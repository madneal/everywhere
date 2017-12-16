package file;

import constants.CommonConstants;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.util.ArrayList;
import java.util.List;

public class FileUtil {

    final static Logger logger = LoggerFactory.getLogger(FileUtil.class);

    public static List<FileBean> getFolderFiles(String path) {
        List<FileBean> fileBeans = new ArrayList<>();
        File file = new File(path);
        try {
            if (file.isDirectory()) {
                if (isPassExcludePath(path)) {
                    fileBeansAddForDirectory(fileBeans, file);
                }
            } else {
                String extName = getFileExtension(file);
                if (CommonConstants.DOCFILES.contains(extName)) {
//                    System.out.println("Scan file:" + path);
                    fileBeans.add(FileBeanParser.getFileBean(file));
                }
            }
        } catch (Exception e) {
            logger.error(path, e);
        }
        return fileBeans;
    }

    private static void fileBeansAddForDirectory(List<FileBean> fileBeans, File file) throws Exception {
        File[] files = file.listFiles();
        if (files != null) {
            for (File file1: files) {
                fileBeans.addAll(getFolderFiles(file1.getAbsolutePath()));
            }
        }
    }

    private static boolean isPassExcludePath(String filePath) {
        for (String excludePath : CommonConstants.EXCLUDE_FILE_PATHS) {
            if (filePath.contains(excludePath)) {
                return false;
            }
        }
        return true;
    }

    private static String getFileExtension(File file) {
        String fileName = file.getName();
        return fileName.substring(fileName.lastIndexOf(".") + 1);
    }

    public static List<String> getDriver() {
        List<String> driverList = new ArrayList<>();
        File[] fs = File.listRoots();
        for (int i = 0; i < fs.length; i++) {
            driverList.add(fs[i].getAbsolutePath());
        }
        return driverList;
    }

    public static void deleteFile(File file) {
        if (file.isDirectory()) {
            String[] list = file.list();
            for (String child : list) {
                deleteFile(new File(file, child));
            }
        }
        file.delete();
    }
}
