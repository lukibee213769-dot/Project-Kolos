"""Tests for Kolos class support."""

import pytest
from compilers.parser import parse
from runtime.proto_vm.ast_evaluator import evaluate, EvaluationError, KolosInstance


class TestClassDefinition:
    """Test class definition and instantiation."""

    def test_simple_class(self):
        """Test creating a simple class with a method."""
        code = '''
        class Dog {
            fn bark() {
                print "Woof!";
            }
        }
        '''
        # Should not raise
        result = evaluate(parse(code))

    def test_class_with_constructor(self):
        """Test class with constructor."""
        code = '''
        class Dog {
            fn constructor(name) {
                this.name = name;
            }
            
            fn bark() {
                return this.name;
            }
        }
        
        let d = new Dog("Buddy");
        d.bark()
        '''
        result = evaluate(parse(code))
        assert result == "Buddy"

    def test_instance_creation(self):
        """Test creating an instance of a class."""
        code = '''
        class Point {
            fn constructor(x, y) {
                this.x = x;
                this.y = y;
            }
        }
        
        let p = new Point(3, 4);
        p
        '''
        result = evaluate(parse(code))
        # Should return a KolosInstance
        assert isinstance(result, KolosInstance)

    def test_method_call_with_args(self):
        """Test calling a method with arguments."""
        code = '''
        class Calculator {
            fn add(a, b) {
                return a + b;
            }
        }
        
        let calc = new Calculator();
        calc.add(2, 3)
        '''
        result = evaluate(parse(code))
        assert result == 5

    def test_multiple_methods(self):
        """Test class with multiple methods."""
        code = '''
        class Math {
            fn multiply(a, b) {
                return a * b;
            }
            
            fn square(x) {
                return this.multiply(x, x);
            }
        }
        
        let m = new Math();
        m.square(4)
        '''
        result = evaluate(parse(code))
        assert result == 16

    def test_object_property_access(self):
        """Test accessing object properties."""
        code = '''
        class Person {
            fn constructor(age) {
                this.age = age;
            }
        }
        
        let p = new Person(25);
        p.age
        '''
        result = evaluate(parse(code))
        assert result == 25

    def test_method_with_this_reference(self):
        """Test using this in methods."""
        code = '''
        class Circle {
            fn constructor(radius) {
                this.radius = radius;
            }
            
            fn area() {
                return 3.14 * this.radius ** 2;
            }
        }
        
        let c = new Circle(5);
        c.area()
        '''
        result = evaluate(parse(code))
        assert abs(result - 78.5) < 0.1


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
