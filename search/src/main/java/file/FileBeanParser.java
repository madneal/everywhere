package file;

import com.monitorjbl.xlsx.StreamingReader;
import org.apache.pdfbox.pdmodel.PDDocument;
import org.apache.pdfbox.text.PDFTextStripper;
import org.apache.poi.hssf.usermodel.HSSFWorkbook;
import org.apache.poi.hwpf.extractor.WordExtractor;
import org.apache.poi.poifs.filesystem.OfficeXmlFileException;
import org.apache.poi.ss.usermodel.*;
import org.apache.poi.xwpf.extractor.XWPFWordExtractor;
import org.apache.poi.xwpf.usermodel.XWPFDocument;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.util.Scanner;

public class FileBeanParser {
    final static Logger logger = LoggerFactory.getLogger(FileBeanParser.class);

    public static FileBean getFileBean(File file) throws Exception {
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
            content = readExcel(is);
        } else if (filepath.endsWith(".pdf")) {
            content = readPdf(is);
        } else if (filepath.endsWith(".txt")) {
            content = readTxt(is);
        }
        return content;
    }

    public static FileMagic getFileMagic(InputStream is) throws IOException {
        is = FileMagic.prepareToCheckMagic(is);
        return FileMagic.valueOf(is);
    }

    @SuppressWarnings("deprecation" )
    private static String readExcel(InputStream inp) throws Exception {
        Workbook wb;
        StringBuilder sb = new StringBuilder();
        boolean isXls = false;
        if (getFileMagic(inp) == FileMagic.OLE2) {
            wb = new HSSFWorkbook(inp);
            isXls = true;
        } else {
            wb = StreamingReader.builder()
                    .rowCacheSize(1000)    // number of rows to keep in memory (defaults to 10)
                    .bufferSize(4096)     // buffer size to use when reading InputStream to file (defaults to 1024)
                    .open(inp);            // InputStream or File for XLSX file (required)
        }
        sb = readSheet(wb, sb, isXls);
        wb.close();
        if (inp != null) {
            inp.close();
        }
        return sb.toString();
    }

    @SuppressWarnings({ "static-access", "deprecation" })
    private static StringBuilder readSheet(Workbook wb, StringBuilder sb, boolean isXls) throws Exception {
        for (Sheet sheet: wb) {
            for (Row r: sheet) {
                for (Cell cell: r) {
                    if (cell.getCellType() == Cell.CELL_TYPE_STRING) {
                        sb.append(cell.getStringCellValue());
                        sb.append(" ");
                    } else if (cell.getCellType() == Cell.CELL_TYPE_NUMERIC) {
                        if (isXls) {
                            DataFormatter formatter = new DataFormatter();
                            sb.append(formatter.formatCellValue(cell));
                        } else {
                            sb.append(cell.getStringCellValue());
                        }
                        sb.append(" ");
                    }
                }
            }
        }
        return sb;
    }

    private static String readDoc (String filePath, InputStream is) throws Exception {
        String text= "";
        is = FileMagic.prepareToCheckMagic(is);
        try {
            if (FileMagic.valueOf(is) == FileMagic.OLE2) {
                WordExtractor ex = new WordExtractor(is);
                text = ex.getText();
                ex.close();
            } else if(FileMagic.valueOf(is) == FileMagic.OOXML) {
                XWPFDocument doc = new XWPFDocument(is);
                XWPFWordExtractor extractor = new XWPFWordExtractor(doc);
                text = extractor.getText();
                extractor.close();
            }
        } catch (OfficeXmlFileException e) {
            logger.error(filePath, e);
        } finally {
            if (is != null) {
                is.close();
            }
        }
        return text;
    }

    private static String readTxt(InputStream is) throws Exception {
        StringBuilder sb = new StringBuilder();
        Scanner s = new Scanner(is);
        while (s.hasNext()) {
            sb.append(s.next());
        }
        if (is != null) {
            is.close();
        }
        return sb.toString();
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
}
