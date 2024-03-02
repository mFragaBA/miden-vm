use core::ops::ControlFlow;

use crate::{ast::*, Felt, Span};

pub trait Visit<T = ()> {
    fn visit_module(&mut self, module: &Module) -> ControlFlow<T> {
        visit_module(self, module)
    }
    fn visit_import(&mut self, import: &Import) -> ControlFlow<T> {
        visit_import(self, import)
    }
    fn visit_export(&mut self, export: &Export) -> ControlFlow<T> {
        visit_export(self, export)
    }
    fn visit_procedure(&mut self, procedure: &Procedure) -> ControlFlow<T> {
        visit_procedure(self, procedure)
    }
    fn visit_procedure_alias(&mut self, alias: &ProcedureAlias) -> ControlFlow<T> {
        visit_procedure_alias(self, alias)
    }
    fn visit_block(&mut self, block: &Block) -> ControlFlow<T> {
        visit_block(self, block)
    }
    fn visit_op(&mut self, op: &Op) -> ControlFlow<T> {
        visit_op(self, op)
    }
    fn visit_inst(&mut self, inst: &Span<Instruction>) -> ControlFlow<T> {
        visit_inst(self, inst)
    }
    fn visit_advice_injector(&mut self, injector: Span<&AdviceInjectorNode>) -> ControlFlow<T> {
        visit_advice_injector(self, injector)
    }
    fn visit_debug_options(&mut self, options: Span<&DebugOptions>) -> ControlFlow<T> {
        visit_debug_options(self, options)
    }
    fn visit_exec(&mut self, target: &InvocationTarget) -> ControlFlow<T> {
        visit_exec(self, target)
    }
    fn visit_call(&mut self, target: &InvocationTarget) -> ControlFlow<T> {
        visit_call(self, target)
    }
    fn visit_syscall(&mut self, target: &InvocationTarget) -> ControlFlow<T> {
        visit_syscall(self, target)
    }
    fn visit_procref(&mut self, target: &InvocationTarget) -> ControlFlow<T> {
        visit_procref(self, target)
    }
    fn visit_invoke_target(&mut self, target: &InvocationTarget) -> ControlFlow<T> {
        visit_invoke_target(self, target)
    }
    fn visit_immediate_u8(&mut self, imm: &Immediate<u8>) -> ControlFlow<T> {
        visit_immediate_u8(self, imm)
    }
    fn visit_immediate_u16(&mut self, imm: &Immediate<u16>) -> ControlFlow<T> {
        visit_immediate_u16(self, imm)
    }
    fn visit_immediate_u32(&mut self, imm: &Immediate<u32>) -> ControlFlow<T> {
        visit_immediate_u32(self, imm)
    }
    fn visit_immediate_felt(&mut self, imm: &Immediate<Felt>) -> ControlFlow<T> {
        visit_immediate_felt(self, imm)
    }
    fn visit_immediate_error_code(&mut self, code: &Immediate<u32>) -> ControlFlow<T> {
        visit_immediate_error_code(self, code)
    }
}
impl<'a, V, T> Visit<T> for &'a mut V
where
    V: ?Sized + Visit<T>,
{
    fn visit_module(&mut self, module: &Module) -> ControlFlow<T> {
        (**self).visit_module(module)
    }
    fn visit_import(&mut self, import: &Import) -> ControlFlow<T> {
        (**self).visit_import(import)
    }
    fn visit_export(&mut self, export: &Export) -> ControlFlow<T> {
        (**self).visit_export(export)
    }
    fn visit_procedure(&mut self, procedure: &Procedure) -> ControlFlow<T> {
        (**self).visit_procedure(procedure)
    }
    fn visit_procedure_alias(&mut self, alias: &ProcedureAlias) -> ControlFlow<T> {
        (**self).visit_procedure_alias(alias)
    }
    fn visit_block(&mut self, block: &Block) -> ControlFlow<T> {
        (**self).visit_block(block)
    }
    fn visit_op(&mut self, op: &Op) -> ControlFlow<T> {
        (**self).visit_op(op)
    }
    fn visit_inst(&mut self, inst: &Span<Instruction>) -> ControlFlow<T> {
        (**self).visit_inst(inst)
    }
    fn visit_advice_injector(&mut self, injector: Span<&AdviceInjectorNode>) -> ControlFlow<T> {
        (**self).visit_advice_injector(injector)
    }
    fn visit_debug_options(&mut self, options: Span<&DebugOptions>) -> ControlFlow<T> {
        (**self).visit_debug_options(options)
    }
    fn visit_exec(&mut self, target: &InvocationTarget) -> ControlFlow<T> {
        (**self).visit_exec(target)
    }
    fn visit_call(&mut self, target: &InvocationTarget) -> ControlFlow<T> {
        (**self).visit_call(target)
    }
    fn visit_syscall(&mut self, target: &InvocationTarget) -> ControlFlow<T> {
        (**self).visit_syscall(target)
    }
    fn visit_procref(&mut self, target: &InvocationTarget) -> ControlFlow<T> {
        (**self).visit_procref(target)
    }
    fn visit_invoke_target(&mut self, target: &InvocationTarget) -> ControlFlow<T> {
        (**self).visit_invoke_target(target)
    }
    fn visit_immediate_u8(&mut self, imm: &Immediate<u8>) -> ControlFlow<T> {
        (**self).visit_immediate_u8(imm)
    }
    fn visit_immediate_u16(&mut self, imm: &Immediate<u16>) -> ControlFlow<T> {
        (**self).visit_immediate_u16(imm)
    }
    fn visit_immediate_u32(&mut self, imm: &Immediate<u32>) -> ControlFlow<T> {
        (**self).visit_immediate_u32(imm)
    }
    fn visit_immediate_felt(&mut self, imm: &Immediate<Felt>) -> ControlFlow<T> {
        (**self).visit_immediate_felt(imm)
    }
    fn visit_immediate_error_code(&mut self, code: &Immediate<u32>) -> ControlFlow<T> {
        (**self).visit_immediate_error_code(code)
    }
}

