```
binding = NAME [':' type]
bindings = binding {',' binding}

typebinding = 'type' NAME ['<' generictypeswithdefaults '>'] '=' type

path = '@' [NAME]
     | path '/' NAME
-- i don't know how i feel about this import syntax
import = 'local' binding '=' 'import' path
export = 'export' bindings '=' exprs

-- we could just pick one, i guess?
fnword = 'fn' | 'function'

block = {stat [';']} [expr]
stat = var '=' expr
     | var compoundop expr
     | 'while' expr 'do' block 'end'
     | 'repeat' block 'until' expr
     | 'for' bindings 'in' exprs 'do' block 'end'
     | fnword funcname ['<' generictypes '>'] '(' [parameters] ')' [ ':' type] block 'end'
     | 'module' binding 'do' properties 'end'
     | 'local' bindings '=' exprs
     | ['export'] typebinding
     | import | export
     | 'break' | 'continue' | 'return'

var = NAME | prefixexp
vars = var {',' var}
prefixexpr = var | functioncall | '(' expr ')'
functioncall = prefixexpr '(' [exprs] ')'
             | prefixexpr ':' NAME  '(' [exprs] ')'

expr = unop expr { binop expr }
     | 'do' block 'end'
     | 'if' expr 'then' block {'elseif' expr 'then' block} ['else' block] 'end'
     | fnword ['<' generictypes '>'] '(' [parameters] ')' [ ':' type] block 'end'
     | simpleexpr '::' type
     | simpleexpr
simpleexpr = prefixexpr
           | literal
exprs = expr {separator expr} [separator]
separator = ',' | ';'

literal = NUMBER
        | STRING
        | 'nil'
        | 'true'
        | 'false'
        | table
        | array
literals = literal {',' literal}

array = '[' [exprs] ']'
table = '{' [properties] '}'
properties = property {separator property} [separator]
property = '[' expr ']' '=' expr
         | NAME '=' expr
         | typebinding

compoundop :: '+=' | '-=' | '*=' | '/=' | '//=' | '%=' | '^=' | '..='
binop = '+' | '-' | '*' | '/' | '//' | '^' | '%' | '..' | '<' | '<=' | '>' | '>=' | '==' | '~=' | 'and' | 'or'
unop = '-' | 'not' | '#'

type = [simpletype {'?'}] {'|' simpletype {'?'}}
     | [simpletype {'?'}] {'&' simpletype {'?'}}
types = type {',' type}

generictype = NAME
generictypes = generictype {',' generictype}
generictypewithdefault = generictype ['=' type]
generictypeswithdefaults = generictypewithdefault {',' generictypeswithdefaults}

tabletype = '{' [proplist] '}'
propertytypes = propertytypeorindexer {separator propertytypeorindexer} [separator]
propertytype = NAME ':' type
indexer = '[' type ']' ':' type
propertytypeorindexer = ['read' | 'write'] (propertytype | indexer)
                      | 'type' NAME ['<' generictypes '>']
proplist = propertytypeorindexer {',' propertytypeorindexer}

singleton = STRING | 'true' | 'false'
simpletype = 'nil'
           | singleton
           | 'boolean'
           | 'string'
           | 'number'
           | '(' types ')'
           | ['<' generictypes '>'] '(' types ')' '->' type
           | tabletype
           | '[' type ']'
```
