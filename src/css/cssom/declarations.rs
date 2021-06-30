use std::collections::HashSet;
use std::collections::HashMap;

type Declaration<'a> = HashMap<&'a str, &'a str>;

pub type Declarations<'a> = HashSet<Declaration<'a>>;

