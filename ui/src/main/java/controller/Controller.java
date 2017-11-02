package controller;

import javafx.scene.input.InputMethodEvent;
import model.SearchResult;

import java.util.ArrayList;
import java.util.List;

public class Controller {

    public void getSearchTextChanged(InputMethodEvent event) {
        System.out.println("txt:" + event.getCommitted());
    }

    public List<SearchResult> getSearchResult() {
        List<SearchResult> searchResults = new ArrayList<>();
        return searchResults;
    }
}
