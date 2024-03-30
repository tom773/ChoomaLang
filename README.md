<h1>Chooma Lang</h1>

<p>A completely useless programming language built on top of Rust.</p>

<h3>Idea:</h3>
<p>The main idea of this project is to:

  1. Take user input. Ex. "1 + 1;"
  2. Parse this into some AST tree, so maybe (+ (1, 1))
  3. Then parse this into op-codes, for example "OP_ADD OP_CONSTANT OP_CONSTANT"
  4. Implement the functions in inline x86_64 Assembly code
  5. Inline Assembly syscalls will be the main driver of the language
  6. In future, maybe compile this into an .asm file? 

</p>
