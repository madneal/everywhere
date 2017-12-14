package search;

import constants.CommonConstants;
import constants.LuceneConstants;
import org.apache.lucene.analysis.Analyzer;
import org.apache.lucene.analysis.cn.smart.SmartChineseAnalyzer;
import org.apache.lucene.document.Document;
import org.apache.lucene.index.DirectoryReader;
import org.apache.lucene.index.IndexReader;
import org.apache.lucene.index.Term;
import org.apache.lucene.queryparser.classic.QueryParser;
import org.apache.lucene.search.*;
import org.apache.lucene.search.highlight.*;
import org.apache.lucene.search.uhighlight.UnifiedHighlighter;
import org.apache.lucene.store.Directory;
import org.apache.lucene.store.FSDirectory;

import java.io.IOException;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

public class SearchUtil {
    private static Analyzer analyzer = new SmartChineseAnalyzer();
    private static IndexSearcher searcher;

    public static IndexSearcher getSearcher() throws IOException {
        Directory directory = FSDirectory.open(Paths.get(CommonConstants.INDEX_FILE_PATH));
        IndexReader reader = DirectoryReader.open(directory);
        return new IndexSearcher(reader);
    }

    public static TopDocs getTopDocs(IndexSearcher searcher, Query query, int num) throws IOException {
        return searcher.search(query, num);
    }

    public static Query buildQuery(String searchText, String searchField) {
        Query query = null;
        try {
            if (searchField.equals(LuceneConstants.CONTENT)) {
                QueryParser qp = new QueryParser(searchField, analyzer);
                query = qp.parse(searchText);
            } else {
                Term term = new Term(LuceneConstants.PATH, searchText);
                query = new TermQuery(term);
            }
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

    private static List<String> getContextListByTopDocs(Query query, TopDocs topDocs, IndexSearcher searcher) {
        List<String> contextList = new ArrayList<>();
        UnifiedHighlighter highlighter = new UnifiedHighlighter(searcher, analyzer);
        try {
            String[] fragments = highlighter.highlight(LuceneConstants.CONTENT, query, topDocs);
            for (String fragment: fragments) {
                contextList.add(getBestFragment(fragment));
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        return contextList;
    }

    private static String getBestFragment(String fragment) throws Exception {
        int startPosition = fragment.indexOf("<b>");
        int endPosition = fragment.indexOf("</b>");
        if (startPosition == -1) {
            return "未获取查询结果摘要，请到特定文件搜索结果";
        }
        if (startPosition > 10) {
            startPosition -= 10;
        }
        if (endPosition + 10 < fragment.length()) {
            endPosition += 10;
        }
        return fragment.substring(startPosition, endPosition);

    }

    public static List<SearchedResult> executeSearch(String searchText, String searchField) {
        List<SearchedResult> searchResults = new ArrayList<>();
        try {
            Query query = buildQuery(searchText, searchField);
            searcher = getSearcher();
            TopDocs topDocs = getTopDocs(searcher, query, 50);
            int totalHits = topDocs.totalHits;
            if (totalHits > 50) {
                topDocs = getTopDocs(searcher, query, totalHits);
            }
            List<Document> documentList = getDocumentListByScoreDocs(topDocs.scoreDocs);
            List<String> contextList = getContextListByTopDocs(query, topDocs, searcher);
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
