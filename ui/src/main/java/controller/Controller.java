package controller;

import javafx.fxml.FXML;
import javafx.scene.control.TextField;
import javafx.scene.input.InputMethodEvent;

public class Controller {
    @FXML
    private TextField textField;


    public void getSearchTextChanged(InputMethodEvent event) {
        System.out.println("txt:" + textField.getText());
    }
}
