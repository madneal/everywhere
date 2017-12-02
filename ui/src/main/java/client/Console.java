package client;

import javafx.application.Platform;
import javafx.scene.control.TextArea;

import java.io.IOException;
import java.io.OutputStream;

public class Console extends OutputStream {
    private TextArea console;

    public Console(TextArea console) {
        this.console = console;
    }

    public void appendTetx(String valueOf) {
        Platform.runLater(() -> console.appendText(valueOf));
    }

    public void write(int b) throws IOException {
        appendTetx(String.valueOf((char)b));
    }
}
