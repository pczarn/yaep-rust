pub enum LookaheadLevel {
    None,
    Static,
    Dynamic(usize),
}

  /* The following value is debug level:
     <0 - print translation for graphviz.
     0 - print nothing.
     1 - print statistics.
     2 - print parse tree.
     3 - print rules, parser list
     4 - print sets.
     5 - print also nonstart situations.
     6 - print additionally lookaheads. */
pub enum DebugLevel {
    Graphviz(usize),
    None,
    Stats,
    ParseTree,
    Rules,
    Sets,
    NonstartSituations,
    Lookaheads,
}

pub struct InternalGrammar {
    axiom: Symbol,
    end_marker: Symbol,
    term_error: Symbol,
    lookahead_level: LookaheadLevel,
    recovery_token_matches: usize,
    debug_level: DebugLevel,
    /* The following value is TRUE if we need only one parse. */
    one_parse_p: bool,
    /* The following value is TRUE if we need parse(s) with minimal
     costs. */
    cost_p: bool,
    /* The following value is TRUE if we need to make error recovery. */
    error_recovery_p: bool,
    symbs: Vec<Symbol>,
    rules: Vec<Rule>,
    term_sets: Vec<TermSet>,
}

// /* The following describes symbol of grammar. */
// struct symb
// {
//   /* The following is external representation of the symbol.  It
//      should be allocated by parse_alloc because the string will be
//      referred from parse tree. */
//   const char *repr;
//   union
//   {
//     struct
//     {
//       /* The following value is code of the terminal symbol. */
//       int code;
//       /* The following member is order number of the terminal. */
//       int term_num;
//     } term;
//     struct
//     {
//       /* The following refers for all rules with the nonterminal
//          symbol is in the left hand side of the rules. */
//       struct rule *rules;
//       /* The following member is order number of the nonterminal. */
//       int nonterm_num;
//       /* The following value is nonzero if nonterminal may derivate
//          itself.  In other words there is a grammar loop for this
//          nonterminal. */
//       int loop_p;
//       /* The following members are FIRST and FOLLOW sets of the
//          nonterminal. */
//       term_set_el_t *first, *follow;
//     } nonterm;
//   } u;
//   /* The following member is TRUE if it is nonterminal. */
//   char term_p;
//   /* The following member value (if defined) is TRUE if the symbol is
//      accessible (derivated) from the axiom. */
//   char access_p;
//   /* The following member is TRUE if it is a termainal or it is a
//      nonterminal which derivates a terminal string. */
//   char derivation_p;
//   /* The following is TRUE if it is nonterminal which may derivate
//      empty string. */
//   char empty_p;
//   /* The following member is order number of symbol. */
//   int num;
// #ifdef USE_CORE_SYMB_HASH_TABLE
//   /* The following is used as cache for subsequent search for
//      core_symb_vect with given symb. */
//   struct core_symb_vect *cached_core_symb_vect;
// #endif
// };

pub struct Symb {
    repr: String,
    u: KindSymbInfo,
    /* The following member is TRUE if it is nonterminal. */
    term_p: bool,
    /* The following member value (if defined) is TRUE if the symbol is
        accessible (derivated) from the axiom. */
    access_p: bool,
    /* The following member is TRUE if it is a termainal or it is a
        nonterminal which derivates a terminal string. */
    derivation_p: bool,
    /* The following is TRUE if it is nonterminal which may derivate
        empty string. */
    empty_p: bool,
    /* The following member is order number of symbol. */
    num: u32,
    /* The following is used as cache for subsequent search for
    core_symb_vect with given symb. */
    #[cfg(use_core_symb_hash_table)]
    cached_core_symb_vect: Box<CoreSymbVect>, // or Vec, map?
}

pub union KindSymbInfo {
    terminal: TerminalSymbInfo,
    nonterminal: NonterminalSymbInfo,
}

pub struct TerminalSymbInfo {
    code: u32,
    /* The following member is order number of the terminal. */
    term_num: u32,
}

pub struct NonterminalSymbInfo {
    rules: Vec<Rule>,
    /* The following member is order number of the nonterminal. */
    nonterm_num: u32,
    /* The following value is nonzero if nonterminal may derivate
       itself.  In other words there is a grammar loop for this
       nonterminal. */
    loop_p: u32,
    /* The following members are FIRST and FOLLOW sets of the
       nonterminal. */
    first: Bitset,
    follow: Bitset,
}

// /* The following structure contians all information about grammar
//    vocabulary. */
// struct symbs
// {
//   /* The following is number of all symbols and terminals.  The
//      variables can be read externally. */
//   int n_terms, n_nonterms;

//   /* All symbols are placed in the following object. */
// #ifndef __cplusplus
//   os_t symbs_os;
// #else
//   os_t *symbs_os;
// #endif

