#include <mcl.h>

#include <codecvt>
#include <fstream>
#include <locale>
#include <unordered_map>
#include <../../lib/json/single_include/nlohmann/json.hpp>

#include <parserTypes.h>
#include <parserWrap.h>
#include <parserCore.h>

        #include <util.h>
using namespace std::literals::string_literals;
using json = nlohmann::json;

json convertTree2Single(json);

void operator<<(parserWrap& ctx, std::string name){
    json j;
    std::ifstream jsonin;
    jsonin.open("mcls/"+name+".json");
    jsonin >> j;
    
    // load datas
    json puts=convertTree2Single(j["put"]);
    json pointers=j["pointers"];
    json functions=convertTree2Single(j["functions"]);
    auto converter=std::wstring_convert<std::codecvt_utf8<wchar_t>>();
    std::wstring pointerasm;
    
    parserWrap compiler;

    // compile pointers
    for(auto pointer: j["pointers"]){
        auto name=pointer[0].get<std::string>();
        auto expr=pointer[1].get<std::string>();
        std::wcout<<"setting of "<<converter.from_bytes(name)<<std::endl;
        
        std::wstring source=converter.from_bytes(name+" = "+expr);
        
        pointerasm += ctx.compile(source);
    }
    ctx.clear();

    for(auto [name,obj]: puts.items()){
        std::wcout<<"compile of "<<converter.from_bytes(name)<<std::endl;

        auto type=obj["type"].get<std::string>();
        if(type=="MineCode"){
            std::wstring source=converter.from_bytes(obj["proc"].get<std::string>());
            ctx.ctx.puts[name]=converter.to_bytes(ctx.compile(source));
        }else if(type=="asm"){
            ctx.ctx.puts[name]=obj["proc"].get<std::string>();
        }
    }
    std::wcout<<"compiled all"<<std::endl;
    ctx.clear();
    ctx.ctx.stream<<pointerasm;
}

bool isShallow(json src){
    for(auto [key,val]:src.items()){
        if(val.is_object()){
            return false;
        }
    }
    return true;
}

json convertTree2Single(json src){
    json dest;
    if(isShallow(src)){
        return src;
    }else{
        for(auto [key,val]: src.items()){
            json tmp = convertTree2Single(val);
            if(isShallow(tmp)){
                dest[key]=tmp;
            }else{
                for (auto [cKey,cVal]: tmp.items()){
                    dest[key+"."+cKey]=cVal;
                }
            }
            
        }
    }
    return dest;
}