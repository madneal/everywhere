package controller;

import javafx.scene.input.InputMethodEvent;
import model.SearchedResult;

import java.util.ArrayList;
import java.util.List;

public class Controller {

    public void getSearchTextChanged(InputMethodEvent event) {
        String searchText = "";
        if (event.getCommitted() != "") {
            searchText += event.getCommitted();

        }
        System.out.println("txt:" + event.getCommitted());
    }

    public List<SearchedResult> getSearchResult() {
        List<SearchedResult> searchedResults = new ArrayList<>();
        return searchedResults;
    }
}