//   /* All references to the symbols, terminals, nonterminals are stored
//      in the following vlos.  The indexes in the arrays are the same as
//      corresponding symbol, terminal, and nonterminal numbers. */
// #ifndef __cplusplus
//   vlo_t symbs_vlo;
//   vlo_t terms_vlo;
//   vlo_t nonterms_vlo;
// #else
//   vlo_t *symbs_vlo;
//   vlo_t *terms_vlo;
//   vlo_t *nonterms_vlo;
// #endif

//   /* The following are tables to find terminal by its code and symbol by
//      its representation. */
//   hash_table_t repr_to_symb_tab;	/* key is `repr' */
//   hash_table_t code_to_symb_tab;	/* key is `code' */
// #ifdef SYMB_CODE_TRANS_VECT
//   /* If terminal symbol codes are not spared (in this case the member
//      value is not NULL, we use translation vector instead of hash
//      table.  */
//   struct symb **symb_code_trans_vect;
//   int symb_code_trans_vect_start;
//   int symb_code_trans_vect_end;
// #endif
// };

pub struct Symbs {
    n_terms: u32,
    n_nonterms: u32,
    symbs: Vec<Symb>,
    // symbs_vlo: String, // or Vec<Symb>?
    terms: Vec<usize>, // or Vec<TerminalSymbInfo>  // holds indices into `symbs`
    nonterms: Vec<usize>, // or Vec<NonterminalSymbInfo> // holds indices into `symbs`
    repr_to_symb: HashMap<String, usize>, /* key is `repr' */
    code_to_symb: HashMap<u32, usize>,    /* key is `code' */
    #[cfg(symb_code_trans_vect)]
    symb_code_trans_vect: Option<Vec<usize>>,
    #[cfg(symb_code_trans_vect)]
    symb_code_trans_vect_start: u32,
    #[cfg(symb_code_trans_vect)]
    symb_code_trans_vect_end: u32,
}

// /* Hash of symbol representation. */
// static unsigned
// symb_repr_hash (hash_table_entry_t s)
// {
//   unsigned result = jauquet_prime_mod32;
//   const char *str = ((struct symb *) s)->repr;
//   int i;

//   for (i = 0; str[i] != '\0'; i++)
//     result = result * hash_shift + (unsigned) str[i];
//   return result;
// }

// /* Equality of symbol representations. */
// static int
// symb_repr_eq (hash_table_entry_t s1, hash_table_entry_t s2)
// {
//   return strcmp (((struct symb *) s1)->repr, ((struct symb *) s2)->repr) == 0;
// }

// /* Hash of terminal code. */
// static unsigned
// symb_code_hash (hash_table_entry_t s)
// {
//   struct symb *symb = ((struct symb *) s);

//   assert (symb->term_p);
//   return symb->u.term.code;
// }

// /* Equality of terminal codes. */
// static int
// symb_code_eq (hash_table_entry_t s1, hash_table_entry_t s2)
// {
//   struct symb *symb1 = ((struct symb *) s1);
//   struct symb *symb2 = ((struct symb *) s2);

//   assert (symb1->term_p && symb2->term_p);
//   return symb1->u.term.code == symb2->u.term.code;
// }

// ---
// no equivalent -- we have impls for String and u32

// /* Initialize work with symbols and returns storage for the
//    symbols. */
// static struct symbs *
// symb_init (void)
// {
//   void *mem;
//   struct symbs *result;

//   mem = yaep_malloc (grammar->alloc, sizeof (struct symbs));
//   result = (struct symbs *) mem;
//   OS_CREATE (result->symbs_os, grammar->alloc, 0);
//   VLO_CREATE (result->symbs_vlo, grammar->alloc, 1024);
//   VLO_CREATE (result->terms_vlo, grammar->alloc, 512);
//   VLO_CREATE (result->nonterms_vlo, grammar->alloc, 512);
//   result->repr_to_symb_tab =
//     create_hash_table (grammar->alloc, 300, symb_repr_hash, symb_repr_eq);
//   result->code_to_symb_tab =
//     create_hash_table (grammar->alloc, 200, symb_code_hash, symb_code_eq);
// #ifdef SYMB_CODE_TRANS_VECT
//   result->symb_code_trans_vect = NULL;
// #endif
//   result->n_nonterms = result->n_terms = 0;
//   return result;
// }

impl Symbs {
    pub fn new() -> Self {
        Symbs {
            n_terms: 0,
            n_nonterms: 0,
            symbs: vec![],
            // symbs_vlo: String, // or Vec<Symb>?
            // terms: String, // or Vec<TerminalSymbInfo>
            // nonterms: String, // or Vec<NonterminalSymbInfo>
            repr_to_symb: HashMap::new(),
            code_to_symb: HashMap::new(),
            #[cfg(symb_code_trans_vect)]
            symb_code_trans_vect: None,
        }
    }
}

