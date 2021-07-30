#pragma once
#ifndef EVAL_H
#define EVAL_H
#include <string>

namespace parserTypes {
class Expr;
class expo;
class term;

class ptr;
namespace primary {
class BasePrimary;
class Pointer;
}  // namespace primary
}  // namespace parserTypes
class parserCore;
namespace eval {
void Expr(parserCore *that, parserTypes::expr::Expr val, int dest = 13);
void Expo(parserCore *that, parserTypes::expr::expo val, int dest = 13);
void Term(parserCore *that, parserTypes::expr::term val, int dest = 13);
void Power(parserCore *that, parserTypes::primary::BasePrimary &val,
           int dest = 13);
void Ptr(parserCore *that, parserTypes::primary::Pointer &val, int dest = 13);
void Var(parserCore *that, std::wstring obj, int dest);
void Ptr_Addr(parserCore *that, parserTypes::primary::Pointer &obj, int dest);
}  // namespace eval

#endif