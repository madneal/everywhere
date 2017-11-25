package index;

import constants.CommonConstants;
import constants.LuceneConstants;
import file.FileBean;
import org.apache.lucene.document.Document;
import org.apache.lucene.document.Field;
import org.apache.lucene.document.StringField;
import org.apache.lucene.document.TextField;
import org.apache.lucene.index.IndexWriter;
import org.apache.lucene.index.IndexWriterConfig;
import org.apache.lucene.index.Term;
import searchutils.UtilsTool;

import java.io.IOException;
import java.util.List;

public class BaseIndex {

    private static void indexDocs(IndexWriter writer, List<FileBean> t) throws Exception{
        for (FileBean t2 : t) {
            indexDoc(writer, t2);
        }
    }

    private static void indexDoc(IndexWriter writer, FileBean t) throws Exception {
        Document doc = new Document();
        if (t.getContent() != null) {
            doc.add(new TextField(LuceneConstants.PATH, t.getFilepath(), Field.Store.YES));
            doc.add(new StringField(LuceneConstants.MODIFIED, UtilsTool.getDateStrByLastModified(t.getLastModified()), Field.Store.YES));
            doc.add(new TextField(LuceneConstants.CONTENT, t.getContent(), CommonConstants.IS_OPEN_CONTEXT ? Field.Store.YES : Field.Store.NO));
//            System.out.println("added to document:" + t.getFilepath());
            if (writer.getConfig().getOpenMode() == IndexWriterConfig.OpenMode.CREATE){
                writer.addDocument(doc);
            } else{
                writer.updateDocument(new Term(LuceneConstants.PATH, t.getFilepath()), doc);
            }
        }
    }

    public static void runIndex(List<FileBean> list) throws Exception {
        IndexWriter writer = null;
        try {
            writer = IndexUtil.getIndexWriter(CommonConstants.INDEX_FILE_PATH, false);
            indexDocs(writer, list);
        } catch (IOException e) {
            e.printStackTrace();
        } finally {
            try {
                writer.commit();
                writer.close();
            } catch (IOException e) {
                e.printStackTrace();
            }
        }
    }
}
