//! `{expr:...}` — the algebra inside artifacts.
//!
//! A tiny, safe, deterministic expression language: arithmetic, comparisons,
//! a few math functions, and exactly one data source — `ref(part, stat[,
//! field])`, which reads the exact statistics computed over another part's
//! generated rows. No network, no eval, no dependencies; a hand-rolled
//! recursive-descent parser with the usual precedence. This is what makes
//! "the algebra writes the artifact" literal: the LLM never does arithmetic.

use std::collections::BTreeMap;

/// Stats surface an expression can read: `ref("orders", "sum", "price")`.
pub type RefFn<'a> = &'a dyn Fn(&str, &str, Option<&str>) -> Result<f64, String>;

/// Evaluate an expression to a number.
///
/// # Errors
/// Any syntax error (with position), unknown function, unknown name, or a
/// failed `ref` lookup (propagated with the part/stat context).
pub fn eval(src: &str, refs: RefFn) -> Result<f64, String> {
    let mut p = Parser {
        s: src.as_bytes(),
        i: 0,
        refs,
    };
    let v = p.parse_or()?;
    p.skip_ws();
    if p.i != p.s.len() {
        return Err(format!(
            "unexpected {:?} at byte {} (expression continues past its end)",
            p.s[p.i] as char, p.i
        ));
    }
    Ok(v)
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
    refs: RefFn<'a>,
}

impl<'a> Parser<'a> {
    fn skip_ws(&mut self) {
        while self.i < self.s.len() && self.s[self.i].is_ascii_whitespace() {
            self.i += 1;
        }
    }

    fn peek(&self) -> Option<u8> {
        self.s.get(self.i).copied()
    }

