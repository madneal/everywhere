package client;

import constants.CommonConstants;
import javafx.application.Application;
import javafx.fxml.FXMLLoader;
import javafx.scene.Scene;
import javafx.scene.layout.AnchorPane;
import javafx.scene.layout.BorderPane;
import javafx.stage.Stage;

import java.io.File;
import java.io.IOException;
import java.net.URL;

public class ClientWindow extends Application {

    private Stage primaryStage;
    private BorderPane rootLayout;

    @Override
    public void start(Stage primaryStage) {
        this.primaryStage = primaryStage;
        this.primaryStage.setTitle("Search Everywhere");
        initRootLayout();
        showMainTab();
    }

    /**
     * Initializes the root layout.
     */
    public void initRootLayout() {
        try {
            FXMLLoader loader = new FXMLLoader();
            loader.setLocation(getURL(CommonConstants.ROOT_LAYOUT_PATH));
            rootLayout = (BorderPane) loader.load();

            // Show the scene containing the root layout.
            Scene scene = new Scene(rootLayout);
            primaryStage.setScene(scene);
            primaryStage.setMaximized(true);
            primaryStage.show();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public void showMainTab() {
        try {
            FXMLLoader loader = new FXMLLoader();
            loader.setLocation(getURL(CommonConstants.SETTING_PATH));
            AnchorPane settingView = (AnchorPane) loader.load();
            rootLayout.setCenter(settingView);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static void main(String[] args) {
        // disable apache pdfbox logging warn
        System.setProperty("org.apache.commons.logging.Log",
                "org.apache.commons.logging.impl.NoOpLog");
        launch(args);
    }

    private static URL getURL(String filepath) {
        File file = new File(filepath);
        URL url = null;
        try {
            url = file.toURI().toURL();
        } catch (Exception e) {
            e.printStackTrace();
        }
        return url;
    }
}