pub fn visit_module<V, T>(visitor: &mut V, module: &Module) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    for import in module.imports.iter() {
        visitor.visit_import(import)?;
    }
    for export in module.procedures.iter() {
        visitor.visit_export(export)?;
    }

    ControlFlow::Continue(())
}

#[inline(always)]
pub fn visit_import<V, T>(_visitor: &mut V, _import: &Import) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    ControlFlow::Continue(())
}

pub fn visit_export<V, T>(visitor: &mut V, export: &Export) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    match export {
        Export::Procedure(ref procedure) => visitor.visit_procedure(procedure),
        Export::Alias(ref alias) => visitor.visit_procedure_alias(alias),
    }
}

pub fn visit_procedure<V, T>(visitor: &mut V, procedure: &Procedure) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    visitor.visit_block(procedure.body())
}

#[inline(always)]
pub fn visit_procedure_alias<V, T>(_visitor: &mut V, _alias: &ProcedureAlias) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    ControlFlow::Continue(())
}

pub fn visit_block<V, T>(visitor: &mut V, block: &Block) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    for op in block.iter() {
        visitor.visit_op(op)?;
    }
    ControlFlow::Continue(())
}

pub fn visit_op<V, T>(visitor: &mut V, op: &Op) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    match op {
        Op::If {
            ref then_blk,
            ref else_blk,
            ..
        } => {
            visitor.visit_block(then_blk)?;
            visitor.visit_block(else_blk)
        }
        Op::While { ref body, .. } | Op::Repeat { ref body, .. } => visitor.visit_block(body),
        Op::Inst(ref inst) => visitor.visit_inst(inst),
    }
}