    fn eat(&mut self, tok: &str) -> bool {
        self.skip_ws();
        if self.s[self.i..].starts_with(tok.as_bytes()) {
            self.i += tok.len();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, tok: &str) -> Result<(), String> {
        if self.eat(tok) {
            Ok(())
        } else {
            Err(format!("expected {tok:?} at byte {}", self.i))
        }
    }

    // or < and < comparison < additive < multiplicative < unary < primary

    fn parse_or(&mut self) -> Result<f64, String> {
        let mut v = self.parse_and()?;
        loop {
            if self.eat("||") {
                let r = self.parse_and()?;
                v = f64::from(v != 0.0 || r != 0.0);
            } else {
                return Ok(v);
            }
        }
    }

    fn parse_and(&mut self) -> Result<f64, String> {
        let mut v = self.parse_cmp()?;
        loop {
            if self.eat("&&") {
                let r = self.parse_cmp()?;
                v = f64::from(v != 0.0 && r != 0.0);
            } else {
                return Ok(v);
            }
        }
    }

    fn parse_cmp(&mut self) -> Result<f64, String> {
        let mut v = self.parse_add()?;
        loop {
            self.skip_ws();
            let op = if self.eat(">=") {
                ">="
            } else if self.eat("<=") {
                "<="
            } else if self.eat("==") {
                "=="
            } else if self.eat("!=") {
                "!="
            } else if self.eat(">") {
                ">"
            } else if self.eat("<") {
                "<"
            } else {
                return Ok(v);
            };
            let r = self.parse_add()?;
            v = f64::from(match op {
                ">=" => v >= r,
                "<=" => v <= r,
                "==" => v == r,
                "!=" => v != r,
                ">" => v > r,
                _ => v < r,
            });
        }
    }

    fn parse_add(&mut self) -> Result<f64, String> {
        let mut v = self.parse_mul()?;
        loop {
            if self.eat("+") {
                v += self.parse_mul()?;
            } else if self.eat("-") {
                v -= self.parse_mul()?;
            } else {
                return Ok(v);
            }
        }
    }

    fn parse_mul(&mut self) -> Result<f64, String> {
        let mut v = self.parse_unary()?;
        loop {
            if self.eat("*") {
                v *= self.parse_unary()?;
            } else if self.eat("/") {
                let d = self.parse_unary()?;
                if d == 0.0 {
                    return Err(format!("division by zero at byte {}", self.i));
                }
                v /= d;
            } else if self.eat("%") {
                let d = self.parse_unary()?;
                if d == 0.0 {
                    return Err(format!("modulo by zero at byte {}", self.i));
                }
                v %= d;
            } else {
                return Ok(v);
            }
        }
    }

    fn parse_unary(&mut self) -> Result<f64, String> {
        self.skip_ws();
        if self.eat("-") {
            return Ok(-self.parse_unary()?);
        }
        if self.eat("+") {
            return self.parse_unary();
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<f64, String> {
        self.skip_ws();
        match self.peek() {
            None => Err("unexpected end of expression".into()),
            Some(b'(') => {
                self.i += 1;
                let v = self.parse_or()?;
                self.expect(")")?;
                Ok(v)
            }
            Some(c) if c.is_ascii_digit() => self.parse_number(),
            Some(c) if c.is_ascii_alphabetic() || c == b'_' => self.parse_ident(),
            Some(c) => Err(format!("unexpected {c:?} at byte {}", self.i)),
        }
    }

    fn parse_number(&mut self) -> Result<f64, String> {
        let start = self.i;
        while matches!(self.peek(), Some(c) if c.is_ascii_digit() || c == b'.') {
            self.i += 1;
        }
        let text = std::str::from_utf8(&self.s[start..self.i]).unwrap();
        text.parse::<f64>()
            .map_err(|_| format!("bad number {text:?} at byte {start}"))
    }

    fn parse_ident(&mut self) -> Result<f64, String> {
        let start = self.i;
        while matches!(self.peek(), Some(c) if c.is_ascii_alphanumeric() || c == b'_') {
            self.i += 1;
        }
        let name = std::str::from_utf8(&self.s[start..self.i]).unwrap();
        self.skip_ws();
        if self.peek() == Some(b'(') {
            self.i += 1;
            let mut args: Vec<f64> = Vec::new();
            let mut first_raw: Option<Vec<String>> = None; // ref() takes string args
            self.skip_ws();
            if self.peek() != Some(b')') {
                loop {
                    self.skip_ws();
                    if name == "ref" {
                        let a = self.parse_string_arg()?;
                        first_raw.get_or_insert_with(Vec::new).push(a);
                    } else {
                        let v = self.parse_or()?;
                        args.push(v);
                    }
                    self.skip_ws();
                    if self.eat(",") {
                        continue;
                    }
                    break;
                }
            }
            self.expect(")")?;
            return call(name, &args, first_raw.as_deref(), self.refs);
        }
        // bare name: only pi is known
        match name {
            "pi" => Ok(std::f64::consts::PI),
            "e" => Ok(std::f64::consts::E),
            other => Err(format!(
                "unknown name {other:?} at byte {start} (functions need parentheses; data comes via ref(...))"
            )),
        }
    }

    /// A quoted string argument: 'orders' or "orders" (no escapes, no quotes inside).
    fn parse_string_arg(&mut self) -> Result<String, String> {
        self.skip_ws();
        let q = self
            .peek()
            .ok_or_else(|| "unexpected end of expression in string argument".to_string())?;
        if q != b'\'' && q != b'"' {
            return Err(format!(
                "ref() arguments must be quoted strings at byte {}",
                self.i
            ));
        }
        self.i += 1;
        let start = self.i;
        while matches!(self.peek(), Some(c) if c != q) {
            self.i += 1;
        }
        if self.peek().is_none() {
            return Err("unterminated string in ref() argument".into());
        }
        let text = std::str::from_utf8(&self.s[start..self.i])
            .unwrap()
            .to_string();
        self.i += 1; // closing quote
        Ok(text)
    }
}

/// Call a function. `ref` is special: its arguments are the raw strings.
fn call(name: &str, args: &[f64], raw: Option<&[String]>, refs: RefFn) -> Result<f64, String> {
    let arity = |n: usize| -> Result<(), String> {
        if args.len() == n {
            Ok(())
        } else {
            Err(format!(
                "{name}() takes {n} argument(s), got {}",
                args.len()
            ))
        }
    };
    match name {
        "ref" => {
            let raw = raw.ok_or("ref() needs quoted string arguments")?;
            if raw.len() < 2 || raw.len() > 3 {
                return Err(format!(
                    "ref(part, stat[, field]) takes 2-3 string arguments, got {}",
                    raw.len()
                ));
            }
            refs(&raw[0], &raw[1], raw.get(2).map(|s| s.as_str()))
        }
        "if" => {
            arity(3)?;
            Ok(if args[0] != 0.0 { args[1] } else { args[2] })
        }
        "min" => {
            arity(2)?;
            Ok(args[0].min(args[1]))
        }
        "max" => {
            arity(2)?;
            Ok(args[0].max(args[1]))
        }
        "abs" => {
            arity(1)?;
            Ok(args[0].abs())
        }
        "floor" => {
            arity(1)?;
            Ok(args[0].floor())
        }
        "ceil" => {
            arity(1)?;
            Ok(args[0].ceil())
        }
        "round" => {
            arity(1)?;
            Ok(args[0].round())
        }
        "sqrt" => {
            arity(1)?;
            if args[0] < 0.0 {
                return Err("sqrt() of a negative number".into());
            }
            Ok(args[0].sqrt())
        }
        "pow" => {
            arity(2)?;
            Ok(args[0].powf(args[1]))
        }
        "clamp" => {
            arity(3)?;
            Ok(args[0].max(args[1]).min(args[2]))
        }
        other => Err(format!(
            "unknown function {other:?}; have ref, if, min, max, abs, floor, ceil, round, sqrt, pow, clamp"
        )),
    }
}

/// Format an expression result like a stat: integral as integer, else 6
/// decimals trimmed (same convention as the `{ref:...}` renderer).
pub fn fmt_num(x: f64) -> String {
    if x.is_finite() && x.fract() == 0.0 && x.abs() < 1e15 {
        return format!("{}", x as i64);
    }
    let s = format!("{x:.6}");
    let t = s.trim_end_matches('0').trim_end_matches('.');
    t.to_string()
}

/// A no-data ref lookup for standalone use (every ref fails).
pub fn no_refs() -> impl Fn(&str, &str, Option<&str>) -> Result<f64, String> {
    |_p, _s, _f| Err("no data context".to_string())
}

/// Build the `ref` closure over a stats map (part -> stat[:field] -> number).
pub fn ref_lookup<'a>(
    stats: &'a BTreeMap<String, BTreeMap<String, f64>>,
) -> impl Fn(&str, &str, Option<&str>) -> Result<f64, String> + 'a {
    move |part, stat, field| {
        let key = match field {
            Some(f) => format!("{stat}:{f}"),
            None => stat.to_string(),
        };
        stats
            .get(part)
            .and_then(|m| m.get(&key))
            .copied()
            .ok_or_else(|| {
                format!(
                    "ref({part:?}, {stat:?}{}, …) has no value",
                    field.map(|f| format!(", {f:?}")).unwrap_or_default()
                )
            })
    }
}
