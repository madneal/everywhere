package client;

import constants.CommonConstants;
import javafx.application.Application;
import javafx.fxml.FXMLLoader;
import javafx.scene.Scene;
import javafx.scene.control.TabPane;
import javafx.scene.layout.AnchorPane;
import javafx.scene.layout.BorderPane;
import javafx.stage.Stage;

import java.io.File;
import java.io.IOException;
import java.net.URL;

public class ClientWindow extends Application {

    private Stage primaryStage;
    private BorderPane rootLayout;
    private TabPane mainTab;

    @Override
    public void start(Stage primaryStage) {
        this.primaryStage = primaryStage;
        this.primaryStage.setTitle("Search Everywhere");
        initRootLayout();
        showPersonOverview();
//        SingleSelectionModel<Tab> selectionModel = mainTab.getSelectionModel();
//        selectionModel.select(0);
    }

    /**
     * Initializes the root layout.
     */
    public void initRootLayout() {
        try {
            // Load root layout from fxml file.
            FXMLLoader loader = new FXMLLoader();
            loader.setLocation(getURL(CommonConstants.ROOT_LAYOUT_PATH));
            rootLayout = (BorderPane) loader.load();

            // Show the scene containing the root layout.
            Scene scene = new Scene(rootLayout);
            primaryStage.setScene(scene);
            primaryStage.show();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    /**
     * Shows the person overview inside the root layout.
     */
    public void showPersonOverview() {
        try {
            // Load person overview.
            FXMLLoader loader = new FXMLLoader();
            loader.setLocation(getURL(CommonConstants.SETTING_PATH));
            AnchorPane settingView = (AnchorPane) loader.load();
            rootLayout.setCenter(settingView);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    /**
     * Returns the main stage.
     * @return
     */
    public Stage getPrimaryStage() {
        return primaryStage;
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