// /* Return symbol (or NULL if it does not exist) whose representation
//    is REPR. */
// static struct symb *
// symb_find_by_repr (const char *repr)
// {
//   struct symb symb;

//   symb.repr = repr;
//   return (struct symb *) *find_hash_table_entry (symbs_ptr->repr_to_symb_tab,
// 						 &symb, FALSE);
// }
impl Symbs {
    fn symb_by_repr(&self, repr: &str) -> Option<&Symb> {
        self.repr_to_symb.get(repr).map(|idx| self.symbs[idx])
    }
}


// /* Return symbol (or NULL if it does not exist) which is terminal with
//    CODE. */
// #if MAKE_INLINE
// INLINEurn symbol (or NULL if it does not exist) which is terminal with
//    CODE. */
// #if MAKE_INLINE
// INLINE
// #endif
// static struct symb *
// symb_find_by_code (int code)
// {
//   struct symb symb;

// #ifdef SYMB_CODE_TRANS_VECT
//   if (symbs_ptr->symb_code_trans_vect != NULL)
//     {
//       if ((code < symbs_ptr->symb_code_trans_vect_start)
//           || (code >= symbs_ptr->symb_code_trans_vect_end))
//         {
//           return NULL;
//         }
//       else
//         {
//           return symbs_ptr->symb_code_trans_vect
//             [code - symbs_ptr->symb_code_trans_vect_start];
//         }
//     }
// #endif
//   symb.term_p = TRUE;
//   symb.u.term.code = code;
//   return (struct symb *) *find_hash_table_entry (symbs_ptr->code_to_symb_tab,
// 						 &symb, FALSE);
// }
// #endif
// static struct symb *
// symb_find_by_code (int code)
// {
//   struct symb symb;

// #ifdef SYMB_CODE_TRANS_VECT
//   if (symbs_ptr->symb_code_trans_vect != NULL)
//     {
//       if ((code < symbs_ptr->symb_code_trans_vect_start)
//           || (code >= symbs_ptr->symb_code_trans_vect_end))
//         {
//           return NULL;
//         }
//       else
//         {
//           return symbs_ptr->symb_code_trans_vect
//             [code - symbs_ptr->symb_code_trans_vect_start];
//         }
//     }
// #endif
//   symb.term_p = TRUE;
//   symb.u.term.code = code;
//   return (struct symb *) *find_hash_table_entry (symbs_ptr->code_to_symb_tab,
// 						 &symb, FALSE);
// }

impl Symbs {
    fn symb_by_code(&self, code: u32) -> Option<&Symb> {
        #[cfg(symb_code_trans_vect)]
        if let &Some(ref vect) = &self.symb_code_trans_vect {
            if code < self.symb_code_trans_vect_start || code >= self.symb_code_trans_vect_end {
                None
            } else {
                Some(&self.symb_code_trans_vect[code - self.symb_code_trans_vect_start])
            }
        }
        self.code_to_symb.get(code).map(|idx| &self.symbs[idx])
    }
}

// #[cfg(not(symb_code_trans_vect))]
// impl Symbs {
//     fn symb_by_code(&self, code: u32) -> Option<&Symb> {
//     }
// }

// /* The function creates new terminal symbol and returns reference for
//    it.  The symbol should be not in the tables.  The function should
//    create own copy of name for the new symbol. */
// static struct symb *
// symb_add_term (const char *name, int code)
// {
//   struct symb symb, *result;
//   hash_table_entry_t *repr_entry, *code_entry;

//   symb.repr = name;
//   symb.term_p = TRUE;
//   symb.num = symbs_ptr->n_nonterms + symbs_ptr->n_terms;
//   symb.u.term.code = code;
//   symb.u.term.term_num = symbs_ptr->n_terms++;
//   symb.empty_p = FALSE;
//   repr_entry =
//     find_hash_table_entry (symbs_ptr->repr_to_symb_tab, &symb, TRUE);
//   assert (*repr_entry == NULL);
//   code_entry =
//     find_hash_table_entry (symbs_ptr->code_to_symb_tab, &symb, TRUE);
//   assert (*code_entry == NULL);
//   OS_TOP_ADD_STRING (symbs_ptr->symbs_os, name);
//   symb.repr = (char *) OS_TOP_BEGIN (symbs_ptr->symbs_os);
//   OS_TOP_FINISH (symbs_ptr->symbs_os);
//   OS_TOP_ADD_MEMORY (symbs_ptr->symbs_os, &symb, sizeof (struct symb));
//   result = (struct symb *) OS_TOP_BEGIN (symbs_ptr->symbs_os);
//   OS_TOP_FINISH (symbs_ptr->symbs_os);
//   *repr_entry = (hash_table_entry_t) result;
//   *code_entry = (hash_table_entry_t) result;
//   VLO_ADD_MEMORY (symbs_ptr->symbs_vlo, &result, sizeof (struct symb *));
//   VLO_ADD_MEMORY (symbs_ptr->terms_vlo, &result, sizeof (struct symb *));
//   return result;
// }

