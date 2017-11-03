package file;

import com.monitorjbl.xlsx.StreamingReader;
import constants.CommonConstants;
import org.apache.pdfbox.pdmodel.PDDocument;
import org.apache.pdfbox.text.PDFTextStripper;
import org.apache.poi.hssf.usermodel.HSSFWorkbook;
import org.apache.poi.hwpf.extractor.WordExtractor;
import org.apache.poi.openxml4j.exceptions.OLE2NotOfficeXmlFileException;
import org.apache.poi.ss.usermodel.Cell;
import org.apache.poi.ss.usermodel.Row;
import org.apache.poi.ss.usermodel.Sheet;
import org.apache.poi.ss.usermodel.Workbook;
import org.apache.poi.xwpf.extractor.XWPFWordExtractor;
import org.apache.poi.xwpf.usermodel.XWPFDocument;

import java.io.File;
import java.io.FileInputStream;
import java.io.InputStream;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;

public class FileUtil {

    public static List<FileBean> getFolderFiles(String path) {
        List<FileBean> fileBeans = new ArrayList<FileBean>();
        File file = new File(path);
        String absoluteFilepath = file.getAbsolutePath();
        String extName = getFileExtension(file);
        try {
            if (file.isDirectory()) {
                if (isPassExcludePath(absoluteFilepath)) {
                    fileBeansAddForDirectory(fileBeans, file);
                }
            } else {
                if (CommonConstants.DOCFILES.contains(extName)) {
                    System.out.println("Scan file:" + path);
                    fileBeans.add(getFilebean(file));
                }
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        return fileBeans;
    }

    private static void fileBeansAddForDirectory(List<FileBean> fileBeans, File file) throws Exception {
        File[] files = file.listFiles();
        if (files != null) {
            for (File file1: files) {
                fileBeans.addAll(getFolderFiles(file1.getAbsolutePath()));
            }
        }
    }

    private static boolean isPassExcludePath(String filePath) {
        for (String excludePath : CommonConstants.EXCLUDE_FILE_PATHS) {
            if (filePath.contains(excludePath)) {
                return false;
            }
        }
        return true;
    }

    private static String getFileExtension(File file) {
        String fileName = file.getName();
        return fileName.substring(fileName.lastIndexOf(".") + 1);
    }

    private static FileBean getFilebean(File file) throws Exception {
        FileBean fileBean = new FileBean();
        String filepath = file.getAbsolutePath();
        String content;
        InputStream is = getInputStream(filepath);
        content = getContent(filepath, is);
        fileBean.setFilepath(filepath);
        fileBean.setLastModified(file.lastModified());
        fileBean.setContent(content);
        return fileBean;
    }

    private static InputStream getInputStream(String filepath) throws Exception {
        return new FileInputStream(filepath);
    }

    private static String getContent(String filepath, InputStream is) throws Exception {
        String content = "";
        if (filepath.endsWith(".doc") || filepath.endsWith(".docx")) {
            content = readDoc(filepath, is);
        } else if (filepath.endsWith(".xls") || filepath.endsWith(".xlsx")) {
            content = readExcel(filepath, is);
        } else if (filepath.endsWith(".pdf")) {
            content = readPdf(is);
        } else if (filepath.endsWith(".txt")) {
            content = readTxt(is);
        }
        return content;
    }

    @SuppressWarnings("deprecation" )
    private static String readExcel(String filePath, InputStream inp) throws Exception {
        Workbook wb;
        StringBuilder sb = new StringBuilder();
        try {
            if (filePath.endsWith(".xls")) {
                wb = new HSSFWorkbook(inp);
            } else {
                wb = StreamingReader.builder()
                        .rowCacheSize(1000)    // number of rows to keep in memory (defaults to 10)
                        .bufferSize(4096)     // buffer size to use when reading InputStream to file (defaults to 1024)
                        .open(inp);            // InputStream or File for XLSX file (required)
            }
            for (Sheet sheet: wb) {
                for (Row r: sheet) {
                    for (Cell cell: r) {
                        sb.append(getValue(cell));
                        sb.append(" ");
                    }
                }
            }

            wb.close();
        } catch (OLE2NotOfficeXmlFileException e) {
            e.printStackTrace();
        } finally {
            if (inp != null) {
                inp.close();
            }
        }
        return sb.toString();
    }

    @SuppressWarnings({ "static-access", "deprecation" })
    private static String getValue(Cell cell) {
        if (cell.getCellType() == cell.CELL_TYPE_BOOLEAN) {
            return String.valueOf(cell.getBooleanCellValue());
        } else if (cell.getCellType() == cell.CELL_TYPE_NUMERIC) {
            return String.valueOf(cell.getNumericCellValue());
        } else if (cell.getCellType() == cell.CELL_TYPE_FORMULA) {
            return String.valueOf(cell.getCellFormula());
        } else if (cell.getCellType() == cell.CELL_TYPE_ERROR) {
            return String.valueOf(cell.getErrorCellValue());
        } else {
            return String.valueOf(cell.getStringCellValue());
        }
    }

    private static String readDoc (String filePath, InputStream is) throws Exception {
        String text= "";
        try {
            if (filePath.endsWith("doc")) {
                WordExtractor ex = new WordExtractor(is);
                text = ex.getText();
                ex.close();
                is.close();
            } else if(filePath.endsWith("docx")) {
                XWPFDocument doc = new XWPFDocument(is);
                XWPFWordExtractor extractor = new XWPFWordExtractor(doc);
                text = extractor.getText();
                extractor.close();
                is.close();
            }
        } catch (OLE2NotOfficeXmlFileException e) {
            e.printStackTrace();
        } finally {
            if (is != null) {
                is.close();
            }
        }
        return text;
    }

    private static String readTxt(InputStream is) throws Exception {
        StringBuilder sb = new StringBuilder();
        Scanner s = new Scanner(is).useDelimiter("\\A");
        while (s.hasNext()) {
            sb.append(s.next());
        }
        if (is != null) {
            is.close();
        }
        String str = sb.toString().replace("\n", "");
        str = str.replace("\r", "\r\n");
        return str;
    }

    private static String readPdf(InputStream is) throws Exception {
        String result;
        PDDocument doc = PDDocument.load(is);
        PDFTextStripper stripper = new PDFTextStripper();
        result = stripper.getText(doc);
        if(doc!= null) {
            doc.close();
        }
        if (is != null) {
            is.close();
        }
        return result;
    }

    public static List<String> getDriver() {
        List<String> driverList = new ArrayList<>();
        File[] fs = File.listRoots();
        for (int i = 0; i < fs.length; i++) {
            driverList.add(fs[i].getAbsolutePath());
        }
        return driverList;
    }

}
