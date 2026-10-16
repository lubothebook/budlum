# P35 scan: BudL compiler (read-only audit, Tier R2)

Base commit: 9d46064. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md (no bud-compiler entry found by rg), .github baselines (only two bud-compiler lines, see Known below).

## COVERAGE

Read in full (every line, tests included):
- budzero/bud-compiler/Cargo.toml 1-20
- budzero/bud-compiler/src/ast.rs 1-109
- budzero/bud-compiler/src/lexer.rs 1-99
- budzero/bud-compiler/src/parser.rs 1-808
- budzero/bud-compiler/src/sema.rs 1-788
- budzero/bud-compiler/src/codegen.rs 1-996
- budzero/bud-compiler/src/lib.rs 1-1757 (two reads, 1-700 and 700-1757)
- budzero/bud-compiler/src/ir/mod.rs 1-397
- budzero/bud-compiler/src/ir/lower.rs 1-1092
- budzero/bud-compiler/src/ir/verify.rs 1-767
- budzero/bud-compiler/src/ir/display.rs 1-272

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md lines 110-165 (sections 5 and 6) plus the Z3/Z7/Z8 lines of CLAUDE.md (rg); notes/P29-scan.md lines 1-30.

Outside the part (rg first, then at most 80 lines): budzero/bud-vm/src/lib.rs 280-330, 345-600 (Opcode semantics for Load, Store, Jnz, Call, Ret, Push, Pop, SRead, SWrite, Syscall, memory_word_addr 993-1002); budzero/bud-isa/src/lib.rs 55-80, 160-170, 255-300 (is_experimental, requires_mainnet_activation, decode paths); budzero/bud-cli/src/main.rs 765-800 (compile call site); fuzz/fuzz_targets/budl_compile.rs 1-60; budzero/Cargo.toml 1-40; .github/*baseline* (rg on bud-compiler), .github/workflows (rg on experimental, bud-compiler).

Not read: the logos crate source (the lexer behaviour on non-ASCII error spans and on the block comment regex is argued from the regex, not from running it), bud-vm field helpers, the AIR side of any opcode, bud-cli beyond the call site, docs/BudL_SPEC.md (only used through the include_str test). No cargo, no test run: every claim comes from reading source. Findings marked Likely name the missing step.

Extra-check summary:
- Miscompilation vs VM semantics: four silent miscompiles found in codegen.rs (P35-F01, F02, F03, F05) and one scoping mismatch (F06). All are in code that sema accepts as valid. The existing tests avoid every trigger shape (match on a variable only, nested struct built through a separate let, literal patterns 0 and 1).
- Overflow and width: literals are bound-checked against P in Expr::Int (codegen.rs:596-606) and the base 2^30 decomposition is correct (checked by hand for the three chunks). Match patterns bypass that path (F05). Mapping slots are cut to 32 bits inside the VM (F04). u64 is a Goldilocks element, wraps silently (F18).
- Determinism: compiled bytecode does not depend on HashMap order (struct_layouts, struct_field_types, function_returns are only looked up by key; func_offsets likewise). Only the diagnostic text can differ between runs (F24).
- Panics on hostile source: no unwrap, expect, direct indexing from source or `panic!` in non-test code. The three `unreachable!` in parser.rs (528, 544, 576) and the one in ir/lower.rs:836 are guarded by the loop conditions. `args[0]`, `args[1]` in codegen.rs:856-858 and 884-885 rely on the sema arity check, which always runs in compile(). The real abort path is stack exhaustion on left-deep expressions (F08), which the nesting guard does not cover.
- Unwired parts: the whole ir/ module (2528 lines) has no caller outside its own tests (F14). `is_pub`, Token::Witness, Contract.name, Parser._source are never read (F20).
- Z3: one `#[allow(dead_code)]` at codegen.rs:25, no `#[ignore]`, no continue-on-error in this crate. Six tests are gated by `#[cfg(feature = "experimental")]` and no workflow enables that feature (F12). The git history of the allow is not recoverable (a single import commit e7e3a45 touches the crate).
- Z7: `#![forbid(unsafe_code)]` at lib.rs:3, rg finds no `unsafe`. Z8: no `f32`/`f64` in the crate.

Known (not repeated as findings): .github/idle-code-baseline.txt:762 (parser.rs const MAX_NESTING_DEPTH) and :763 (sema.rs fn is_opaque_bytes32).

## FINDINGS

### P35-F01
File: budzero/bud-compiler/src/codegen.rs:367-398 (cause at 368 and 390-393)
Severity: High
Axis: correctness
Confidence: Certain
Evidence:
```
367:  let scrutinee_reg = self.generate_expr(scrutinee, scope, storage);
368:  self.next_reg = saved_reg;
...
390:  let pat_reg = self.alloc_reg();
391:  self.emit(Opcode::Load, pat_reg, 0, 0, *val as i32);
392:  let diff_reg = self.alloc_reg();
393:  self.emit(Opcode::Sub, diff_reg, scrutinee_reg, pat_reg, 0);
```
Description: When the scrutinee is not a plain identifier, generate_expr returns a temporary register at or above saved_reg. Line 368 then resets next_reg to saved_reg, so the first alloc_reg at 390 returns that same register and the Load at 391 overwrites the scrutinee. diff becomes pat - pat = 0, the Jnz at 395 never skips, and the first integer arm always runs. Example: `match (storage::state) { 0 => {...}, 1 => {...}, _ => {...} };` StorageRead allocates reg saved_reg, the pattern Load reuses it. Same for `match (a + b)`, `match (f())`, `match (s.x)`, `match (m[k])`.
Impact: Any contract that dispatches on a storage value, call result or arithmetic expression executes arm 1 regardless of the value. Access or state-machine logic is bypassed with no compile or run-time error. All tests (lib.rs:762-811) use `match (x)` with a let-bound identifier, whose register sits below saved_reg, so none sees it.
Trigger: compile any contract with a non-identifier scrutinee and an integer first arm, run in bud-vm. Expected wrong result: arm 1 body runs for every input.
Known: none

### P35-F02
File: budzero/bud-compiler/src/codegen.rs:215-221, 271-287, 437-441, 647-649
Severity: High
Axis: correctness
Confidence: Certain
Evidence:
```
215:  let struct_type = self.expr_struct_type(expr, scope);
216:  let reg = self.generate_expr(expr, scope, storage);
217:  scope.insert(name.clone(), VarInfo { reg, struct_type });
...
285:  self.emit(Opcode::Add, target_reg, reg, 0, 0);
...
647:  Expr::Ident(name) => match scope.get(name) { Some(vi) => vi.reg,
```
Description: `let b = a;` binds b to the register of a (Ident returns vi.reg without a copy). A later `b = b + 1;` emits `Add reg_a, tmp, r0` and so changes a. The same alias applies to the loop bound: `for i in 0..n` takes end_reg = n's register (line 438), so `n = n - 1;` in the body moves the bound while the loop runs; IR lowering (lower.rs:344-359) snapshots the bound, so the two backends already disagree.
Impact: Silent wrong values in valid, type-correct programs, e.g. `fn f(a: u64) -> u64 { let b = a; a = a + 1; return b; }` returns a + 1. Hand trace: a in r1, b aliased to r1, Binary result in r2, Assign writes r1.
Trigger: compile the example, call f(5), expect 5, get 6.
Known: none

### P35-F03
File: budzero/bud-compiler/src/codegen.rs:702-704, 718-744, 751-754
Severity: High
Axis: correctness
Confidence: Certain
Evidence:
```
703:  let ptr_reg = self.alloc_reg();
704:  self.emit(Opcode::Add, ptr_reg, 31, 0, 0); // copy heap ptr to ptr_reg
...
719:  let val_reg = self.generate_expr(val, scope, storage);
...
743:  self.emit(Opcode::Store, 0, ptr_reg, val_reg, offset as i32);
...
754:  self.emit(Opcode::Add, 31, 31, size_reg, 0); // bump heap pointer
```
Description: The outer literal copies r31 before it evaluates its field values and bumps r31 only after the stores. If a field value itself allocates (a nested struct literal, or a call to a function that builds a struct), the inner object is placed at the same address as the outer one. The outer Store of field k then overwrites the inner field k (outer field 0 stores the inner pointer into inner field 0).
Impact: `Outer { inner: Inner { pad: 1, y: 42 } }` yields an inner struct whose first word is its own address. The only nested test (lib.rs:228-252) builds Inner in a separate let first, which avoids the overlap. Wrong data in any contract that nests a literal.
Trigger: compile `let o = Outer { inner: Inner { pad: 1, y: 42 } }; emit E(o.inner.pad);` Expected 1, observed the heap address of the object.
Known: none

### P35-F04
File: budzero/bud-compiler/src/codegen.rs:263-268, 691-699; budzero/bud-vm/src/lib.rs:498-516 (VM side, cross-part)
Severity: High
Axis: correctness
Confidence: Likely (missing step: run two colliding keys through bud-vm; the cut is read directly from source)
Evidence:
```
codegen.rs:266  self.emit(Opcode::Poseidon, target_slot_reg, base_reg, key_reg, 0);
codegen.rs:268  self.emit(Opcode::SWrite, 0, val_reg, target_slot_reg, -1);
bud-vm lib.rs:499  let slot = if inst.imm == -1 { src2_val as i32 } else { inst.imm };
bud-vm lib.rs:510  (SWrite, same expression)
```
Description: A mapping entry lives at the Poseidon hash of (map slot index, key), but the VM keeps storage in a map keyed by i32 and truncates the 64-bit hash to its low 32 bits. Two distinct keys collide after about 2^16 trials, and a key can be ground (about 2^32 hashes) so that `balances[k]` equals a small plain slot such as `storage::counter` (slot 0), or another user's entry.
Impact: A user who controls a mapping key (an address-like u64) can alias another entry or a scalar storage field and read or overwrite it. Plain slots are `index as i32` (codegen.rs:179, 231), the same space. Needs a contract with a Map and user-chosen keys, which is the documented use (lib.rs:164-181).
Trigger: precondition: contract with `Map<u64,u64>` keyed by caller-supplied value. Entry point: codegen.rs:259-268 and 676-699. Input: a key with the same low 32 hash bits as a victim key or slot. Result: write lands in the victim slot. No working exploit code.
Known: none (route to the VM part too)

### P35-F05
File: budzero/bud-compiler/src/codegen.rs:389-392; budzero/bud-compiler/src/parser.rs:591-595; budzero/bud-compiler/src/ast.rs:74
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
codegen.rs:391  self.emit(Opcode::Load, pat_reg, 0, 0, *val as i32);
parser.rs:591   Token::Int(val) => { let v = *val; self.consume(); MatchPattern::IntLit(v) }
```
Description: IntLit holds a u64, but the Load immediate is cut with `as i32`. A pattern of 4294967296 becomes 0, 3000000000 becomes a negative immediate (VM loads P - |imm|, vm lib.rs:352-358). Sema does not range check patterns and the Goldilocks bound check (codegen.rs:596-606) lives only in Expr::Int, which patterns never reach. Hex patterns are not accepted by parse_match_arm (only Token::Int), so only decimal literals trigger it.
Impact: `match (x) { 4294967296 => A, _ => B }` runs A when x == 0 and not when x == 2^32. Literals at or above P are not rejected either.
Trigger: compile the match above, run with x = 0.
Known: none

### P35-F06
File: budzero/bud-compiler/src/codegen.rs:346-430 against budzero/bud-compiler/src/sema.rs:547-552
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
sema.rs:548   let mut arm_env = env.clone();      // arm bindings are scoped
codegen.rs:409  self.generate_stmt(s, scope, storage);   // scope is the outer &mut map
```
Description: Sema scopes a `let` inside a match arm to the arm; codegen passes the outer scope map, so the arm's `let x` replaces the outer binding of x and then next_reg is reset (line 415), so the register is reused. `let x = 1; match (y) { 0 => { let x = 5; }, _ => {} }; emit E(x);` passes sema and then reads the arm's register.
Impact: Wrong or stale value after a match when an arm shadows an outer name. Register file reuse makes the value arbitrary.
Trigger: the example above, y = 1.
Known: none

### P35-F07
File: budzero/bud-compiler/src/sema.rs:534-553; budzero/bud-compiler/src/codegen.rs:364-366; budzero/bud-compiler/src/lib.rs:813-815
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
sema.rs:534   // Pattern matching. The scrutinee must be an integer ...
sema.rs:536   // Child scope. Exhaustiveness is checked.16; for
sema.rs:537   // Now we just require the arm to syntactically parse and
codegen.rs:364  // The semantic analyzer enforces that the last arm is a
codegen.rs:365  // Wildcard (otherwise the chain has no fall-through
```
Description: Neither the codegen comment nor the test comment (the wildcard is "semantic-checked") is true: sema checks only the scrutinee type and the arm bodies. A match without `_` silently does nothing when no arm hits, a `_` in the middle makes later arms dead, and duplicate integer arms are accepted (first wins). Match is also a statement only (parser.rs:416 demands a trailing `;`).
Impact: A forgotten default arm silently skips the handling; reviewers trust the stated guarantee.
Trigger: `match (x) { 0 => {...}, };` compiles.
Known: none

### P35-F08
File: budzero/bud-compiler/src/parser.rs:514-583 (loops that build left-deep trees), sema.rs:560-785, codegen.rs:577-948; budzero/bud-compiler/src/lib.rs:59-75
Severity: Medium
Axis: security
Confidence: Likely (missing step: compile `1+1+...` with 100000 terms under the default 8 MiB main stack and a 2 MiB thread stack)
Evidence:
```
parser.rs:540  while matches!(self.peek(), Token::Plus | Token::Minus) {
parser.rs:547      left = Expr::Binary(Box::new(left), op, Box::new(right));
lib.rs:59      pub fn compile(source: &str, profile: IsaProfile) -> Result<Vec<u64>, CompileError> {
```
Description: MAX_NESTING_DEPTH (parser.rs:45) bounds recursion through parse_expr and parse_stmt, but operator chains and `a.b.c.d...` chains are parsed in loops, so a flat source of N terms yields an AST of depth N. sema analyze_expr, codegen generate_expr, expr_struct_type (FieldAccess chain) and the compiler-generated Drop of the Box chain all recurse once per level. compile() has no source length cap (the fuzz target caps at 64 KiB, fuzz_targets/budl_compile.rs:44, the product code does not). The only test of this shape uses 50 terms (lib.rs:676-687).
Impact: A hostile or generated source of a few hundred KB can overflow the stack. Release builds use panic = "abort" (budzero/Cargo.toml profile), so the host process dies. Today the only caller is bud-cli; if the compiler is put behind an endpoint (the fuzz header calls that the plan) it is a remote abort.
Trigger: precondition: any caller of compile on untrusted text. Entry point lib.rs:59. Input: `contract C { pub fn main() { let x = 1 + 1 + ... ; } }` with 10^5 terms. Expected wrong result: process abort instead of CompileError.
Known: the depth guard exists (idle-code-baseline.txt:762 lists only the const as idle); this route is not covered by it.

### P35-F09
File: budzero/bud-compiler/src/lexer.rs:95-97
Severity: Medium
Axis: correctness
Confidence: Likely (missing step: lex `/* a **/ x /* b */` with logos 0.16)
Evidence:
```
96:  #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
```
Description: The body alternative `\*[^/]` consumes a star together with the following star, so a comment that ends with an even run of stars before the slash (`**/`, `****/`) cannot end there: the final star is eaten as the pair of the previous one. The regex then continues to the next `*/` anywhere later in the file. `/** note **/ constrain(ok); /* x */` lexes as one comment and `constrain(ok);` disappears. With no later `*/`, the text lexes as Slash and Star tokens and the parse fails instead.
Impact: Silent removal of source (including `constrain`) between a `**/` comment and the next block comment; the file a reviewer reads is not the file that is compiled.
Trigger: the sample above.
Known: none

### P35-F10
File: budzero/bud-compiler/src/parser.rs:693-712 with 419-444
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
parser.rs:431   let end = self.parse_expr()?;
parser.rs:432   self.expect(Token::BraceOpen)?;
parser.rs:693   } else if self.peek() == &Token::BraceOpen {   // struct literal
```
Description: In `for i in 0..n { ... }` the end bound is a bare identifier followed by `{`, which parse_primary reads as the start of a struct literal `n { sum : ...`. The body's first statement (`sum = sum + i;`) then fails with "Expected Colon". Only literal bounds (`0..5`, as in every test, example_loop.bud and the docs) or non-identifier bounds work.
Impact: Loops over a variable length cannot be written; the failure is a parse error, not a miscompile. Programs must wrap the bound in parentheses or a call, which no document says.
Trigger: `fn f(n: u64) { for i in 0..n { constrain(1); } }`.
Known: none

