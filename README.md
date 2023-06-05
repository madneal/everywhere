# everywhere :mag:

The name is inspired by everything, an awesome tool to search files in your PC. But the limitation is it can only search for filename. Recently, I am developing a tool to scan the documents in PC to search for some specific kind of data. A idea hits me that if I can develop a tool where I can search the content of files. It is the creation of this tool.

I wish this tool will perform just like its name!:smirk:

![qzsG6.gif](https://s1.ax1x.com/2017/12/14/qzsG6.gif)
![](http://okja9ah81.bkt.clouddn.com/everywhere.gif)

# Changelog
For detailed changelog, please refer [CHANGELOG](https://github.com/madneal/everywhere/blob/master/CHANGELOG.md)

### v0.1.3 [:arrow_down:](https://github.com/madneal/everywhere/releases/download/v0.1.3/app.zip)
* display the index process by a console
* fix cannot tell docx and doc for some files
* add about buton to see the website
* clear data when the search text is empty

### v0.1.2
* index by each path
* modify text read method

### v0.1.1
* update readme
* only click the cell instead of the whole row

### v0.1
* initial the project

## Usage
1. Download the tool from the corresponding tag.
2. Unzip the `app.zip`, and run the `everywhere.exe` in the folder.
3. The client window will show. For the first time, you should click the **index button** to index files in your PC(The index time depends on the number and size of your files. In my PC, it is less than 5 minutes). Then you can search everwhere :punch:

## Build the exe file
* First, build the `ui` module to a jar. If you are using IDEA, you can export the `ui` jar like this:
[![RHHun.md.png](https://s1.ax1x.com/2017/11/23/RHHun.md.png)](https://imgchr.com/i/RHHun)
* The main Class is `ClientWindow`.
* JSmooth is utilized to build exe in this case.

## Main features
* Indexed files include: doc, docx, xls, xslx, txt, pdf
* Nearly realtime search after index
* Now, the search supports Chinese and English words. Maybe the search still need optimization.
* There are two search types, including path and content. You can search file content and filepath.

## Environment requirements
* The application is based on x64 OS JDK.
* The JRE version is 1.8. The JRE has been included in the file folder.

## Main techniques
* [lucene](https://lucene.apache.org/core/)
* [javaFX](http://www.oracle.com/technetwork/java/javafx/overview/index.html)
* [POI](https://poi.apache.org/)

# Advanced setting
There are some config options in `config.yaml`. It can be used to config some environment for the application. For example, you can config `excludeFilePathList`,  and `fileList`. However, the fileList only includes `doc, docx, xls, xslx, txt,pdf`.

# Licene
[Apache License 2.0](https://github.com/madneal/everywhere/blob/master/LICENSE)
