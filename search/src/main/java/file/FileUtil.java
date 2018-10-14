package file;

import constants.CommonConstants;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.io.IOException;
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
                if (isPassFileTest(file)) {
//                    System.out.println("Scan file:" + path);
                    fileBeans.add(FileBeanParser.getFileBean(file));
                }
            }
        } catch (Exception e) {
            logger.error(path, e);
        }
        return fileBeans;
    }

    public static List<String> getFileList(String folder) {
        List<String> fileList = new ArrayList<>();
        try {
            File file = new File(folder);
            if (file.isDirectory() && isPassExcludePath(folder)) {
                fileAddForDirectory(fileList, file);
            } else {
                if (isFileLengthValid(file)) {
                    if (isDocument(file)) {
                        fileList.add(folder);
                    }
                    if (isCompressFile(file)) {
                        fileList.add(folder);
                    }
                }
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        return fileList;
    }

    public static boolean isCompressFile(File file) {
        String extName = getFileExtension(file);
        return CommonConstants.COMPRESS_FILES.contains(extName);
    }

    public static boolean isFileLengthValid(File file) {
        return file.length() < CommonConstants.LIMIT_FILE_SIZE;
    }

    private static void fileAddForDirectory(List<String> fileList, File file) {
        File[] files = file.listFiles();
        if (files != null && files.length != 0) {
            for (File f: files) {
                fileList.addAll(getFileList(f.getAbsolutePath()));
            }
        }
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

    private static boolean isPassFileTest(File file) {
        return CommonConstants.DOCFILES.contains(getFileExtension(file)) &&
                (file.length() < CommonConstants.LIMIT_FILE_SIZE);
    }

    private static boolean isDocument(File file) {
        String extName = getFileExtension(file);
        return CommonConstants.DOCFILES.contains(extName);
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
