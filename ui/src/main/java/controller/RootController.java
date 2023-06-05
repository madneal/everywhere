package controller;

import client.ClientWindow;

public class RootController {
    public void openAbout() {
        ClientWindow clientWindow = new ClientWindow();
        clientWindow.getHostServices().showDocument("https://madneal.github.io/everywhere");
    }
}
