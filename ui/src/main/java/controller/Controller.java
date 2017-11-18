package controller;

import client.ClientWindow;
import constants.LuceneConstants;
import index.IndexUtil;
import javafx.application.ConditionalFeature;
import javafx.application.Platform;
import javafx.collections.FXCollections;
import javafx.collections.ObservableList;
import javafx.event.ActionEvent;
import javafx.event.EventHandler;
import javafx.fxml.FXML;
import javafx.scene.control.*;
import javafx.scene.control.cell.PropertyValueFactory;
import javafx.scene.input.InputMethodEvent;
import javafx.scene.input.KeyEvent;
import javafx.scene.input.MouseButton;
import javafx.scene.input.MouseEvent;
import javafx.scene.paint.Color;
import javafx.util.Callback;
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
    private Button indexBtn;

    @FXML
    private TableColumn<SearchedResult, String> filepathCol;

    @FXML
    private TableColumn<SearchedResult, String> contextCol;

    String searchText = "";

    public void runIndex(ActionEvent e) {
        System.out.println(e);
        indexBtn.setText("indexing");
        executeIndex();
        indexBtn.setText("index finished!");
        indexBtn.setText("index");
    }

    private void executeIndex() {
        ConfigSetting configSetting = ConfigController.readConfig();
        IndexUtil.executeIndex(configSetting.getSearchMethod());
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
//        searchTextId.setText(searchText);
//        searchTextId.end();
        if (!searchText.isEmpty()) {
            List<SearchedResult> searchedResults = getSearchResult(searchText);
            showTableData(searchedResults);
        } else {
            tview.setItems(null);
        }
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
                Hyperlink hyperlink = new Hyperlink(result.getFilepath());
                result.setContext(searchedResult.getContext());
                result.setFilepath(searchedResult.getFilepath());
                result.setHyperlink(new Hyperlink(result.getFilepath()));
                searchedResult.setHyperlink(hyperlink);
                filepathCol.setCellFactory(new Callback<TableColumn<SearchedResult, String>, TableCell<SearchedResult, String>>() {
                    @Override
                    public TableCell<SearchedResult, String> call(TableColumn<SearchedResult, String> col) {
                        final TableCell<SearchedResult, String> cell = new TableCell<>();
                        cell.textProperty().bind(cell.itemProperty()); // in general might need to subclass TableCell and override updateItem(...) here
                        cell.setTextFill(Color.BLUE);
                        cell.setOnMouseClicked(new EventHandler<MouseEvent>() {
                            @Override
                            public void handle(MouseEvent event) {
                                if (event.getButton() == MouseButton.PRIMARY) {
                                    cell.setTextFill(Color.GRAY);
                                    ClientWindow clientWindow = new ClientWindow();
                                    clientWindow.getHostServices().showDocument(cell.getText());
                                    // handle right click on cell...
                                    // access cell data with cell.getItem();
                                    // access row data with (Person)cell.getTableRow().getItem();
                                }
                            }
                        });
                        return cell ;
                    }
                });
                filepathCol.setCellValueFactory(new PropertyValueFactory<SearchedResult, String>("filepath"));
                contextCol.setCellValueFactory(new PropertyValueFactory<SearchedResult, String>("context"));
                list.add(result);
            }
            tview.setItems(list);
        }
    }

}
