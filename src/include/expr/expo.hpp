#pragma once

#include <vector>

#include "power.hpp"
namespace parserTypes {
struct expo {
  std::vector<struct power> parts;
  bool isSingle();
};
}  // namespace parserTypes