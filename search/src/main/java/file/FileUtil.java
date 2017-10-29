package file;

import constants.*;

import java.io.File;
import java.io.FileInputStream;
import java.io.InputStream;
import java.util.ArrayList;
import java.util.List;

public class FileUtil {

    public static List<Filebean> getFolderFiles(String path) {
        List<Filebean> filebeans = new ArrayList<Filebean>();
        File file = new File(path);
        String absoluteFilepath = file.getAbsolutePath();
        String extName = getFileExtension(file);
        try {
            if (file.isDirectory()) {
                if (isPassExcludePath(absoluteFilepath)) {
                    fileBeansAddForDirectory(filebeans, file);
                }
            } else {
                if (CommonConstant.DOCFILES.contains(extName)) {

                }
            }
            return filebeans;
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    private static void fileBeansAddForDirectory(List<Filebean> filebeans, File file) throws Exception {
        File[] files = file.listFiles();
        if (files != null) {
            for (File file1: files) {
                filebeans.addAll(getFolderFiles(file1.getAbsolutePath()));
            }
        }
    }

    private static boolean isPassExcludePath(String filePath) {
        for (String excludePath : CommonConstant.EXCLUDE_FILE_PATHS) {
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

    private static Filebean getFilebean(File file) throws Exception {
        Filebean filebean = new Filebean();
        String filepath = file.getAbsolutePath();
        String content;
        InputStream is = getInputStream(filepath);
    }

    private static InputStream getInputStream(String filepath) throws Exception {
        return new FileInputStream(filepath);
    }

    private static String getContent(String filepath, InputStream is) throws Exception {
        String content = "";
        return content;
    }
}
