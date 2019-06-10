use crate::parser::ast::*;
use crate::parser::error::ParseError;
use crate::parser::grammar::{BaikLexer, Rule};
use crate::parser::input_location::InputLocation;
use crate::parser::precedence;
use pest::iterators::{Pair, Pairs};
use pest::Parser;

#[derive(Debug, Clone, PartialEq)]
pub struct Term {
    location: InputLocation,
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    // Basic types
    Atom,
    Boolean,
    Float,
    Integer,
    String,
    Ty,

    // Compound types
    Array,
    Map,

    // Expressions.
    Binary,
    Constructor,
    Unary,

    Call,
    Declaration,
    Function,
    If,
    Local,
    MethodCall,
    PropertyGet,
    PropertySet,

    TypeDef,
    TraitDef,
    ImplDef,
    PublicMethod,
    PublicMethodSpec,
    PrivateMethod,
    StaticMethod,
    StaticMethodSpec,
}

#[derive(Debug, Clone, PartialEq)]
enum Inner {
    Array(Vec<Term>),
    Atom(Atom),
    Binary(Binary, Box<Term>, Box<Term>),
    Boolean(Boolean),
    Call(Box<Term>, Vec<Term>),
    Constructor(Ty, Vec<(Identifier, Term)>),
    Declaration(Identifier, Box<Term>),
    Float(Float),
    Function(Function),
    If(Box<Term>, Vec<Term>, Vec<Term>),
    ImplDef(Ty, Vec<Term>),
    Integer(Integer),
    Local(Local),
    Map(Vec<(Term, Term)>),
    MethodCall(Box<Term>, Identifier, Vec<Term>),
    PrivateMethod(Identifier, Vec<(Identifier, TypeSpec)>, Vec<Term>),
    PropertyGet(Identifier),
    PropertySet(Vec<(Identifier, Term)>),
    PublicMethod(Identifier, Vec<(Identifier, TypeSpec)>, Vec<Term>),
    PublicMethodSpec(Identifier, Vec<(Identifier, TypeSpec)>, Option<TypeSpec>),
    StaticMethod(Identifier, Vec<(Identifier, TypeSpec)>, Vec<Term>),
    StaticMethodSpec(Identifier, Vec<(Identifier, TypeSpec)>, Option<TypeSpec>),
    String(String),
    TraitDef(Ty, Option<TypeSpec>, Vec<Term>),
    Ty(Ty),
    TypeDef(Ty, Vec<(Identifier, TypeSpec)>, Vec<Term>),
    Unary(Unary, Box<Term>),
}

impl Term {
    pub fn input(source: &str) -> Result<Vec<Term>, ParseError> {
        let result = BaikLexer::parse(Rule::input, source);
        match result {
            Ok(pairs) => {
                let terms: Vec<Term> = pairs
                    .take_while(|pair| pair.as_rule() != Rule::EOI)
                    .map(Term::from)
                    .collect();
                Ok(terms)
            }
            Err(err) => Err(ParseError::from(err)),
        }
    }

    pub fn file(source: &str) -> Result<Vec<Term>, ParseError> {
        let result = BaikLexer::parse(Rule::file, source);
        match result {
            Ok(pairs) => {
                let terms: Vec<Term> = pairs
                    .take_while(|pair| pair.as_rule() != Rule::EOI)
                    .map(Term::from)
                    .collect();
                Ok(terms)
            }
            Err(err) => Err(ParseError::from(err)),
        }
    }

    pub fn create_infix(op: Binary, lhs: Term, rhs: Term) -> Term {
        Term {
            location: op.location().clone(),
            inner: Inner::Binary(op, Box::new(lhs), Box::new(rhs)),
        }
    }

    pub fn node_type(&self) -> NodeType {
        match self.inner {
            Inner::Array(..) => NodeType::Array,
            Inner::Atom(..) => NodeType::Atom,
            Inner::Boolean(..) => NodeType::Boolean,
            Inner::Binary(..) => NodeType::Binary,
            Inner::Call(..) => NodeType::Call,
            Inner::Ty(..) => NodeType::Ty,
            Inner::Constructor(..) => NodeType::Constructor,
            Inner::Declaration(..) => NodeType::Declaration,
            Inner::Function(..) => NodeType::Function,
            Inner::Float(..) => NodeType::Float,
            Inner::If(..) => NodeType::If,
            Inner::Integer(..) => NodeType::Integer,
            Inner::Local(..) => NodeType::Local,
            Inner::Map(..) => NodeType::Map,
            Inner::MethodCall(..) => NodeType::MethodCall,
            Inner::PropertyGet(..) => NodeType::PropertyGet,
            Inner::PropertySet(..) => NodeType::PropertySet,
            Inner::String(..) => NodeType::String,
            Inner::Unary(..) => NodeType::Unary,
            Inner::TypeDef(..) => NodeType::TypeDef,
            Inner::TraitDef(..) => NodeType::TraitDef,
            Inner::ImplDef(..) => NodeType::ImplDef,
            Inner::PublicMethod(..) => NodeType::PublicMethod,
            Inner::PublicMethodSpec(..) => NodeType::PublicMethodSpec,
            Inner::PrivateMethod(..) => NodeType::PrivateMethod,
            Inner::StaticMethod(..) => NodeType::StaticMethod,
            Inner::StaticMethodSpec(..) => NodeType::StaticMethodSpec,
        }
    }