impl Symbs {
    fn add_term(&mut self, name: String, code: u32) -> usize {
        match self.repr_to_symb.entry(name) {
            Occupied(occupied) => { panic!("a symbol with this name already exists in the table") }
            Vacant(vacant) => {
                vacant.insert(self.symbs.len());
            }
        }
        match self.code_to_symb.entry(code) {
            Occupied(occupied) => { panic!("a symbol with this code already exists in the table") }
            Vacant(vacant) => {
                vacant.insert(self.symbs.len());
            }
        }
        let symb = Symb {
            repr: name,
            u: KindSymbInfo {
                terminal: TerminalSymbInfo {
                    code,
                    term_num: self.n_terms,
                },
            },
            term_p: true,
            empty_p: false,
            derivation_p: false,
            access_p: false,
            num: self.n_terms + self.n_nonterms,
        };
        self.n_terms += 1;
        let order = self.symbs.len();
        self.terms.push(order);
        self.symbs.push(symb);
        order
    }
}

// /* The function creates new nonterminal symbol and returns reference
//    for it.  The symbol should be not in the table.  The function
//    should create own copy of name for the new symbol. */
// static struct symb *
// symb_add_nonterm (const char *name)
// {
//   struct symb symb, *result;
//   hash_table_entry_t *entry;

//   symb.repr = name;
//   symb.term_p = FALSE;
//   symb.num = symbs_ptr->n_nonterms + symbs_ptr->n_terms;
//   symb.u.nonterm.rules = NULL;
//   symb.u.nonterm.loop_p = 0;
//   symb.u.nonterm.nonterm_num = symbs_ptr->n_nonterms++;
//   entry = find_hash_table_entry (symbs_ptr->repr_to_symb_tab, &symb, TRUE);
//   assert (*entry == NULL);
//   OS_TOP_ADD_STRING (symbs_ptr->symbs_os, name);
//   symb.repr = (char *) OS_TOP_BEGIN (symbs_ptr->symbs_os);
//   OS_TOP_FINISH (symbs_ptr->symbs_os);
//   OS_TOP_ADD_MEMORY (symbs_ptr->symbs_os, &symb, sizeof (struct symb));
//   result = (struct symb *) OS_TOP_BEGIN (symbs_ptr->symbs_os);
//   OS_TOP_FINISH (symbs_ptr->symbs_os);
//   *entry = (hash_table_entry_t) result;
//   VLO_ADD_MEMORY (symbs_ptr->symbs_vlo, &result, sizeof (struct symb *));
//   VLO_ADD_MEMORY (symbs_ptr->nonterms_vlo, &result, sizeof (struct symb *));
//   return result;
// }

impl Symbs {
    fn add_nonterm(&mut self, name: String) -> usize {
        match self.repr_to_symb.entry(name) {
            Occupied(occupied) => { panic!("a symbol with this name already exists in the table") }
            Vacant(vacant) => {
                vacant.insert(self.symbs.len());
            }
        }
        match self.code_to_symb.entry(code) {
            Occupied(occupied) => { panic!("a symbol with this code already exists in the table") }
            Vacant(vacant) => {
                vacant.insert(self.symbs.len());
            }
        }
        let symb = Symb {
            repr: name,
            u: KindSymbInfo {
                terminal: NonterminalSymbInfo {
                    rules: None,
                    loop_p: 0,
                    nonterm_num: self.n_nonterms,
                },
            },
            term_p: true,
            empty_p: false,
            derivation_p: false,
            access_p: false,
            num: self.n_terms + self.n_nonterms,
        };
        self.n_nonterms += 1;
        let order = self.symbs.len();
        self.nonterms.push(order);
        self.symbs.push(symb);
        order
    }
}

// /* The following function return N-th symbol (if any) or NULL
//    otherwise. */
// static struct symb *
// symb_get (int n)
// {
//   struct symb *symb;

//   if (n < 0 || (VLO_LENGTH (symbs_ptr->symbs_vlo) / sizeof (struct symb *)
// 		<= (size_t) n))
//     return NULL;
//   symb = ((struct symb **) VLO_BEGIN (symbs_ptr->symbs_vlo))[n];
//   assert (symb->num == n);
//   return symb;
// }

impl Symbs {
    fn get(&self, n: usize) -> Option<&Symb> {
        if let Some(symb) = self.symbs.get(n) {
            assert_eq!(symb.num, n as u32);
        }
        self.symbs.get(n)
    }
}

// /* The following function return N-th symbol (if any) or NULL
//    otherwise. */
// static struct symb *
// term_get (int n)
// {
//   struct symb *symb;