### P35-F11
File: budzero/bud-compiler/src/parser.rs:220-227; budzero/bud-compiler/src/lib.rs:62-63
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
parser.rs:220   self.expect(Token::BraceClose)?;
parser.rs:222   Ok(Contract { name, storage, structs, functions })
```
Description: parse_contract returns after the closing brace and compile() never checks that the token stream is exhausted. A second contract, extra functions placed after the brace, or any garbage that lexes is accepted and dropped.
Impact: Code that looks deployed (for example a second `contract`) is not in the bytecode; the program hash covers only the first contract.
Trigger: `contract A { pub fn main() { } } contract B { pub fn main() { constrain(0); } }` compiles to A only.
Known: none

### P35-F12
File: budzero/bud-compiler/src/lib.rs:82, 265, 298, 425, 453, 472; budzero/bud-compiler/src/codegen.rs:962-985; budzero/bud-isa/src/lib.rs:59-61
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
```
lib.rs:298   #[cfg(feature = "experimental")]
lib.rs:299   fn the_uncovered_operators_produce_the_right_result() {
bud-isa lib.rs:59   pub fn is_experimental(&self) -> bool { false }
rg 'experimental' .github/workflows  -> no cargo invocation with the feature
```
Description: The tests for Neq, Lt, Gt, Lte, operator precedence and parentheses, comments, the for loop, and the u64 division gate compile only with the `experimental` feature, which no workflow enables. Since is_experimental() always returns false they would also pass under the default feature set (they use IsaProfile::Experimental only because they were written when the gate existed), so the gating hides them for no reason. The codegen experimental branch (962-985) is dead for the same reason, and the `profile` argument has no effect.
Impact: The comparison opcodes and precedence have no executed test in CI; a swapped Lt/Gt would pass. Overlaps with the Z3 principle (tests skipped).
Trigger: not applicable.
Known: none

### P35-F13
File: budzero/bud-compiler/src/codegen.rs:510-516
Severity: Medium
Axis: integration
Confidence: Certain
Evidence:
```
510:  Stmt::Emit(_name, args) => {
511:      for arg in args { let reg = ...; self.emit(Opcode::Log, 0, reg, 0, 0); }
```
Description: The event name is dropped and arguments are logged as bare words with no count or delimiter. `emit Transfer(1, 2);` and `emit Mint(1, 2);` give the same events, and `emit A(1); emit B(2);` equals `emit C(1, 2);`. Sema ignores both name and arity (sema.rs:529-533).
Impact: Any consumer that decodes `vm.events` cannot tell events apart, so off-chain indexers or light clients that rely on event logs can be confused by a contract that emits ambiguous sequences.
Trigger: compile the two emits above and compare vm.events.
Known: none

### P35-F14
File: budzero/bud-compiler/src/ir/mod.rs:1-10; budzero/bud-compiler/src/ir/lower.rs:1-8; budzero/bud-compiler/src/ir/verify.rs:1-9
Severity: Medium
Axis: integration
Confidence: Certain
Evidence:
```
rg -n 'lower_contract|verify_program|verify_function|LoweringError|VerifyError|\.effect\(\)|result_type\(' --glob '*.rs' . | rg -v 'budzero/bud-compiler/src/ir'
  -> only bud_proof::adapter::VerifyError (unrelated); no caller of the IR
compile() (lib.rs:59-75): parse, sema, codegen. No IR step.
```
Description: BudIR (2528 lines: mod 397, lower 1092, verify 767, display 272) is `pub` but nothing calls lower_contract or verify_program outside its own tests. The module headers admit it ("the next integration step"). The lower.rs header says the functions are "reached by compile()", which is not true today.
Impact: Dead code under the rule "everything coded must be wired"; the header text can mislead a reader into believing the verifier gates compile(). The IR path also diverges from codegen semantics (scoping, loop bound, see F15), so wiring it later changes behaviour.
Trigger: not applicable.
Known: none (the module is not in the dead-pub-api baseline; the headers are the only record)

### P35-F15
File: budzero/bud-compiler/src/ir/lower.rs:213-215, 269-306, 344-359; budzero/bud-compiler/src/ir/verify.rs:250-262, 266-296
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
verify.rs:256   if *cond_ty != IrType::Bool { return Err(VerifyError::BranchConditionNotBool {
lower.rs:213    if !ctx.current_block_terminated() { ctx.set_terminator(Terminator::Return(None)); }
```
Description: Lowering and verifier disagree on valid BudL. (a) Sema accepts u64 conditions (`if (x)`, `while (n)`, sema.rs:403-410), lowering branches on the U64 value, the verifier rejects it. (b) A non-void function whose every path returns inside if/else leaves an empty merge block that receives `Return(None)`, the verifier reports ReturnTypeMismatch. (c) A function that falls off its end returns None (verifier error) while codegen returns 0 (codegen.rs:186-192). (d) Scoping: lowering scopes if/while bodies, sema and codegen leak them. (e) The verifier skips type checks for Load, Store, WriteLocal against the local type, Call arity and types, Assert and Const range (no P bound), and `valid_blocks` is unused (verify.rs:394). (f) FieldAccess always yields U64 (lower.rs:619-624), so nested `a.b.c` is unsupported.
Impact: Only matters once wired (F14); today it is a latent mismatch between two front-to-back paths.
Known: none

### P35-F16
File: budzero/bud-compiler/src/sema.rs:412-431; budzero/bud-compiler/src/codegen.rs:186-192
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
codegen.rs:188  let zero = self.alloc_reg();
codegen.rs:189  self.emit(Opcode::Load, zero, 0, 0, 0);
codegen.rs:190  self.emit(Opcode::Push, 0, zero, 0, 0);
```
Description: Sema does not check that every path of a function with a return type returns. The epilogue pushes 0 as the result. For `-> u64` that is a plausible value, for a struct type it is a null pointer, so `f().x` is a Load at address 0 + offset (valid VM memory, vm lib.rs:993-1002) and returns whatever sits there. main may also declare parameters (the program calls it with an empty stack), unchecked.
Impact: Silent wrong result or a stack underflow VmError instead of a compile error.
Known: none

### P35-F17
File: budzero/bud-compiler/src/sema.rs:480-498; budzero/bud-compiler/src/codegen.rs:288-345
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
sema.rs:483   for s in then_branch { self.analyze_stmt(s, env, errors); }   // env is not cloned
```
Description: `if` and `while` bodies share the outer environment in both sema and codegen. A name bound only inside a branch stays visible afterwards, and when the branch did not run its register holds an old temporary. `if (c) { let t = 1; } emit E(t);` compiles. A `let x` inside the branch also replaces the outer x for the rest of the function. The For loop and (in sema only) match clone the scope, so the three constructs disagree.
Impact: Read of an uninitialised register; unexpected shadowing.
Known: none

### P35-F18
File: budzero/bud-compiler/src/sema.rs:41-47, 691-785
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
sema.rs:706   // Booleans are permitted in arithmetic because BudL exposes no logical/bitwise operators
sema.rs:783   l_ty
```
Description: `u64` is a Goldilocks element. Add, Sub, Mul wrap modulo P with no checked variant, so `balance - amount` with amount > balance yields P - k (a huge value) with no failure, and a comparison after it is true. Bool arithmetic (`flag + flag`) is typed Bool but may hold 2 or P - 1. The sema comments state the field reading for `/` but the language offers no way to ask for overflow detection, and the name u64 suggests 2^64 wrap. Values from P to 2^64 - 1 are not representable although the type is named u64 (the literal check rejects them, runtime inputs like msg::sender are not reduced by the compiler).
Impact: Underflow of balances and counters is silent; reviewers who read u64 as machine integer are misled.
Known: none (design choice documented in sema.rs:66-79)

### P35-F19
File: budzero/bud-compiler/src/codegen.rs:883-916; budzero/bud-isa/src/lib.rs:66-76, 167
Severity: Low
Axis: invariant(Z1)
Confidence: Certain
Evidence:
```
lib.rs:615   // test_verify_merkle_proof_constant_path_ok: compile(..., IsaProfile::Production) is_ok
bud-isa lib.rs:66   pub fn requires_mainnet_activation(&self) -> bool { matches!(self, Opcode::VerifyMerkle | ...
```
Description: The compiler emits VerifyMerkle under IsaProfile::Production and never consults requires_mainnet_activation, which only decode_for_mainnet applies. The Z1 gate therefore lives only at decode time. The compiler gives no diagnostic that the produced bytecode will be refused on mainnet while the gate is closed.
Impact: Late failure (at deploy or run), not an unsafe acceptance. Noted because AUDIT_PLAN section 5 asks to confirm the gate stays closed in production; in this part it is enforced nowhere at compile time.
Known: STATUS.md decision Z1 (gate closed) covers the VM side only.

### P35-F20
File: budzero/bud-compiler/src/codegen.rs:25; budzero/bud-compiler/src/ast.rs:21; budzero/bud-compiler/src/lexer.rs:19-20; budzero/bud-compiler/src/parser.rs:9
Severity: Low
Axis: dead code
Confidence: Certain
Evidence:
```
codegen.rs:25  #[allow(dead_code)]
codegen.rs:26  pub struct Codegen {
```
Description: A struct-wide `allow(dead_code)` (Z3) hides which field is unused; the likely one is `profile`, read only under the experimental cfg. Never read: Function.is_pub (so `pub` has no meaning, any function is callable and only `main` is an entry), Contract.name, Parser._source, Token::Witness (a reserved keyword the parser has no rule for), Token::Else is used, Token::In is used.
Impact: Maintenance risk, a language keyword with no feature.
Known: none

### P35-F21
File: budzero/bud-compiler/src/lexer.rs:97
Severity: Low
Axis: correctness
Confidence: Likely (missing step: lex a CRLF file)
Evidence:
```
97:  #[regex(r"[ \t\n\f]+", logos::skip)]
```
Description: Carriage return is not in the whitespace set. A source with CRLF line ends is rejected with "unexpected token" at the first line end outside a line comment.
Impact: Contracts checked out with CRLF (Windows) fail to compile; a diagnostics issue, no safety effect.
Known: none

### P35-F22
File: budzero/bud-compiler/src/sema.rs:188, 197, 291; budzero/bud-compiler/src/parser.rs:262, 466, 637, 682, 707; budzero/bud-compiler/src/codegen.rs:101
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
sema.rs:291   self.functions.insert(f.name.clone(), (params, ret_ty));
parser.rs:682  args.push(self.parse_expr()?); if self.peek() == &Token::Comma { self.consume(); }
```
Description: Duplicate function, struct, storage field and parameter names are accepted; the last definition wins in sema and in codegen's func_offsets (a second `main` replaces the first). Commas between arguments, parameters, emit arguments and struct fields are optional, so `f(1 2)` is two arguments. A user function named `poseidon` is shadowed by the built-in without a message.
Impact: A contract with a copy-pasted duplicate function compiles to the last copy.
Known: none

### P35-F23
File: budzero/bud-compiler/src/sema.rs:465-467, 505-506, 529-533, 579; budzero/bud-compiler/src/codegen.rs:551-575
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
sema.rs:579   Expr::StorageRead(_) => Type::U64,
sema.rs:465   Stmt::StorageWrite(_, expr) => { self.analyze_expr(expr, env, errors); }
```
Description: Storage reads are always u64 whatever the declared type, storage writes are not type checked against the field (a Map field can be written as a scalar, a struct pointer can be stored), `for` bounds and emit arguments are untyped, and `storage::missing` is caught only later by codegen as CodegenError. Struct values from a mapping (`let s = m[1]; s.x`) pass sema and fail in codegen (expr_struct_type returns None for MappingRead, codegen.rs:807-816).
Impact: Type holes in storage code; a late, less clear error for valid-looking programs.
Known: none

### P35-F24
File: budzero/bud-compiler/src/sema.rs:317, 628-635
Severity: Info
Axis: correctness (determinism)
Confidence: Certain
Evidence:
```
sema.rs:628  for fname in sfields.keys() {
sema.rs:317  Err(errors.remove(0))
```
Description: Missing-field errors are generated in HashMap key order and only the first is returned, so for a literal missing two fields the named field can differ between runs. The emitted bytecode is deterministic (no map is iterated in codegen).
Impact: Diagnostic text only.
Known: none

### P35-F25
File: budzero/bud-compiler/src/codegen.rs:746-758; budzero/bud-compiler/src/lib.rs:49-57
Severity: Info
Axis: correctness
Confidence: Certain
Evidence:
```
codegen.rs:754  self.emit(Opcode::Add, 31, 31, size_reg, 0); // bump heap pointer
```
Description: The heap is a bump allocator with no bound and no free. With HEAP_BASE 4096 and the documented minimum VM memory of 8192, a struct literal in a loop exhausts the 4096 spare bytes after 256 two-word structs and the VM halts with InvalidMemoryAccess. The compiler gives no warning and accepts a struct literal inside `while` or `for`.
Impact: Runtime failure of loops that allocate; also allocation in a callee is never reclaimed.
Known: none

### P35-F26
File: budzero/bud-compiler/src/sema.rs:19-27, 303-308; budzero/bud-compiler/src/codegen.rs:871-882
Severity: Info
Axis: correctness
Confidence: Certain
Evidence:
```
codegen.rs:872  self.emit(Opcode::Syscall, res, 0, 0, 1);   // msg::sender() -> one 64-bit word
```
Description: msg::sender() and Address fields are one 64-bit word (sema comment states it). Identity checks written with this value compare 64 bits of a longer identity.
Impact: Access control by sender is as strong as the 64-bit projection; the sema comment calls it a measured limitation. Listed so the owner sees it in the audit.
Known: none (documented in source)