    pub fn array(&self) -> Option<&Vec<Term>> {
        match self.inner {
            Inner::Array(ref terms) => Some(terms),
            _ => None,
        }
    }

    pub fn atom(&self) -> Option<&Atom> {
        match self.inner {
            Inner::Atom(ref node) => Some(node),
            _ => None,
        }
    }

    pub fn boolean(&self) -> Option<&Boolean> {
        match self.inner {
            Inner::Boolean(ref node) => Some(node),
            _ => None,
        }
    }

    pub fn binary(&self) -> Option<(&Binary, &Term, &Term)> {
        match self.inner {
            Inner::Binary(ref op, ref lhs, ref rhs) => Some((op, lhs, rhs)),
            _ => None,
        }
    }

    pub fn call(&self) -> Option<(&Term, &Vec<Term>)> {
        match self.inner {
            Inner::Call(ref callee, ref args) => Some((callee, args)),
            _ => None,
        }
    }

    pub fn constructor(&self) -> Option<(&Ty, &Vec<(Identifier, Term)>)> {
        match self.inner {
            Inner::Constructor(ref ty, ref properties) => Some((ty, properties)),
            _ => None,
        }
    }

    pub fn declaration(&self) -> Option<(&Identifier, &Term)> {
        match self.inner {
            Inner::Declaration(ref name, ref value) => Some((name, value)),
            _ => None,
        }
    }

    pub fn float(&self) -> Option<&Float> {
        match self.inner {
            Inner::Float(ref node) => Some(node),
            _ => None,
        }
    }

    pub fn function(&self) -> Option<&Function> {
        match self.inner {
            Inner::Function(ref function) => Some(function),
            _ => None,
        }
    }

    pub fn if_expr(&self) -> Option<(&Term, &Vec<Term>, &Vec<Term>)> {
        match self.inner {
            Inner::If(ref test, ref positives, ref negatives) => Some((test, positives, negatives)),
            _ => None,
        }
    }

    pub fn impldef(&self) -> Option<(&Ty, &Vec<Term>)> {
        match self.inner {
            Inner::ImplDef(ref ty, ref body) => Some((ty, body)),
            _ => None,
        }
    }

    pub fn integer(&self) -> Option<&Integer> {
        match self.inner {
            Inner::Integer(ref node) => Some(node),
            _ => None,
        }
    }

    pub fn local(&self) -> Option<&Local> {
        match self.inner {
            Inner::Local(ref node) => Some(node),
            _ => None,
        }
    }

    pub fn map(&self) -> Option<&Vec<(Term, Term)>> {
        match self.inner {
            Inner::Map(ref map) => Some(map),
            _ => None,
        }
    }

