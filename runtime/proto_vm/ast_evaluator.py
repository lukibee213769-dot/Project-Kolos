"""Evaluator for Kolos compiler AST."""

from dataclasses import dataclass
from typing import Any, Dict, List

from compilers.parser import (
    NumberNode,
    StringNode,
    BooleanNode,
    IdentifierNode,
    UnaryNode,
    BinaryNode,
    VarDeclNode,
    AssignNode,
    PropertyAssignNode,
    BlockNode,
    IfNode,
    WhileNode,
    FunctionDefNode,
    ReturnNode,
    CallNode,
    PrintNode,
    ProgramNode,
    ClassDefNode,
    MethodDefNode,
    NewNode,
    ThisNode,
    PropertyAccessNode,
    MethodCallNode,
)


class EvaluationError(Exception):
    """Raised when AST evaluation fails."""


class ReturnException(Exception):
    """Internal signal for return statements in AST evaluation."""
    def __init__(self, value: Any):
        self.value = value


@dataclass
class KolosFunction:
    name: str
    params: List[str]
    body: object
    closure: Dict[str, Any]


@dataclass
class KolosClass:
    name: str
    methods: Dict[str, 'KolosFunction']


class KolosInstance:
    def __init__(self, klass: KolosClass):
        self.klass = klass
        self.fields: Dict[str, Any] = {}
    
    def __repr__(self):
        return f"<{self.klass.name} instance>"


