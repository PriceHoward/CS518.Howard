#include <fstream>
#include <iostream>
#include <string>
#include <err.h>

void fileManipulation(std::string fileName){
    std::fstream iFile;
    std::fstream oFile;
    int lineCount = 1;

    iFile.open(fileName, std::ios::in);
    if(!iFile.is_open())
        err(1, "!COULD NOT OPEN INPUT FILE!");

    oFile.open("corrected_"+fileName, std::ios::out);
    if(!oFile.is_open())
        err(1, "!COULD NOT OPEN OUTPUT FILE!");

    std::string line;
    while(std::getline(iFile, line))
    {
        if(line.find_first_not_of(" \t\r\n") != std::string::npos)
        {
            line.erase(0, line.find_first_not_of(" \t\r\n"));
            line.erase(line.find_last_not_of(" \t\r\n") + 1);
            oFile << std::to_string(lineCount) + ": " + line + "\n"; 
            lineCount++;
        }
    }
    if(iFile.is_open())
        iFile.close();
    if(oFile.is_open())
        oFile.close();
}

int main()
{
    fileManipulation("CoolText.txt");
}