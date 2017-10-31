package setting;

import constants.Constants;
import org.yaml.snakeyaml.Yaml;
import org.yaml.snakeyaml.constructor.Constructor;

import java.io.File;
import java.io.FileOutputStream;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Paths;

public class ConfigController {

    public static ConfigSetting readConfig() {
        ConfigSetting configSetting = null;
        try {
            InputStream configIs = Files.newInputStream(Paths.get(Constants.CONFIG_FILEPATH));
            Yaml yaml = new Yaml(new Constructor(ConfigSetting.class));
            configSetting = yaml.loadAs(configIs, ConfigSetting.class);
        } catch (Exception e) {
            e.printStackTrace();
        }
        return configSetting;
    }

    public static void writeConfigToYaml(ConfigSetting configSetting) {
        try {
            Yaml yaml = new Yaml();
            String output = yaml.dump(configSetting);
            byte[] sourceByte = output.getBytes();
            File file = new File(Constants.CONFIG_FILEPATH);
            if (!file.exists()) {
                file.createNewFile();
            }
            FileOutputStream fileOutputStream = new FileOutputStream(file);
            fileOutputStream.write(sourceByte);
            fileOutputStream.close();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