pub fn visit_inst<V, T>(visitor: &mut V, inst: &Span<Instruction>) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    use Instruction::*;
    let span = inst.span();
    match &**inst {
        U32ShrImm(ref imm) | U32ShlImm(ref imm) | U32RotrImm(ref imm) | U32RotlImm(ref imm)
        | AdvPush(ref imm) => visitor.visit_immediate_u8(imm),
        Locaddr(ref imm) | LocLoad(ref imm) | LocLoadW(ref imm) | LocStore(ref imm)
        | LocStoreW(ref imm) => visitor.visit_immediate_u16(imm),
        AssertWithError(ref code)
        | AssertEqWithError(ref code)
        | AssertEqwWithError(ref code)
        | AssertzWithError(ref code)
        | U32AssertWithError(ref code)
        | U32Assert2WithError(ref code)
        | U32AssertWWithError(ref code) => visitor.visit_immediate_error_code(code),
        AddImm(ref imm) | SubImm(ref imm) | MulImm(ref imm) | DivImm(ref imm) | ExpImm(ref imm)
        | EqImm(ref imm) | NeqImm(ref imm) | Push(ref imm) => visitor.visit_immediate_felt(imm),
        U32WrappingAddImm(ref imm)
        | U32OverflowingAddImm(ref imm)
        | U32WrappingSubImm(ref imm)
        | U32OverflowingSubImm(ref imm)
        | U32WrappingMulImm(ref imm)
        | U32OverflowingMulImm(ref imm)
        | U32DivImm(ref imm)
        | U32ModImm(ref imm)
        | U32DivModImm(ref imm)
        | MemLoadImm(ref imm)
        | MemLoadWImm(ref imm)
        | MemStoreImm(ref imm)
        | MemStoreWImm(ref imm)
        | Emit(ref imm)
        | Trace(ref imm) => visitor.visit_immediate_u32(imm),
        AdvInject(ref injector) => visitor.visit_advice_injector(Span::new(span, injector)),
        Exec(ref target) => visitor.visit_exec(target),
        Call(ref target) => visitor.visit_call(target),
        SysCall(ref target) => visitor.visit_syscall(target),
        ProcRef(ref target) => visitor.visit_procref(target),
        Debug(ref options) => visitor.visit_debug_options(Span::new(span, options)),
        Assert | AssertEq | AssertEqw | Assertz | Add | Sub | Mul | Div | Neg | ILog2 | Inv
        | Incr | Pow2 | Exp | ExpBitLength(_) | Not | And | Or | Xor | Eq | Neq | Eqw | Lt
        | Lte | Gt | Gte | IsOdd | Ext2Add | Ext2Sub | Ext2Mul | Ext2Div | Ext2Neg | Ext2Inv
        | U32Test | U32TestW | U32Assert | U32Assert2 | U32AssertW | U32Split | U32Cast
        | U32WrappingAdd | U32OverflowingAdd | U32OverflowingAdd3 | U32WrappingAdd3
        | U32WrappingSub | U32OverflowingSub | U32WrappingMul | U32OverflowingMul
        | U32OverflowingMadd | U32WrappingMadd | U32Div | U32Mod | U32DivMod | U32And | U32Or
        | U32Xor | U32Not | U32Shr | U32Shl | U32Rotr | U32Rotl | U32Popcnt | U32Clz | U32Ctz
        | U32Clo | U32Cto | U32Lt | U32Lte | U32Gt | U32Gte | U32Min | U32Max | Drop | DropW
        | PadW | Dup0 | Dup1 | Dup2 | Dup3 | Dup4 | Dup5 | Dup6 | Dup7 | Dup8 | Dup9 | Dup10
        | Dup11 | Dup12 | Dup13 | Dup14 | Dup15 | DupW0 | DupW1 | DupW2 | DupW3 | Swap1 | Swap2
        | Swap3 | Swap4 | Swap5 | Swap6 | Swap7 | Swap8 | Swap9 | Swap10 | Swap11 | Swap12
        | Swap13 | Swap14 | Swap15 | SwapW1 | SwapW2 | SwapW3 | SwapDw | MovUp2 | MovUp3
        | MovUp4 | MovUp5 | MovUp6 | MovUp7 | MovUp8 | MovUp9 | MovUp10 | MovUp11 | MovUp12
        | MovUp13 | MovUp14 | MovUp15 | MovUpW2 | MovUpW3 | MovDn2 | MovDn3 | MovDn4 | MovDn5
        | MovDn6 | MovDn7 | MovDn8 | MovDn9 | MovDn10 | MovDn11 | MovDn12 | MovDn13 | MovDn14
        | MovDn15 | MovDnW2 | MovDnW3 | CSwap | CSwapW | CDrop | CDropW | PushU8(_)
        | PushU16(_) | PushU32(_) | PushFelt(_) | PushWord(_) | PushU8List(_) | PushU16List(_)
        | PushU32List(_) | PushFeltList(_) | Sdepth | Caller | Clk | MemLoad | MemLoadW
        | MemStore | MemStoreW | MemStream | AdvPipe | AdvLoadW | Hash | HMerge | HPerm
        | MTreeGet | MTreeSet | MTreeMerge | MTreeVerify | FriExt2Fold4 | RCombBase | DynExec
        | DynCall | Breakpoint => ControlFlow::Continue(()),
    }
}

