package searchutils;

import java.text.SimpleDateFormat;
import java.util.Date;

public class UtilsTool {

    public static String getDateStrByLastModified(long lastModified) {
        Date date = new Date(lastModified);
        SimpleDateFormat dateFormat = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");
        return dateFormat.format(date);
    }
}
