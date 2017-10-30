import org.apache.lucene.document.Document;
import org.apache.lucene.search.ScoreDoc;
import org.apache.lucene.search.TopDocs;

import java.util.ArrayList;
import java.util.List;

public class QueryResult {

    String conditionType;

    String queryCondition;

    TopDocs docs;

    ScoreDoc[] scoreDocs;

    boolean isSuccess;

    int totalHits;

    List<String> contextList;

    List<Document> documentList = new ArrayList<>();

    public List<Document> getDocumentList() {
        return documentList;
    }

    public void setDocumentList(List<Document> documentList) {
        this.documentList = documentList;
    }

    public boolean getIsSuccess() {
        return isSuccess;
    }

    public void setIsSucess(boolean success) {
        isSuccess = success;
    }

    public String getConditionType() {
        return conditionType;
    }

    public void setConditionType(String conditionType) {
        this.conditionType = conditionType;
    }

    public int getTotalHits() {
        return totalHits;
    }

    public void setTotalHits(int totalHits) {
        this.totalHits = totalHits;
    }

    public String getQueryCondition() {
        return queryCondition;
    }

    public void setQueryCondition(String queryCondition) {
        this.queryCondition = queryCondition;
    }

    public TopDocs getDocs() {
        return docs;
    }

    public void setDocs(TopDocs docs) {
        this.docs = docs;
    }

    public ScoreDoc[] getScoreDocs() {
        return scoreDocs;
    }

    public void setScoreDocs(ScoreDoc[] scoreDocs) {
        this.scoreDocs = scoreDocs;
    }

    public List<String> getContextList() {
        return contextList;
    }

    public void setContextList(List<String> contextList) {
        this.contextList = contextList;
    }

}
