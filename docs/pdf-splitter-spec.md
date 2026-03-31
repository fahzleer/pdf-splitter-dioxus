# PDF Splitter — Dioxus Edition
## Full Spec Document (FP-First, Dioxus UI)

## Table of Contents

1. [FP Philosophy](#1-fp-philosophy)
2. [Event Storming](#2-event-storming)
3. [Domain-Driven Design](#3-domain-driven-design)
4. [Architecture Design](#4-architecture-design)
5. [Dioxus UI Design](#5-dioxus-ui-design)
6. [Specification](#6-specification)
7. [Planning](#7-planning)
8. [Test Cases Design (TDD)](#8-test-cases-design-tdd)

---

## 1. FP Philosophy

> Why FP here — not trend-chasing, but solving real problems.

### 1.1 The Problem with Imperative PDF Splitter

Writing this the old way (OOP + mutable state) causes these problems:

```rust
// ❌ Imperative — state can change anywhere
struct PdfSplitter {
    input_path: String,
    pages_per_file: usize,
    current_chunk: usize,    // shared mutable state
    errors: Vec<String>,     // hidden side effect
}
```

**Problems (matches Lecture 1):**
- State mutates anywhere → hard to debug
- Methods hide side effects → hard to test
- Calling `split()` twice gives different results → Referential Transparency gone

### 1.2 The FP Fix

Three core ideas (Lectures 1–2):

```
Immutability    → values don't change after creation
Pure Functions  → same input always gives same output
Composition     → small functions combined into pipelines
```

```rust
// ✅ FP — pure function, immutable data
fn calculate_chunks(total_pages: usize, pages_per_file: usize)
    -> Result<Vec<PageRange>, SplitError>
{
    // no self, no mutable state
    // call it a hundred times — same result every time
}
```

### 1.3 ADTs Instead of Primitive Obsession (Lecture 3)

```rust
// ❌ String/bool for state — compiler can't help
fn process(status: &str) { /* "shiped" typo, caught at runtime, oops */ }

// ✅ ADT — compiler checks every case
enum JobStatus {
    Created,
    Validating,
    Splitting { progress: usize, total: usize },
    Completed { files: Vec<PathBuf> },
    Failed { error: SplitError },
}
// Miss a case in match → compiler error immediately
```

### 1.4 Railway Oriented Programming — Error Handling (Lecture 4)

```rust
// ❌ OOP — panic (hidden side effect)
fn validate(path: &str) -> PdfDocument {
    if !path.ends_with(".pdf") { panic!("Not a PDF!"); }
}

// ✅ FP — Railway (Result type)
fn validate(path: &Path) -> Result<PdfDocument, SplitError> {
    // Green track: Ok(document)
    // Red track:   Err(SplitError::NotAPdf)
}

// Pipeline with ? operator
fn split_workflow(input: &Path, config: &SplitConfig) -> Result<Vec<PathBuf>, SplitError> {
    let doc     = validate(input)?;
    let chunks  = calculate_chunks(&doc, config)?;
    let results = write_chunks(&doc, &chunks, config)?;
    Ok(results)
}
```

### 1.5 Dioxus Components = Pure Functions (Lecture 2 applied to UI)

```rust
// A Dioxus component is a pure function
// props (input) → Element (output)
// Same props → same UI always ← Referential Transparency

#[component]
fn SplitProgress(progress: usize, total: usize) -> Element {
    rsx! {
        div { class: "progress",
            div { style: "width: {progress * 100 / total}%" }
            p { "{progress} / {total} files created" }
        }
    }
}
```

### 1.6 State Monad in Dioxus (Chapter 9)

`use_signal` in Dioxus is a State Monad in practice:

```
State s a : s -> (a, s)   ← State Monad concept
Signal<T>  : get() -> T   ← Dioxus equivalent
```

State is threaded through the Dioxus runtime automatically — just declare what state a component needs.

---

### 1.7 FP Architecture: Enforcing Domain Boundaries

#### Layer Structure

```
┌──────────────────────────────────────┐
│           Controller Layer            │  ← handles HTTP request
│    (Decode / Validate / Transform)    │
└──────────────┬───────────────────────┘
               │
     ┌─────────┴──────────┐
     ↓                    ↓
┌──────────┐   ┌──────────────────────┐
│  Invalid │   │     Domain Input      │
│  Request │   │  (Clean / Pure Data)  │ ← no JWT, headers, IP
└──────────┘   └──────────┬───────────┘
                           │
                           ▼
              ┌─────────────────────────┐
              │  Service / Domain Layer  │
              │     (Pure Function)      │ ← zero IO
              └──────────┬──────────────┘
                         │
                         ▼
              ┌─────────────────────────┐
              │  Side Effect Descriptions│
              │  (Actions / Commands)    │
              │  [InsertUser, LogEvent]  │
              └──────────┬──────────────┘
                         │
                         ▼
              ┌─────────────────────────┐
              │   Side-effect Executor   │ ← runs real IO
              └─────────────────────────┘
```

**Layer responsibilities:**

**Controller** — handles HTTP, JWT, headers, validation. Must not pass infrastructure concerns into Domain.

**Domain** — receives clean data only. Pure function. Returns descriptions of what should happen, not the actions themselves.

**Executor** — runs DB, logging, external calls. No business logic.

#### The Problem

```
JWT tokens, auth headers, request metadata can leak from
Controller into Domain with nothing stopping it.
```

**What we need:**
- Domain cannot access JWT, headers, or metadata
- Violations caught at compile time
- Business logic stays pure and testable

---

### 1.8 Scala 3: Can It Enforce Domain Boundaries at Compile Time?

**Short answer: partly yes, but discipline still required.**

#### Option 1 — Multi-Module Build (strongest)

```scala
lazy val domain = project
  .settings(libraryDependencies := Seq())  // zero infra in classpath

lazy val infrastructure = project
  .dependsOn(domain)
  .settings(libraryDependencies += "org.http4s" %% "http4s-server" % "...")
```

Result: domain module has no `JwtToken` class — any import causes compile error.

#### Option 2 — Opaque Types

```scala
opaque type JwtToken = String  // structure invisible outside package

// domain can't do anything with JwtToken — compiler enforces it
def createUser(name: String, email: String): List[Action] = ???
```

#### Option 3 — Phantom Types

```scala
sealed trait Validated
sealed trait Raw

case class UserInput[State](name: String, email: String)

def validate(input: UserInput[Raw]): Either[Error, UserInput[Validated]] = ???
def createUser(input: UserInput[Validated]): List[Action] = ???
// Pass Raw instead of Validated → compile error
```

#### Scala 3 Summary

| Mechanism | Enforces? | Level |
|---|---|---|
| Multi-module build | ✅ strongest | Classpath |
| Opaque Types | ✅ type doesn't leak | Compiler |
| Phantom Types | ✅ enforces tagged data | Compiler |
| Package visibility | ⚠️ partial | Compiler |
| Convention only | ❌ devs can bypass | — |

---

### 1.9 Haskell / OCaml / Elm / Roc — Who Enforces at Compiler Level?

#### Comparison

| Language | IO Separation | Domain Boundary | Enforce Level |
|---|---|---|---|
| **Haskell** | ✅✅ IO monad | ✅ module system | Compiler + Module |
| **OCaml** | ⚠️ effect handlers | ✅✅ .mli signatures | Module (strong) |
| **Elm** | ✅✅ Cmd/Sub | ✅ module visibility | Compiler + Architecture |
| **Roc** | ✅✅✅ Platform | ✅✅ platform boundary | Language design level |

#### Haskell

```haskell
-- Pure — type system guarantees no IO
createUser :: UserInput -> [Action]
createUser input = [InsertUser (toUser input), LogEvent "user_created"]

-- IO — explicit in the type
executeActions :: [Action] -> IO ()
executeActions actions = mapM_ execute actions
```

**Strength:** `IO a` vs `a` enforced by type checker — no hiding IO in pure functions.
**Weakness:** "no JWT in domain" still needs disciplined module structure.

#### OCaml

```ocaml
(* domain/user_service.mli — interface file *)
val create_user : string -> string -> action list
(* JwtToken not in the signature → compiler rejects any attempt to use it *)
```

**Strength:** `.mli` files are a hard wall — implementation can't exceed what's declared.
**Weakness:** Effect handlers (OCaml 5) still new; skipping `.mli` removes all protection.

#### Elm

```elm
update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        UserCreated input ->
            ( { model | users = input :: model.users }, Cmd.none )
-- No direct IO in update — runtime enforces it
```

**Strength:** Elm runtime is the enforcer — IO outside Cmd/Sub is impossible.
**Weakness:** Frontend web only.

#### Roc

```roc
-- Pure function — no way to call HTTP, DB, or JWT here
createUser : UserInput -> List Action
createUser = \input -> [InsertUser (toUser input), LogEvent "user_created"]

-- Impure — explicit
handleRequest : Request -> Task Response []
handleRequest = \req ->
    jwt = extractJwt req.headers   # stays at controller
    actions = createUser (toUserInput req.body)
    Task.await (executeActions actions) \_ -> Task.ok { status: 200 }
```

**Why Roc is strongest:**
1. Platform defines what effects app code can access — if JWT decode isn't exposed, domain can never call it
2. `a -> b` guarantees pure; `a -> Task b *` declares effects — no loophole
3. No escape hatch (`unsafePerformIO` doesn't exist)
4. Language-level separation, not convention

**Weakness:** Pre-1.0, small ecosystem, web platform not production-stable yet.

#### Final Comparison

```
Language     Pure/IO Separation    No-Infra-In-Domain    Overall
──────────   ──────────────────    ──────────────────    ───────
Scala 3      ⚠️  needs design       ✅ multi-module        B+
Haskell      ✅✅ IO monad           ⚠️  module discipline   A-
OCaml        ⚠️  effect handlers    ✅✅ .mli signatures     A-
Elm          ✅✅ Cmd/Sub (FE only)  ✅  TEA architecture    A
Roc          ✅✅✅ platform system   ✅✅ language design     A+ (pre-1.0)
```

**For practical web backend:** Haskell (strongest production-ready), Scala 3 (most flexible, large JVM ecosystem).

**For this project (Rust + Dioxus):** Rust module visibility + separate crates + trait boundaries gives enforcement close to Scala 3 multi-module. Domain functions without `async` or `Result<_, IoError>` are pure by nature.

---

## 2. Event Storming

### 2.1 Event Flow Overview

```
User Interaction       UI Events (Dioxus)          System Events       Output
────────────────       ──────────────────          ─────────────       ──────

[Drag / Browse]   ──→  FileDragged
                        │
                        ▼
                   FileSelected { path }
                        │
                        ▼ (async)
                   FileValidating... ──→ [spinner]
                        │
                   ┌────┴────────────────┐
                   ▼                     ▼
             FileValidated         ValidationFailed
             { doc, pages }        { error }
                   │                     │
                   ▼                     ▼
             [show PDF info]       [show error]
                   │
[set pages_per_file]│
                   ▼
              ConfigChanged { pages_per_file }
                   │
                   ▼ (real-time preview)
              ChunksCalculated { chunks }
                   │
                   ▼
             [preview table]
                   │
[press Split!]     │
                   ▼
              SplitRequested
                   │
                   ▼ (async, spawn)
              SplitStarted { total }
                   │
              ┌────┴────┐
              ▼         ▼
        ChunkCreated  ChunkFailed
              │
              ▼
        ProgressUpdated { done, total }
              │
        SplitCompleted / SplitAborted
              │
              ▼
        [result list]
        [Open Folder button]
```

### 2.2 Event Catalog

| Event | Trigger | Payload | Next |
|---|---|---|---|
| `FileSelected` | user picks file | `{ path: PathBuf }` | `FileValidating` |
| `FileValidating` | async validate starts | `{}` | `FileValidated` or `ValidationFailed` |
| `FileValidated` | validate succeeds | `{ total_pages, file_size }` | `ConfigChanged` |
| `ValidationFailed` | validate fails | `{ error: SplitError }` | show error, wait |
| `ConfigChanged` | user changes settings | `{ pages_per_file, output_dir }` | `ChunksCalculated` |
| `ChunksCalculated` | pure calc (instant) | `{ chunks: Vec<PageRange> }` | show preview |
| `SplitRequested` | press Split button | `{}` | `SplitStarted` |
| `SplitStarted` | async begins | `{ total: usize }` | `ChunkCreated` × N |
| `ChunkCreated` | chunk succeeds | `{ index, path }` | `ProgressUpdated` |
| `ChunkFailed` | chunk fails | `{ index, error }` | log, continue |
| `SplitCompleted` | all done | `{ files, errors }` | show result |
| `SplitAborted` | fatal error | `{ error, completed }` | show error |
| `OpenOutputFolder` | press Open Folder | `{ path }` | open file manager |
| `ResetRequested` | press Reset | `{}` | back to initial state |

### 2.3 Error Events

```
ValidationFailed
├── FileNotFound
├── NotAPdf
├── CorruptedPdf
├── EmptyPdf
├── EncryptedPdf
└── PermissionDenied

ChunkFailed
├── PageExtractionError
├── WriteError
└── DiskFullError

SplitAborted
├── TooManyErrors
└── OutputDirError
```

---

## 3. Domain-Driven Design

### 3.1 Ubiquitous Language

| Term | Meaning |
|---|---|
| **Source PDF** | the original input file |
| **Page** | one page in a PDF (1-indexed) |
| **Chunk** | a group of pages that becomes one new PDF |
| **Page Range** | an inclusive [start, end] range |
| **Split Config** | split settings (immutable value object) |
| **Chunk Plan** | the calculated list of all chunks (pure computation) |
| **Split Job** | one split operation with a full lifecycle |
| **Job Status** | current state of a Split Job (ADT) |
| **Output File** | the new PDF produced |
| **App State** | full Dioxus app state held in a Signal |

### 3.2 Domain Model

```
┌─────────────────────────────────────────────────────────┐
│                       Split Job                          │
│   (Entity — has identity, lifecycle, ADT status)         │
│                                                         │
│  ┌───────────────┐      ┌──────────────────────────┐    │
│  │  SourcePdf    │─────→│       SplitConfig         │    │
│  │ (Value Obj)   │      │      (Value Object)        │    │
│  │               │      │  pages_per_file: usize     │    │
│  │ path: PathBuf │      │  output_dir: PathBuf       │    │
│  │ total_pages   │      │  naming: NamingPattern     │    │
│  │ file_size     │      │  overwrite: bool           │    │
│  └───────────────┘      └──────────────────────────┘    │
│          │                          │                    │
│          └──────────┬───────────────┘                    │
│                     ▼  (pure function)                   │
│           ┌──────────────────┐                           │
│           │    ChunkPlan     │                           │
│           │  chunks: Vec<..> │                           │
│           │  total_files     │                           │
│           └────────┬─────────┘                           │
│         ┌──────────┼──────────┐                          │
│         ▼          ▼          ▼                          │
│   ChunkResult  ChunkResult  ChunkResult ...              │
└─────────────────────────────────────────────────────────┘
```

### 3.3 ADT Definitions

```rust
#[derive(Debug, Clone, PartialEq)]
enum NamingPattern {
    Sequential,
    WithPageRange,
    Custom { prefix: String },
}

#[derive(Debug, Clone, PartialEq)]
enum JobStatus {
    Idle,
    Validating,
    Ready { total_pages: usize },
    Splitting { done: usize, total: usize },
    Completed { files: Vec<PathBuf>, errors: Vec<SplitError> },
    Failed { error: SplitError },
}

#[derive(Debug, Clone, thiserror::Error)]
enum SplitError {
    #[error("File not found: {path}")]        FileNotFound { path: PathBuf },
    #[error("Not a PDF: {path}")]             NotAPdf { path: PathBuf },
    #[error("Corrupted PDF: {path}")]         CorruptedPdf { path: PathBuf },
    #[error("Empty PDF: {path}")]             EmptyPdf { path: PathBuf },
    #[error("Encrypted PDF: {path}")]         EncryptedPdf { path: PathBuf },
    #[error("Permission denied: {path}")]     PermissionDenied { path: PathBuf },
    #[error("Invalid config")]                InvalidConfig,
    #[error("Disk full")]                     DiskFull,
    #[error("Write error: {msg}")]            WriteError { msg: String },
}
```

### 3.4 Pure Domain Functions

```rust
fn calculate_chunks(total_pages: usize, pages_per_file: usize)
    -> Result<ChunkPlan, SplitError>
{
    if total_pages == 0 || pages_per_file == 0 {
        return Err(SplitError::InvalidConfig);
    }
    let chunks: Vec<PageRange> = (0..)
        .map(|i| {
            let start = i * pages_per_file + 1;
            let end   = (start + pages_per_file - 1).min(total_pages);
            PageRange { start, end }
        })
        .take_while(|r| r.start <= total_pages)
        .collect();

    Ok(ChunkPlan { total_files: chunks.len(), chunks })
}

fn generate_output_path(index: usize, range: &PageRange, config: &SplitConfig) -> PathBuf {
    let filename = match &config.naming {
        NamingPattern::Sequential          => format!("output_{}.pdf", index + 1),
        NamingPattern::WithPageRange       => format!("pages_{}-{}.pdf", range.start, range.end),
        NamingPattern::Custom { prefix }   => format!("{}_{}.pdf", prefix, index + 1),
    };
    config.output_dir.join(filename)
}
```

### 3.5 Bounded Contexts

```
┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐
│  Input Context   │→ │  Core Context    │→ │  Output Context  │
│                  │  │                  │  │                  │
│ File selection   │  │ Page counting    │  │ File writing     │
│ Validation       │  │ Chunk calc       │  │ Naming           │
│ PDF parsing      │  │ Page extraction  │  │ Directory mgmt   │
│ UI: drag/drop    │  │ Pure functions   │  │ Progress report  │
└──────────────────┘  └──────────────────┘  └──────────────────┘
                    Dioxus App State (Signal)
```

---

## 4. Architecture Design

### 4.1 Layer Architecture

```
┌──────────────────────────────────────────────────────┐
│                  Dioxus UI Layer                      │
│  FileDropZone  ConfigPanel  SplitProgress  Results   │
│  Pure fn       Pure fn      Pure fn        Pure fn   │
├──────────────────────────────────────────────────────┤
│               Application Layer                       │
│  split_service.rs  app_state.rs                      │
│  Async orchestration, Signal<AppState> wiring        │
├──────────────────────────────────────────────────────┤
│                 Domain Layer                          │
│  calculate_chunks()  generate_output_path()          │
│  ADTs: JobStatus, SplitError, AppScreen              │
│  100% pure — no IO anywhere                          │
├──────────────────────────────────────────────────────┤
│             Infrastructure Layer                      │
│  lopdf wrapper, file I/O                             │
│  implements traits defined by domain                 │
└──────────────────────────────────────────────────────┘
```

**Rules:**
- Domain has zero dependencies (Cargo workspace enforces this)
- Application depends on Domain only
- Infrastructure implements Domain traits
- UI depends on Application + Domain

### 4.2 Module Structure

```
pdf-splitter-dioxus/
├── Cargo.toml                    # workspace root
├── crates/
│   ├── domain/                   # separate crate — NO infra deps
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── models.rs
│   │       ├── split_logic.rs
│   │       ├── naming.rs
│   │       └── errors.rs
│   │
│   ├── infrastructure/           # depends on domain
│   │   ├── Cargo.toml            # lopdf, std::fs
│   │   └── src/
│   │       ├── pdf_reader.rs
│   │       ├── pdf_writer.rs
│   │       └── file_system.rs
│   │
│   └── app/                      # Dioxus app
│       ├── Cargo.toml            # dioxus, tokio, rfd
│       └── src/
│           ├── main.rs
│           ├── components/
│           ├── screens/
│           └── application/
```

```toml
# crates/domain/Cargo.toml
[dependencies]
thiserror = "2"
# no lopdf, no tokio, no dioxus
# compiler enforces domain knows nothing about infrastructure
```

### 4.3 Trait Abstractions

```rust
trait PdfReader: Send + Sync {
    fn load(&self, path: &Path) -> Result<PdfDocument, SplitError>;
    fn count_pages(&self, doc: &PdfDocument) -> usize;
    fn is_encrypted(&self, doc: &PdfDocument) -> bool;
}

trait PdfWriter: Send + Sync {
    fn extract_pages(&self, doc: &PdfDocument, range: &PageRange) -> Result<Vec<u8>, SplitError>;
    fn save(&self, data: &[u8], path: &Path) -> Result<(), SplitError>;
}

trait FileSystem: Send + Sync {
    fn ensure_dir(&self, path: &Path) -> Result<(), SplitError>;
    fn file_exists(&self, path: &Path) -> bool;
    fn open_in_explorer(&self, path: &Path);
}
```

### 4.4 Data Flow — Railway Pipeline

```
user picks file
      │
      ▼
validate(path)          → Result<SourcePdf, SplitError>
      │ Ok
      ▼
calculate_chunks()      → Result<ChunkPlan, SplitError>  (PURE)
      │ Ok
      ▼
for chunk in chunks:
  extract_pages()       → Result<Vec<u8>, SplitError>
      │ Ok
      ▼
  save_file()           → Result<PathBuf, SplitError>
      │ Ok
      ▼
  emit ChunkCreated     → update Signal → UI re-renders
      │
      ▼
SplitCompleted { files, errors }
```

---

## 5. Dioxus UI Design

### 5.1 Dioxus Concepts

| Concept | Dioxus | FP Equivalent |
|---|---|---|
| Component | `fn Foo(props) -> Element` | Pure Function |
| Props | `struct FooProps { ... }` | Immutable arguments |
| Signal | `use_signal(|| initial)` | State Monad |
| Event | `onclick`, `ondrop` | IO effect triggering state change |
| `rsx!` | HTML-like macro | Declarative output |

### 5.2 App State

```rust
#[derive(Debug, Clone, PartialEq)]
struct AppState {
    screen: AppScreen,
    source_pdf: Option<SourcePdf>,
    config: SplitConfig,
    chunk_plan: Option<ChunkPlan>,
    job_status: JobStatus,
    chunk_results: Vec<ChunkResult>,
}

impl AppState {
    fn with_file(self, pdf: SourcePdf) -> Self {
        Self { source_pdf: Some(pdf), screen: AppScreen::FileLoaded, ..self }
    }

    fn with_status(self, status: JobStatus) -> Self {
        let screen = match &status {
            JobStatus::Splitting { .. } => AppScreen::Splitting,
            JobStatus::Completed { .. } => AppScreen::Done,
            JobStatus::Failed { .. }    => AppScreen::Error { msg: "Split failed".into() },
            _                           => self.screen.clone(),
        };
        Self { job_status: status, screen, ..self }
    }
}
```

### 5.3 Root App Component

```rust
#[component]
fn App() -> Element {
    let mut state = use_signal(AppState::new);
    rsx! {
        div { class: "app",
            match state.read().screen.clone() {
                AppScreen::Welcome      => rsx! { WelcomeScreen { state } },
                AppScreen::FileLoaded   => rsx! { FileLoadedScreen { state } },
                AppScreen::Splitting    => rsx! { SplittingScreen { state } },
                AppScreen::Done         => rsx! { DoneScreen { state } },
                AppScreen::Error { msg } => rsx! { ErrorDisplay { message: msg } },
            }
        }
    }
}
```

### 5.4 Component Hierarchy

```
App (root — holds Signal<AppState>)
├── WelcomeScreen
│   └── FileDropZone
│       ├── drop area
│       └── BrowseButton
├── FileLoadedScreen
│   ├── PdfInfoCard
│   ├── ConfigPanel
│   │   ├── PagesSlider
│   │   ├── OutputDirPicker
│   │   ├── NamingSelect
│   │   └── OverwriteToggle
│   ├── ChunkPreview (real-time, pure)
│   └── SplitButton
├── SplittingScreen
│   ├── SplitProgress
│   └── ChunkStatusList
└── DoneScreen
    ├── SummaryCard
    ├── ResultList
    ├── OpenFolderButton
    └── StartOverButton
```

### 5.5 UI Screens

#### Screen 1: Welcome
```
┌────────────────────────────────────────────┐
│              PDF Splitter                   │
│  ┌──────────────────────────────────────┐  │
│  │     🗂  Drop a PDF here to start      │  │
│  │           [ Browse... ]              │  │
│  └──────────────────────────────────────┘  │
└────────────────────────────────────────────┘
```

#### Screen 2: File Loaded
```
┌────────────────────────────────────────────┐
│  📄 document.pdf    10 pages    2.4 MB      │
│  Pages per file:  [────●──────] 3           │
│  Output folder:   ./output/   [ Browse... ] │
│  Preview: 4 files → [1-3] [4-6] [7-9] [10] │
│                   [ Split! ]               │
└────────────────────────────────────────────┘
```

#### Screen 3: Splitting
```
┌────────────────────────────────────────────┐
│  [██████████████░░░░░░░░]  2 / 4           │
│  Creating: output_3.pdf                    │
│  ✓ output_1.pdf  ✓ output_2.pdf  ⧗ ...    │
└────────────────────────────────────────────┘
```

#### Screen 4: Done
```
┌────────────────────────────────────────────┐
│  ✅ Done! 4 files created                   │
│  ✓ output_1.pdf  1-3   820 KB              │
│  ✓ output_2.pdf  4-6   910 KB              │
│  [ Open Output Folder ]  [ Split Another ] │
└────────────────────────────────────────────┘
```

---

## 6. Specification

### 6.1 Functional Requirements

- **FR-001** Accept PDF via drag & drop or file browser
- **FR-002** Show file info (name, pages, size)
- **FR-003** Configure split via UI (slider, picker, dropdown)
- **FR-004** Real-time chunk preview (pure function — instant)
- **FR-005** Split with progress bar (async, UI stays responsive)
- **FR-006** Show results + Open Folder button
- **FR-007** Clear error handling — state the cause and the fix

### 6.2 Non-Functional Requirements

- **NFR-001** Split 100 pages < 10s; preview < 1ms
- **NFR-002** Partial success; cleanup on error
- **NFR-003** Windows, macOS, Linux (Dioxus Desktop)
- **NFR-004** PDF v1.0–2.0, scanned, with annotations

---

## 7. Planning

### 7.1 Implementation Phases

```
Phase 1  Domain crate + TDD                  Day 1–2
Phase 2  Core logic — all tests green        Day 3
Phase 3  Infrastructure crate                Day 4–5
Phase 4  Application layer (async/Signal)    Day 6
Phase 5  Dioxus UI (5 screens, 9 components) Day 7–9
Phase 6  Polish, edge cases, QA              Day 10
```

### 7.2 TDD Workflow

```
RED    → write the test first
GREEN  → implement just enough to pass
REFACTOR → clean up without changing behaviour
repeat
```

### 7.3 Cargo Workspace Enforcement

```toml
# crates/domain/Cargo.toml
[dependencies]
thiserror = "2"
# no lopdf — compiler makes it impossible to import lopdf in domain

# crates/infrastructure/Cargo.toml
[dependencies]
pdf-splitter-domain = { path = "../domain" }
lopdf = "0.34"

# crates/app/Cargo.toml
[dependencies]
pdf-splitter-domain         = { path = "../domain" }
pdf-splitter-infrastructure = { path = "../infrastructure" }
dioxus = { version = "0.6", features = ["desktop"] }
tokio  = { version = "1",   features = ["full"] }
rfd    = "0.14"
```

---

## 8. Test Cases Design (TDD)

### 8.1 Unit Tests — Domain Models

```
TC-M001  PageRange normal             start=1, end=2 → Ok
TC-M002  PageRange.len()              start=3, end=7 → 5
TC-M003  start > end                  → Err(InvalidConfig)
TC-M004  start = 0                    → Err(InvalidConfig)
TC-M005  single-page range            start=3, end=3 → Ok, len=1
TC-C001  SplitConfig default          pages_per_file=2, overwrite=false
TC-C002  pages_per_file = 0           → Err(InvalidConfig)
TC-C003  immutable clone              original config unchanged
TC-J001  match JobStatus all arms     no compiler warning
TC-J002  Splitting { done:3, total:5} progress = 60%
```

### 8.2 Unit Tests — Split Logic (Pure Functions)

```
TC-L001  10 pages, ppf=2 (exact)      → 5 chunks [1-2,3-4,5-6,7-8,9-10]
TC-L002  7 pages, ppf=3 (remainder)   → 3 chunks [1-3,4-6,7-7]
TC-L003  total < ppf                  → 1 chunk [1-1]
TC-L004  ppf = 1                      → 5 chunks [1-1,2-2,...]
TC-L005  ppf = total                  → 1 chunk [1-10]
TC-L006  ppf > total                  → 1 chunk [1-10]
TC-L007  100 pages, ppf=7             → 15 chunks, last=[99-100]
TC-L008  total = 0                    → Err(InvalidConfig)
TC-L009  ppf = 0                      → Err(InvalidConfig)

Property-based (proptest):
TC-L010  sum(chunk.len) == total_pages       for all valid inputs
TC-L011  chunks.len == ceil(total/ppf)       for all valid inputs
TC-L012  1 <= chunk.len <= ppf              for all chunks
TC-L013  chunks are contiguous, no overlap   for all valid inputs
```

### 8.3 Unit Tests — Output Naming

```
TC-N001  Sequential index=0           → "./out/output_1.pdf"
TC-N002  Sequential index=4           → "./out/output_5.pdf"
TC-N003  WithPageRange 1-3            → "./out/pages_1-3.pdf"
TC-N004  Custom "scan" index=2        → "./out/scan_3.pdf"
TC-N005  prefix with spaces           → valid path
TC-N006  prefix="../evil"             → Err(InvalidConfig)
```

### 8.4 Unit Tests — AppState Transitions

```
TC-S001  AppState::new()              screen=Welcome, status=Idle
TC-S002  with_file() doesn't mutate   original state.source_pdf still None
TC-S003  with_config() returns new    original config unchanged
TC-S004  with_status(Splitting)       screen → Splitting automatically
TC-S005  with_status(Completed)       screen → Done automatically
```

### 8.5 Integration Tests — File Validation

```
TC-V001  normal PDF, 10 pages         → Ok(SourcePdf { total_pages: 10 })
TC-V002  file not found               → Err(FileNotFound)
TC-V003  JPEG renamed to .pdf         → Err(NotAPdf)
TC-V004  corrupted PDF                → Err(CorruptedPdf)
TC-V005  0-page PDF                   → Err(EmptyPdf)
TC-V006  password-protected PDF       → Err(EncryptedPdf)
TC-V007  500-page PDF                 → Ok (no crash, no OOM)
```

### 8.6 Integration Tests — PDF Split E2E

```
TC-E001  10p, ppf=2 → 5 files         each has 2 pages, opens fine
TC-E002  7p, ppf=3 → last file has 1p [3p, 3p, 1p]
TC-E003  1-page PDF                   → 1 file
TC-E004  output dir doesn't exist     → created automatically
TC-E005  file clash, overwrite=false  → Err (no overwrite)
TC-E006  file clash, overwrite=true   → overwrite succeeds
TC-E007  dry_run=true                 → returns plan only, no files created
TC-E008  scanned PDF                  → split succeeds, quality unchanged
TC-E009  PDF with annotations         → annotations stay on their pages
```

### 8.7 Integration Tests — Dioxus Components

```
TC-UI001  FileDropZone renders         class="drop-zone", Browse button present
TC-UI002  ConfigPanel shows config     slider value = config.pages_per_file
TC-UI003  ConfigPanel emits new value  on_config_changed called, original unchanged
TC-UI004  SplitProgress done=3,total=5 bar width="60%", text="3 / 5 files"
TC-UI005  SplitProgress done=0         bar width="0%"
TC-UI006  App screen=Welcome           WelcomeScreen visible
TC-UI007  App screen=Done              DoneScreen visible, Open Folder button present
TC-UI008  ChunkPreview ppf=2 total=10  shows 5 rows [1-2,3-4,...]
TC-UI009  ChunkPreview ppf changes     recalculates instantly (pure fn)
```

### 8.8 Test Coverage Matrix

```
                      Unit   Integration   UI   Property
                      ─────  ───────────   ──   ────────
Domain Models          ✓
Config                 ✓
JobStatus              ✓
Split Logic            ✓                         ✓
Naming                 ✓
AppState               ✓
File Validation                   ✓
PDF I/O E2E                       ✓
Dioxus Components                       ✓
Error Handling         ✓          ✓       ✓

Total: 59 planned test cases
(+ property-based running 100–1000 examples each)
```

---

## Summary

### FP Principles Applied

| Lecture | Concept | Applied Where |
|---|---|---|
| L1 | Immutability, Pure Functions | Entire domain layer |
| L2 | Pure Functions | `calculate_chunks()`, AppState transitions |
| L3 | ADTs | `JobStatus`, `SplitError`, `AppScreen` |
| L4 | Railway / Result | Every `fn → Result<T, SplitError>` |
| L5 | Testing | 59 test cases |
| Ch.9 | State Monad | `AppState` + Dioxus `Signal` |
| §1.7 | FP Architecture | Cargo workspace separation |
| §1.8 | Scala 3 patterns | Multi-module + opaque types for web backend |
| §1.9 | Language comparison | Haskell IO, Roc Platform |

### Architecture Summary

```
Dioxus UI (pure components)
    ↓
Application Layer (async, Signal<AppState>)
    ↓
Domain Layer (100% pure — no infra in Cargo.toml)
    ↓
Infrastructure Layer (lopdf, std::fs — separate crate)

Domain boundary  = enforced by Cargo workspace
Pure/IO separation = enforced by Rust type system
```

> "If the domain crate has no `lopdf` in its `Cargo.toml`,
>  there is no way to import `lopdf` in domain code.
>  That is the strongest compile-time enforcement Rust can give."

---

## 9. What's Left — Remaining Phases

> Phases 1 and 2 are done. Domain is solid — pure functions, ADTs, the lot. Here's what's still on the list.

---

### DONE — Phase 1: Domain Crate + TDD

Pure domain models, ADTs, smart constructors, `calculate_chunks`, `generate_output_path`. 40 tests green.

### DONE — Phase 2: Infrastructure Crate

`pdf_reader.rs`, `pdf_writer.rs`, `file_system.rs` — real PDF I/O with lopdf. TC-V and TC-E all passing. Split a 500-page PDF without issue.

---

### TODO — Phase 3: Application Layer

Glues domain + infrastructure with async coordination.

```
crates/app/src/
  split_service.rs   — async orchestration, progress updates via Signal
  app_state.rs       — AppState transitions wired to Dioxus Signal<AppState>
```

- `split_service.rs` — takes `SourcePdf + SplitConfig`, runs on a background thread, fires `Signal<AppState>` updates per `ChunkResult`
- `app_state.rs` — bridges pure `.with_status()` transitions with Dioxus reactivity
- File picker via `rfd`, path feeds into `validate_pdf`

Keep IO out of the domain layer.

---

### TODO — Phase 4: Dioxus UI — Screens + Components

Components are pure functions of `AppState` — no hidden mutable state anywhere.

```
crates/app/src/
  main.rs
  components/
    file_drop_zone.rs    (TC-UI001)
    config_panel.rs      (TC-UI002-003)
    chunk_preview.rs     (TC-UI008-009)
    split_progress.rs    (TC-UI004-005)
  screens/
    welcome_screen.rs
    file_loaded_screen.rs
    splitting_screen.rs
    done_screen.rs       (TC-UI006-007)
    error_screen.rs
  assets/style.css
```

TC-UI001–TC-UI009 to be written and turned green.

---

### TODO — Phase 5: Polish + QA

- Error messages — tell the user exactly what went wrong and how to fix it
- Open Folder button — fires `open::that(output_dir)` after split
- Edge cases — scanned PDFs, annotated PDFs, large files (NFR-001: 100p < 10s)
- Cross-platform paths — macOS, Windows, Linux
- Final coverage audit — 100% domain, 100% infrastructure, as high as possible on app layer

---

### Scoreboard

```
Phase   What                               Status
──────  ─────────────────────────────────  ──────────────
  1     Domain crate + 40 unit tests        DONE
  2     Infrastructure + 15 integration     DONE
  3     Application layer (async/Signal)    TODO
  4     Dioxus UI (5 screens, 9 components) TODO
  5     Polish, edge cases, QA              TODO

Tests passing:   55 / 59
Remaining:       TC-UI001–009 (Dioxus components)
```
