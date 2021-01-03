#pragma once
#ifndef MCL_H
#define MCL_H

#include <string>
#include <map>
#include <vector>

namespace parserTypes
{
    class parserContext;
} // namespace parserTypes


class mcl{
    using json=nlohmann::basic_json<>;
public:
    json raw;
    mcl(std::string name);
};

void operator<<(parserTypes::parserContext&, mcl);

#endif