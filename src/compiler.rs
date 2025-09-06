use crate::parser::{BinOpValue, Expr, Program, Statement, UnOpValue, VarOp, VariableDeclaration};
use std::collections::HashMap;

use inkwell::{
    builder::Builder, context::Context, module::{Linkage, Module}, types::BasicMetadataTypeEnum, values::{FloatValue, FunctionValue, PointerValue}, AddressSpace
};

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,

    variables: HashMap<char, PointerValue<'ctx>>,
    fn_value_opt: Option<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    fn compile_program(&mut self) -> Result<FunctionValue<'ctx>, &'static str> {
        todo!()
    }
}

fn compile<'ctx>(program: Program) -> FunctionValue<'ctx> {
    let context = Context::create();
    let builder = context.create_builder();
    let module = context.create_module("main");

    let print = module.add_function(
        "printf",
        context.i32_type().fn_type(&[context.ptr_type(AddressSpace::default()).into()], true),
        //context.i32_type().fn_type(&[context.i8_type().into()], true),
        Some(Linkage::External),
    );

    let fn_type = context.i64_type().fn_type(&[], false);
    let fn_val = module.add_function("main", fn_type, None);

    let entry = context.append_basic_block(fn_val, "entry");


    for statement in program.into_iter() {
        match statement {
            Statement::PrintExpr(print_expr) => {
                todo!();
            },
            Statement::VariableDeclaration(var_decl) => {
                build_var_declaration(&context, &builder, &module, var_decl);
            },
        }
    }

    builder.position_at_end(entry);
    let ret = builder.build_alloca(context.i64_type(), "return").unwrap();
    builder.build_return(Some(&ret)).unwrap();

    todo!()
}

fn build_var_declaration<'ctx>(context: &'ctx Context, builder: &'ctx Builder<'_>, module: &'ctx Module<'_>, var_decl: VariableDeclaration) -> PointerValue<'ctx> {
    let ident = &var_decl.ident.to_string()[..];
    let alloca = builder.build_alloca(context.f64_type(), ident).unwrap();
    builder.build_store(alloca, compile_expr(context, builder, module, var_decl.expr)).unwrap();
    alloca
}

fn compile_expr<'ctx>(context: &'ctx Context, builder: &'ctx Builder, module: &Module, expr: Expr) -> FloatValue<'ctx> {
    match expr {
        Expr::Value(value) => {
            todo!();
        },
        Expr::Number(num) => {
            context.f64_type().const_float(num)
        },
        Expr::Ident(chr) => {
            // Need a hashmap with my variables with a pointer to each to pass here
            let _ = builder.build_load(context.f64_type(), todo!(), chr);
            todo!()
        },
    }
}
