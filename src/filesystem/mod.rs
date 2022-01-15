#![allow(dead_code)]
use std::collections::HashMap;
use std::path::Path;

pub type Replacements = HashMap<String, String>;

struct Source<'a>(&'a Path);
struct Destination<'a>(&'a Path);
struct ProposedMove<'a>(Source<'a>, Destination<'a>);

struct ProposedMoves<'a> {
    moves: Vec<ProposedMove<'a>>,
    transformer: fn(Replacements, String) -> String,
}