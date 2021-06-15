#![allow(dead_code)]

use crate::ast::*;
use crate::for_impl::{discriminant, Rc};
use crate::lexer::*;
use tablam::prelude::*;

const END_OF_ROW: [Option<Token>; 2] = [Some(Token::RowSeparator), Some(Token::EndVector)];

pub struct Parser<'source> {
    scanner: Scanner<'source>,
}

impl<'source> Parser<'source> {
    pub fn new(buffer: &'source str) -> Self {
        Parser {
            scanner: Scanner::new(buffer),
        }
    }

    fn accept(&mut self) -> Option<(Token, TokenData)> {
        self.scanner.accept()
    }

    fn peek(&mut self) -> Option<Token> {
        self.scanner.peek()
    }

    pub fn peek_both(&mut self) -> Option<(Token, TokenData)> {
        self.scanner.peek_both()
    }

    pub fn parse(&mut self) -> Return {
        let mut lines = Vec::new();
        let line = self.parse_ast(0)?;
        //dbg!(&line);
        let mut is_eof = line.is_eof();
        lines.push(line);

        while !is_eof {
            let line = self.parse_ast(0)?;
            //dbg!(&line);

            is_eof = line.is_eof();
            if !is_eof {
                lines.push(line);
            }
        }

        Ok(Expression::Block(lines.into()))
    }

    fn check_next_token(&mut self, expected: Token) -> std::result::Result<Token, ErrorLang> {
        if let Some((found, data)) = self.peek_both() {
            //dbg!(&found);
            return if discriminant(&found) == discriminant(&expected) {
                self.accept();
                Ok(found)
            } else {
                let feedback = ErrorLang::Unexpected(found, expected, data);
                Err(feedback)
            };
        }

        Err(ErrorLang::Eof)
    }

    fn check_and_accept_next(&mut self, expected: Token) -> std::result::Result<Token, ErrorLang> {
        let result = self.check_next_token(expected);
        match result {
            Ok(token) => Ok(token),
            Err(error) => {
                self.accept();
                Err(error)
            }
        }
    }

    fn match_at_least_one(
        &mut self,
        conditions: Vec<Token>,
    ) -> std::result::Result<Token, ErrorLang> {
        let mut result = Err(ErrorLang::Eof);
        for expected in conditions {
            result = self.check_next_token(expected);
            match &result {
                Err(_) => continue,
                Ok(_) => return result,
            }
        }

        result
    }

    fn continue_until_expression(&mut self, conditions: Vec<Token>) -> Return {
        let mut result;
        for expected in conditions {
            result = self.check_next_token(expected);
            match result {
                Err(error) => return Err(error),
                Ok(_) => continue,
            }
        }

        self.parse_ast(0)
    }

    fn prefix_binding_power(token: &Token) -> ((), u8) {
        match token {
            Token::Minus => ((), 11),
            Token::Not => ((), 12),
            _ => panic!("bad op: {:?}", token),
        }
    }

    fn postfix_binding_power(token: &Token) -> Option<(u8, ())> {
        let res = match token {
            Token::LeftParentheses => (13, ()),
            _ => return None,
        };
        Some(res)
    }

    fn infix_binding_power(token: &Token) -> Option<(u8, u8)> {
        let res = match token {
            Token::Or => (1, 2),
            Token::And => (3, 4),
            Token::Equal
            | Token::NotEqual
            | Token::Greater
            | Token::GreaterEqual
            | Token::Less
            | Token::LessEqual => (6, 5),
            Token::Plus | Token::Minus => (7, 8),
            Token::Multiplication | Token::Division => (9, 10),
            _ => return None,
        };
        Some(res)
    }

