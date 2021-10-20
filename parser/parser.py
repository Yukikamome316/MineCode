from typing import Iterable, Optional, Union

from parser.stmt.func import Func

from .stmt.stmt import Stmt
from .stmt.mcl import Mcl

from .stmt.program import Program


from ..lexer import token
from ..lexer.layer import Element, Layer, Token


class Parser:
    def __init__(self, elements: Iterable[Element]):
        self.elements = elements

    def peek(self, consume=True) -> Optional[Element]:
        if not self.elements:
            return None

        element = self.elements[0]

        if consume:
            self.elements.pop(0)

        return element

    def has_elements(self) -> bool:
        return len(self.elements) > 0

    def expect_token(self, token_type: type, exception=True, consume=True) -> Union[int, str, None]:
        tok = self.peek(consume)

        if not isinstance(tok, Token):
            if not exception:
                return None
            raise Exception(f"Expected {token_type}, got {tok}")

        token = tok.token
        if not isinstance(token, token_type):
            if not exception:
                return None
            raise Exception(f"Expected {token_type}, got {tok}")

        return token.value

    def expect_number(self, exception=True, consume=True) -> int:
        return self.expect_token(token.NumberToken, exception, consume)

    def expect_string(self, exception=True, consume=True) -> str:
        return self.expect_token(token.StringToken, exception, consume)

    def expect_identifier(self, expected: str = "", exception=True, consume=True) -> str:
        test = self.expect_token(token.IdentifierToken, exception, consume)
        if expected == "":
            return test

        if test == expected:
            return test

        raise Exception(f"Expected {expected}, got {test}")

    def expect_operator(self, expected: str = "", exception=True, consume=True):
        test = self.expect_token(token.OperatorToken, exception, consume)
        if expected == "":
            return test

        if test == expected:
            return test

        raise Exception(f"Expected {expected}, got {test}")

    def read_block(self) -> Iterable[Stmt]:
        elm = self.peek(False)
        if not elm:
            raise Exception("Unexpected end of file")

        if not isinstance(elm, Layer):
            raise Exception("Expected layer, got {elm}")

        self.elements.pop(0)

        parser = Parser(elm.tokens)
        return parser.parse().stmts

    def parse(self) -> Program:
        ret = Program()

        stmt = self.parse_stmt()
        while self.has_elements():
            ret.add_stmt(stmt)
            stmt = self.parse_stmt()

        return ret

    def parse_stmt(self) -> Stmt:
        tok = self.expect_identifier()

        if tok == "for":
            return self.parse_stmt_for()

        if tok == "while":
            return self.parse_stmt_while()

        if tok == "if":
            return self.parse_stmt_if()

        if tok == "func":
            return self.parse_stmt_func()

        if tok == "mcl":
            return self.parse_stmt_mcl()

        if tok == "return":
            return self.parse_stmt_return()

        return self.parse_stmt_expr()

    def parse_stmt_mcl(self) -> Mcl:
        mcl = self.expect_identifier()
        return Mcl(mcl)

    def parse_stmt_func(self) -> Func:
        name = self.expect_identifier()
        self.expect_operator("(")
        if self.expect
        self.expect_operator(")")
        body = self.read_block()

        return Func(name, args, body)
