use oxc_allocator::{Allocator, Box, CloneIn};
use oxc_ast::{ast::*, visit::walk_mut::*, AstBuilder, VisitMut, NONE};
use oxc_span::SPAN;

use super::utils::get_line_number;

pub struct AstReplacer<'a> {
    source_text: String,
    ast_builder: AstBuilder<'a>,
    allocator: &'a Allocator
}

impl<'a> AstReplacer<'a> {
    pub fn new(allocator: &'a Allocator, source_text: String) -> Self {
        let ast_builder = AstBuilder::new(allocator);
        Self {
            ast_builder,
            source_text,
            allocator
        }
    }

    pub fn build(&mut self, program: &mut Program<'a>) {
        self.visit_program(program);
    }

    fn handle_conditional_expression(
        &mut self,
        it: &mut ExpressionStatement<'a>,
        expr_stmt: &ConditionalExpression,
        line: usize
    ) {
        let call_expr = self.create_debug_call(line, {
            let mut args = self.ast_builder.vec();
            args.push(Argument::from(
                self.ast_builder.expression_conditional(
                    SPAN,
                    expr_stmt.test.clone_in(&self.allocator),
                    expr_stmt.consequent.clone_in(&self.allocator),
                    expr_stmt.alternate.clone_in(&self.allocator)
                ),
            ));
            args
        });

        it.expression = Expression::CallExpression(Box::new_in(call_expr, &self.allocator));
    }

    fn handle_template_literal_expression(
        &mut self,
        it: &mut ExpressionStatement<'a>,
        expr_stmt: &TemplateLiteral,
        line: usize,
    ) {
        let call_expr = self.create_debug_call(line, {
            let mut args = self.ast_builder.vec();
            args.push(Argument::from(
                self.ast_builder.expression_template_literal(
                    SPAN,
                    expr_stmt.quasis.clone_in(&self.allocator),
                    expr_stmt.expressions.clone_in(&self.allocator),
                ),
            ));
            args
        });

        it.expression = Expression::CallExpression(Box::new_in(call_expr, &self.allocator));
    }

    fn handle_array_expression(
        &mut self,
        it: &mut ExpressionStatement<'a>,
        expr_stmt: &ArrayExpression<'a>,
        line: usize,
    ) {
        let call_expr = self.create_debug_call(line, {
            let mut args = self.ast_builder.vec();
            args.push(Argument::from(self.ast_builder.expression_array(
                SPAN,
                expr_stmt.elements.clone_in(&self.allocator),
                expr_stmt.trailing_comma.clone(),
            )));
            args
        });

        it.expression = Expression::CallExpression(Box::new_in(call_expr, &self.allocator));
    }

    fn handle_parenthesized_expression(
        &mut self,
        it: &mut ExpressionStatement<'a>,
        expr_stmt: &ParenthesizedExpression<'a>,
        line: usize,
    ) {
        let call_expr = self.create_debug_call(line, {
            let mut args = self.ast_builder.vec();
            args.push(Argument::from(self.ast_builder.expression_parenthesized(
                SPAN,
                expr_stmt.expression.clone_in(&self.allocator),
            )));
            args
        });

        it.expression = Expression::CallExpression(Box::new_in(call_expr, &self.allocator));
    }

    fn handle_binary_expression(
        &mut self,
        it: &mut ExpressionStatement<'a>,
        expr_stmt: &BinaryExpression<'a>,
        line: usize,
    ) {
        let call_expr = self.create_debug_call(line, {
            let mut args = self.ast_builder.vec();
            args.push(Argument::from(self.ast_builder.expression_binary(
                SPAN,
                expr_stmt.left.clone_in(&self.allocator),
                expr_stmt.operator.clone(),
                expr_stmt.right.clone_in(&self.allocator),
            )));
            args
        });

        it.expression = Expression::CallExpression(Box::new_in(call_expr, &self.allocator));
    }