    pub fn method_call(&self) -> Option<(&Term, &Identifier, &Vec<Term>)> {
        match self.inner {
            Inner::MethodCall(ref callee, ref method_name, ref arguments) => {
                Some((callee, method_name, arguments))
            }
            _ => None,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn private_method(
        &self,
    ) -> Option<(&Identifier, &Vec<(Identifier, TypeSpec)>, &Vec<Term>)> {
        match self.inner {
            Inner::PrivateMethod(ref name, ref args, ref body) => Some((name, args, body)),
            _ => None,
        }
    }

    pub fn property_get(&self) -> Option<&Identifier> {
        match self.inner {
            Inner::PropertyGet(ref name) => Some(name),
            _ => None,
        }
    }

    pub fn property_set(&self) -> Option<&Vec<(Identifier, Term)>> {
        match self.inner {
            Inner::PropertySet(ref values) => Some(values),
            _ => None,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn public_method(&self) -> Option<(&Identifier, &Vec<(Identifier, TypeSpec)>, &Vec<Term>)> {
        match self.inner {
            Inner::PublicMethod(ref name, ref args, ref body) => Some((name, args, body)),
            _ => None,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn public_method_spec(
        &self,
    ) -> Option<(&Identifier, &Vec<(Identifier, TypeSpec)>, Option<&TypeSpec>)> {
        match self.inner {
            Inner::PublicMethodSpec(ref name, ref args, ref rval) => {
                Some((name, args, rval.as_ref()))
            }
            _ => None,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn static_method(&self) -> Option<(&Identifier, &Vec<(Identifier, TypeSpec)>, &Vec<Term>)> {
        match self.inner {
            Inner::StaticMethod(ref name, ref args, ref body) => Some((name, args, body)),
            _ => None,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn static_method_spec(
        &self,
    ) -> Option<(&Identifier, &Vec<(Identifier, TypeSpec)>, Option<&TypeSpec>)> {
        match self.inner {
            Inner::StaticMethodSpec(ref name, ref args, ref rval) => {
                Some((name, args, rval.as_ref()))
            }
            _ => None,
        }
    }

    pub fn string(&self) -> Option<&String> {
        match self.inner {
            Inner::String(ref node) => Some(node),
            _ => None,
        }
    }

    pub fn traitdef(&self) -> Option<(&Ty, Option<&TypeSpec>, &Vec<Term>)> {
        match self.inner {
            Inner::TraitDef(ref ty, ref reqs, ref body) => Some((ty, reqs.as_ref(), body)),
            _ => None,
        }
    }

    pub fn ty(&self) -> Option<&Ty> {
        match self.inner {
            Inner::Ty(ref node) => Some(node),
            _ => None,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn typedef(&self) -> Option<(&Ty, &Vec<(Identifier, TypeSpec)>, &Vec<Term>)> {
        match self.inner {
            Inner::TypeDef(ref ty, ref props, ref body) => Some((ty, props, body)),
            _ => None,
        }
    }

    pub fn unary(&self) -> Option<(&Unary, &Term)> {
        match self.inner {
            Inner::Unary(ref op, ref rhs) => Some((op, rhs)),
            _ => None,
        }
    }
}

impl Location for Term {
    fn location(&self) -> &InputLocation {
        &self.location
    }
}

impl<'a> From<Pair<'a, Rule>> for Term {
    fn from(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::array => {
                let mut result = Vec::new();

                for element in pair.clone().into_inner() {
                    result.push(Term::from(element));
                }

                Term {
                    location: InputLocation::from(pair),
                    inner: Inner::Array(result),
                }
            }
            Rule::atom => Term {
                location: InputLocation::from(pair.clone()),
                inner: Inner::Atom(Atom::from(pair)),
            },
            Rule::boolean => Term {
                location: InputLocation::from(pair.clone()),
                inner: Inner::Boolean(Boolean::from(pair)),
            },
            Rule::call_local => {
                let mut inner = pair.clone().into_inner();
                let callee = Term::from(inner.next().unwrap());
                let mut arguments = Vec::new();
                for argument in inner {
                    match argument.as_rule() {
                        Rule::call_argument => {
                            arguments.push(Term::from(argument.into_inner().next().unwrap()))
                        }
                        _ => unreachable!("Expected call argument but found {:?}", argument),
                    }
                }
                Term {
                    location: InputLocation::from(pair),
                    inner: Inner::Call(Box::new(callee), arguments),
                }
            }
            Rule::call_method => {
                let mut inner = pair.clone().into_inner();
                let callee = Term::from(inner.next().unwrap());
                unroll_method_call(callee, inner)
            }
            Rule::typename => Term {
                location: InputLocation::from(pair.clone()),
                inner: Inner::Ty(Ty::from(pair)),
            },
            Rule::constructor => {
                let mut inner = pair.clone().into_inner();
                let ty = Ty::from(inner.next().unwrap());
                let mut properties = Vec::new();

                for property in inner {
                    match property.as_rule() {
                        Rule::constructor_property => {
                            let mut proppairs = property.into_inner();
                            let name = Identifier::from(proppairs.next().unwrap());
                            let value = Term::from(proppairs.next().unwrap());
                            properties.push((name, value));
                        }
                        _ => unreachable!("Expected constructor property but found {:?}", property),
                    }
                }

                Term {
                    location: InputLocation::from(pair),
                    inner: Inner::Constructor(ty, properties),
                }
            }
            Rule::declaration => {
                let mut inner = pair.clone().into_inner();
                let identifier = Identifier::from(inner.next().unwrap());
                let assign = inner.next().unwrap();
                assert_eq!(assign.as_rule(), Rule::assign);
                let value = Term::from(inner.next().unwrap());
                Term {
                    location: InputLocation::from(pair),
                    inner: Inner::Declaration(identifier, Box::new(value)),
                }
            }
            Rule::defprivatemethod => {
                let mut inner = pair.clone().into_inner();
                let name = Identifier::from(inner.next().unwrap());
                let args = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|p| {
                        let mut inner = p.into_inner();
                        let keyword = Identifier::from(inner.next().unwrap());
                        let typespec = TypeSpec::from(inner.next().unwrap());
                        (keyword, typespec)
                    })
                    .collect();
                let block = inner.next().unwrap().into_inner().map(Term::from).collect();
                Term {
                    location: InputLocation::from(pair),
                    inner: Inner::PrivateMethod(name, args, block),
                }
            }
            Rule::defpublicmethod => {
                let mut inner = pair.clone().into_inner();
                let name = Identifier::from(inner.next().unwrap());
                let args = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|p| {
                        let mut inner = p.into_inner();
                        let keyword = Identifier::from(inner.next().unwrap());
                        let typespec = TypeSpec::from(inner.next().unwrap());
                        (keyword, typespec)
                    })
                    .collect();
                let block = inner.next().unwrap().into_inner().map(Term::from).collect();
                Term {
                    location: InputLocation::from(pair),
                    inner: Inner::PublicMethod(name, args, block),
                }
            }
            Rule::defpublicspec => {
                let mut inner = pair.clone().into_inner();
                let name = Identifier::from(inner.next().unwrap());
                let args = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|p| {
                        let mut inner = p.into_inner();
                        let keyword = Identifier::from(inner.next().unwrap());
                        let typespec = TypeSpec::from(inner.next().unwrap());
                        (keyword, typespec)
                    })
                    .collect();
                if name.has_predicate() {
                    Term {
                        location: InputLocation::from(pair),
                        inner: Inner::PublicMethodSpec(name, args, None),
                    }
                } else {
                    let rval = TypeSpec::from(inner.next().unwrap().into_inner().next().unwrap());
                    Term {
                        location: InputLocation::from(pair),
                        inner: Inner::PublicMethodSpec(name, args, Some(rval)),
                    }
                }
            }
            Rule::defstaticmethod => {
                let mut inner = pair.clone().into_inner();
                let name = Identifier::from(inner.next().unwrap());
                let args = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|p| {
                        let mut inner = p.into_inner();
                        let keyword = Identifier::from(inner.next().unwrap());
                        let typespec = TypeSpec::from(inner.next().unwrap());
                        (keyword, typespec)
                    })
                    .collect();
                let block = inner.next().unwrap().into_inner().map(Term::from).collect();
                Term {
                    location: InputLocation::from(pair),
                    inner: Inner::StaticMethod(name, args, block),
                }
            }
            Rule::defstaticspec => {
                let mut inner = pair.clone().into_inner();
                let name = Identifier::from(inner.next().unwrap());
                let args = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|p| {
                        let mut inner = p.into_inner();
                        let keyword = Identifier::from(inner.next().unwrap());
                        let typespec = TypeSpec::from(inner.next().unwrap());
                        (keyword, typespec)
                    })
                    .collect();
                if name.has_predicate() {
                    Term {
                        location: InputLocation::from(pair),
                        inner: Inner::StaticMethodSpec(name, args, None),
                    }
                } else {
                    let rval = TypeSpec::from(inner.next().unwrap().into_inner().next().unwrap());
                    Term {
                        location: InputLocation::from(pair),
                        inner: Inner::StaticMethodSpec(name, args, Some(rval)),
                    }
                }
            }
            Rule::float => Term {
                location: InputLocation::from(pair.clone()),
                inner: Inner::Float(Float::from(pair)),
            },
            Rule::function => Term {
                location: InputLocation::from(pair.clone()),
                inner: Inner::Function(Function::from(pair)),
            },
            Rule::if_expression => {
                let mut inner = pair.clone().into_inner();
                let test = Term::from(inner.next().unwrap());
                let positives = inner.next().unwrap().into_inner().map(Term::from).collect();
                let negatives = inner.next();
                if negatives.is_some() {
                    let negatives = negatives.unwrap().into_inner().map(Term::from).collect();
                    Term {
                        location: InputLocation::from(pair),
                        inner: Inner::If(Box::new(test), positives, negatives),
                    }
                } else {
                    Term {
                        location: InputLocation::from(pair),
                        inner: Inner::If(Box::new(test), positives, Vec::default()),
                    }
                }
            }
            Rule::impldef => {
                let mut inner = pair.clone().into_inner();
                let ty = Ty::from(inner.next().unwrap());
                let block = inner.next().unwrap().into_inner().map(Term::from).collect();
                Term {
                    location: InputLocation::from(pair),
                    inner: Inner::ImplDef(ty, block),
                }
            }
            Rule::infix => precedence::climb(pair),
            Rule::instance_infix => precedence::climb(pair),
            Rule::integer => Term {
                location: InputLocation::from(pair.clone()),
                inner: Inner::Integer(Integer::from(pair)),
            },
            Rule::local => Term {
                location: InputLocation::from(pair.clone()),
                inner: Inner::Local(Local::from(pair)),
            },
            Rule::map => {
                let mut contents = Vec::new();

                for map_pair in pair.clone().into_inner() {
                    let mut inner = map_pair.into_inner();
                    let keypair = inner.next().unwrap();
                    let key = match keypair.as_rule() {
                        Rule::keyword => Term {
                            location: InputLocation::from(keypair.clone()),
                            inner: Inner::Atom(Atom::from(keypair)),
                        },
                        _ => Term::from(keypair),
                    };
                    let value = Term::from(inner.next().unwrap());
                    contents.push((key, value));
                }

                Term {
                    location: InputLocation::from(pair),
                    inner: Inner::Map(contents),
                }
            }
            Rule::property_get => Term {
                location: InputLocation::from(pair.clone()),
                inner: Inner::PropertyGet(Identifier::from(pair)),
            },
            Rule::property_set => {
                let values = pair
                    .clone()
                    .into_inner()
                    .map(|pair| {
                        let mut inner = pair.into_inner();
                        let name = Identifier::from(inner.next().unwrap());
                        let value = Term::from(inner.next().unwrap());
                        (name, value)
                    })
                    .collect();
                Term {
                    location: InputLocation::from(pair),
                    inner: Inner::PropertySet(values),
                }
            }
            Rule::string => Term {
                location: InputLocation::from(pair.clone()),
                inner: Inner::String(String::from(pair)),
            },
            Rule::unary => {
                let mut inner = pair.clone().into_inner();
                let operator = Unary::from(inner.next().unwrap());
                let rhs = Term::from(inner.next().unwrap());
                Term {
                    location: InputLocation::from(pair),
                    inner: Inner::Unary(operator, Box::new(rhs)),
                }
            }
            Rule::traitdef => {
                let mut inner = pair.clone().into_inner();
                let ty = Ty::from(inner.next().unwrap());
                let reqs = match inner.next().unwrap().into_inner().next() {
                    Some(p) => Some(TypeSpec::from(p)),
                    None => None,
                };
                let block = inner.next().unwrap().into_inner().map(Term::from).collect();
                Term {
                    location: InputLocation::from(pair),
                    inner: Inner::TraitDef(ty, reqs, block),
                }
            }
            Rule::typedef => {
                let mut inner = pair.clone().into_inner();
                let ty = Ty::from(inner.next().unwrap());
                let props = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|p| {
                        let mut inner = p.into_inner();
                        let keyword = Identifier::from(inner.next().unwrap());
                        let typespec = TypeSpec::from(inner.next().unwrap());
                        (keyword, typespec)
                    })
                    .collect();
                let block = inner.next().unwrap().into_inner().map(Term::from).collect();
                Term {
                    location: InputLocation::from(pair),
                    inner: Inner::TypeDef(ty, props, block),
                }
            }

            _ => panic!("Unexpected pair {:#?}", pair),
        }
    }
}

// Expects to be called with the inner of a `call_method` pair.
//
// Recursively builds a stack of method call terms from the repeating pattern
// off identifier followed by arguments.
fn unroll_method_call(callee: Term, mut inner: Pairs<'_, Rule>) -> Term {
    let method_name = Identifier::from(inner.next().unwrap());
    let mut arguments = Vec::new();

    while inner.peek().is_some() && inner.peek().unwrap().as_rule() == Rule::call_argument {
        let argument = inner.next().unwrap();
        arguments.push(Term::from(argument.into_inner().next().unwrap()));
    }

    let result = Term {
        location: callee.location().clone(), // this location is wrong.
        inner: Inner::MethodCall(Box::new(callee), method_name, arguments),
    };

    if inner.peek().is_some() && inner.peek().unwrap().as_rule() == Rule::ident {
        return unroll_method_call(result, inner);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::ast::{binary, unary, Value};

    #[test]
    fn test_array() {
        let terms = Term::input("[ 1, :two, \"three\" ]").unwrap();
        assert_eq!(terms[0].node_type(), NodeType::Array);
        let array = terms[0].array().unwrap();
        assert_eq!(array[0].integer().unwrap().value_ref(), &1);
        assert_eq!(array[1].atom().unwrap().value_ref(), "two");
        assert_eq!(array[2].string().unwrap().value_ref(), "three");
        assert_eq!(array.len(), 3);
    }

    #[test]
    fn test_array_empty() {
        let terms = Term::input("[]").unwrap();
        let array = terms[0].array().unwrap();
        assert_eq!(array.len(), 0);
    }

    #[test]
    fn test_atom() {
        let terms = Term::input(":baik").unwrap();
        assert_eq!(terms[0].node_type(), NodeType::Atom);
        let term = terms[0].atom().unwrap();
        assert_eq!(term.value_ref(), "baik");
    }

    #[test]
    fn test_binary() {
        let terms = Term::input("1 * 2 + 3 / 4").unwrap();
        assert_eq!(terms[0].node_type(), NodeType::Binary);

        let (op0, lhs0, rhs0) = terms[0].binary().unwrap();
        assert_eq!(op0.value_ref(), &binary::Operator::Plus);

        let (op1, lhs1, rhs1) = lhs0.binary().unwrap();
        assert_eq!(op1.value_ref(), &binary::Operator::Multiply);
        assert_eq!(lhs1.integer().unwrap().value_ref(), &1);
        assert_eq!(rhs1.integer().unwrap().value_ref(), &2);

        let (op2, lhs2, rhs2) = rhs0.binary().unwrap();
        assert_eq!(op2.value_ref(), &binary::Operator::Divide);
        assert_eq!(lhs2.integer().unwrap().value_ref(), &3);
        assert_eq!(rhs2.integer().unwrap().value_ref(), &4);
    }

    #[test]
    fn test_boolean_true() {
        let terms = Term::input("benar").unwrap();
        assert_eq!(terms[0].node_type(), NodeType::Boolean);
        let term = terms[0].boolean().unwrap().clone();
        assert_eq!(term.value(), true);
    }

    #[test]
    fn test_boolean_false() {
        let terms = Term::input("salah").unwrap();
        let term = terms[0].boolean().unwrap().clone();
        assert_eq!(term.value(), false);
    }

    #[test]
    fn test_call() {
        let terms = Term::input("hello(\"Baik\", \"Lang\")").unwrap();
        assert_eq!(terms[0].node_type(), NodeType::Call);
        let (callee, arguments) = terms[0].call().unwrap();
        let local = callee.local().unwrap();
        assert_eq!(local.value_ref(), "hello");
        assert_eq!(arguments[0].string().unwrap().value_ref(), "Baik");
        assert_eq!(arguments[1].string().unwrap().value_ref(), "Lang");
    }

    #[test]
    fn test_call_no_args() {
        let terms = Term::input("hello()").unwrap();
        let (callee, arguments) = terms[0].call().unwrap();
        let local = callee.local().unwrap();
        assert_eq!(local.value_ref(), "hello");
        assert_eq!(arguments.len(), 0);
    }

    #[test]
    fn test_typename() {
        let terms = Term::input("Baik").unwrap();
        assert_eq!(terms[0].node_type(), NodeType::Ty);
        let term = terms[0].ty().unwrap().clone();
        assert_eq!(term.value(), "Baik");
    }

    #[test]
    fn test_constructor_empty() {
        let terms = Term::input("Character {}").unwrap();
        assert_eq!(terms[0].node_type(), NodeType::Constructor);
        let (ty, properties) = terms[0].constructor().unwrap();
        assert_eq!(ty.value_ref(), "Character");
        assert_eq!(properties.len(), 0);
    }

    #[test]
    fn test_constructor() {
        let terms = Term::input("Character { name: \"Baik Lang\" }").unwrap();
        let (ty, properties) = terms[0].constructor().unwrap();
        assert_eq!(ty.value_ref(), "Character");
        assert_eq!(properties.len(), 1);
        let (key, value) = &properties[0];
        assert_eq!(key.value_ref(), "name");
        assert_eq!(value.string().unwrap().value_ref(), "Baik Lang");
    }

    #[test]
    fn test_traitdef_with_bounds() {
        let terms = Term::file("trait Integer: Add + Subtract").unwrap();
        let (ty, reqs, body) = terms[0].traitdef().unwrap();
        assert_eq!(ty.value_ref(), "Integer");
        assert!(reqs.is_some());
        let reqs = reqs.unwrap();
        let tys = reqs.value_ref();
        assert_eq!(tys.len(), 2);
        assert_eq!(body.len(), 0);

        assert_eq!(tys[0].value_ref(), "Add");
        assert_eq!(tys[1].value_ref(), "Subtract");
    }

    #[test]
    // fn test_typedef() {
    //     let terms = Term::file("type Delorean(speed: Integer)").unwrap();
    //     let (ty, props, body) = terms[0].typedef().unwrap();
    //     assert_eq!(ty.value_ref(), "Delorean");
    //     assert_eq!(props.len(), 1);
    //     assert_eq!(body.len(), 0);

    //     let (key, value) = &props[0];
    //     assert_eq!(key.value_ref(), "speed");
    //     assert_eq!(value.value_ref()[0].value_ref(), "Integer");
    // }

    #[test]
    // fn test_typedef_with_impl() {
    //     let terms = Term::file(
    //         r#"
    //             type Delorean(speed: Integer) do
    //                 impl TimeMachine
    //             end
    //         "#,
    //     )
    //     .unwrap();
    //     let (ty, props, body) = terms[0].typedef().unwrap();
    //     assert_eq!(ty.value_ref(), "Delorean");
    //     assert_eq!(props.len(), 1);
    //     assert_eq!(body.len(), 1);

    //     let (key, value) = &props[0];
    //     assert_eq!(key.value_ref(), "speed");
    //     assert_eq!(value.value_ref()[0].value_ref(), "Integer");

    //     let (tr, body) = body[0].impldef().unwrap();
    //     assert_eq!(tr.value_ref(), "TimeMachine");
    //     assert_eq!(body.len(), 0);
    // }

    #[test]
    // fn test_typdef_with_methods() {
    //     let terms = Term::file(
    //         r#"
    //             type Delorean(speed) do
    //                 defs new() do
    //                     Delorean { speed: 0 }
    //                 end
    //             end
    //     "#,
    //     )
    //     .unwrap();
    //     let (ty, props, body) = terms[0].typedef().unwrap();
    //     assert_eq!(ty.value_ref(), "Delorean");
    //     assert_eq!(props.len(), 1);
    //     assert_eq!(body.len(), 1);

    //     let (key, value) = &props[0];
    //     assert_eq!(key.value_ref(), "speed");
    //     assert_eq!(value.value_ref()[0].value_ref(), "Integer");

    //     let (name, args, block) = &body[0].static_method().unwrap();
    //     assert_eq!(name.value_ref(), "new");
    //     assert!(args.is_empty());
    //     assert_eq!(block.len(), 1);
    // }

    #[test]
    fn test_declaration() {
        let terms = Term::input("speed = 88").unwrap();
        assert_eq!(terms[0].node_type(), NodeType::Declaration);

        let (ident, value) = terms[0].declaration().unwrap();
        let value = value.integer().unwrap();
        assert_eq!(ident.value_ref(), "speed");
        assert_eq!(value.value_ref(), &88);
    }

    #[test]
    fn test_float() {
        let terms = Term::input("1.23").unwrap();
        assert_eq!(terms[0].node_type(), NodeType::Float);
        let node = terms[0].float().unwrap();
        assert!(node.value_ref() - 1.23 < std::f64::EPSILON);
    }

    #[test]
    fn test_integer_decimal() {
        let terms = Term::input("123").unwrap();
        assert_eq!(terms[0].node_type(), NodeType::Integer);
        let node = terms[0].integer().unwrap();
        assert_eq!(node.value_ref(), &123);
        assert_eq!(node.radix(), 10);
    }

    #[test]
    fn test_integer_hexadecimal() {
        let terms = Term::input("0x123").unwrap();
        let node = terms[0].integer().unwrap();
        assert_eq!(node.value_ref(), &291);
        assert_eq!(node.radix(), 16);
    }

    #[test]
    fn test_integer_octal() {
        let terms = Term::input("0o123").unwrap();
        let node = terms[0].integer().unwrap();
        assert_eq!(node.value_ref(), &83);
        assert_eq!(node.radix(), 8);
    }

    #[test]
    fn test_integer_binary() {
        let terms = Term::input("0b0101").unwrap();
        let node = terms[0].integer().unwrap();
        assert_eq!(node.value_ref(), &5);
        assert_eq!(node.radix(), 2);
    }

    #[test]
    fn test_integer_zero() {
        let terms = Term::input("0").unwrap();
        let node = terms[0].integer().unwrap();
        assert_eq!(node.value_ref(), &0);
        assert_eq!(node.radix(), 10);
    }

    #[test]
    fn test_local() {
        let terms = Term::input("baik").unwrap();
        assert_eq!(terms[0].node_type(), NodeType::Local);
        let node = terms[0].local().unwrap();
        assert_eq!(node.value_ref(), "baik");
    }

    #[test]
    fn test_map() {
        let terms = Term::input("{ baik: \"Lang\", \"speed\" => 88 }").unwrap();
        assert_eq!(terms[0].node_type(), NodeType::Map);

        let map = terms[0].map().unwrap();
        assert_eq!(map.len(), 2);

        let (key0, value0) = &map[0];
        assert_eq!(key0.atom().unwrap().value_ref(), "baik");
        assert_eq!(value0.string().unwrap().value_ref(), "Lang");

        let (key1, value1) = &map[1];
        assert_eq!(key1.string().unwrap().value_ref(), "speed");
        assert_eq!(value1.integer().unwrap().value_ref(), &88);
    }

    #[test]
    fn test_map_empty() {
        let terms = Term::input("{ }").unwrap();
        let map = terms[0].map().unwrap();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_method_call() {
        let terms = Term::input("greeter.hello(\"Baik\", \"Lang\")").unwrap();
        let (callee, method_name, arguments) = terms[0].method_call().unwrap();
        let local = callee.local().unwrap();
        assert_eq!(local.value_ref(), "greeter");
        assert_eq!(method_name.value_ref(), "hello");
        assert_eq!(arguments[0].string().unwrap().value_ref(), "Baik");
        assert_eq!(arguments[1].string().unwrap().value_ref(), "Lang");
    }

    #[test]
    fn test_method_call_multi() {
        let terms = Term::input("delorean.target_year(1985).accellerate(88)").unwrap();

        let (callee0, method_name0, arguments0) = terms[0].method_call().unwrap();
        assert_eq!(method_name0.value_ref(), "accellerate");
        assert_eq!(arguments0.len(), 1);
        assert_eq!(arguments0[0].integer().unwrap().value_ref(), &88);

        let (callee1, method_name1, arguments1) = callee0.method_call().unwrap();
        assert_eq!(callee1.local().unwrap().value_ref(), "delorean");
        assert_eq!(method_name1.value_ref(), "target_year");
        assert_eq!(arguments1.len(), 1);
        assert_eq!(arguments1[0].integer().unwrap().value_ref(), &1985);
    }

    #[test]
    fn test_method_call_empty() {
        let terms = Term::input("greeter.hello()").unwrap();

        let (callee, method_name, arguments) = terms[0].method_call().unwrap();
        let local = callee.local().unwrap();
        assert_eq!(local.value_ref(), "greeter");
        assert_eq!(method_name.value_ref(), "hello");
        assert_eq!(arguments.len(), 0);
    }

    #[test]
    fn test_method_call_on_typename() {
        let terms = Term::input("My.Greeter.hello()").unwrap();

        let (callee, method_name, arguments) = terms[0].method_call().unwrap();
        let ty = callee.ty().unwrap();
        assert_eq!(ty.value_ref(), "My.Greeter");
        assert_eq!(method_name.value_ref(), "hello");
        assert_eq!(arguments.len(), 0);
    }

    #[test]
    fn test_string() {
        let terms = Term::input("\"Baik Lang\"").unwrap();
        assert_eq!(terms[0].node_type(), NodeType::String);

        let term = terms[0].string().unwrap();
        assert_eq!(term.value_ref(), "Baik Lang");
    }

    #[test]
    fn test_string_empty() {
        let terms = Term::input("\"\"").unwrap();
        let term = terms[0].string().unwrap();
        assert_eq!(term.value_ref(), "");
    }

    #[test]
    fn test_unary() {
        let terms = Term::input("+123").unwrap();
        assert_eq!(terms[0].node_type(), NodeType::Unary);

        let (operator, term) = terms[0].unary().unwrap();
        assert_eq!(operator.value_ref(), &unary::Operator::Plus);
        assert_eq!(term.integer().unwrap().value_ref(), &123);
    }

    #[test]
    fn test_infix() {
        let terms = Term::input("(1 + 2) / (3 - 4)").unwrap();
        assert_eq!(terms[0].node_type(), NodeType::Binary);

        let (op0, lhs0, rhs0) = terms[0].binary().unwrap();
        assert_eq!(op0.value_ref(), &binary::Operator::Divide);

        let (op1, lhs1, rhs1) = lhs0.binary().unwrap();
        assert_eq!(op1.value_ref(), &binary::Operator::Plus);
        assert_eq!(lhs1.integer().unwrap().value_ref(), &1);
        assert_eq!(rhs1.integer().unwrap().value_ref(), &2);

        let (op2, lhs2, rhs2) = rhs0.binary().unwrap();
        assert_eq!(op2.value_ref(), &binary::Operator::Minus);
        assert_eq!(lhs2.integer().unwrap().value_ref(), &3);
        assert_eq!(rhs2.integer().unwrap().value_ref(), &4);
    }

    #[test]
    // fn test_anonymous_function() {
    //     let terms = Term::input(
    //         r#"
    //             fn (speed) {
    //                  "WAT"
    //                ,
    //                (speed: Float) do
    //                  "WAT"
    //                end
    //     "#,
    //     )
    //     .unwrap();

    //     let function = terms[0].function().unwrap();
    //     let clauses = function.value_ref();
    //     assert_eq!(clauses.len(), 2);

    //     let clause0 = &clauses[0];
    //     let clause1 = &clauses[1];

    //     assert_eq!(clause0.arguments().len(), 1);
    //     assert_eq!(clause0.body().len(), 1);
    //     assert_eq!(clause1.arguments().len(), 1);
    //     assert_eq!(clause1.body().len(), 1);
    // }

    #[test]
    fn test_if() {
        let terms = Term::input(" jika benar { 123 } ").unwrap();
        let (_test, positive, negative) = terms[0].if_expr().unwrap();
        assert_eq!(positive.len(), 1);
        assert!(negative.is_empty());
    }

    #[test]
    fn test_if_else() {
        let terms = Term::input(" jika benar { 123 } lainnya { 456 }").unwrap();
        let (_test, positive, negative) = terms[0].if_expr().unwrap();
        assert_eq!(positive.len(), 1);
        assert_eq!(negative.len(), 1);
    }

    // #[test]
    // fn test_acceptance() {
    //     let terms = Term::input(
    //         r#"
    //             type Character(name: String) do
    //                 defs new(wat: String) do
    //                     Character { foo: name }
    //                 end
    //             end
    //         "#,
    //     );
    //     assert!(terms.is_ok());
    // }
}