//   if (n < 0 || (VLO_LENGTH (symbs_ptr->terms_vlo)
// 		/ sizeof (struct symb *) <= (size_t) n))
//     return NULL;
//   symb = ((struct symb **) VLO_BEGIN (symbs_ptr->terms_vlo))[n];
//   assert (symb->term_p && symb->u.term.term_num == n);
//   return symb;
// }

impl Symbs {
    fn term_get(&self, n: usize) -> Option<&Symb> {
        if let Some(&idx) = self.terms.get(n) {
            let symb = &self.symbs[idx];
            assert!(symb.term_p);
            unsafe {
                assert_eq!(symb.u.terminal.term_num, n as u32);
            }
        }
        self.terms.get(n)
    }
}

// /* The following function return N-th symbol (if any) or NULL
//    otherwise. */
// static struct symb *
// nonterm_get (int n)
// {
//   struct symb *symb;

//   if (n < 0 || (VLO_LENGTH (symbs_ptr->nonterms_vlo) / sizeof (struct symb *)
// 		<= (size_t) n))
//     return NULL;
//   symb = ((struct symb **) VLO_BEGIN (symbs_ptr->nonterms_vlo))[n];
//   assert (!symb->term_p && symb->u.nonterm.nonterm_num == n);
//   return symb;
// }

impl Symbs {
    fn nonterm_get(&self, n: usize) -> Option<&Symb> {
        if let Some(&idx) = self.nonterms.get(n) {
            let symb = &self.symbs[idx];
            assert!(!symb.term_p);
            unsafe {
                assert_eq!(symb.u.nonterminal.nonterm_num, n as u32);
            }
        }
        self.nonterms.get(n)
    }
}

// #ifndef NO_YAEP_DEBUG_PRINT

// /* The following function prints symbol SYMB to file F.  Terminal is
//    printed with its code if CODE_P. */
// static void
// symb_print (FILE * f, struct symb *symb, int code_p)
// {
//   fprintf (f, "%s", symb->repr);
//   if (code_p && symb->term_p)
//     fprintf (f, "(%d)", symb->u.term.code);
// }

// #endif /* #ifndef NO_YAEP_DEBUG_PRINT */

// ---
// will happen as derive(Debug) or something like that

// #ifdef SYMB_CODE_TRANS_VECT

// #define SYMB_CODE_TRANS_VECT_SIZE 10000

// static void_VECT

// #define SYMB_CODE_TRANS_VECT_SIZE 10000

// static void
// symb_finish_adding_terms (void)
// {
//   int i, max_code, min_code;
//   struct symb *symb;
//   void *mem;

//   for (min_code = max_code = i = 0; (symb = term_get (i)) != NULL; i++)
//     {
//       if (i == 0 || min_code > symb->u.term.code)
// 	min_code = symb->u.term.code;
//       if (i == 0 || max_code < symb->u.term.code)
// 	max_code = symb->u.term.code;
//     }
//   assert (i != 0);
//   if (max_code - min_code < SYMB_CODE_TRANS_VECT_SIZE)
//     {
//       symbs_ptr->symb_code_trans_vect_start = min_code;
//       symbs_ptr->symb_code_trans_vect_end = max_code + 1;
//       mem = yaep_malloc (grammar->alloc,
//           sizeof (struct symb*) * (max_code - min_code + 1));
//       symbs_ptr->symb_code_trans_vect = (struct symb **) mem;
//       for (i = 0; (symb = term_get (i)) != NULL; i++)
// 	symbs_ptr->symb_code_trans_vect[symb->u.term.code - min_code] = symb;
//     }
// }
// #endif
// symb_finish_adding_terms (void)
// {
//   int i, max_code, min_code;
//   struct symb *symb;
//   void *mem;

//   for (min_code = max_code = i = 0; (symb = term_get (i)) != NULL; i++)
//     {
//       if (i == 0 || min_code > symb->u.term.code)
// 	min_code = symb->u.term.code;
//       if (i == 0 || max_code < symb->u.term.code)
// 	max_code = symb->u.term.code;
//     }
//   assert (i != 0);
//   if (max_code - min_code < SYMB_CODE_TRANS_VECT_SIZE)
//     {
//       symbs_ptr->symb_code_trans_vect_start = min_code;
//       symbs_ptr->symb_code_trans_vect_end = max_code + 1;
//       mem = yaep_malloc (grammar->alloc,
//           sizeof (struct symb*) * (max_code - min_code + 1));
//       symbs_ptr->symb_code_trans_vect = (struct symb **) mem;
//       for (i = 0; (symb = term_get (i)) != NULL; i++)
// 	symbs_ptr->symb_code_trans_vect[symb->u.term.code - min_code] = symb;
//     }
// }
// #endif

#[cfg(symb_code_trans_vect)]
const SYMB_CODE_TRANS_VECT_SIZE: u32 = 10_000;
#[cfg(not(symb_code_trans_vect))]
const SYMB_CODE_TRANS_VECT_SIZE: u32 = 0;