pub fn visit_advice_injector<V, T>(
    visitor: &mut V,
    node: Span<&AdviceInjectorNode>,
) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    match node.into_inner() {
        AdviceInjectorNode::PushMapValImm { offset: ref imm }
        | AdviceInjectorNode::PushMapValNImm { offset: ref imm }
        | AdviceInjectorNode::InsertHdwordImm { domain: ref imm } => {
            visitor.visit_immediate_u8(imm)
        }
        AdviceInjectorNode::PushU64Div
        | AdviceInjectorNode::PushExt2intt
        | AdviceInjectorNode::PushSmtGet
        | AdviceInjectorNode::PushSmtSet
        | AdviceInjectorNode::PushSmtPeek
        | AdviceInjectorNode::PushMapVal
        | AdviceInjectorNode::PushMapValN
        | AdviceInjectorNode::PushMtNode
        | AdviceInjectorNode::InsertMem
        | AdviceInjectorNode::InsertHdword
        | AdviceInjectorNode::InsertHperm
        | AdviceInjectorNode::PushSignature { .. } => ControlFlow::Continue(()),
    }
}

pub fn visit_debug_options<V, T>(visitor: &mut V, options: Span<&DebugOptions>) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    match options.into_inner() {
        DebugOptions::StackTop(ref imm) | DebugOptions::LocalRangeFrom(ref imm) => {
            visitor.visit_immediate_u16(imm)
        }
        DebugOptions::MemInterval(ref imm1, ref imm2) => {
            visitor.visit_immediate_u32(imm1)?;
            visitor.visit_immediate_u32(imm2)
        }
        DebugOptions::LocalInterval(ref imm1, ref imm2) => {
            visitor.visit_immediate_u16(imm1)?;
            visitor.visit_immediate_u16(imm2)
        }
        DebugOptions::StackAll | DebugOptions::MemAll | DebugOptions::LocalAll => {
            ControlFlow::Continue(())
        }
    }
}

#[inline]
pub fn visit_exec<V, T>(visitor: &mut V, target: &InvocationTarget) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    visitor.visit_invoke_target(target)
}

#[inline]
pub fn visit_call<V, T>(visitor: &mut V, target: &InvocationTarget) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    visitor.visit_invoke_target(target)
}

#[inline]
pub fn visit_syscall<V, T>(visitor: &mut V, target: &InvocationTarget) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    visitor.visit_invoke_target(target)
}

#[inline]
pub fn visit_procref<V, T>(visitor: &mut V, target: &InvocationTarget) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    visitor.visit_invoke_target(target)
}

#[inline(always)]
pub fn visit_invoke_target<V, T>(_visitor: &mut V, _target: &InvocationTarget) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    ControlFlow::Continue(())
}

#[inline(always)]
pub fn visit_immediate_u8<V, T>(_visitor: &mut V, _imm: &Immediate<u8>) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    ControlFlow::Continue(())
}

#[inline(always)]
pub fn visit_immediate_u16<V, T>(_visitor: &mut V, _imm: &Immediate<u16>) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    ControlFlow::Continue(())
}

#[inline(always)]
pub fn visit_immediate_u32<V, T>(_visitor: &mut V, _imm: &Immediate<u32>) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    ControlFlow::Continue(())
}

#[inline(always)]
pub fn visit_immediate_felt<V, T>(_visitor: &mut V, _imm: &Immediate<Felt>) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    ControlFlow::Continue(())
}

#[inline(always)]
pub fn visit_immediate_error_code<V, T>(_visitor: &mut V, _imm: &Immediate<u32>) -> ControlFlow<T>
where
    V: ?Sized + Visit<T>,
{
    ControlFlow::Continue(())
}

