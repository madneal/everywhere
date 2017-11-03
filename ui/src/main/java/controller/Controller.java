package controller;

import constants.LuceneConstants;
import javafx.scene.input.InputMethodEvent;
import search.SearchedResult;
import search.SearchUtil;

import java.util.ArrayList;
import java.util.List;

public class Controller {

    public void getSearchTextChanged(InputMethodEvent event) {
        String searchText = "";
        if (!event.getCommitted().isEmpty()) {
            searchText += event.getCommitted();
            List<SearchedResult> searchedResults = SearchUtil.executeSearch(searchText, LuceneConstants.CONTENT);
            for (SearchedResult searchedResult: searchedResults) {
                System.out.println(searchedResult.getFilepath());
            }
        }
        System.out.println("txt:" + event.getCommitted());
    }

    public List<SearchedResult> getSearchResult() {
        List<SearchedResult> searchedResults = new ArrayList<>();
        return searchedResults;
    }
}
