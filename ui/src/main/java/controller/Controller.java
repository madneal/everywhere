package controller;

import constants.LuceneConstants;
import javafx.application.ConditionalFeature;
import javafx.application.Platform;
import javafx.fxml.FXML;
import javafx.scene.control.TextField;
import javafx.scene.input.InputMethodEvent;
import javafx.scene.input.KeyEvent;
import search.SearchUtil;
import search.SearchedResult;

import java.util.List;

public class Controller {
    @FXML
    public TextField searchTextId;
    String searchText = "";

    public void getSearchTextChanged(InputMethodEvent event) {
        Platform.isSupported(ConditionalFeature.INPUT_METHOD);
        System.out.println("input" + event.getEventType());
        if (!event.getCommitted().isEmpty()) {
            searchText += event.getCommitted();
            System.out.println(searchTextId);
            searchTextId.setText(searchText);
            searchTextId.end();
            List<SearchedResult> searchedResults = getSearchResult(searchText);
        }
    }

    public void getKeyTyped(KeyEvent keyEvent) {
        if (keyEvent.getCharacter() == "\b") {
            searchText = searchText.substring(0, searchText.length() - 1);
        } else {
            if (!keyEvent.getCharacter().contains("\\")) {
               searchText += keyEvent.getCharacter();
            }
        }
        List<SearchedResult> searchedResults = getSearchResult(searchText);
    }

    public List<SearchedResult> getSearchResult(String searchText) {
        List<SearchedResult> searchedResults = SearchUtil.executeSearch(searchText, LuceneConstants.CONTENT);
        return searchedResults;
    }
}
