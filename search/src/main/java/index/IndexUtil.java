package index;

import constants.CommonConstants;
import file.FileBean;
import file.FileBeanParser;
import file.FileUtil;
import org.apache.lucene.analysis.Analyzer;
import org.apache.lucene.analysis.cn.smart.SmartChineseAnalyzer;
import org.apache.lucene.index.IndexWriter;
import org.apache.lucene.index.IndexWriterConfig;
import org.apache.lucene.index.LogByteSizeMergePolicy;
import org.apache.lucene.index.LogMergePolicy;
import org.apache.lucene.store.Directory;
import org.apache.lucene.store.FSDirectory;

import java.io.File;
import java.io.IOException;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

public class IndexUtil {

    public static void executeIndex(String searchType) {
        try {
            List<FileBean> fileBeans = new ArrayList<>();
            int totalCount = 0;
            if (CommonConstants.FULL_SEARCH.equals(searchType)) {
                List<String> driverPaths = FileUtil.getDriver();
                for (String driver : driverPaths) {
                    totalCount += runIndexByEachPath(driver);
                }
            } else {
                for (String path: CommonConstants.INPUT_DATA_PATH_LIST) {
                    totalCount += runIndexByEachPath(path);
                }
            }
            CommonConstants.TOTAL_FILE_NUM = String.valueOf(totalCount);
            BaseIndex.runIndex(fileBeans);
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public static void indexForFiles(List<String> filepathList) {
        List<FileBean> fileBeanMyArray = new ArrayList<>(1);
        try {
            for (String filepath: filepathList) {
                File file = new File(filepath);
                fileBeanMyArray.add(FileBeanParser.getFileBean(file));
            }
            BaseIndex.runIndex(fileBeanMyArray);
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    private static int runIndexByEachPath(String path) throws Exception {
        File dir = new File(path);
        String[] files = dir.list();
        int totalNum = 0;
        if (files != null) {
            for (String file: files) {
                file = path + "\\" + file;
                System.out.println("processing " + file);
                List<FileBean> fileBeans = new ArrayList<>();
                fileBeans.addAll(FileUtil.getFolderFiles(file));
                totalNum += fileBeans.size();
                BaseIndex.runIndex(fileBeans);
            }
        }
        return totalNum;
    }

    public static IndexWriter getIndexWriter(String indexPath, boolean create) throws IOException {
        Directory dir = FSDirectory.open(Paths.get(indexPath));
        Analyzer analyzer = new SmartChineseAnalyzer();
        IndexWriterConfig iwc = new IndexWriterConfig(analyzer);
        LogMergePolicy mergePolicy = new LogByteSizeMergePolicy();
        mergePolicy.setMergeFactor(50);
        mergePolicy.setMaxMergeDocs(5000);
        if (create){
            iwc.setOpenMode(IndexWriterConfig.OpenMode.CREATE);
        } else {
            iwc.setOpenMode(IndexWriterConfig.OpenMode.CREATE_OR_APPEND);
        }
        return new IndexWriter(dir, iwc);
    }
}
