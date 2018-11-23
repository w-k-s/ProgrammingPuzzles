/**
*Electron Configuration Challenge
*Author: W.K.S.
*Description: http://codegolf.stackexchange.com/questions/62030/electron-configuration-diagrams
*/
#include <iostream>
#include <map>
#include <string>

struct Element{
    std::string symbol;
    unsigned char electrons;
};

constexpr int x[20] = {3,5,2,3,6,5,2,5,6,3,1,3,7,5,1,5,7,3,0,0};
constexpr int y[20] = {4,4,3,6,5,2,5,6,3,2,3,7,5,1,5,7,3,1,3,5};
const std::map<std::string,Element> ELEMENT_ELECTRON_MAP = {
    {"Hydrogen",{"H",1}},
    {"Helium",{"He",2}},
    {"Lithium",{"Li",3}},
    {"Berylium",{"Be",4}},
    {"Boron",{"B",5}},
    {"Carbon",{"C",6}},
    {"Nitrogen",{"N",7}},
    {"Oxygen",{"O",8}},
    {"Flourine",{"F",9}},
    {"Neon",{"Ne",10}},
    {"Sodium",{"Na",11}},
    {"Natrium",{"Na",11}},
    {"Magnesium",{"Mg",12}},
    {"Aluminium",{"Al",13}},
    {"Silicon",{"Si",14}},
    {"Phosphorus",{"P",15}},
    {"Sulfur",{"S",16}},
    {"Chlorine",{"Cl",17}},
    {"Argon",{"Ar",18}},
    {"Potassium",{"K",19}},
    {"Kadium",{"K",19}},
    {"Calcium",{"Ca",20}}
};

int main(int argc, const char * argv[]) {
    
    Element e;
    try{
        e = ELEMENT_ELECTRON_MAP.at(argv[1]);
    }catch(std::out_of_range){
        std::cout<<"Element '"<<argv[1]<<"' unknown.";
        exit(0);
    }
    
    bool display[8][8];
    
    memset(display, 0, sizeof(display));
    for(int i=0;i<e.electrons;i++){
        display[x[i]][y[i]] = true;
    }
    
    
    for (int i=0; i<8; i++) {
        for (int j=0; j<8; j++) {
            if (i == 4 && j==4) {
                std::cout<<e.symbol;
            }else{
                std::cout<< (display[i][j]? "X":" ");
            }
        }
        std::cout<<"\n";
    }
    
}