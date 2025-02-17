use crate::ast::{AstNode, BinaryOperator};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct CodeGenerator {
    assembly: String,
    label_count: usize,
    var_map: HashMap<String, i32>,
    current_stack_offset: i32,
    string_literals: Vec<String>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            assembly: String::new(),
            label_count: 0,
            var_map: HashMap::new(),
            current_stack_offset: 0,
            string_literals: Vec::new(),
        }
    }

    fn emit(&mut self, line: &str) {
        self.assembly.push_str(line);
        self.assembly.push('\n');
    }

    fn get_new_label(&mut self) -> String {
        let label = format!(".L{}", self.label_count);
        self.label_count += 1;
        label
    }

    fn get_var_location(&mut self, name: &str) -> i32 {
        if let Some(&offset) = self.var_map.get(name) {
            offset
        } else {
            self.current_stack_offset -= 8;
            self.var_map
                .insert(name.to_string(), self.current_stack_offset);
            self.current_stack_offset
        }
    }

    fn add_string_literal(&mut self, s: &str) -> usize {
        let index = self.string_literals.len();
        self.string_literals.push(s.to_string());
        index
    }

    fn collect_string_literals(&mut self, node: &AstNode) {
        match node {
            AstNode::StringLiteral(s) => {
                self.add_string_literal(s);
            }
            AstNode::PrintLn(expr) => {
                self.collect_string_literals(expr);
            }
            AstNode::Block(statements) => {
                for stmt in statements {
                    self.collect_string_literals(stmt);
                }
            }
            AstNode::If(cond, then_branch, else_branch) => {
                self.collect_string_literals(cond);
                self.collect_string_literals(then_branch);
                if let Some(else_node) = else_branch {
                    self.collect_string_literals(else_node);
                }
            }
            AstNode::While(cond, body) => {
                self.collect_string_literals(cond);
                self.collect_string_literals(body);
            }
            AstNode::FunctionDecl(_, _, body) => {
                self.collect_string_literals(body);
            }
            AstNode::FunctionPredecl(_, _) => {
                // Nothing to collect for predeclarations
            }
            AstNode::BinaryOp(left, _, right) => {
                self.collect_string_literals(left);
                self.collect_string_literals(right);
            }
            AstNode::Assignment(_, value) => {
                self.collect_string_literals(value);
            }
            AstNode::FunctionCall(_, args) => {
                for arg in args {
                    self.collect_string_literals(arg);
                }
            }
            _ => {}
        }
    }

    fn generate_node(&mut self, node: &AstNode) {
        match node {
            AstNode::Number(n) => {
                self.emit(&format!("    pushq ${}", n));
            }
            AstNode::Variable(name) => {
                let offset = self.get_var_location(name);
                self.emit(&format!("    pushq {}(%rbp)", offset));
            }
            AstNode::StringLiteral(s) => {
                let index = if let Some(idx) = self.string_literals.iter().position(|x| x == s) {
                    idx
                } else {
                    self.add_string_literal(s)
                };
                self.emit(&format!("    leaq str{}(%rip), %rax", index));
                self.emit("    pushq %rax");
            }
            AstNode::Assignment(name, value) => {
                self.generate_node(value);
                let offset = self.get_var_location(name);
                self.emit("    popq %rax");
                self.emit(&format!("    movq %rax, {}(%rbp)", offset));
            }
            AstNode::PrintLn(expr) => {
                self.generate_node(expr);
                self.emit("    popq %rsi"); // Get value/pointer to print

                match &**expr {
                    AstNode::StringLiteral(_) => {
                        // String printing
                        self.emit("    movq %rsi, %rdi"); // Setup for strlen
                        self.emit("    call strlen");
                        self.emit("    movq %rax, %rdx"); // Length for write
                        self.emit("    movq %rsi, %rsi"); // String ptr
                        self.emit("    movq $1, %rdi"); // stdout
                        self.emit("    movq $1, %rax"); // sys_write
                        self.emit("    syscall");
                    }
                    _ => {
                        // Number printing
                        self.emit("    movq %rsi, %rdi"); // Number to print
                        self.emit("    subq $32, %rsp"); // Buffer space
                        self.emit("    movq %rsp, %rsi"); // Buffer ptr
                        self.emit("    movq $32, %rdx"); // Buffer size
                        self.emit("    movq $10, %rcx"); // Base 10
                        self.emit("    call itoa");

                        self.emit("    movq %rax, %rdx"); // Length
                        self.emit("    movq %rsp, %rsi"); // String ptr
                        self.emit("    movq $1, %rdi"); // stdout
                        self.emit("    movq $1, %rax"); // sys_write
                        self.emit("    syscall");

                        self.emit("    addq $32, %rsp"); // Restore stack
                    }
                }
            }
            AstNode::BinaryOp(left, op, right) => {
                self.generate_node(left);
                self.generate_node(right);

                self.emit("    popq %rcx");
                self.emit("    popq %rax");

                match op {
                    BinaryOperator::Add => self.emit("    addq %rcx, %rax"),
                    BinaryOperator::Subtract => self.emit("    subq %rcx, %rax"),
                    BinaryOperator::Multiply => self.emit("    imulq %rcx, %rax"),
                    BinaryOperator::Divide => {
                        self.emit("    cqo"); // Sign extend RAX into RDX
                        self.emit("    idivq %rcx");
                    }
                    BinaryOperator::Equals => {
                        self.emit("    cmpq %rcx, %rax");
                        self.emit("    sete %al");
                        self.emit("    movzbq %al, %rax");
                    }
                    BinaryOperator::Less => {
                        self.emit("    cmpq %rcx, %rax");
                        self.emit("    setl %al");
                        self.emit("    movzbq %al, %rax");
                    }
                    BinaryOperator::Greater => {
                        self.emit("    cmpq %rcx, %rax");
                        self.emit("    setg %al");
                        self.emit("    movzbq %al, %rax");
                    }
                    BinaryOperator::LessEqual => {
                        self.emit("    cmpq %rcx, %rax");
                        self.emit("    setle %al");
                        self.emit("    movzbq %al, %rax");
                    }
                    BinaryOperator::GreaterEqual => {
                        self.emit("    cmpq %rcx, %rax");
                        self.emit("    setge %al");
                        self.emit("    movzbq %al, %rax");
                    }
                }

                self.emit("    pushq %rax");
            }
            AstNode::If(condition, then_branch, else_branch) => {
                let else_label = self.get_new_label();
                let end_label = self.get_new_label();

                self.generate_node(condition);
                self.emit("    popq %rax");
                self.emit("    testq %rax, %rax");
                self.emit(&format!("    je {}", else_label));

                self.generate_node(then_branch);
                self.emit(&format!("    jmp {}", end_label));

                self.emit(&format!("{}:", else_label));
                if let Some(else_branch) = else_branch {
                    self.generate_node(else_branch);
                }

                self.emit(&format!("{}:", end_label));
            }
            AstNode::While(condition, body) => {
                let start_label = self.get_new_label();
                let end_label = self.get_new_label();

                self.emit(&format!("{}:", start_label));

                self.generate_node(condition);
                self.emit("    popq %rax");
                self.emit("    testq %rax, %rax");
                self.emit(&format!("    je {}", end_label));

                self.generate_node(body);
                self.emit(&format!("    jmp {}", start_label));

                self.emit(&format!("{}:", end_label));
            }
            AstNode::Block(statements) => {
                for stmt in statements {
                    self.generate_node(stmt);
                }
            }
            AstNode::FunctionDecl(name, params, body) => {
                // Save old state
                let old_var_map = self.var_map.clone();
                let old_stack_offset = self.current_stack_offset;

                // Reset state for new function
                self.var_map.clear();
                self.current_stack_offset = 0;

                // Function prologue
                self.emit(&format!("{}:", name));
                self.emit("    pushq %rbp");
                self.emit("    movq %rsp, %rbp");
                self.emit("    subq $256, %rsp"); // Reserve stack space

                // Store parameters in stack
                for (i, param) in params.iter().enumerate() {
                    let offset = self.get_var_location(param);
                    match i {
                        0 => self.emit(&format!("    movq %rdi, {}(%rbp)", offset)),
                        1 => self.emit(&format!("    movq %rsi, {}(%rbp)", offset)),
                        2 => self.emit(&format!("    movq %rdx, {}(%rbp)", offset)),
                        3 => self.emit(&format!("    movq %rcx, {}(%rbp)", offset)),
                        4 => self.emit(&format!("    movq %r8, {}(%rbp)", offset)),
                        5 => self.emit(&format!("    movq %r9, {}(%rbp)", offset)),
                        _ => {
                            let stack_param_offset = (i - 6 + 2) * 8;
                            self.emit(&format!("    movq {}(%rbp), %rax", stack_param_offset));
                            self.emit(&format!("    movq %rax, {}(%rbp)", offset));
                        }
                    }
                }

                // Generate function body
                self.generate_node(body);

                // Function epilogue
                self.emit("    movq %rbp, %rsp");
                self.emit("    popq %rbp");
                self.emit("    ret");

                // Restore old state
                self.var_map = old_var_map;
                self.current_stack_offset = old_stack_offset;
            }
            AstNode::FunctionPredecl(_, _) => {
                // Nothing to generate for predeclarations
            }
            AstNode::FunctionCall(name, args) => {
                // Push arguments in reverse order
                for arg in args.iter().rev() {
                    self.generate_node(arg);
                }

                // Pop arguments into registers
                for (i, _) in args.iter().enumerate() {
                    match i {
                        0 => self.emit("    popq %rdi"),
                        1 => self.emit("    popq %rsi"),
                        2 => self.emit("    popq %rdx"),
                        3 => self.emit("    popq %rcx"),
                        4 => self.emit("    popq %r8"),
                        5 => self.emit("    popq %r9"),
                        _ => {} // Stack arguments stay on stack
                    }
                }

                // Call function and push return value
                self.emit(&format!("    call {}", name));
                self.emit("    pushq %rax");
            }
            AstNode::Return(value) => {
                if let Some(expr) = value {
                    self.generate_node(expr);
                    self.emit("    popq %rax");
                }

                self.emit("    movq %rbp, %rsp");
                self.emit("    popq %rbp");
                self.emit("    ret");
            }
        }
    }

    pub fn generate(&mut self, ast: &[AstNode]) -> String {
        self.assembly.clear();
        self.string_literals.clear(); // Clear any previous strings

        // First collect all string literals from the AST
        for node in ast {
            self.collect_string_literals(node);
        }

        // Data section
        self.emit(".section .data");
        self.emit("newline:");
        self.emit("    .string \"\\n\"");

        self.emit("");
        self.emit("# String literals");

        // Pre-format string declarations
        let string_declarations: Vec<String> = self
            .string_literals
            .iter()
            .enumerate()
            .flat_map(|(i, s)| vec![format!("str{}:", i), format!("    .string \"{}\"", s)])
            .collect();

        // Emit string declarations
        for decl in string_declarations {
            self.emit(&decl);
        }

        // Text section
        self.emit("");
        self.emit(".section .text");

        // Helper function for string length
        self.emit("");
        self.emit("strlen:");
        self.emit("    pushq %rbp");
        self.emit("    movq %rsp, %rbp");
        self.emit("    movq $-1, %rax");
        self.emit(".Lstrlen_loop:");
        self.emit("    incq %rax");
        self.emit("    movb (%rdi,%rax), %cl");
        self.emit("    testb %cl, %cl");
        self.emit("    jnz .Lstrlen_loop");
        self.emit("    popq %rbp");
        self.emit("    ret");

        // itoa function (for number printing)
        self.emit("");
        self.emit("itoa:");
        self.emit("    pushq %rbp");
        self.emit("    movq %rsp, %rbp");
        self.emit("    pushq %rbx");
        self.emit("    pushq %r12");
        self.emit("    pushq %r13");

        self.emit("    movq %rdi, %rax"); // Number to convert
        self.emit("    movq %rsi, %r12"); // Buffer
        self.emit("    movq $0, %r13"); // Length

        // Handle negative numbers
        self.emit("    cmpq $0, %rax");
        self.emit("    jge .Lpositive");
        self.emit("    negq %rax");
        self.emit("    movb $45, (%r12)"); // Store '-'
        self.emit("    incq %r12");
        self.emit("    incq %r13");

        self.emit(".Lpositive:");
        self.emit("    movq %rax, %rbx"); // Save number
        self.emit("    movq $0, %r8"); // Digit count

        // Count digits
        self.emit(".Lcount:");
        self.emit("    movq $0, %rdx");
        self.emit("    movq $10, %rcx");
        self.emit("    divq %rcx");
        self.emit("    incq %r8");
        self.emit("    cmpq $0, %rax");
        self.emit("    jne .Lcount");

        self.emit("    addq %r8, %r13"); // Add to length
        self.emit("    addq %r8, %r12"); // Point to end
        self.emit("    decq %r12"); // Back up one
        self.emit("    movq %rbx, %rax"); // Restore number

        // Convert digits
        self.emit(".Lconvert:");
        self.emit("    movq $0, %rdx");
        self.emit("    movq $10, %rcx");
        self.emit("    divq %rcx");
        self.emit("    addb $48, %dl"); // To ASCII
        self.emit("    movb %dl, (%r12)"); // Store digit
        self.emit("    decq %r12"); // Move back
        self.emit("    cmpq $0, %rax");
        self.emit("    jne .Lconvert");

        // Return length in rax
        self.emit("    movq %r13, %rax");

        self.emit("    popq %r13");
        self.emit("    popq %r12");
        self.emit("    popq %rbx");
        self.emit("    popq %rbp");
        self.emit("    ret");

        // Generate all functions first
        for node in ast {
            if let AstNode::FunctionDecl(_, _, _) = node {
                self.generate_node(node);
            }
        }

        // Main program
        self.emit("");
        self.emit(".global _start");
        self.emit("");
        self.emit("_start:");
        self.emit("    pushq %rbp");
        self.emit("    movq %rsp, %rbp");
        self.emit("    subq $256, %rsp"); // Stack frame

        // Generate non-function code
        for node in ast {
            if let AstNode::FunctionDecl(_, _, _) = node {
                continue;
            }
            self.generate_node(node);
        }

        // Exit
        self.emit("");
        self.emit("    movq %rbp, %rsp");
        self.emit("    popq %rbp");
        self.emit("    movq $60, %rax"); // exit syscall
        self.emit("    xorq %rdi, %rdi"); // status 0
        self.emit("    syscall");

        self.assembly.clone()
    }
}