pub trait VisitMut<T = ()> {
    fn visit_mut_module(&mut self, module: &mut Module) -> ControlFlow<T> {
        visit_mut_module(self, module)
    }
    fn visit_mut_import(&mut self, import: &mut Import) -> ControlFlow<T> {
        visit_mut_import(self, import)
    }
    fn visit_mut_export(&mut self, export: &mut Export) -> ControlFlow<T> {
        visit_mut_export(self, export)
    }
    fn visit_mut_procedure(&mut self, procedure: &mut Procedure) -> ControlFlow<T> {
        visit_mut_procedure(self, procedure)
    }
    fn visit_mut_procedure_alias(&mut self, alias: &mut ProcedureAlias) -> ControlFlow<T> {
        visit_mut_procedure_alias(self, alias)
    }
    fn visit_mut_block(&mut self, block: &mut Block) -> ControlFlow<T> {
        visit_mut_block(self, block)
    }
    fn visit_mut_op(&mut self, op: &mut Op) -> ControlFlow<T> {
        visit_mut_op(self, op)
    }
    fn visit_mut_inst(&mut self, inst: &mut Span<Instruction>) -> ControlFlow<T> {
        visit_mut_inst(self, inst)
    }
    fn visit_mut_advice_injector(
        &mut self,
        injector: Span<&mut AdviceInjectorNode>,
    ) -> ControlFlow<T> {
        visit_mut_advice_injector(self, injector)
    }
    fn visit_mut_debug_options(&mut self, options: Span<&mut DebugOptions>) -> ControlFlow<T> {
        visit_mut_debug_options(self, options)
    }
    fn visit_mut_exec(&mut self, target: &mut InvocationTarget) -> ControlFlow<T> {
        visit_mut_exec(self, target)
    }
    fn visit_mut_call(&mut self, target: &mut InvocationTarget) -> ControlFlow<T> {
        visit_mut_call(self, target)
    }
    fn visit_mut_syscall(&mut self, target: &mut InvocationTarget) -> ControlFlow<T> {
        visit_mut_syscall(self, target)
    }
    fn visit_mut_procref(&mut self, target: &mut InvocationTarget) -> ControlFlow<T> {
        visit_mut_procref(self, target)
    }
    fn visit_mut_invoke_target(&mut self, target: &mut InvocationTarget) -> ControlFlow<T> {
        visit_mut_invoke_target(self, target)
    }
    fn visit_mut_immediate_u8(&mut self, imm: &mut Immediate<u8>) -> ControlFlow<T> {
        visit_mut_immediate_u8(self, imm)
    }
    fn visit_mut_immediate_u16(&mut self, imm: &mut Immediate<u16>) -> ControlFlow<T> {
        visit_mut_immediate_u16(self, imm)
    }
    fn visit_mut_immediate_u32(&mut self, imm: &mut Immediate<u32>) -> ControlFlow<T> {
        visit_mut_immediate_u32(self, imm)
    }
    fn visit_mut_immediate_felt(&mut self, imm: &mut Immediate<Felt>) -> ControlFlow<T> {
        visit_mut_immediate_felt(self, imm)
    }
    fn visit_mut_immediate_error_code(&mut self, code: &mut Immediate<u32>) -> ControlFlow<T> {
        visit_mut_immediate_error_code(self, code)
    }
}
impl<'a, V, T> VisitMut<T> for &'a mut V
where
    V: ?Sized + VisitMut<T>,
{
    fn visit_mut_module(&mut self, module: &mut Module) -> ControlFlow<T> {
        (**self).visit_mut_module(module)
    }
    fn visit_mut_import(&mut self, import: &mut Import) -> ControlFlow<T> {
        (**self).visit_mut_import(import)
    }
    fn visit_mut_export(&mut self, export: &mut Export) -> ControlFlow<T> {
        (**self).visit_mut_export(export)
    }
    fn visit_mut_procedure(&mut self, procedure: &mut Procedure) -> ControlFlow<T> {
        (**self).visit_mut_procedure(procedure)
    }
    fn visit_mut_procedure_alias(&mut self, alias: &mut ProcedureAlias) -> ControlFlow<T> {
        (**self).visit_mut_procedure_alias(alias)
    }
    fn visit_mut_block(&mut self, block: &mut Block) -> ControlFlow<T> {
        (**self).visit_mut_block(block)
    }
    fn visit_mut_op(&mut self, op: &mut Op) -> ControlFlow<T> {
        (**self).visit_mut_op(op)
    }
    fn visit_mut_inst(&mut self, inst: &mut Span<Instruction>) -> ControlFlow<T> {
        (**self).visit_mut_inst(inst)
    }
    fn visit_mut_advice_injector(
        &mut self,
        injector: Span<&mut AdviceInjectorNode>,
    ) -> ControlFlow<T> {
        (**self).visit_mut_advice_injector(injector)
    }
    fn visit_mut_debug_options(&mut self, options: Span<&mut DebugOptions>) -> ControlFlow<T> {
        (**self).visit_mut_debug_options(options)
    }
    fn visit_mut_exec(&mut self, target: &mut InvocationTarget) -> ControlFlow<T> {
        (**self).visit_mut_exec(target)
    }
    fn visit_mut_call(&mut self, target: &mut InvocationTarget) -> ControlFlow<T> {
        (**self).visit_mut_call(target)
    }
    fn visit_mut_syscall(&mut self, target: &mut InvocationTarget) -> ControlFlow<T> {
        (**self).visit_mut_syscall(target)
    }
    fn visit_mut_procref(&mut self, target: &mut InvocationTarget) -> ControlFlow<T> {
        (**self).visit_mut_procref(target)
    }
    fn visit_mut_invoke_target(&mut self, target: &mut InvocationTarget) -> ControlFlow<T> {
        (**self).visit_mut_invoke_target(target)
    }
    fn visit_mut_immediate_u8(&mut self, imm: &mut Immediate<u8>) -> ControlFlow<T> {
        (**self).visit_mut_immediate_u8(imm)
    }
    fn visit_mut_immediate_u16(&mut self, imm: &mut Immediate<u16>) -> ControlFlow<T> {
        (**self).visit_mut_immediate_u16(imm)
    }
    fn visit_mut_immediate_u32(&mut self, imm: &mut Immediate<u32>) -> ControlFlow<T> {
        (**self).visit_mut_immediate_u32(imm)
    }
    fn visit_mut_immediate_felt(&mut self, imm: &mut Immediate<Felt>) -> ControlFlow<T> {
        (**self).visit_mut_immediate_felt(imm)
    }
    fn visit_mut_immediate_error_code(&mut self, code: &mut Immediate<u32>) -> ControlFlow<T> {
        (**self).visit_mut_immediate_error_code(code)
    }
}

