package controller;

import javafx.beans.value.ChangeListener;
import javafx.fxml.FXML;
import javafx.scene.control.TextField;

public class Controller {
    @FXML
    private TextField textField;

    textField.textProperty().addListener(new ChangeListener<String>() {
        @Override
        public void changed(ObservableValue<? extends String> observable,
                String oldValue, String newValue) {

            outputTextArea.appendText("TextField Text Changed (newValue: " + newValue + ")\n");
        }
    });
}
