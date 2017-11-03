package search;

import constants.CommonConstants;
import index.IndexUtil;
import model.SearchedResult;
import org.apache.lucene.analysis.Analyzer;
import org.apache.lucene.analysis.standard.StandardAnalyzer;
import org.apache.lucene.document.Document;
import org.apache.lucene.index.DirectoryReader;
import org.apache.lucene.queryparser.classic.QueryParser;
import org.apache.lucene.search.IndexSearcher;
import org.apache.lucene.search.Query;
import org.apache.lucene.search.ScoreDoc;
import org.apache.lucene.search.TopDocs;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

public class SearchUtil {
    private static Analyzer analyzer = new StandardAnalyzer();
    private static IndexSearcher searcher;

    public static IndexSearcher getSearcher(String indexPath) throws IOException {
        DirectoryReader reader = DirectoryReader.open(IndexUtil.getIndexWriter(indexPath, false));
        return new IndexSearcher(reader);
    }

    public static TopDocs getTopDocs(IndexSearcher searcher, Query query, int num) throws IOException {
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

    private static List<Document> getDocumentListByScoreDocs(ScoreDoc[] scoreDocs) {
        List<Document> documentList = new ArrayList<>();
        try {
            for (ScoreDoc scoreDoc: scoreDocs) {
                Document document = searcher.doc(scoreDoc.doc);
                documentList.add(document);
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        return documentList;
    }

    private static List<String> getContextListByDocumentList(List<Document> documentList) {
        List<String> contextList = new ArrayList<>();
        return contextList;
    }
    private static List<SearchedResult> getSearchedResultsByTopDocs(TopDocs topDocs) {
        List<SearchedResult> searchedResults = new ArrayList<>();
        return searchedResults;
    }

    public static List<SearchedResult> executeSearch(String searchText, String searchField) {
        List<SearchedResult> searchResults = new ArrayList<>();
        try {
            Query query = buildQuery(searchText, searchField);
            searcher = getSearcher(CommonConstants.INDEX_FILE_PATH);
            TopDocs topDocs = getTopDocs(searcher, query, 10);
        } catch (Exception e) {
            e.printStackTrace();
        }
        return searchResults;
    }
}
