package constants;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class CommonConstant {
    public static List<String> DOCFILES = new ArrayList<String>(Arrays.asList(
            "doc", "docx", "xls", "xlsx", "pdf", "txt"
    ));
    public static List<String> EXCLUDE_FILE_PATHS = new ArrayList<String>(Arrays.asList(
            "node_modules"
    ));
}