pub fn visit_mut_module<V, T>(visitor: &mut V, module: &mut Module) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    for import in module.imports.iter_mut() {
        visitor.visit_mut_import(import)?;
    }
    for export in module.procedures.iter_mut() {
        visitor.visit_mut_export(export)?;
    }

    ControlFlow::Continue(())
}

#[inline(always)]
pub fn visit_mut_import<V, T>(_visitor: &mut V, _import: &mut Import) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    ControlFlow::Continue(())
}

pub fn visit_mut_export<V, T>(visitor: &mut V, export: &mut Export) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    match export {
        Export::Procedure(ref mut procedure) => visitor.visit_mut_procedure(procedure),
        Export::Alias(ref mut alias) => visitor.visit_mut_procedure_alias(alias),
    }
}

pub fn visit_mut_procedure<V, T>(visitor: &mut V, procedure: &mut Procedure) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    visitor.visit_mut_block(procedure.body_mut())
}

#[inline(always)]
pub fn visit_mut_procedure_alias<V, T>(
    _visitor: &mut V,
    _alias: &mut ProcedureAlias,
) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    ControlFlow::Continue(())
}

pub fn visit_mut_block<V, T>(visitor: &mut V, block: &mut Block) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    for op in block.iter_mut() {
        visitor.visit_mut_op(op)?;
    }
    ControlFlow::Continue(())
}

pub fn visit_mut_op<V, T>(visitor: &mut V, op: &mut Op) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    match op {
        Op::If {
            ref mut then_blk,
            ref mut else_blk,
            ..
        } => {
            visitor.visit_mut_block(then_blk)?;
            visitor.visit_mut_block(else_blk)
        }
        Op::While { ref mut body, .. } | Op::Repeat { ref mut body, .. } => {
            visitor.visit_mut_block(body)
        }
        Op::Inst(ref mut inst) => visitor.visit_mut_inst(inst),
    }
}