    fn handle_logical_expression(
        &mut self,
        it: &mut ExpressionStatement<'a>,
        expr_stmt: &LogicalExpression<'a>,
        line: usize,
    ) {
        let call_expr = self.create_debug_call(line, {
            let mut args = self.ast_builder.vec();
            args.push(Argument::from(self.ast_builder.expression_logical(
                SPAN,
                expr_stmt.left.clone_in(&self.allocator),
                expr_stmt.operator.clone(),
                expr_stmt.right.clone_in(&self.allocator),
            )));
            args
        });

        it.expression = Expression::CallExpression(Box::new_in(call_expr, &self.allocator));
    }

    fn handle_boolean_literal(
        &mut self,
        it: &mut ExpressionStatement<'a>,
        expr_stmt: &BooleanLiteral,
        line: usize,
    ) {
        let call_expr = self.create_debug_call(line, {
            let mut args = self.ast_builder.vec();
            args.push(Argument::from(
                self.ast_builder
                    .expression_boolean_literal(SPAN, expr_stmt.value.clone()),
            ));
            args
        });

        it.expression = Expression::CallExpression(Box::new_in(call_expr, &self.allocator));
    }

    fn handle_string_literal(
        &mut self,
        it: &mut ExpressionStatement<'a>,
        expr_stmt: &StringLiteral<'a>,
        line: usize,
    ) {
        let call_expr = self.create_debug_call(line, {
            let mut args = self.ast_builder.vec();
            args.push(Argument::from(self.ast_builder.expression_string_literal(
                SPAN,
                expr_stmt.value.clone(),
                expr_stmt.raw.clone(),
            )));
            args
        });

        it.expression = Expression::CallExpression(Box::new_in(call_expr, &self.allocator));
    }

    fn handle_numeric_literal(
        &mut self,
        it: &mut ExpressionStatement<'a>,
        expr_stmt: &NumericLiteral<'a>,
        line: usize,
    ) {
        let call_expr = self.create_debug_call(line, {
            let mut args = self.ast_builder.vec();
            args.push(Argument::from(self.ast_builder.expression_numeric_literal(
                SPAN,
                expr_stmt.value,
                expr_stmt.raw,
                NumberBase::Decimal,
            )));
            args
        });

        it.expression = Expression::CallExpression(Box::new_in(call_expr, &self.allocator));
    }

    fn handle_identifier(
        &mut self,
        it: &mut ExpressionStatement<'a>,
        expr_stmt: &IdentifierReference<'a>,
        line: usize,
    ) {

        let call_expr = self.create_debug_call(line, {
            let mut args = self.ast_builder.vec();
            args.push(Argument::from(
                self.ast_builder
                    .expression_identifier_reference(SPAN, expr_stmt.name.clone()),
            ));
            args
        });
        it.expression = Expression::CallExpression(Box::new_in(call_expr, &self.allocator));
    }

    fn handle_static_member_expression(
        &mut self,
        it: &mut ExpressionStatement<'a>,
        expr_stmt: &StaticMemberExpression<'a>,
        line: usize,
    ) {
        let new_call = self.create_debug_call(line, {
            let mut items = self.ast_builder.vec();

            let mut current_object = expr_stmt;
            let mut full_expression = Vec::new();

            loop {
                full_expression.push(current_object.property.name.as_str());

                if let Expression::StaticMemberExpression(inner_expr) = &current_object.object {
                    current_object = inner_expr;
                } else if let Expression::Identifier(identifier) = &current_object.object {
                    full_expression.push(&identifier.name);
                    break;
                } else {
                    break;
                }
            }

            let full_expression_str = full_expression
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join(".");

            items.push(Argument::from(
                self.ast_builder
                    .expression_identifier_reference(SPAN, full_expression_str),
            ));

            items
        });

        it.expression = Expression::CallExpression(Box::new_in(new_call, &self.allocator));
    }

