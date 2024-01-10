var searchIndex = JSON.parse('{\
"lispdm":{"doc":"LispDM is a Scheme interpreter. Currently, it supports …","t":"NENNNDDNNNNEGNIGNENNNNNNGEGENNNNNNNLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLOLLLLLLLLLLLLLLLLLLLLLLLLLKLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL","n":["Any","Arity","AtLeast","Boolean","Char","Engine","EnvRef","Err","Err","EvalError","Exact","Expr","Exprs","Float","FromExpr","FromExprResult","Integer","LispDMError","List","Ok","Ok","ParseError","Procedure","Procedure","ProcedureFn","ProcedureKind","ProcedureResult","ProcedureReturn","Range","SpecialForm","String","Symbol","TailCall","Value","Void","add","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","copy","cwd","default","default","env","eq","eq","eq","eq","eval","exprs","extend","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from_expr","from_expr","get","get_expr","has","has_macro","into","into","into","into","into","into","into","into","is_boolean","is_char","is_dotted_list","is_empty_list","is_float","is_integer","is_list","is_procedure","is_proper_list","is_root","is_specific_symbol","is_string","is_symbol","is_truthy","is_void","kind","new_dotted_list","new_empty_list","new_proper_list","new_string","new_symbol","new_without_prelude","register_fn","set","set_cwd","to_owned","to_owned","to_owned","to_owned","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id"],"q":[[0,"lispdm"],[161,"alloc::string"],[162,"core::convert"],[163,"std::path"],[164,"core::result"],[165,"core::fmt"],[166,"core::fmt"],[167,"alloc::collections::vec_deque"],[168,"core::option"],[169,"alloc::string"]],"d":["Any number of arguments.","Represents the arity of a procedure.","At least this many arguments.","Boolean","Character","Provides the main functionality of LispDM.","Reference to the environment.","Contains the error value","Contains the error value","Error occurred while evaluating expressions.","Exact number of arguments.","Represents all possible values in interpreter.","A list of <code>Expr</code>s. Also can be created with <code>exprs!</code> macro.","Real number","The exit point for turning <code>Expr</code> into Rust types.","Represents result of conversion from <code>Expr</code> to another type","Integer number","Error type that represents all possible errors in LispDM.","An immutable list of expressions.","Contains the success value","Contains the success value","Error occurred while parsing source code.","Procedure","A normal procedure. Arguments <em>are</em> evaluated before the …","The type of a procedure function.","Represents the kind of a procedure.","The result of a procedure call.","The return value of a procedure. Used to determine whether …","From min to max arguments.","A special form. Arguments <em>are not</em> evaluated before the …","A reference to mutable string.","Symbol","Perform a tail call of <code>Expr</code> in <code>EnvRef</code>.","Return <code>Expr</code>.","Unspecified value","Adds a new binding to the environment. If the binding …","","","","","","","","","","","","","","","","","","","","","","","Creates a reference to the deep copy of underlying …","Returns the current working directory of the environment.","","","Returns reference to the root environment.","","","","","Evaluates the given source code and returns the result. …","A helper macro to construct <code>Exprs</code> from given expressions","Creates a reference to a new environment that has this …","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","Returns the argument unchanged.","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Tries to convert <code>Expr</code> into <code>Self</code>. Returns …","","<code>get_expr</code> with a type conversion.","Returns the expression bound to <code>name</code>. If the no expression …","Checks if <code>self</code> contains a binding with <code>name</code>.","Checks if <code>self</code> contains a macro with <code>name</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Tries to convert <code>self</code> into <code>T</code>, which is a type that …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Checks if <code>self</code> is a <code>Expr::Boolean</code>","Checks if <code>self</code> is a <code>Expr::Char</code>","Checks if <code>self</code> represents a dotted list This means that …","Checks if <code>self</code> represents an empty list <code>&#39;()</code>","Checks if <code>self</code> is a <code>Expr::Float</code>","Checks if <code>self</code> is a <code>Expr::Integer</code>","checks if <code>self</code> is a <code>Expr::List</code>","Checks if <code>self</code> is a <code>Expr::Procedure</code>","Checks if <code>self</code> represents a proper list","Checks if <code>self</code> is the root environment.","Checks if <code>self</code> is a <code>Expr::Symbol</code> with given <code>name</code>","Checks if <code>self</code> is a <code>Expr::String</code>","Checks if <code>self</code> is a <code>Expr::Symbol</code>","Returns true if <code>self</code> represents a true value","Checks if <code>self</code> is a <code>Expr::Void</code>","Returns string representation of the type of <code>self</code>","Creates new dotted list like <code>&#39;(1 2 . 3)</code> from <code>Exprs</code>","Creates new empty list <code>&#39;()</code>","Creates new proper list like <code>&#39;(1 2 3)</code> from <code>Exprs</code>","Creates new <code>Expr::String</code> from any type that implements …","Creates new <code>Expr::Symbol</code> from any type that implements …","Creates a new instance of <code>Engine</code> without loading the …","Registers a new procedure in the root environment. Can be …","Sets new value to an existing binding. Returns an <code>Err</code> if …","Sets the current working directory of the environment.","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[7,0,7,4,4,0,0,12,28,13,7,0,0,4,0,0,4,0,4,12,28,13,4,6,0,0,0,0,7,6,4,4,29,29,4,1,29,9,1,4,6,7,13,29,9,1,4,6,7,13,1,4,6,7,1,4,6,7,1,1,9,1,9,1,4,6,7,9,0,1,1,4,4,6,7,7,13,13,29,9,1,4,4,4,4,4,4,4,4,4,4,6,7,13,15,4,1,1,1,1,29,9,1,4,4,6,7,13,4,4,4,4,4,4,4,4,4,1,4,4,4,4,4,4,4,4,4,4,4,9,9,1,1,1,4,6,7,4,7,13,29,9,1,4,6,7,13,29,9,1,4,6,7,13,29,9,1,4,6,7,13],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[1,2,-1],3,[[5,[4]]]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[1,1],[4,4],[6,6],[7,7],[[-1,-2],3,[],[]],[[-1,-2],3,[],[]],[[-1,-2],3,[],[]],[[-1,-2],3,[],[]],[1,1],[1,8],[[],9],[[],1],[9,1],[[1,1],10],[[4,4],10],[[6,6],10],[[7,7],10],[[9,11],[[14,[[12,[-1]],13]]],15],0,[1,1],[[1,16],17],[[4,16],17],[[4,16],17],[[6,16],17],[[7,16],17],[[7,16],17],[[13,16],17],[[13,16],17],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[18,4],[[[19,[-1]]],4,[[5,[4]]]],[20,4],[-1,-1,[]],[11,4],[21,4],[10,4],[[[22,[-1]]],4,[[5,[4]]]],[2,4],[[[3,[-1,-2]]],4,[[5,[4]]],[[5,[4]]]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[4,[[12,[-1]]],[]],[4,[[12,[4]]]],[[1,11],[[23,[[12,[-1]]]]],15],[[1,11],[[23,[4]]]],[[1,11],10],[[1,11],10],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[4,[[12,[-1]]],15],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[4,10],[4,10],[4,10],[4,10],[4,10],[4,10],[4,10],[4,10],[4,10],[1,10],[[4,11],10],[4,10],[4,10],[4,10],[4,10],[4,11],[24,4],[[],4],[24,4],[-1,4,[[5,[2]]]],[-1,4,[[5,[2]]]],[[],9],[[9,-1,6,7,25],3,26],[[1,2,-1],[[14,[3,2]]],[[5,[4]]]],[[1,8],3],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,2,[]],[-1,2,[]],[-1,2,[]],[-1,[[14,[-2]]],[],[]],[-1,[[14,[-2]]],[],[]],[-1,[[14,[-2]]],[],[]],[-1,[[14,[-2]]],[],[]],[-1,[[14,[-2]]],[],[]],[-1,[[14,[-2]]],[],[]],[-1,[[14,[-2]]],[],[]],[-1,[[14,[-2]]],[],[]],[-1,[[14,[-2]]],[],[]],[-1,[[14,[-2]]],[],[]],[-1,[[14,[-2]]],[],[]],[-1,[[14,[-2]]],[],[]],[-1,[[14,[-2]]],[],[]],[-1,[[14,[-2]]],[],[]],[-1,27,[]],[-1,27,[]],[-1,27,[]],[-1,27,[]],[-1,27,[]],[-1,27,[]],[-1,27,[]]],"c":[],"p":[[3,"EnvRef",0],[3,"String",161],[15,"tuple"],[4,"Expr",0],[8,"Into",162],[4,"ProcedureKind",0],[4,"Arity",0],[3,"PathBuf",163],[3,"Engine",0],[15,"bool"],[15,"str"],[6,"FromExprResult",0],[4,"LispDMError",0],[4,"Result",164],[8,"FromExpr",0],[3,"Formatter",165],[6,"Result",165],[15,"f64"],[3,"Vec",166],[15,"char"],[15,"i64"],[3,"VecDeque",167],[4,"Option",168],[6,"Exprs",0],[6,"ProcedureFn",0],[8,"ToString",161],[3,"TypeId",169],[6,"ProcedureResult",0],[4,"ProcedureReturn",0]],"b":[[71,"impl-Display-for-Expr"],[72,"impl-Debug-for-Expr"],[74,"impl-Debug-for-Arity"],[75,"impl-Display-for-Arity"],[76,"impl-Debug-for-LispDMError"],[77,"impl-Display-for-LispDMError"],[81,"impl-From%3Cf64%3E-for-Expr"],[82,"impl-From%3CVec%3CT%3E%3E-for-Expr"],[83,"impl-From%3Cchar%3E-for-Expr"],[85,"impl-From%3C%26str%3E-for-Expr"],[86,"impl-From%3Ci64%3E-for-Expr"],[87,"impl-From%3Cbool%3E-for-Expr"],[88,"impl-From%3CVecDeque%3CT%3E%3E-for-Expr"],[89,"impl-From%3CString%3E-for-Expr"],[90,"impl-From%3C(A,+B)%3E-for-Expr"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};