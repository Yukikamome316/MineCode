#pragma once
#ifndef PARSER_CORE_H
#define PARSER_CORE_H

#include <string>
#include <vector>
#include <sstream>

namespace parserTypes
{
    class condChild;
    class cond;
    class parserContext;
} // namespace parserTypes


namespace parserCore
{
    using Arg=std::pair<std::wstring,std::wstring>;
    using Range=std::pair<int,int>;

    int Int  (parserTypes::parserContext&);
    std::wstring ident(parserTypes::parserContext&);

    Arg arg(parserTypes::parserContext&);
    std::wstring attribute(parserTypes::parserContext&);
    std::wstring ptr(parserTypes::parserContext&);
    
    std::wstring editable(parserTypes::parserContext&);
    std::wstring constant(parserTypes::parserContext&);
    std::wstring value(parserTypes::parserContext&);
    
    std::wstring power (parserTypes::parserContext&);
    std::wstring expo  (parserTypes::parserContext&);
    std::wstring term  (parserTypes::parserContext&);
    std::wstring expr  (parserTypes::parserContext&);

    struct cond cond  (parserTypes::parserContext&);
    struct condChild cond_inner  (parserTypes::parserContext&);

    Range range  (parserTypes::parserContext&);

    void program(parserTypes::parserContext&);
    void stmt(parserTypes::parserContext&);
    void func(parserTypes::parserContext&);
    void If(parserTypes::parserContext&);
    void For(parserTypes::parserContext&);
    void While(parserTypes::parserContext&);
    void put(parserTypes::parserContext&);
    void assign(parserTypes::parserContext&);
} // namespace parserWrap


#endif