pub fn visit_mut_inst<V, T>(visitor: &mut V, inst: &mut Span<Instruction>) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    use Instruction::*;
    let span = inst.span();
    match &mut **inst {
        U32ShrImm(ref mut imm)
        | U32ShlImm(ref mut imm)
        | U32RotrImm(ref mut imm)
        | U32RotlImm(ref mut imm)
        | AdvPush(ref mut imm) => visitor.visit_mut_immediate_u8(imm),
        Locaddr(ref mut imm)
        | LocLoad(ref mut imm)
        | LocLoadW(ref mut imm)
        | LocStore(ref mut imm)
        | LocStoreW(ref mut imm) => visitor.visit_mut_immediate_u16(imm),
        AssertWithError(ref mut code)
        | AssertEqWithError(ref mut code)
        | AssertEqwWithError(ref mut code)
        | AssertzWithError(ref mut code)
        | U32AssertWithError(ref mut code)
        | U32Assert2WithError(ref mut code)
        | U32AssertWWithError(ref mut code) => visitor.visit_mut_immediate_error_code(code),
        AddImm(ref mut imm) | SubImm(ref mut imm) | MulImm(ref mut imm) | DivImm(ref mut imm)
        | ExpImm(ref mut imm) | EqImm(ref mut imm) | NeqImm(ref mut imm) | Push(ref mut imm) => {
            visitor.visit_mut_immediate_felt(imm)
        }
        U32WrappingAddImm(ref mut imm)
        | U32OverflowingAddImm(ref mut imm)
        | U32WrappingSubImm(ref mut imm)
        | U32OverflowingSubImm(ref mut imm)
        | U32WrappingMulImm(ref mut imm)
        | U32OverflowingMulImm(ref mut imm)
        | U32DivImm(ref mut imm)
        | U32ModImm(ref mut imm)
        | U32DivModImm(ref mut imm)
        | MemLoadImm(ref mut imm)
        | MemLoadWImm(ref mut imm)
        | MemStoreImm(ref mut imm)
        | MemStoreWImm(ref mut imm)
        | Emit(ref mut imm)
        | Trace(ref mut imm) => visitor.visit_mut_immediate_u32(imm),
        AdvInject(ref mut injector) => visitor.visit_mut_advice_injector(Span::new(span, injector)),
        Exec(ref mut target) => visitor.visit_mut_exec(target),
        Call(ref mut target) => visitor.visit_mut_call(target),
        SysCall(ref mut target) => visitor.visit_mut_syscall(target),
        ProcRef(ref mut target) => visitor.visit_mut_procref(target),
        Debug(ref mut options) => visitor.visit_mut_debug_options(Span::new(span, options)),
        Assert | AssertEq | AssertEqw | Assertz | Add | Sub | Mul | Div | Neg | ILog2 | Inv
        | Incr | Pow2 | Exp | ExpBitLength(_) | Not | And | Or | Xor | Eq | Neq | Eqw | Lt
        | Lte | Gt | Gte | IsOdd | Ext2Add | Ext2Sub | Ext2Mul | Ext2Div | Ext2Neg | Ext2Inv
        | U32Test | U32TestW | U32Assert | U32Assert2 | U32AssertW | U32Split | U32Cast
        | U32WrappingAdd | U32OverflowingAdd | U32OverflowingAdd3 | U32WrappingAdd3
        | U32WrappingSub | U32OverflowingSub | U32WrappingMul | U32OverflowingMul
        | U32OverflowingMadd | U32WrappingMadd | U32Div | U32Mod | U32DivMod | U32And | U32Or
        | U32Xor | U32Not | U32Shr | U32Shl | U32Rotr | U32Rotl | U32Popcnt | U32Clz | U32Ctz
        | U32Clo | U32Cto | U32Lt | U32Lte | U32Gt | U32Gte | U32Min | U32Max | Drop | DropW
        | PadW | Dup0 | Dup1 | Dup2 | Dup3 | Dup4 | Dup5 | Dup6 | Dup7 | Dup8 | Dup9 | Dup10
        | Dup11 | Dup12 | Dup13 | Dup14 | Dup15 | DupW0 | DupW1 | DupW2 | DupW3 | Swap1 | Swap2
        | Swap3 | Swap4 | Swap5 | Swap6 | Swap7 | Swap8 | Swap9 | Swap10 | Swap11 | Swap12
        | Swap13 | Swap14 | Swap15 | SwapW1 | SwapW2 | SwapW3 | SwapDw | MovUp2 | MovUp3
        | MovUp4 | MovUp5 | MovUp6 | MovUp7 | MovUp8 | MovUp9 | MovUp10 | MovUp11 | MovUp12
        | MovUp13 | MovUp14 | MovUp15 | MovUpW2 | MovUpW3 | MovDn2 | MovDn3 | MovDn4 | MovDn5
        | MovDn6 | MovDn7 | MovDn8 | MovDn9 | MovDn10 | MovDn11 | MovDn12 | MovDn13 | MovDn14
        | MovDn15 | MovDnW2 | MovDnW3 | CSwap | CSwapW | CDrop | CDropW | PushU8(_)
        | PushU16(_) | PushU32(_) | PushFelt(_) | PushWord(_) | PushU8List(_) | PushU16List(_)
        | PushU32List(_) | PushFeltList(_) | Sdepth | Caller | Clk | MemLoad | MemLoadW
        | MemStore | MemStoreW | MemStream | AdvPipe | AdvLoadW | Hash | HMerge | HPerm
        | MTreeGet | MTreeSet | MTreeMerge | MTreeVerify | FriExt2Fold4 | RCombBase | DynExec
        | DynCall | Breakpoint => ControlFlow::Continue(()),
    }
}