    fn handle_call_expression(
        &mut self,
        it: &mut ExpressionStatement<'a>,
        call_expr: &CallExpression<'a>,
        line: usize,
    ) {
        // if let Expression::Identifier(identifier) = &call_expr.callee {
        //     if identifier.name == "setTimeout" {
        //         return;
        //     }
        // }
        let new_expr =
            self.create_debug_call(line, {
                let mut items = self.ast_builder.vec();

                if let Expression::StaticMemberExpression(static_member) = &call_expr.callee {
                    let object = &static_member.object;

                    if let Expression::Identifier(identifier) = object {
        
                        if identifier.name == "console" {
                            
                            let mut new_arguments = self.ast_builder.vec();

                            for arg in &call_expr.arguments {
                                new_arguments.push(arg.clone_in(&self.allocator));
                            }
            
                            let new_expr = self.create_debug_call(line, new_arguments);
            
                            it.expression = Expression::CallExpression(Box::new_in(new_expr, &self.allocator));

                            return;
                        }
                        

                    }
                }

                items.push(Argument::from(
                    self.ast_builder.expression_call(
                        SPAN,
                        call_expr.callee.clone_in(&self.allocator),
                        call_expr.type_parameters.clone_in(&self.allocator),
                        call_expr.arguments.clone_in(&self.allocator),
                        call_expr.optional
                    )
                ));

                items
            });

        it.expression = Expression::CallExpression(Box::new_in(new_expr, &self.allocator));
    }

    fn create_debug_call(
        &self,
        line: usize,
        additional_args: oxc_allocator::Vec<'a, Argument<'a>>,
    ) -> CallExpression<'a> {
        let mut args = self.ast_builder.vec();

        args.push(Argument::from(self.ast_builder.expression_numeric_literal(
            SPAN,
            line as f64,
            line.to_string(),
            NumberBase::Decimal,
        )));
        for arg in additional_args {
            args.push(Argument::from(arg));
        }

        self.ast_builder.call_expression(
            SPAN,
            self.ast_builder
                .expression_identifier_reference(SPAN, "Xtal"),
            NONE,
            args,
            false,
        )
    }
}

impl<'a> VisitMut<'a> for AstReplacer<'a> {
    fn visit_expression_statement(&mut self, it: &mut ExpressionStatement<'a>) {
        let line = get_line_number(&self.source_text, it.span.start.try_into().unwrap());
        let expression = &it.expression.clone_in(&self.allocator);

        match expression {
            Expression::ConditionalExpression(expr) => self.handle_conditional_expression(it, expr, line),
            Expression::TemplateLiteral(expr) => {
                self.handle_template_literal_expression(it, expr, line)
            }
            Expression::ArrayExpression(expr) => self.handle_array_expression(it, expr, line),
            Expression::ParenthesizedExpression(expr) => {
                self.handle_parenthesized_expression(it, expr, line)
            }
            Expression::BinaryExpression(expr) => self.handle_binary_expression(it, expr, line),
            Expression::LogicalExpression(expr) => self.handle_logical_expression(it, expr, line),
            Expression::StaticMemberExpression(expr) => {
                self.handle_static_member_expression(it, expr, line)
            }
            Expression::BooleanLiteral(expr) => self.handle_boolean_literal(it, expr, line),
            Expression::StringLiteral(expr) => self.handle_string_literal(it, expr, line),
            Expression::NumericLiteral(expr) => self.handle_numeric_literal(it, expr, line),
            Expression::Identifier(expr) => self.handle_identifier(it, expr, line),
            Expression::CallExpression(expr) => self.handle_call_expression(it, expr, line),
            _ => {}
        }

        if let Expression::CallExpression(call_expr) = &it.expression {
            if let Expression::Identifier(identifier) = &call_expr.callee {
                if identifier.name == "Xtal" {
                    // Already wrapped, no need to wrap again
                    return;
                }
            }
        }

        walk_expression_statement(self, it);
    }
}
