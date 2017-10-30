package search;

import index.IndexUtil;
import org.apache.lucene.index.DirectoryReader;
import org.apache.lucene.search.IndexSearcher;
import org.apache.lucene.search.Query;
import org.apache.lucene.search.TopDocs;

import java.io.IOException;

public class SearchUtil {

    public static IndexSearcher getSearcher(String indexPath) throws IOException {
        DirectoryReader reader = DirectoryReader.open(IndexUtil.getIndexWriter(indexPath, false));
        return new IndexSearcher(reader);
    }

    public static TopDocs getResults(IndexSearcher searcher, Query query, int num) throws IOException {
        return searcher.search(query, num);
    }
}