pub fn visit_mut_advice_injector<V, T>(
    visitor: &mut V,
    node: Span<&mut AdviceInjectorNode>,
) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    match node.into_inner() {
        AdviceInjectorNode::PushMapValImm {
            offset: ref mut imm,
        }
        | AdviceInjectorNode::PushMapValNImm {
            offset: ref mut imm,
        }
        | AdviceInjectorNode::InsertHdwordImm {
            domain: ref mut imm,
        } => visitor.visit_mut_immediate_u8(imm),
        AdviceInjectorNode::PushU64Div
        | AdviceInjectorNode::PushExt2intt
        | AdviceInjectorNode::PushSmtGet
        | AdviceInjectorNode::PushSmtSet
        | AdviceInjectorNode::PushSmtPeek
        | AdviceInjectorNode::PushMapVal
        | AdviceInjectorNode::PushMapValN
        | AdviceInjectorNode::PushMtNode
        | AdviceInjectorNode::InsertMem
        | AdviceInjectorNode::InsertHdword
        | AdviceInjectorNode::InsertHperm
        | AdviceInjectorNode::PushSignature { .. } => ControlFlow::Continue(()),
    }
}

pub fn visit_mut_debug_options<V, T>(
    visitor: &mut V,
    options: Span<&mut DebugOptions>,
) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    match options.into_inner() {
        DebugOptions::StackTop(ref mut imm) | DebugOptions::LocalRangeFrom(ref mut imm) => {
            visitor.visit_mut_immediate_u16(imm)
        }
        DebugOptions::MemInterval(ref mut imm1, ref mut imm2) => {
            visitor.visit_mut_immediate_u32(imm1)?;
            visitor.visit_mut_immediate_u32(imm2)
        }
        DebugOptions::LocalInterval(ref mut imm1, ref mut imm2) => {
            visitor.visit_mut_immediate_u16(imm1)?;
            visitor.visit_mut_immediate_u16(imm2)
        }
        DebugOptions::StackAll | DebugOptions::MemAll | DebugOptions::LocalAll => {
            ControlFlow::Continue(())
        }
    }
}

#[inline]
pub fn visit_mut_exec<V, T>(visitor: &mut V, target: &mut InvocationTarget) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    visitor.visit_mut_invoke_target(target)
}

#[inline]
pub fn visit_mut_call<V, T>(visitor: &mut V, target: &mut InvocationTarget) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    visitor.visit_mut_invoke_target(target)
}

#[inline]
pub fn visit_mut_syscall<V, T>(visitor: &mut V, target: &mut InvocationTarget) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    visitor.visit_mut_invoke_target(target)
}

#[inline]
pub fn visit_mut_procref<V, T>(visitor: &mut V, target: &mut InvocationTarget) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    visitor.visit_mut_invoke_target(target)
}

#[inline(always)]
pub fn visit_mut_invoke_target<V, T>(
    _visitor: &mut V,
    _target: &mut InvocationTarget,
) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    ControlFlow::Continue(())
}

#[inline(always)]
pub fn visit_mut_immediate_u8<V, T>(_visitor: &mut V, _imm: &mut Immediate<u8>) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    ControlFlow::Continue(())
}

#[inline(always)]
pub fn visit_mut_immediate_u16<V, T>(_visitor: &mut V, _imm: &mut Immediate<u16>) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    ControlFlow::Continue(())
}

#[inline(always)]
pub fn visit_mut_immediate_u32<V, T>(_visitor: &mut V, _imm: &mut Immediate<u32>) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    ControlFlow::Continue(())
}

#[inline(always)]
pub fn visit_mut_immediate_felt<V, T>(
    _visitor: &mut V,
    _imm: &mut Immediate<Felt>,
) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    ControlFlow::Continue(())
}

#[inline(always)]
pub fn visit_mut_immediate_error_code<V, T>(
    _visitor: &mut V,
    _imm: &mut Immediate<u32>,
) -> ControlFlow<T>
where
    V: ?Sized + VisitMut<T>,
{
    ControlFlow::Continue(())
}
