package client;

import javafx.application.Platform;
import javafx.scene.control.TextArea;

import java.io.IOException;
import java.io.OutputStream;
import java.io.PrintStream;
import java.io.UnsupportedEncodingException;
import java.util.ArrayList;
import java.util.List;

public class Console extends OutputStream {
    private TextArea console;
    private PrintStream ps;
    private static List<Byte> bytes = new ArrayList<>();

    public Console(TextArea console) {
        this.console = console;
        this.ps = System.out;
    }

    private void update() {
        byte[] array = new byte[bytes.size()];
        int q = 0;
        for (Byte current : bytes) {
            array[q] = current;
            q++;
        }
        try {
            console.setText(new String(array, "UTF-8"));
            console.setScrollTop(Double.MAX_VALUE);
        } catch (UnsupportedEncodingException e) {
            e.printStackTrace();
        }
    }
    @Override
    public void write(int i) throws IOException {
        Platform.runLater(() ->  {
            bytes.add((byte)i);
            ps.write(i);
            update();
        });
    }

    @Override
    public void write(byte[] i) {
        Platform.runLater(() -> {
            for (byte b : i) {
                bytes.add(b);
                ps.write(b);
                update();
            }
        });
    }
}