class ASTEvaluator:
    """Evaluate Kolos AST nodes."""

    def __init__(self, variables=None, parent=None):
        self.variables = {} if variables is None else dict(variables)
        self.parent = parent

    def get_variable(self, name: str) -> Any:
        if name in self.variables:
            return self.variables[name]
        if self.parent is not None:
            return self.parent.get_variable(name)
        raise EvaluationError(f"Undefined variable: {name}")

    def set_variable(self, name: str, value: Any):
        if name in self.variables:
            self.variables[name] = value
            return
        if self.parent is not None and self.parent.has_variable(name):
            self.parent.set_variable(name, value)
            return
        self.variables[name] = value

    def has_variable(self, name: str) -> bool:
        if name in self.variables:
            return True
        if self.parent is not None:
            return self.parent.has_variable(name)
        return False

    def evaluate(self, node):
        if node is None:
            return None

        if isinstance(node, NumberNode):
            return node.value

        if isinstance(node, StringNode):
            return node.value

        if isinstance(node, BooleanNode):
            return node.value

        if isinstance(node, IdentifierNode):
            return self.get_variable(node.name)

        if isinstance(node, VarDeclNode):
            val = self.evaluate(node.initializer) if node.initializer is not None else None
            self.variables[node.name] = val
            return val

        if isinstance(node, AssignNode):
            val = self.evaluate(node.value)
            self.set_variable(node.name, val)
            return val

        if isinstance(node, PropertyAssignNode):
            obj = self.evaluate(node.obj)
            val = self.evaluate(node.value)
            if not isinstance(obj, KolosInstance):
                raise EvaluationError(f"Cannot assign property to non-object")
            obj.fields[node.property] = val
            return val

        if isinstance(node, PrintNode):
            val = self.evaluate(node.expression)
            print(val)
            return val

        if isinstance(node, ReturnNode):
            val = self.evaluate(node.value) if node.value is not None else None
            raise ReturnException(val)

        if isinstance(node, BlockNode):
            res = None
            for stmt in node.statements:
                res = self.evaluate(stmt)
            return res

        if isinstance(node, ProgramNode):
            res = None
            for stmt in node.statements:
                res = self.evaluate(stmt)
            return res

        if isinstance(node, IfNode):
            cond = self.evaluate(node.condition)
            if cond:
                return self.evaluate(node.then_branch)
            elif node.else_branch is not None:
                return self.evaluate(node.else_branch)
            return None

        if isinstance(node, WhileNode):
            res = None
            while self.evaluate(node.condition):
                res = self.evaluate(node.body)
            return res

        if isinstance(node, FunctionDefNode):
            fn = KolosFunction(node.name, node.params, node.body, self.variables)
            self.variables[node.name] = fn
            return fn

        if isinstance(node, CallNode):
            fn = self.get_variable(node.callee)
            if not isinstance(fn, KolosFunction):
                raise EvaluationError(f"'{node.callee}' is not a function")
            if len(node.args) != len(fn.params):
                raise EvaluationError(
                    f"Function '{node.callee}' expects {len(fn.params)} arguments, "
                    f"got {len(node.args)}"
                )
            arg_values = [self.evaluate(arg) for arg in node.args]
            scope = dict(fn.closure)
            for param, arg_val in zip(fn.params, arg_values):
                scope[param] = arg_val
            call_evaluator = ASTEvaluator(variables=scope, parent=self)
            try:
                return call_evaluator.evaluate(fn.body)
            except ReturnException as ret:
                return ret.value

        if isinstance(node, UnaryNode):
            operand = self.evaluate(node.operand)
            operator = node.operator.name

            if operator == "PLUS":
                return +operand

            if operator == "MINUS":
                return -operand

            if operator == "NOT":
                return not operand

            raise EvaluationError(
                f"Unsupported unary operator: {node.operator}"
            )

        if isinstance(node, BinaryNode):
            left = self.evaluate(node.left)
            right = self.evaluate(node.right)
            operator = node.operator.name

            if operator == "PLUS":
                return left + right

            if operator == "MINUS":
                return left - right

            if operator == "STAR":
                return left * right

            if operator == "SLASH":
                return left / right

            if operator == "POWER":
                return left ** right

            if operator == "MODULO":
                return left % right

            if operator == "EQ":
                return left == right

            if operator == "NE":
                return left != right

            if operator == "LT":
                return left < right

            if operator == "LE":
                return left <= right

            if operator == "GT":
                return left > right

            if operator == "GE":
                return left >= right

            if operator == "AND":
                return left and right

            if operator == "OR":
                return left or right

            raise EvaluationError(
                f"Unsupported binary operator: {node.operator}"
            )

        if isinstance(node, ClassDefNode):
            methods = {}
            for method_def in node.methods:
                method = KolosFunction(
                    method_def.name,
                    method_def.params,
                    method_def.body,
                    self.variables
                )
                methods[method_def.name] = method
            klass = KolosClass(node.name, methods)
            self.variables[node.name] = klass
            return klass

        if isinstance(node, NewNode):
            klass = self.get_variable(node.class_name)
            if not isinstance(klass, KolosClass):
                raise EvaluationError(f"'{node.class_name}' is not a class")
            
            instance = KolosInstance(klass)
            
            # Call constructor if it exists
            if "constructor" in klass.methods:
                constructor = klass.methods["constructor"]
                if len(node.args) != len(constructor.params):
                    raise EvaluationError(
                        f"Constructor for '{node.class_name}' expects "
                        f"{len(constructor.params)} arguments, got {len(node.args)}"
                    )
                arg_values = [self.evaluate(arg) for arg in node.args]
                scope = dict(constructor.closure)
                scope["this"] = instance
                for param, arg_val in zip(constructor.params, arg_values):
                    scope[param] = arg_val
                call_evaluator = ASTEvaluator(variables=scope, parent=self)
                try:
                    call_evaluator.evaluate(constructor.body)
                except ReturnException:
                    pass
            
            return instance

        if isinstance(node, ThisNode):
            return self.get_variable("this")

        if isinstance(node, PropertyAccessNode):
            obj = self.evaluate(node.object)
            if not isinstance(obj, KolosInstance):
                raise EvaluationError(f"Cannot access property on non-object")
            if node.property in obj.fields:
                return obj.fields[node.property]
            raise EvaluationError(f"Object has no property '{node.property}'")

        if isinstance(node, MethodCallNode):
            obj = self.evaluate(node.obj)
            if not isinstance(obj, KolosInstance):
                raise EvaluationError(f"Cannot call method on non-object")
            
            if node.method not in obj.klass.methods:
                raise EvaluationError(
                    f"Class '{obj.klass.name}' has no method '{node.method}'"
                )
            
            method = obj.klass.methods[node.method]
            if len(node.args) != len(method.params):
                raise EvaluationError(
                    f"Method '{node.method}' expects {len(method.params)} arguments, "
                    f"got {len(node.args)}"
                )
            
            arg_values = [self.evaluate(arg) for arg in node.args]
            scope = dict(method.closure)
            scope["this"] = obj
            for param, arg_val in zip(method.params, arg_values):
                scope[param] = arg_val
            
            call_evaluator = ASTEvaluator(variables=scope, parent=self)
            try:
                return call_evaluator.evaluate(method.body)
            except ReturnException as ret:
                return ret.value

        raise EvaluationError(
            f"Unsupported AST node: {type(node).__name__}"
        )


def evaluate(node, variables=None):
    """Evaluate a Kolos AST node."""
    return ASTEvaluator(variables).evaluate(node)
