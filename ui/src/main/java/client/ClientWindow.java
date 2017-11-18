package client;

import constants.CommonConstants;
import javafx.application.Application;
import javafx.fxml.FXMLLoader;
import javafx.scene.Scene;
import javafx.scene.control.ComboBox;
import javafx.scene.layout.AnchorPane;
import javafx.scene.layout.BorderPane;
import javafx.stage.Stage;

import java.io.File;
import java.io.IOException;
import java.net.URL;

public class ClientWindow extends Application {

    private Stage primaryStage;
    private BorderPane rootLayout;
    private ComboBox comboType;

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
//            primaryStage.show();
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
            primaryStage.show();
//
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static void main(String[] args) {
        launch(args);
    }

    private static URL getURL(String filepath) {
        File file = new File(filepath);
        URL url = null;
        try {
            url = file.toURL();
        } catch (Exception e) {
            e.printStackTrace();
        }
        return url;
    }
}
