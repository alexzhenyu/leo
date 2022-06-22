// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{access::*, Node};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// An access expressions, extracting a smaller part out of a whole.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessExpression {
    // /// An `array[index]` expression.
    // Array(ArrayAccess),
    // /// An expression accessing a range of an array.
    // ArrayRange(ArrayRangeAccess),
    /// An expression accessing a field in a structure, e.g., `circuit_var.field`.
    Member(MemberAccess),
    // /// Access to a tuple field using its position, e.g., `tuple.1`.
    // Tuple(TupleAccess),
    /// Access to an associated variable of a circuit.
    AssociatedVariable(AssociatedVariableAccess),
    /// Access to an associated function of a circuit.
    AssociatedFunction(AssociatedFunctionCall),
}

impl Node for AccessExpression {
    fn span(&self) -> Span {
        match self {
            AccessExpression::Member(n) => n.span(),
            AccessExpression::AssociatedVariable(n) => n.span(),
            AccessExpression::AssociatedFunction(n) => n.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            AccessExpression::Member(n) => n.set_span(span),
            AccessExpression::AssociatedVariable(n) => n.set_span(span),
            AccessExpression::AssociatedFunction(n) => n.set_span(span),
        }
    }
}

impl fmt::Display for AccessExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AccessExpression::*;

        match self {
            // Array(access) => access.fmt(f),
            // ArrayRange(access) => access.fmt(f),
            Member(access) => access.fmt(f),
            // Tuple(access) => access.fmt(f),
            AssociatedVariable(access) => access.fmt(f),
            AssociatedFunction(access) => access.fmt(f),
        }
    }
}
