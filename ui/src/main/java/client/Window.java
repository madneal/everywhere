package client;

import index.IndexUtil;
import javafx.application.Application;
import javafx.fxml.FXMLLoader;
import javafx.scene.Scene;
import javafx.scene.layout.AnchorPane;
import javafx.scene.layout.BorderPane;
import javafx.stage.Stage;
import setting.ConfigController;
import setting.ConfigSetting;

import java.io.IOException;

public class Window extends Application {

    private Stage primaryStage;
    private BorderPane rootLayout;
    private ConfigSetting configSetting;

    @Override
    public void start(Stage primaryStage) {
        this.primaryStage = primaryStage;
        this.primaryStage.setTitle("Search Everywhere");
        configSetting = ConfigController.readConfig();
        initRootLayout();
        showPersonOverview();
    }

    /**
     * Initializes the root layout.
     */
    public void initRootLayout() {
        try {
            // Load root layout from fxml file.
            FXMLLoader loader = new FXMLLoader();
            loader.setLocation(Window.class.getResource("../view/RootLayout.fxml"));
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
            loader.setLocation(Window.class.getResource("../view/Setting.fxml"));
            AnchorPane settingView = (AnchorPane) loader.load();

            // Set person overview into the center of root layout.
            rootLayout.setCenter(settingView);
            if (!configSetting.getHasCreateIndex()) {
                IndexUtil.executeIndex(configSetting.getSearchMethod());
                configSetting.setHasCreateIndex(true);
                ConfigController.writeConfigToYaml(configSetting);
            }
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
}