impl Symbs {
    fn finish_adding_terms(&mut self) {
        unsafe {
            let symbs = self.terms.iter().map(|&idx| &self.symbs[idx].u.terminal.code);
            let min_code = symbs.clone().min().unwrap();
            let max_code = symbs.max().unwrap();
            if max_code - min_code < SYMB_CODE_TRANS_VECT_SIZE {
                self.symb_code_trans_vect_start = min_code;
                self.symb_code_trans_vect_end = max_code + 1;
                self.symb_code_trans_vect = Some(vec![0; max_code - min_code + 1]);
                for &idx in &self.terms {
                    let symb = self.symbs[idx];
                    self.symb_code_trans_vect[symb.u.terminal.code] = idx;
                }
            }
        }
    }
}

// /* Free memory for symbols. */
// static void
// symb_empty (struct symbs *symbs)
// {
//   if (symbs == NULL)
//     return;
// #ifdef SYMB_CODE_TRANS_VECT
//   if (symbs_ptr->symb_code_trans_vect != NULL)
//     {
//       yaep_free (grammar->alloc, symbs_ptr->symb_code_trans_vect);
//       symbs_ptr->symb_code_trans_vect = NULL;
//     }
// #endif
//   empty_hash_table (symbs->repr_to_symb_tab);
//   empty_hash_table (symbs->code_to_symb_tab);
//   VLO_NULLIFY (symbs->nonterms_vlo);
//   VLO_NULLIFY (symbs->terms_vlo);
//   VLO_NULLIFY (symbs->symbs_vlo);
//   OS_EMPTY (symbs->symbs_os);
//   symbs->n_nonterms = symbs->n_terms = 0;
// }

impl Symbs {
    fn clear(&mut self) {
        self.repr_to_symb.clear();
        self.code_to_symb.clear();
        self.symbs.clear();
        self.nonterms.clear();
        self.terms.clear();
        self.n_nonterms = 0;
        self.n_terms = 0;
    }
}

// /* Finalize work with symbols. */
// static void
// symb_fin (struct symbs *symbs)
// {
//   if (symbs == NULL)
//     return;
// #ifdef SYMB_CODE_TRANS_VECT
//   if (symbs_ptr->symb_code_trans_vect != NULL)
//     yaep_free (grammar->alloc, symbs_ptr->symb_code_trans_vect);
// #endif
//   delete_hash_table (symbs_ptr->repr_to_symb_tab);
//   delete_hash_table (symbs_ptr->code_to_symb_tab);
//   VLO_DELETE (symbs_ptr->nonterms_vlo);
//   VLO_DELETE (symbs_ptr->terms_vlo);
//   VLO_DELETE (symbs_ptr->symbs_vlo);
//   OS_DELETE (symbs_ptr->symbs_os);
//   yaep_free (grammar->alloc, symbs);
//   symbs = NULL;
// }

// ---
// covered by Drop impl for Symbs

// /* This page contains abstract data set of terminals. */

// /* The following is element of term set hash table. */
// struct tab_term_set
// {
//   /* Number of set in the table. */
//   int num;
//   /* The terminal set itself. */
//   term_set_el_t *set;
// };

struct TabTermSet {
    num: u32,
    set: BitVec,
}

// /* The following container for the abstract data. */
// struct term_sets
// {
//   /* All terminal sets are stored in the following os. */
// #ifndef __cplusplus
//   os_t term_set_os;
// #else
//   os_t *term_set_os;
// #endif

//   /* The following variables can be read externally.  Their values are
//      number of terminal sets and their overall size. */
//   int n_term_sets, n_term_sets_size;

//   /* The following is hash table of terminal sets (key is member
//      `set'). */
//   hash_table_t term_set_tab;

//   /* References to all structure tab_term_set are stored in the
//      following vlo. */
// #ifndef __cplusplus
//   vlo_t tab_term_set_vlo;
// #else
//   vlo_t *tab_term_set_vlo;
// #endif
// };

struct TermSets {
    term_set: ?
    n_term_sets: u32,
    n_term_sets_size: u32,
    term_set_tab: ?
}

// /* Hash of table terminal set. */
// static unsigned
// term_set_hash (hash_table_entry_t s)
// {
//   term_set_el_t *set = ((struct tab_term_set *) s)->set;
//   term_set_el_t *bound;
//   int size;
//   unsigned result = jauquet_prime_mod32;

//   size = ((symbs_ptr->n_terms + CHAR_BIT * sizeof (term_set_el_t) - 1)
// 	  / (CHAR_BIT * sizeof (term_set_el_t)));
//   bound = set + size;
//   while (set < bound)
//     result = result * hash_shift + *set++;
//   return result;
// }

