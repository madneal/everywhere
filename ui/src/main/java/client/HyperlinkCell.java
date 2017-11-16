package client;

import javafx.scene.control.Hyperlink;
import javafx.scene.control.TableCell;
import javafx.scene.control.TableColumn;
import javafx.util.Callback;
import search.SearchedResult;

public class HyperlinkCell implements  Callback<TableColumn<SearchedResult, Hyperlink>, TableCell<SearchedResult, Hyperlink>> {

    @Override
    public TableCell<SearchedResult, Hyperlink> call(TableColumn<SearchedResult, Hyperlink> arg) {
        TableCell<SearchedResult, Hyperlink> cell = new TableCell<SearchedResult, Hyperlink>() {
            @Override
            protected void updateItem(Hyperlink item, boolean empty) {
                setGraphic(item);
            }
        };
        return cell;
    }
}
