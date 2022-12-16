use num_rational::Rational64;

use super::*;
use super::primary::*;

pub struct Token {
    location: Location,
    content: TokenContent,
}

pub enum TokenContent {
    /// Any punctuation mark/operator
    Punct(Punct),

    // Word is any free floating word that is part of the code. It
    // may be an identifier or a keyword
    //Word(Ident),

    Number(Rational64),
}


type TokenStream = Vec<Token>;

/// Processes the PreTokenStream from primary tokenization to assign
/// some general meaning to tokens. This includes:
/// 
/// - Converting periods that are preceded or proceeded by integers into
///     decimals
/// - Sorting words into identifiers and keywords
fn tokenize2(tokens: &PreTokenStream) -> Result<TokenStream, TokenizationError> {
    let mut new_tokens = TokenStream::new();

    let mut skip = 0;

    for i in 0..tokens.len() {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        match tokens[i].content() {
            PreTokenContent::Punct(Punct::Period) => {
                let left = tokens.get(i - 1);
                let right = tokens.get(i + 1);

                let whole = getNumAt(&tokens, i - 1);
                let decimal = getNumAt(&tokens, i + 1);

                if whole.is_some() || decimal.is_some() {
                    let begin = left.map(|t| t.location().begin())
                        .unwrap_or_else(|| tokens[i].location().begin());
                    let len = left.map(|t| t.location().len()).unwrap_or(0)
                        + right.map(|t| t.location().len()).unwrap_or(0) + 1;
                    let location = Location { begin, len };

                    if let Some(token) = tokens.get(i + 2) {
                        if token.content() == &PreTokenContent::Punct(Punct::Period) {
                            return Err(TokenizationError { location, kind: TokenizationErrorKind::TwoDecimalPoints })
                        }
                    }

                    let whole = whole.unwrap_or(LeadingZerosU64::ZERO);
                    let decimal = decimal.unwrap_or(LeadingZerosU64::ZERO);

                    let decimal_digits = (*decimal.value() as f64).log10().ceil() as u8 + 1 + decimal.zeros();
                    let base = 10_i64.pow(decimal_digits as u32);
                    let num = whole.value() * base as u64 + decimal.value();                

                    let numer: i64 = match num.try_into() {
                        Ok(n) => n,
                        Err(e) => return Err(TokenizationError { location, 
                            kind: TokenizationErrorKind::TooPreciseNumber })
                    };

                    new_tokens.push(Token {
                        content: TokenContent::Number(Rational64::new(numer, base)),
                        location,
                    });
                }

                skip = 1;
            }
            _ => todo!()
        }
    }

    Ok(new_tokens)
}

fn getNumAt(tokens: &PreTokenStream, index: usize) -> Option<LeadingZerosU64> {
    if let Some(token) = tokens.get(index + 1) {
        if let PreTokenContent::Number(n) = token.content() {
            return Some(*n);
        }
        else {
            None
        }
    }
    else {
        None
    }
}