/* Equality of terminal sets. */
static int
term_set_eq (hash_table_entry_t s1, hash_table_entry_t s2)
{
  term_set_el_t *set1 = ((struct tab_term_set *) s1)->set;
  term_set_el_t *set2 = ((struct tab_term_set *) s2)->set;
  term_set_el_t *bound;
  int size;

  size = ((symbs_ptr->n_terms + CHAR_BIT * sizeof (term_set_el_t) - 1)
	  / (CHAR_BIT * sizeof (term_set_el_t)));
  bound = set1 + size;
  while (set1 < bound)
    if (*set1++ != *set2++)
      return FALSE;
  return TRUE;
}

/* Initialize work with terminal sets and returns storage for terminal
   sets. */
static struct term_sets *
term_set_init (void)
{
  void *mem;
  struct term_sets *result;

  mem = yaep_malloc (grammar->alloc, sizeof (struct term_sets));
  result = (struct term_sets *) mem;
  OS_CREATE (result->term_set_os, grammar->alloc, 0);
  result->term_set_tab =
    create_hash_table (grammar->alloc, 1000, term_set_hash, term_set_eq);
  VLO_CREATE (result->tab_term_set_vlo, grammar->alloc, 4096);
  result->n_term_sets = result->n_term_sets_size = 0;
  return result;
}

/* Return new terminal SET.  Its value is undefined. */
static term_set_el_t *
term_set_create (void)
{
  int size;
  term_set_el_t *result;

  assert (sizeof (term_set_el_t) <= 8);
  size = 8;
  /* Make it 64 bit multiple to have the same statistics for 64 bit
     machines. */
  size = ((symbs_ptr->n_terms + CHAR_BIT * 8 - 1) / (CHAR_BIT * 8)) * 8;
  OS_TOP_EXPAND (term_sets_ptr->term_set_os, size);
  result = (term_set_el_t *) OS_TOP_BEGIN (term_sets_ptr->term_set_os);
  OS_TOP_FINISH (term_sets_ptr->term_set_os);
  term_sets_ptr->n_term_sets++;
  term_sets_ptr->n_term_sets_size += size;
  return result;
}

/* Make terminal SET empty. */
#if MAKE_INLINE
INLINE
#endif
static void
term_set_clear (term_set_el_t * set)
{
  term_set_el_t *bound;
  int size;

  size = ((symbs_ptr->n_terms + CHAR_BIT * sizeof (term_set_el_t) - 1)
	  / (CHAR_BIT * sizeof (term_set_el_t)));
  bound = set + size;
  while (set < bound)
    *set++ = 0;
}

/* Copy SRC into DEST. */
#if MAKE_INLINE
INLINE
#endif
static void
term_set_copy (term_set_el_t * dest, term_set_el_t * src)
{
  term_set_el_t *bound;
  int size;

  size = ((symbs_ptr->n_terms + CHAR_BIT * sizeof (term_set_el_t) - 1)
	  / (CHAR_BIT * sizeof (term_set_el_t)));
  bound = dest + size;
  while (dest < bound)
    *dest++ = *src++;
}

/* Add all terminals from set OP with to SET.  Return TRUE if SET has
   been changed. */
#if MAKE_INLINE
INLINE
#endif
static int
term_set_or (term_set_el_t * set, term_set_el_t * op)
{
  term_set_el_t *bound;
  int size, changed_p;

  size = ((symbs_ptr->n_terms + CHAR_BIT * sizeof (term_set_el_t) - 1)
	  / (CHAR_BIT * sizeof (term_set_el_t)));
  bound = set + size;
  changed_p = 0;
  while (set < bound)
    {
      if ((*set | *op) != *set)
	changed_p = 1;
      *set++ |= *op++;
    }
  return changed_p;
}

/* Add terminal with number NUM to SET.  Return TRUE if SET has been
   changed. */
#if MAKE_INLINE
INLINE
#endif
static int
term_set_up (term_set_el_t * set, int num)
{
  int ind, changed_p;
  term_set_el_t bit;

  assert (num < symbs_ptr->n_terms);
  ind = num / (CHAR_BIT * sizeof (term_set_el_t));
  bit = ((term_set_el_t) 1) << (num % (CHAR_BIT * sizeof (term_set_el_t)));
  changed_p = (set[ind] & bit ? 0 : 1);
  set[ind] |= bit;
  return changed_p;
}

/* Return TRUE if terminal with number NUM is in SET. */
#if MAKE_INLINE
INLINE
#endif
static int
term_set_test (term_set_el_t * set, int num)
{
  int ind;
  term_set_el_t bit;

  assert (num >= 0 && num < symbs_ptr->n_terms);
  ind = num / (CHAR_BIT * sizeof (term_set_el_t));
  bit = ((term_set_el_t) 1) << (num % (CHAR_BIT * sizeof (term_set_el_t)));
  return (set[ind] & bit) != 0;
}

