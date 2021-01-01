#pragma once
#ifndef MCL_H
#define MCL_H

#include <string>
#include <../../lib/json/single_include/nlohmann/json.hpp>
using json = nlohmann::json;

class mcl{
private:
    json raw;
public:
    mcl(std::wstring name);
};

#endif