    fn parse_lhs(&mut self, op: &Token) -> Return {
        let expr = match op {
            Token::True => Expression::Value(Scalar::Bool(true)),
            Token::False => Expression::Value(Scalar::Bool(false)),
            Token::Integer(number) => Expression::Value(Scalar::I64(*number)),
            Token::Float(number) => Expression::Value(Scalar::F64(*number)),
            Token::Decimal(decimal) => Expression::Value(Scalar::Decimal(*decimal)),
            Token::String(text) => Expression::Value(Scalar::Utf8(Rc::new(text.into()))),
            Token::Var => self.parse_var()?,
            Token::Let => self.parse_let()?,
            Token::Variable(name) => {
                match self.peek() {
                    //Check if is a function name...
                    Some(Token::LeftParentheses) => self.parse_function_call(&name)?,
                    Some(Token::TypeDefiner) => self.parse_parameter_definition(&name)?,
                    Some(Token::Select)
                    | Some(Token::Where)
                    | Some(Token::Limit)
                    | Some(Token::Skip)
                    | Some(Token::Distinct)
                    | Some(Token::Deselect) => self.parse_query(name.into())?,
                    Some(Token::Assignment) => self.parse_var_assignment(name)?,
                    _ => Expression::Variable(name.into()),
                }
            }
            Token::StartVector => self.parse_vector()?,
            Token::Column(name) => Expression::Column(Column::Name(name.into())),
            Token::IndexedColumn(position) => Column::Pos(*position).into(),
            Token::AliasedColumn(alias) => {
                Column::Alias(Box::new(ColumnAlias::rename_name(&alias.from, &alias.to))).into()
            }
            Token::If => self.parse_if()?,
            Token::While => self.parse_where()?,
            token => return Err(ErrorLang::Unimplemented(token.clone())),
        };
        Ok(expr)
    }

    fn parse_ast(&mut self, min_bindpower: u8) -> Return {
        let op = self.accept();
        //dbg!(&op);
        let mut lhs = if let Some((op, _)) = op {
            self.parse_lhs(&op)?
        } else {
            return Ok(Expression::Eof);
        };

        //dbg!(&lhs);
        while let Some((token, data)) = self.peek_both() {
            //dbg!(&token);

            if let Some((l_bp, ())) = Self::postfix_binding_power(&token) {
                if l_bp < min_bindpower {
                    break;
                }
                let token = self.accept();

                lhs = match token {
                    Some((Token::LeftParentheses, _)) => {
                        let rhs = self.parse_ast(0)?;
                        if let Some(Token::RightParentheses) = self.peek() {
                            Expression::Block(vec![lhs, rhs].into())
                        } else {
                            return Err(ErrorLang::UnclosedGroup(Token::LeftParentheses, data));
                        }
                    }
                    _ => continue,
                };
            }

            if let Some((l_bp, r_bp)) = Self::infix_binding_power(&token) {
                if l_bp < min_bindpower {
                    break;
                }
                self.accept();
                let rhs = self.parse_ast(r_bp)?;

                if token.is_binary_operator() {
                    lhs = Expression::BinaryOp(BinaryOperation::new(
                        token,
                        Box::new(lhs),
                        Box::new(rhs),
                    ));
                    continue;
                }

                if lhs.is_indexed_column() || rhs.is_indexed_column() {
                    lhs = Expression::create_bool_condition_qry(token, lhs, rhs);
                    continue;
                }

                if token.is_comparison_operator() {
                    lhs = Expression::ComparisonOp(ComparisonOperation::new(
                        token,
                        Box::new(lhs),
                        Box::new(rhs),
                    ));
                    continue;
                }

                lhs = Expression::Block(vec![lhs, rhs].into());
                continue;
            }

            break;
        }

        Ok(lhs)
    }

    fn parse_var(&mut self) -> Return {
        let result = self.check_next_token(Token::Variable("".to_string()));
        if let Ok(Token::Variable(name)) = result {
            self.check_next_token(Token::Assignment)?;

            return Ok(Expression::Mutable(name, Box::new(self.parse_ast(0)?)));
        }

        Err(result.err().unwrap())
    }

    fn parse_let(&mut self) -> Return {
        let dummy = String::from("");
        let result =
            self.match_at_least_one(vec![Token::Variable(dummy.clone()), Token::Constant(dummy)]);
        if let Ok(Token::Variable(name)) | Ok(Token::Constant(name)) = result {
            self.check_next_token(Token::Assignment)?;

            return Ok(Expression::Immutable(name, Box::new(self.parse_ast(0)?)));
        }

        Err(result.err().unwrap())
    }

    fn parse_var_assignment(&mut self, name: &str) -> Return {
        self.accept();
        Ok(Expression::Mutable(
            name.to_string(),
            Box::new(self.parse_ast(0)?),
        ))
    }

    fn parse_parameter_definition(&mut self, name: &str) -> Return {
        self.accept();
        let result = self.check_and_accept_next(Token::Type(String::from("")));
        if let Ok(Token::Type(type_param)) = result {
            return Ok(Expression::ParameterDefinition(Field::from_str(
                name,
                type_param.as_str(),
            )));
        }

        Err(result.err().unwrap())
    }

