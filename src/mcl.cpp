#include <mcl.h>

#include <fstream>

#include <parserTypes.h>

using namespace std::literals::string_literals;
mcl::mcl(std::string name){
    std::string path;
    path+="mcls/";
    path+=name;
    path+=".json";
    std::ifstream jsonin;
    jsonin.open(path);
    jsonin >> raw;
}

void operator<<(parserTypes::parserContext& ctx, mcl pl){
    
}