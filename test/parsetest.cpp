#include <parser.h>
using namespace std::literals::string_literals;
int main()
{
    parser prs;
    prs.set(L"func sendMessagesToPlayers()\n{\n  for player in ServerPlayer.List.name {\n    flag = [[0x10000000] + 0x10] + 0x100;\n    (*flag)++;\n    player.Chat << f\"{player.name}さん、こんにちは！\";\n}"s);
    prs.debug();
    return 0;
}