    fn parse_param_call(&mut self) -> std::result::Result<Option<ParamCall>, ErrorLang> {
        if let Some((token, _data)) = self.accept() {
            //dbg!(&token);
            if token.is_literal_or_value() {
                let expr = self.parse_ast(0)?;
                Ok(Some(ParamCall::new("", expr)))
            } else {
                Err(ErrorLang::Eof)
            }
        } else {
            Err(ErrorLang::Eof)
        }
    }

    fn parse_function_call(&mut self, name: &str) -> Return {
        //Eat '('
        self.accept();
        let mut params = Vec::new();
        loop {
            if let Some(Token::RightParentheses) = self.peek() {
                self.accept();
                break;
            }

            let expr = self.parse_ast(0)?;
            //dbg!(&expr);

            params.push(ParamCall::new("", expr));

            if Token::RightParentheses
                == self.match_at_least_one(vec![Token::Separator, Token::RightParentheses])?
            {
                break;
            };
        }

        Ok(FunctionCall::new(name, &params).into())
    }

    fn _check_field_count(
        &mut self,
        line_pos: &mut usize,
        field_count: &mut usize,
        expected: usize,
    ) -> Option<ErrorLang> {
        if END_OF_ROW.contains(&self.peek()) {
            if *field_count != expected {
                return Some(ErrorLang::FieldCountNotMatch(
                    expected,
                    *line_pos,
                    *field_count,
                ));
            }
            *line_pos += 1;
            *field_count = 0;
        }
        None
    }

    fn parse_vector(&mut self) -> Return {
        let mut fields = Vec::<Field>::new();
        let mut data = Vec::<Scalar>::new();
        let mut is_header = true;
        let mut field_count = 0usize;
        let mut line_pos = 0usize;
        loop {
            //if empty vector or ends in ; or ,
            if let Some(Token::EndVector) = self.peek() {
                if let Some(err) =
                    self._check_field_count(&mut line_pos, &mut field_count, fields.len())
                {
                    return Err(err);
                }
                self.accept();
                break;
            }
            let cell = self.parse_ast(0)?;

            if is_header {
                //I'm parsing a header definition name:kind,... or infering from values [1,2..]
                match cell.clone() {
                    Expression::ParameterDefinition(field) => fields.push(field.into()),
                    Expression::Value(scalar) => {
                        fields.push(Field::new_owned(
                            format!("col{}", fields.len()),
                            scalar.kind(),
                        ));
                        data.push(scalar)
                    }
                    _ => return Err(ErrorLang::UnexpectedItem(cell)),
                };
                if END_OF_ROW.contains(&self.peek()) {
                    is_header = false;
                    line_pos += 1;
                }
            } else {
                match cell.clone() {
                    Expression::Value(scalar) => data.push(scalar),
                    _ => return Err(ErrorLang::UnexpectedItem(cell)),
                }
                field_count += 1;
                if let Some(err) =
                    self._check_field_count(&mut line_pos, &mut field_count, fields.len())
                {
                    return Err(err);
                }
            }

            if Token::EndVector
                == self.match_at_least_one(vec![
                    Token::Separator,
                    Token::RowSeparator,
                    Token::EndVector,
                ])?
            {
                break;
            };
        }
        dbg!(&data, &fields);

        let vector = if data.is_empty() {
            Vector::new_empty(DataType::Any)
        } else {
            if fields.len() == 1 {
                let kind = data.first().expect("empty vector").kind();
                Vector::new_vector(data, kind)
            } else {
                let schema = Schema::new(fields, None);
                Vector::new_table(data, schema)
            }
        };

        dbg!(&vector);
        Ok(Expression::Value(Scalar::Vector(Rc::new(vector))))
    }

    fn parse_query(&mut self, identifier: Identifier) -> Return {
        let collection = Expression::Variable(identifier);
        let mut operations = QueryOperation::new(
            collection,
            QueryOp::new(Schema::new_single("", DataType::Any)),
        );
        loop {
            operations = match self.peek() {
                Some(Token::Select) => {
                    self.accept();
                    self.parse_select_qry(operations)?
                }
                Some(Token::Deselect) => {
                    self.accept();
                    self.parse_deselect_qry(operations)?
                }
                Some(Token::Where) => {
                    self.accept();
                    self.parse_where_qry(operations)?
                }
                Some(Token::Limit) => {
                    self.accept();
                    self.parse_limit_qry(operations)?
                }
                Some(Token::Skip) => {
                    self.accept();
                    self.parse_skip_qry(operations)?
                }
                Some(Token::Distinct) => {
                    self.accept();
                    self.parse_distinct_qry(operations)
                }
                _ => break,
            }
        }

        Ok(Expression::QueryOperation(operations))
    }

