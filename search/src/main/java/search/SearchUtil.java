package search;

import index.IndexUtil;
import org.apache.lucene.analysis.Analyzer;
import org.apache.lucene.analysis.standard.StandardAnalyzer;
import org.apache.lucene.index.DirectoryReader;
import org.apache.lucene.queryparser.classic.QueryParser;
import org.apache.lucene.search.IndexSearcher;
import org.apache.lucene.search.Query;
import org.apache.lucene.search.TopDocs;

import java.io.IOException;

public class SearchUtil {
    private static Analyzer analyzer = new StandardAnalyzer();

    public static IndexSearcher getSearcher(String indexPath) throws IOException {
        DirectoryReader reader = DirectoryReader.open(IndexUtil.getIndexWriter(indexPath, false));
        return new IndexSearcher(reader);
    }

    public static TopDocs getResults(IndexSearcher searcher, Query query, int num) throws IOException {
        return searcher.search(query, num);
    }

    public static Query buildQuery(String searchText, String searchField) {
        Query query = null;
        try {
            QueryParser qp = new QueryParser(searchField, analyzer);
            query = qp.parse(searchText);
        } catch (Exception e) {
            e.printStackTrace();
        }
        return query;
    }
}
