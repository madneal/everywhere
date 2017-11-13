package controller;

import constants.LuceneConstants;
import javafx.application.ConditionalFeature;
import javafx.application.HostServices;
import javafx.application.Platform;
import javafx.collections.FXCollections;
import javafx.collections.ObservableList;
import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.scene.control.Hyperlink;
import javafx.scene.control.TableColumn;
import javafx.scene.control.TableView;
import javafx.scene.control.TextField;
import javafx.scene.control.cell.PropertyValueFactory;
import javafx.scene.input.InputMethodEvent;
import javafx.scene.input.KeyEvent;
import javafx.scene.input.MouseEvent;
import search.SearchUtil;
import search.SearchedResult;
import setting.ConfigController;
import setting.ConfigSetting;

import java.util.List;

public class Controller {
    @FXML
    public TextField searchTextId;

    @FXML
    private TableView tview;

    @FXML
    private TableColumn<SearchedResult, String> filepathCol;

    @FXML
    private TableColumn<SearchedResult, String> contextCol;

    private HostServices hostServices;

    String searchText = "";

    public void runIndex(ActionEvent e) {
        executeIndex();
    }

    private void executeIndex() {
        ConfigSetting configSetting = ConfigController.readConfig();
        if (configSetting.getHasCreateIndex() == false) {
            configSetting.setHasCreateIndex(true);
        }
        ConfigController.writeConfigToYaml(configSetting);
    }

    public void getSearchTextChanged(InputMethodEvent event) {
        Platform.isSupported(ConditionalFeature.INPUT_METHOD);
        if (!event.getCommitted().isEmpty()) {
            searchText += event.getCommitted();
            searchTextId.setText(searchText);
            searchTextId.end();
            System.out.println(searchText);
            List<SearchedResult> searchedResults = getSearchResult(searchText);
            showTableData(searchedResults);
        }
    }

    public void getKeyTyped(KeyEvent keyEvent) {
        if ("\b".equals(keyEvent.getCharacter()) && !searchText.isEmpty()) {
            searchText = searchText.substring(0, searchText.length() - 1);
        } else {
            if (!keyEvent.getCharacter().contains("\\")) {
               searchText += keyEvent.getCharacter();
            }
        }
        System.out.println(searchText);
        searchTextId.setText(searchText);
        searchTextId.end();
        List<SearchedResult> searchedResults = getSearchResult(searchText);
        showTableData(searchedResults);
    }

    private List<SearchedResult> getSearchResult(String searchText) {
        List<SearchedResult> searchedResults = SearchUtil.executeSearch(searchText, LuceneConstants.CONTENT);
        return searchedResults;

    }

    private void showTableData(List<SearchedResult> searchedResults) {
        ObservableList<SearchedResult> list = FXCollections.observableArrayList();
        if (searchedResults != null && searchedResults.size() != 0) {
            for (SearchedResult searchedResult: searchedResults) {
                SearchedResult result = new SearchedResult();
                result.setContext(searchedResult.getContext());
                result.setFilepath(searchedResult.getFilepath());
                result.setHyperlink(new Hyperlink(result.getFilepath()));
                filepathCol.setCellValueFactory(new PropertyValueFactory<SearchedResult, String>("hyperlink"));
                contextCol.setCellValueFactory(new PropertyValueFactory<SearchedResult, String>("context"));
                list.add(result);
            }
            tview.setItems(list);
        }
    }

    @FXML
    private void clickTable(MouseEvent event) {
        System.out.println(event);
    }

    private void open(String link) {
        hostServices.showDocument(link);
    }
}
