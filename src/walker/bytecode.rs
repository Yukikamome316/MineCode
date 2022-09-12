use crate::ast::{BinaryOp, Expr};

use super::core_trait::Walker;

pub struct ByteCodeWalker {
    stmts: Vec<Vec<u8>>,
}

impl ByteCodeWalker {
    // pub fn new() -> ByteCodeWalker {
    //     ByteCodeWalker { stmts: Vec::new() }
    // }
}

impl Walker for ByteCodeWalker {
    type StmtT = Vec<u8>;
    type ExprT = Vec<u8>;

    fn add_stmt(&mut self, stmt: Self::StmtT) {
        self.stmts.push(stmt);
    }
    fn get_stmts(&self) -> &Vec<Self::StmtT> {
        &self.stmts
    }

    fn walk_load_module(&mut self, module_name: String) {
        let mut ret = vec![0u8];
        ret.extend(module_name.as_bytes().to_vec());
        self.add_stmt(ret)
    }
    fn walk_stmt_expr(&mut self, expr: &Expr) {
        let expr = self.walk_expr(expr);
        self.add_stmt(expr)
    }
    fn walk_func_def(&mut self, _name: String, _args: Vec<String>, _body: &Expr) {
        unimplemented!()
    }

    fn walk_num(&mut self, num: i32) -> Vec<u8> {
        let mut ret = vec![];
        if num < 0x100 {
            ret.push(0x90u8);
            ret.push(num as u8);
        } else if num < 0x10000 {
            ret.push(0x91u8);
            ret.push((num >> 8) as u8);
            ret.push(num as u8);
        } else {
            //if num < 0x100000000
            ret.push(0x92u8);
            ret.push((num >> 24) as u8);
            ret.push((num >> 16) as u8);
            ret.push((num >> 8) as u8);
            ret.push(num as u8);
        }
        /*  else  if num < 0x10000000000000000{
            ret.push(0x93u8);
            ret.push((num >> 56) as u8);
            ret.push((num >> 48) as u8);
            ret.push((num >> 40) as u8);
            ret.push((num >> 32) as u8);
            ret.push((num >> 24) as u8);
            ret.push((num >> 16) as u8);
            ret.push((num >> 8) as u8);
            ret.push(num as u8);

        } */
        ret
    }
    fn walk_ident(&mut self, _ident: String) -> Vec<u8> {
        panic!("walk_ident is not allowed in bytecode")
    }
    fn walk_storage(&mut self, index: usize) -> Self::ExprT {
        let mut ret = vec![0xfdu8];
        ret.push(index.try_into().unwrap());
        ret.push(0u8);
        ret
    }
    fn walk_string(&mut self, string: String) -> Vec<u8> {
        let mut ret = vec![0x94u8];
        ret.extend(string.as_bytes().to_vec());
        ret.push(0u8);
        ret
    }
    fn walk_func_call(&mut self, _func_name: &Expr, _args: &[Expr]) -> Vec<u8> {
        panic!("walk_func_call is not allowed in bytecode")
    }
    fn walk_direct_func_call(&mut self, addr: u64, args: &[Expr]) -> Vec<u8> {
        let mut ret = vec![0xfcu8];
        ret.push((addr >> 0x38) as u8);
        ret.push((addr >> 0x30) as u8);
        ret.push((addr >> 0x28) as u8);
        ret.push((addr >> 0x20) as u8);
        ret.push((addr >> 0x18) as u8);
        ret.push((addr >> 0x10) as u8);
        ret.push((addr >> 0x08) as u8);
        ret.push(addr as u8);
        ret.extend(self.walk_exprs(args));
        ret
    }
    fn walk_ranged(&mut self, _start: &Expr, _end: &Expr) -> Vec<u8> {
        unimplemented!()
    }
    fn walk_reference(&mut self, expr: &Expr) -> Self::ExprT {
        unimplemented!()
    }
    fn walk_dereference(&mut self, expr: &Expr) -> Vec<u8> {
        let mut ret = vec![0x9du8];
        ret.extend(self.walk_expr(expr));
        ret
    }
    fn walk_compile_time(&mut self, _expr: &Expr) -> Vec<u8> {
        unimplemented!()
    }
    fn walk_apply_operator(&mut self, op: BinaryOp, left: &Expr, right: &Expr) -> Vec<u8> {
        let mut ret = vec![op.into()];
        ret.extend(self.walk_expr(left));
        ret.extend(self.walk_expr(right));
        ret
    }
    fn walk_logical_not(&mut self, _expr: &Expr) -> Vec<u8> {
        unimplemented!()
    }
    fn walk_bitwise_not(&mut self, _expr: &Expr) -> Vec<u8> {
        unimplemented!()
    }
    fn walk_negative(&mut self, _expr: &Expr) -> Vec<u8> {
        unimplemented!()
    }
    fn walk_subscript(&mut self, expr: &Expr, index: &Expr) -> Vec<u8> {
        let mut ret = vec![0x9bu8];
        ret.extend(self.walk_expr(expr));
        ret.extend(self.walk_expr(index));
        ret
    }
    fn walk_attribute(&mut self, _expr: &Expr, _attr: String) -> Vec<u8> {
        unimplemented!()
    }
    fn walk_if(&mut self, branches: &[(Expr, Expr)], fallback: &Expr) -> Vec<u8> {
        // TODO: Optimize for the case where there is only one branch.
        // TODO: Optimize for the following case: if a == b => c
        let mut ret = vec![0xf5u8];
        for (cond, body) in branches {
            ret.extend(self.walk_expr(cond));
            ret.extend(self.walk_expr(body));
        }
        ret.extend(self.walk_expr(fallback));

        ret
    }
    fn walk_for(&mut self, _name: String, _iter: &Expr, _body: &Expr, _value: &Expr) -> Vec<u8> {
        unimplemented!()
    }
    fn walk_exprs(&mut self, exprs: &[Expr]) -> Vec<u8> {
        let mut ret = vec![0xf6u8];
        for expr in exprs {
            ret.extend(self.walk_expr(expr));
        }
        ret.push(0xff);
        ret
    }

    fn walk_nil(&mut self) -> Self::ExprT {
        vec![0xf7u8]
    }
    fn walk_any_type(&mut self) -> Self::ExprT {
        unimplemented!()
    }

    fn walk_as(&mut self, v: &Expr, t: &Expr) -> Self::ExprT {
        unimplemented!()
    }

    fn walk_type_holder(&mut self, n: usize) -> Self::ExprT {
        unimplemented!()
    }

    fn walk_assignment(&mut self, a: &Expr, b: &Expr) -> Self::ExprT {
        let mut ret = vec![0xf4u8];
        ret.extend(self.walk_expr(a));
        ret.extend(self.walk_expr(b));
        return ret;
    }
}
