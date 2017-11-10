package search;

import constants.CommonConstants;
import constants.LuceneConstants;
import index.IndexUtil;
import org.apache.lucene.analysis.Analyzer;
import org.apache.lucene.analysis.cn.smart.SmartChineseAnalyzer;
import org.apache.lucene.document.Document;
import org.apache.lucene.index.DirectoryReader;
import org.apache.lucene.queryparser.classic.QueryParser;
import org.apache.lucene.search.IndexSearcher;
import org.apache.lucene.search.Query;
import org.apache.lucene.search.ScoreDoc;
import org.apache.lucene.search.TopDocs;
import org.apache.lucene.search.highlight.*;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

public class SearchUtil {
    private static Analyzer analyzer = new SmartChineseAnalyzer();
    private static IndexSearcher searcher;

    public static IndexSearcher getSearcher(String indexPath) throws IOException {
        if (CommonConstants.writer == null) {
            CommonConstants.writer = IndexUtil.getIndexWriter(CommonConstants.INDEX_FILE_PATH, false);
        }
        DirectoryReader reader = DirectoryReader.open(CommonConstants.writer);
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

    private static List<String> getContextListByDocumentList(List<Document> documentList, Query query, String searchField) {
        List<String> contextList = new ArrayList<>();
        Formatter formatter = new SimpleHTMLFormatter();
        QueryScorer queryScorer = new QueryScorer(query);
        Highlighter highlighter = new Highlighter(formatter, queryScorer);
        Fragmenter fragmenter = new SimpleFragmenter(20);
        highlighter.setTextFragmenter(fragmenter);
        try {
            for (Document doc : documentList) {
                String context = highlighter.getBestFragment(analyzer, searchField, doc.get(searchField));
                contextList.add(context);
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
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
            List<Document> documentList = getDocumentListByScoreDocs(topDocs.scoreDocs);
            List<String> contextList = getContextListByDocumentList(documentList, query, searchField);
            for (int i = 0; i < documentList.size(); i++) {
                SearchedResult searchedResult = new SearchedResult();
                Document document = documentList.get(i);
                searchedResult.setContext(contextList.get(i));
                searchedResult.setFilepath(document.get(LuceneConstants.PATH));
                searchedResult.setLastModified(document.get(LuceneConstants.MODIFIED));
                searchResults.add(searchedResult);
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        return searchResults;
    }
}
