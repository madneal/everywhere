package index;

import constants.CommonConstants;
import file.FileBean;
import file.FileUtil;
import org.apache.lucene.analysis.Analyzer;
import org.apache.lucene.analysis.standard.StandardAnalyzer;
import org.apache.lucene.index.IndexWriter;
import org.apache.lucene.index.IndexWriterConfig;
import org.apache.lucene.index.LogByteSizeMergePolicy;
import org.apache.lucene.index.LogMergePolicy;
import org.apache.lucene.store.Directory;
import org.apache.lucene.store.FSDirectory;

import java.io.IOException;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

public class IndexUtil {

    public static void executeIndex(String searchType) throws Exception {
        List<FileBean> fileBeans = new ArrayList<>();
        if (searchType == CommonConstants.FULL_SEARCH) {
            List<String> driverPaths = FileUtil.getDriver();
            for (String driver : driverPaths) {
                fileBeans.addAll(FileUtil.getFolderFiles(driver));
            }
        } else {
            fileBeans = FileUtil.getFolderFiles(CommonConstants.INPUT_FILE_PATH);
        }
        int totalCount = fileBeans.size();
        CommonConstants.TOTAL_FILE_NUM = String.valueOf(totalCount);
        System.out.println(fileBeans.size());
        System.out.println("开始创建索引");
        BaseIndex.runIndex(fileBeans);
        System.out.println("所有线程都创建索引完毕");
    }

    public static IndexWriter getIndexWriter(String indexPath, boolean create) throws IOException {
        Directory dir = FSDirectory.open(Paths.get(indexPath));
        Analyzer analyzer = new StandardAnalyzer();
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
