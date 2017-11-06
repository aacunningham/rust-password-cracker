extern crate passcode_cracker;

use passcode_cracker::{Expression, Variable, Operator, BinaryExpression, BooleanOperator, Evaluable};

fn main() {
    let val_array = [3, 2, 3, 4];

    let x = Variable {
        name: 'a',
    };

    let y = Variable {
        name: 'b',
    };

    let z = BinaryExpression {
        l_value: Box::new(Expression::Variable(x)),
        operator: Operator::Boolean(BooleanOperator::Equal),
        r_value: Box::new(Expression::Variable(y)),
    };

    let ret = z.evaluate(&val_array);

    println!("{:?}", ret);
}