/* The following function inserts terminal SET into the table and
   returns its number.  If the set is already in table it returns -its
   number - 1 (which is always negative).  Don't set after
   insertion!!! */
static int
term_set_insert (term_set_el_t * set)
{
  hash_table_entry_t *entry;
  struct tab_term_set tab_term_set, *tab_term_set_ptr;

  tab_term_set.set = set;
  entry =
    find_hash_table_entry (term_sets_ptr->term_set_tab, &tab_term_set, TRUE);
  if (*entry != NULL)
    return -((struct tab_term_set *) *entry)->num - 1;
  else
    {
      OS_TOP_EXPAND (term_sets_ptr->term_set_os,
		     sizeof (struct tab_term_set));
      tab_term_set_ptr =
	(struct tab_term_set *) OS_TOP_BEGIN (term_sets_ptr->term_set_os);
      OS_TOP_FINISH (term_sets_ptr->term_set_os);
      *entry = (hash_table_entry_t) tab_term_set_ptr;
      tab_term_set_ptr->set = set;
      tab_term_set_ptr->num = (VLO_LENGTH (term_sets_ptr->tab_term_set_vlo)
			       / sizeof (struct tab_term_set *));
      VLO_ADD_MEMORY (term_sets_ptr->tab_term_set_vlo, &tab_term_set_ptr,
		      sizeof (struct tab_term_set *));
      return ((struct tab_term_set *) *entry)->num;
    }
}

/* The following function returns set which is in the table with
   number NUM. */
#if MAKE_INLINE
INLINE
#endif
static term_set_el_t *
term_set_from_table (int num)
{
  assert (num < VLO_LENGTH (term_sets_ptr->tab_term_set_vlo)
	  / sizeof (struct tab_term_set *));
  return ((struct tab_term_set **)
	  VLO_BEGIN (term_sets_ptr->tab_term_set_vlo))[num]->set;
}

/* Print terminal SET into file F. */
static void
term_set_print (FILE * f, term_set_el_t * set)
{
  int i;

  for (i = 0; i < symbs_ptr->n_terms; i++)
    if (term_set_test (set, i))
      {
	fprintf (f, " ");
	symb_print (f, term_get (i), FALSE);
      }
}

/* Free memory for terminal sets. */
static void
term_set_empty (struct term_sets *term_sets)
{
  if (term_sets == NULL)
    return;
  VLO_NULLIFY (term_sets->tab_term_set_vlo);
  empty_hash_table (term_sets->term_set_tab);
  OS_EMPTY (term_sets->term_set_os);
  term_sets->n_term_sets = term_sets->n_term_sets_size = 0;
}

/* Finalize work with terminal sets. */
static void
term_set_fin (struct term_sets *term_sets)
{
  if (term_sets == NULL)
    return;
  VLO_DELETE (term_sets->tab_term_set_vlo);
  delete_hash_table (term_sets->term_set_tab);
  OS_DELETE (term_sets->term_set_os);
  yaep_free (grammar->alloc, term_sets);
  term_sets = NULL;
}



/* This page is abstract data `grammar rules'. */

/* The following describes rule of grammar. */
struct rule
{
  /* The following is order number of rule. */
  int num;
  /* The following is length of rhs. */
  int rhs_len;
  /* The following is the next grammar rule. */
  struct rule *next;
  /* The following is the next grammar rule with the same nonterminal
     in lhs of the rule. */
  struct rule *lhs_next;
  /* The following is nonterminal in the left hand side of the
     rule. */
  struct symb *lhs;
  /* The following is symbols in the right hand side of the rule. */
  struct symb **rhs;
  /* The following three members define rule translation. */
  const char *anode;		/* abstract node name if any. */
  int anode_cost;		/* the cost of the abstract node if any, otherwise 0. */
  int trans_len;		/* number of symbol translations in the rule translation. */
  /* The following array elements correspond to element of rhs with
     the same index.  The element value is order number of the
     corresponding symbol translation in the rule translation.  If the
     symbol translation is rejected, the corresponding element value is
     negative. */
  int *order;
  /* The following member value is equal to size of all previous rule
     lengths + number of the previous rules.  Imagine that all left
     hand symbol and right hand size symbols of the rules are stored
     in array.  Then the following member is index of the rule lhs in
     the array. */
  int rule_start_offset;
  /* The following is the same string as anode but memory allocated in
     parse_alloc. */
  char *caller_anode;
};

/* The following container for the abstract data. */
struct rules
{
  /* The following is number of all rules and their summary rhs
     length.  The variables can be read externally. */
  int n_rules, n_rhs_lens;
  /* The following is the first rule. */
  struct rule *first_rule;
  /* The following is rule being formed.  It can be read
     externally. */
  struct rule *curr_rule;
  /* All rules are placed in the following object. */
#ifndef __cplusplus
  os_t rules_os;
#else
  os_t *rules_os;
#endif
};



