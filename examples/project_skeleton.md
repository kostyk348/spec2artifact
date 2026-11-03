# ledgerd — project skeleton (kickoff artifact)

## core_module

```rust
use crate::domain::account::AccountId;
use std::fmt;

/// Build an entry; panics on negative minor amounts.
pub fn Entry::new() {
    todo!()
}

/// Amount with direction applied — the only money arithmetic callers need.
pub fn Entry::signed_minor() {
    todo!()
}

/// Render as `+12.34 account` for logs.
pub fn fmt::Display for Entry() {
    todo!()
}

```

## intro

# Why this skeleton

About module boundaries: domain knows nothing about io, so the balance invariant is testable without a database. And, the only public entry type constructor validates amounts, making negative-money bugs unrepresentable. Anyway, that's it.

Let's talk what is deliberately missing. no async runtime until M3 forces the question. Plus, no migration framework before the sqlite writer lands.


## intro

# Why this skeleton

About module boundaries: domain knows nothing about io, so the balance invariant is testable without a database. And, the only public entry type constructor validates amounts, making negative-money bugs unrepresentable. Anyway, that's it.

Let's talk what is deliberately missing. no async runtime until M3 forces the question. Plus, no migration framework before the sqlite writer lands.


## layout

# Repository layout

ledgerd/
├── src/
│   ├── domain/
│   │   ├── account.rs
│   │   ├── entry.rs
│   │   └── mod.rs
│   ├── io/
│   │   ├── csv.rs
│   │   ├── sqlite.rs
│   │   └── mod.rs
│   ├── lib.rs
│   └── main.rs
├── tests/
│   ├── roundtrip.rs
│   └── balance.rs
├── docs/
│   ├── adr-001-storage.md
│   └── adr-002-money-type.md
├── .editorconfig
├── Cargo.toml
└── README.md


## layout

# Repository layout

ledgerd/
├── src/
│   ├── domain/
│   │   ├── account.rs
│   │   ├── entry.rs
│   │   └── mod.rs
│   ├── io/
│   │   ├── csv.rs
│   │   ├── sqlite.rs
│   │   └── mod.rs
│   ├── lib.rs
│   └── main.rs
├── tests/
│   ├── roundtrip.rs
│   └── balance.rs
├── docs/
│   ├── adr-001-storage.md
│   └── adr-002-money-type.md
├── .editorconfig
├── Cargo.toml
└── README.md


## roadmap

# Roadmap to the first internal release

## Milestones
- M1 — domain types compile with property tests on signed arithmetic
- M2 — CSV reader round-trips a 10k-row fixture byte-identically
- M3 — SQLite writer passes the balance invariant suite (sum of entries == account balance)
- M4 — CLI: `ledgerd import` and `ledgerd balance <account>`

## Risk register
- money as f64 is the classic trap — entries carry i64 minor units from day one
- csv parsing of quoted fields: adopt a crate rather than hand-rolling


## roadmap

# Roadmap to the first internal release

## Milestones
- M1 — domain types compile with property tests on signed arithmetic
- M2 — CSV reader round-trips a 10k-row fixture byte-identically
- M3 — SQLite writer passes the balance invariant suite (sum of entries == account balance)
- M4 — CLI: `ledgerd import` and `ledgerd balance <account>`

## Risk register
- money as f64 is the classic trap — entries carry i64 minor units from day one
- csv parsing of quoted fields: adopt a crate rather than hand-rolling


## standup_template

# Daily standup template

- [ ] Yesterday: <one shippable unit>
- [ ] Today: <one shippable unit>
- [ ] Blockers: <link, not prose>


## standup_template

# Daily standup template

- [ ] Yesterday: <one shippable unit>
- [ ] Today: <one shippable unit>
- [ ] Blockers: <link, not prose>


## velocity

Planned vs completed story points (sprints 1–8)

                                       ·●······●
                                    ···         
                               ···●·            
                          ·●···                 
                      ····                      
                 ···●·                          
            ·●···                               
         ···                                    
    ···●·                                       
●···                                            
  (min 0, max 50, n 8; x: 1 → 8)


## velocity

Planned vs completed story points (sprints 1–8)

                                       ·●······●
                                    ···         
                               ···●·            
                          ·●···                 
                      ····                      
                 ···●·                          
            ·●···                               
         ···                                    
    ···●·                                       
●···                                            
  (min 0, max 50, n 8; x: 1 → 8)