    fn parse_distinct_qry(&mut self, operations: QueryOperation) -> QueryOperation {
        operations.distinct()
    }

    fn parse_skip_qry(&mut self, operations: QueryOperation) -> ReturnT<QueryOperation> {
        let offset = self.check_next_token(Token::Integer(164))?;

        Ok(operations.skip(&offset))
    }

    fn parse_limit_qry(&mut self, operations: QueryOperation) -> ReturnT<QueryOperation> {
        let offset = self.check_next_token(Token::Integer(164))?;

        Ok(operations.limit(&offset))
    }

    fn parse_where_qry(&mut self, operations: QueryOperation) -> ReturnT<QueryOperation> {
        let expression = self.parse_ast(0)?;
        let (operator, left, right) = match expression {
            Expression::BoolConditionQry(operator, lhs, rhs) => (operator, lhs, rhs),
            _ => {
                return Err(ErrorLang::Query(String::from(
                    "You must indicate at least one condition.",
                )))
            }
        };

        Ok(operations.filter(&operator, left, right))
    }

    fn parse_deselect_qry(&mut self, operations: QueryOperation) -> ReturnT<QueryOperation> {
        let mut columns = Vec::<Column>::new();
        loop {
            let column = self.parse_ast(0)?;

            match column {
                Expression::Column(column) => columns.push(column),
                _ => return Err(ErrorLang::UnexpectedItem(column)),
            };

            if self.check_next_token(Token::Separator).is_ok() {
                continue;
            }
            break;
        }

        if columns.is_empty() {
            return Err(ErrorLang::Query(String::from(
                "You must indicate at least one column.",
            )));
        }

        Ok(operations.deselect(columns))
    }

    fn parse_select_qry(&mut self, operations: QueryOperation) -> ReturnT<QueryOperation> {
        let mut columns = Vec::<Column>::new();
        loop {
            let column = self.parse_ast(0)?;

            match column {
                Expression::Column(column) => columns.push(column),
                _ => return Err(ErrorLang::UnexpectedItem(column)),
            };

            if self.check_next_token(Token::Separator).is_ok() {
                continue;
            }
            break;
        }

        if columns.is_empty() {
            return Err(ErrorLang::Query(String::from(
                "You must indicate at least one column.",
            )));
        }

        Ok(operations.select(columns))
    }

    fn parse_bool_op(&mut self) -> ReturnT<BoolOperation> {
        if let Some(expr) = self.peek() {
            //dbg!(&expr);
            let op = match expr {
                Token::True => BoolOperation::Bool(true),
                Token::False => BoolOperation::Bool(false),
                Token::Variable(name) => BoolOperation::Var(name),
                _ => return Err(ErrorLang::ExpectedBoolOp(expr)),
            };
            self.accept();
            Ok(op)
        } else {
            Err(ErrorLang::ExpectedBoolOp(Token::Error))
        }
    }

    fn parse_if(&mut self) -> Return {
        let op = self.parse_bool_op()?;

        self.check_and_accept_next(Token::Start)?;

        let mut if_true = Vec::new();
        let mut if_else = Vec::new();
        let mut is_else = false;
        while let Some(t) = self.peek() {
            if t == Token::Else {
                is_else = true;
                self.accept();
                continue;
            }
            if t == Token::End {
                break;
            }
            if is_else {
                if_else.push(self.parse_ast(0)?);
            } else {
                if_true.push(self.parse_ast(0)?);
            }
        }

        if self.peek() == Some(Token::End) {
            self.accept();
        } else {
            return Err(ErrorLang::Eof);
        }

        Ok(Expression::If(
            Box::new(op),
            Box::new(Expression::Block(if_true.into())),
            Box::new(Expression::Block(if_else.into())),
        ))
    }

    fn parse_where(&mut self) -> Return {
        let op = self.parse_bool_op()?;

        self.check_and_accept_next(Token::Start)?;

        let mut block = Vec::new();
        while let Some(t) = self.peek() {
            if t == Token::End {
                break;
            }
            block.push(self.parse_ast(0)?);
        }

        if self.peek() == Some(Token::End) {
            self.accept();
        } else {
            return Err(ErrorLang::Eof);
        }

        Ok(Expression::While(
            Box::new(op),
            Box::new(Expression::Block(block.into())),
        ))
    }
}
