# Cantoboard/librime Rebase Summary

This file summarizes the fork-specific work found in
`https://github.com/Cantoboard/librime` and notes which changes were later picked
or adapted into `https://github.com/TypeDuck-HK/librime`.

The Cantoboard refs were fetched locally into `refs/tmp/cantoboard/*`.

## 1. Scope

- Main branch summarized: `refs/tmp/cantoboard/master`
- Cantoboard master head: `a7b7148c31ce515768cebeb06322e0ff479730a4`
  (`Add quick start API to start Rime without updating schema and config.`,
  2021-11-10)
- Merge base with current `upstream/master`:
  `0ee64d4541b66f7b9397094cdd45260bfb4ac00c`
- Cantoboard master fork delta versus current upstream: 18 Cantoboard-only
  commits, 427 upstream-only commits.
- Cantoboard master delta versus current TypeDuck checkout: 18 Cantoboard-only
  commits, 218 TypeDuck-only commits.

Cantoboard also has experimental side branches:

- `refs/tmp/cantoboard/abv-index`
- `refs/tmp/cantoboard/inc-search`
- `refs/tmp/cantoboard/edge-1`
- `refs/tmp/cantoboard/fix-sort-N-entries`

The main summary below is for Cantoboard `master`. Side-branch work that matters
to TypeDuck is listed separately, because TypeDuck cherry-picked at least one
side-branch commit.

## 2. High-level Themes

Cantoboard's main fork work is concentrated in these areas:

- Mobile/iOS support and startup behavior:
  iOS cross-compilation, mobile keyboard correction maps, disabling dictionary
  compilation on constrained iOS keyboard extensions, and a quick-start API that
  starts Rime without full schema/config maintenance.
- Correction and candidate ordering:
  correction candidates are heavily penalized, corrections are restricted to
  normal spellings, user dictionary candidates stop outranking better system
  dictionary candidates merely because they have the same code length, and
  sentence making is triggered when the first candidate is a correction.
- Reverse lookup behavior:
  reverse lookup can fall back to per-character lookup when a whole phrase has no
  reverse mapping, and reverse lookup pronunciations are sorted by source entry
  weight.
- Dictionary/query performance:
  experiments around `DictEntryIterator` sorting, replacing sparse maps with
  vectors for syllable graph indices, a fixed-size `IndexCode` for table index
  traversal, and slow reverse database build fixes.

## 3. Mobile and Startup Changes

### 3.1 Ignore local editor/system files

Relevant Cantoboard commit:

- `9659a38ac11ee68237a232705d4166c873471565`
  `Add .vscode & .DS_Store to .gitignore.`

Technical details:

- Adds `.vscode` and `.DS_Store` to `.gitignore`.
- This is housekeeping only and has no runtime effect.

TypeDuck status:

- Not directly picked as a Cantoboard change. TypeDuck has separate ignore-file
  churn from its own history and upstream merges.

### 3.2 iOS cross-compilation support

Relevant Cantoboard commits:

- `a9563f79b80d5850d00dcac568bd7288b67d856f`
  `Add iOS cross compilation support.`
- `b261736f99ff7960c5af4e21860425d23f1e1657`
  `Fix iOS cross compilation.`
- `820c4ddfd77951b1aeca7c4f39c0d2023c6fc554`
  `Revert iOS related build changes. I moved them to another repo.`

Technical details:

- Adds `README-iOS.md`.
- Updates `CMakeLists.txt`, `thirdparty.mk`, and `xcode.mk` for iOS builds.
- Upgrades the vendored `glog` pointer because the older version reportedly
  segfaulted in the iPhone simulator.
- Adds iOS-oriented Xcode build variables and third-party build changes.
- Later removes most CMake-level iOS build wiring from this repository when the
  iOS build work was moved elsewhere.
- `b261736f` removes one `thirdparty.mk` line and adjusts `xcode.mk` after the
  initial iOS support.

TypeDuck status:

- Not picked as a TypeDuck runtime feature.
- TypeDuck has separate WebAssembly/mobile-oriented dependency changes in the
  local checkout, but those are not direct Cantoboard iOS build picks.

### 3.3 Disable dictionary recompilation on iOS

Relevant Cantoboard commits:

- `1a4a80e3cc2af9e61cfc62126028199599f41cfe`
  `Disable dict recompilation on iOS.`
- `d833c88a0f6ea26c268e434b2523fe01c4b90463`
  `Add missing include header.`
- `a7b7148c31ce515768cebeb06322e0ff479730a4`
  `Add quick start API to start Rime without updating schema and config.`

Technical details:

- Adds an iOS guard in `src/rime/dict/dict_compiler.cc`.
- The motivation in the commit message is that iOS keyboard extensions have a
  tight memory limit, and dictionary recompilation can crash under that limit.
- The first implementation leaves `DictCompiler::Compile()` mostly intact but
  forces `rebuild_table = false` and `rebuild_prism = false` on iOS devices
  excluding the simulator.
- The final `a7b7148c` version moves the iOS guard earlier:
  `DictCompiler::Compile()` logs that compilation is disabled and returns `true`
  immediately for iOS device builds.
- `d833c88a` adds the missing platform include needed for the `TARGET_OS_IPHONE`
  guard.

TypeDuck status:

- The specific iOS `DictCompiler::Compile()` early return is not picked in the
  current TypeDuck code.
- TypeDuck has its own startup/deployment avoidance work:
  `24f4b3810ed06916821b22dee0528c2ec9bba503`
  (`Prevent Updating Schemata & Configs on Startup`) and
  `df41bc9af0ba70c63f86d3532f929da2368a34f3`
  (`Prevent Dictionary Building on Startup`), but these are not direct
  cherry-picks of the Cantoboard iOS guard.

### 3.4 Quick-start API

Relevant Cantoboard commit:

- `a7b7148c31ce515768cebeb06322e0ff479730a4`
  `Add quick start API to start Rime without updating schema and config.`

Technical details:

- Adds `RimeStartQuick()` to `src/rime_api.cc` and declares it in
  `src/rime_api.h`.
- `RimeStartQuick()` loads deployer modules, runs `clean_old_log_files`, runs
  `installation_update`, schedules `user_dict_upgrade`, and starts maintenance.
- It intentionally avoids the full maintenance path used by
  `RimeStartMaintenance()`.
- In Cantoboard this same commit also changes the iOS dictionary compiler guard,
  as described above.

TypeDuck status:

- Picked/adapted into TypeDuck:
  - TypeDuck `02627c08c27aa825565958f6e713a37ae59b1f3d`
    `Add quick start API to start Rime without updating schema and config.`
  - TypeDuck `70b91220e29492848c816a7228988ceee39661f6`
    `RIME API`
  - TypeDuck `980074cbce4e29c0a90839ba1872b0a27e6da81d`
    `Export All RIME APIs Exported in rime_get_api Individually`
- TypeDuck `02627c08` ports the API function and header declaration, but not the
  Cantoboard iOS `DictCompiler::Compile()` early return.
- TypeDuck `70b91220` adds the quick-start method into `rime_api_t`.
- TypeDuck `980074cb` later keeps it available in the individually exported API
  surface.

## 4. Correction and Candidate Ordering

### 4.1 Prevent correction candidates from preceding normal candidates

Relevant Cantoboard commit:

- `733eedc8ea61d73509425d06d6a9ebbf53f8711f`
  `Fix correction candidates preceding normal candidates.`

Technical details:

- In `src/rime/algo/syllabifier.cc`, changes correction handling so only normal
  spellings can become correction spellings.
- If a prism match is not an exact syllable match and `corrector_` is active,
  the new code checks `props.type != kNormalSpelling`; non-normal spellings are
  skipped rather than marked as corrections.
- Raises the correction penalty substantially in Cantoboard by changing
  `kCorrectionCredibility` from `log(0.01)` to `-50`.
- In `src/rime/dict/table.cc`, moves `TableAccessor accessor(...)` inside the
  spelling-properties loop and calls `query.Access(syll_id, props->credibility)`.
- That makes each spelling path carry its own credibility/penalty into table
  access instead of sharing one accessor for all spellings of the same syllable.

TypeDuck status:

- Explicitly cherry-picked/adapted:
  - TypeDuck `2f79c3abfbe6f485b5f0dfc151d864af60ef6f58`
    `Fix correction candidates preceding normal candidates.`
  - Cherry-pick trailer points to Cantoboard
    `733eedc8ea61d73509425d06d6a9ebbf53f8711f`.
- TypeDuck preserved the important behavior:
  non-normal spellings are not accepted as corrections, and table access carries
  `props->credibility`.
- TypeDuck's adapted commit does not include the exact Cantoboard
  `kCorrectionCredibility = -50` diff in that commit, because the surrounding
  TypeDuck syllabifier code had already diverged.

### 4.2 Do not prefer user-dictionary correction candidates over better system candidates

Relevant Cantoboard commit:

- `d02f26fcced5d9b3b426b394f5cbb868693bdd1c`
  `Fix Script Translator to not prefer correction candidate from user dict.`

Technical details:

- Adds `ScriptTranslation::PreferUserPhrase()` in
  `src/rime/gear/script_translator.cc`.
- Before this change, user dictionary phrases were preferred when their code
  length was at least as long as the system dictionary phrase code length.
- New logic compares both code length and weight:
  user phrases are preferred only if they have a longer code length, or the same
  code length and a weight at least as high as the system phrase.
- Calls `PreferUserPhrase()` from both `Next()` and `PrepareCandidate()`.
- Before sentence construction, temporarily prepares the first candidate and
  checks `syllabifier_->IsCandidateCorrection(*candidate_)`.
- Sentence making is triggered if the translated length is short or if the first
  candidate is a correction, as long as there is more than one syllable edge.
- Cantoboard also changes syllabifier constants:
  `kPenaltyForAmbiguousSyllable = log(0.5)` and
  `kCorrectionCredibility = log(1e-50)`.

TypeDuck status:

- Explicitly cherry-picked/adapted:
  - TypeDuck `76da593b97a02abbbc6cba2e626dd0f3d56cbc6d`
    `Fix Script Translator to not prefer correction candidate from user dict.`
  - Cherry-pick trailer points to Cantoboard
    `d02f26fcced5d9b3b426b394f5cbb868693bdd1c`.
- TypeDuck picked the `PreferUserPhrase()` translator behavior and the
  first-candidate-is-correction sentence-making trigger.
- TypeDuck's adapted commit does not include the syllabifier penalty constant
  changes from Cantoboard in that same commit.

### 4.3 DictEntryIterator sorting experiments

Relevant Cantoboard commits:

- `21e502a6db84c2f15eaafd4ff54a08bf31419c7c`
  `Sort the first N entries returning from dictionary to improve correction candidate ordering.`
- `84c8799b387489db1c690394fb9670417eabd4f8`
  `Revert "Sort the first N entries returning from dictionary to improve correction candidate ordering."`
- `fb080d5fb9768e1759a3ed487e5819eb3424b374`
  `Another attempt to fix DictEntryIterator sorting issue.`
- Side branch `fix-sort-N-entries`:
  `af8c6621ba379914ce2293e866abe30c1466dd58`
  `Sort for the top 30 entries returning from dictionary`

Technical details:

- The first attempt changes `DictEntryIterator::Sort()` to partial-sort the top
  30 remaining chunks instead of only moving the single best chunk to the current
  position.
- This was intended to improve candidate ordering when correction candidates are
  involved.
- That top-30 sort is reverted on Cantoboard master.
- The later attempt changes `DictEntryIterator::FindNextEntry()` so that after
  advancing within a non-exhausted chunk, it calls `Sort()` again and returns
  `true`; if the iterator becomes exhausted, it returns `false`.
- The side branch `fix-sort-N-entries` carries another version of the top-30
  partial-sort idea, but it is not on Cantoboard master.

TypeDuck status:

- No direct TypeDuck cherry-pick found for these sorting commits.
- TypeDuck later receives upstream dictionary iterator/performance refactors,
  but those are not Cantoboard picks.

### 4.4 Do not autocorrect digits; mobile keyboard correction map

Relevant Cantoboard commits:

- `f58b94a0d36e16fdf3f2ab582731f997698f230a`
  `Change autocorrect to not correct digits.`
- `ac4bddd5f65d44e98c05b6520d3923a8073626ce`
  `Change corrector keymap to better fit iOS keyboard.`

Technical details:

- `f58b94a` removes digit keys from the correction adjacency map.
- It also makes letter adjacency less desktop-keyboard-only by adding some
  diagonal/nearby relations, such as `s` near `q`/`w`, `d` near `w`/`e`, and so
  on.
- `ac4bddd5` then replaces the map with an iOS-style mobile keyboard adjacency
  map:
  number row and punctuation adjacency are removed; `p`, `l`, and `m` no longer
  point to bracket/comma/punctuation neighbors; and vertical/diagonal neighbors
  reflect a mobile soft keyboard layout.

TypeDuck status:

- Adapted manually:
  - TypeDuck `8dc9e9c4fe917e1618b9a32ec0b5d2237f233bee`
    `Change corrector keymap to better fit mobile keyboard.`
- TypeDuck does not simply replace the desktop map. It wraps the Cantoboard-like
  mobile map in:
  `#if defined(__APPLE__) && TARGET_OS_IPHONE || defined(__ANDROID__)`
- Therefore desktop builds keep the original desktop keyboard adjacency map,
  while iOS and Android use the mobile map.
- This TypeDuck commit has no cherry-pick trailer, but the subject, author date,
  and map contents match Cantoboard `ac4bddd5` closely, with the Android
  extension and preprocessor gating added by TypeDuck.

## 5. Reverse Lookup Changes

### 5.1 Word-by-word reverse lookup fallback

Relevant Cantoboard commit:

- `52b09e22fd5a32a989fb49fb71b1a986d2b729c3`
  `Change ReverseLookupFilter to reverse lookup word by word if the phrase doesn't have a reverse mapping.`

Technical details:

- Adds `#include <utf8.h>` in `src/rime/gear/reverse_lookup_filter.cc`.
- If `rev_dict_->ReverseLookup(phrase->text(), &codes)` fails for the whole
  phrase, it iterates the phrase text by UTF-8 code point.
- For each character/code point, it constructs a one-character string and calls
  `rev_dict_->ReverseLookup(word_in_phrase, &cur_word_codes)`.
- Each found code is formatted through `comment_formatter_`.
- Per-character codes are joined with spaces and set as the phrase comment.

TypeDuck status:

- Not picked.
- Current TypeDuck reverse lookup work instead focuses on displaying both the
  reverse code and original pronunciation/comment:
  - `3e90bf975d9ad0ff6a50b8f6de5b4fcd25d928fe`
    `Show Both Reverse Code and Original Comment`
  - `3f7b9a36a20e44682e2d3d758cfd48859f4a858a`
    `Separate Reverse Lookup Pronunciations by ;`
- Current TypeDuck `reverse_lookup_filter.cc` still only checks the whole phrase
  in the filter path.

### 5.2 Sort reverse lookup pronunciations by weight

Relevant Cantoboard commits:

- `29bab99103151803b55d9325871b5575475a8f99`
  `Sort reverse lookup by weight in decreasing order.`
- `8adf7ec7cd34010267b9bc48096b56ea38415e19`
  `Fix slow reverse db build.`

Technical details:

- In `src/rime/dict/reverse_lookup_dictionary.cc`, builds a
  `textSyllableWeights` map keyed by `entry text + syllable`, with the source
  entry weight as the value.
- When a reverse lookup key has multiple syllables/codes, copies the set into a
  vector and sorts by descending source entry weight.
- Joins the sorted entries into the stored reverse lookup value.
- The follow-up commit changes the sort lambda capture from by-value to by-
  reference (`&key`, `&textSyllableWeights`) to avoid expensive map copies during
  reverse database construction.

TypeDuck status:

- Not directly picked.
- TypeDuck has its own reverse lookup display changes, but not this reverse DB
  weight-ordering implementation.

## 6. Dictionary and Query Performance

### 6.1 Fixed-size `IndexCode` for table query traversal

Relevant Cantoboard master commit:

- `c865fdfe1f3a74cb67f32fd8ebac6f36b2995bc1`
  `Optimization: Use fixed size runtime allocation free IndexCode struct to speed up Table::Query.`

Related Cantoboard side-branch commits:

- `8d9fe5128aca7e437de17a3d6dc377c2c899fa1c`
  same subject on side branches.
- `cef660ee4e41e66b0780b5fea825dad8a94b3079`
  `Fix IndexCode bug`
- `29a585493ea6ccf229e943100f53766c6d5eacda`
  `Fix IndexCode iterator`

Technical details:

- Adds `kIndexCodeMaxLength = 3` outside `Code`.
- Adds `struct IndexCode : public std::array<SyllableId, kIndexCodeMaxLength>`.
- `IndexCode` tracks its own `size_` and exposes `clear()`, `pop_back()`,
  `push_back()`, and `size()`.
- `TableQuery` stores `IndexCode index_code_` instead of `Code index_code_`.
- `TableAccessor` constructors and `index_code()` return `IndexCode`.
- `TableAccessor::code()` converts `IndexCode` back into a dynamic `Code` only
  when needed, appending `extra_code()` when present.
- The goal is to avoid repeated heap allocation while traversing the fixed first
  three syllables of the table index.

TypeDuck status:

- Not picked in current TypeDuck.
- Current TypeDuck still exposes `Code::kIndexCodeMaxLength` and does not define
  Cantoboard's separate `IndexCode` type.

### 6.2 Vector-backed syllable graph indices

Relevant Cantoboard side-branch commit:

- `5c3c7ba080cce39a1119f16affd56d35ef2144c9`
  `Use vector to speed up`

Technical details:

- Changes `SpellingIndices` from a map/hash map keyed by input position to a
  `vector<SpellingIndex>`.
- `Syllabifier::Transpose()` clears and resizes `graph->indices` to
  `graph->interpreted_length`, then writes indices directly by input position.
- `Dictionary::match_extra_code()`, `Table::Query()`, and
  `UserDictionary::DfsLookup()` use direct indexing instead of
  `indices.find(current_pos)`.
- The side-branch source commit also contained temporary `std::cout` debug lines
  in `syllabifier.cc`.

TypeDuck status:

- Explicitly cherry-picked/adapted:
  - TypeDuck `34e706e27811ecf75febbd586cc5b465b3b1ddca`
    `Use vector to speed up`
  - Cherry-pick trailer points to Cantoboard
    `5c3c7ba080cce39a1119f16affd56d35ef2144c9`.
- TypeDuck keeps the vector-backed `SpellingIndices` idea and updates
  dictionary/table/user-dictionary call sites.
- TypeDuck omits the debug `std::cout` lines from the Cantoboard side-branch
  source commit.
- TypeDuck adapts the `Table::Query()` boundary logic to its own completion and
  prediction code paths.

### 6.3 Other side-branch performance/search experiments

Relevant Cantoboard side-branch commits:

- `d236e7f660815c75dbdfbb181891257b0f95f55b`
  `Cache made sentences.`
- `6efc7ad23bf383233e4d9d0a6f5b46b414d51628`
  `Use hash_map to speed up query`
- `531dfd10c67b2796db7bb53e1a507e6a30adbf9c`
  `ScripTranslator: Cache last query and its results`
- `b107a723e249dbba83d19bf86c7422e64ebe173c`
  `Add semi working code for incremental search`
- `a1f6c5276e312ff3bf788fc672c036a630d0988d`
  `Add first working version to find new edges`
- `b1b6c03dab92b1efa35587e99ddcbc97473c5a98`
  `More bug fixes`

Technical details:

- Sentence caching adds a small cache around `MakeSentence()` results in
  `script_translator.cc`.
- Query-speed experiments change syllable graph index container types before the
  final vector-backed approach.
- Incremental-search experiments add dictionary/table APIs and script translator
  cache state to reuse prior query work as input grows.
- The commit message for `b107a723` explicitly calls the code "semi working" and
  notes a known failure mode when extending an input after cached subtrees have
  already been populated.
- The `edge-1` branch experiments with finding new graph/table edges and changes
  `table.cc`, `table.h`, `string_table.*`, `syllabifier.*`, and
  `vocabulary.h`.

TypeDuck status:

- No direct TypeDuck picks found except the later vector-backed index commit
  `5c3c7ba` described above.
- These side branches should be treated as experiments unless intentionally
  re-evaluated.

### 6.4 Abbreviation/index encoding for initials-only input

Relevant Cantoboard side-branch commit:

- `df1de5b7f61b0406ddaffd13423aa0bd60310924`
  `Add special encode rules to optimize initials only input`

Technical details:

- Adds a new spelling/encoding type for abbreviation encoding.
- Adds dictionary setting support for generating abbreviation encodings.
- During prism build, syllables of length 1 can be collected into a special
  abbreviation syllabary and marked with `kAbbreviationEncoding`.
- Adds `override_code` support while compiling entries:
  `EntryCollector` can collect an override code, `DictCompiler::BuildTable()`
  converts it into syllable IDs, and the resulting `DictEntry` carries it.
- Updates table/dictionary/reverse lookup paths to preserve and query these
  special encodings.
- The branch also contains debug/experimental comments such as `// UFO`.

TypeDuck status:

- Not directly picked.
- TypeDuck's later syllabification work around abbreviations and the important
  `m` initial/final matching problem is separate and should not be assumed to be
  equivalent to this Cantoboard branch.

## 7. Complete Cantoboard Master Commit Inventory

These are the 18 Cantoboard master commits not in current upstream:

1. `9659a38ac11ee68237a232705d4166c873471565`
   `Add .vscode & .DS_Store to .gitignore.`
   - Housekeeping only.
2. `a9563f79b80d5850d00dcac568bd7288b67d856f`
   `Add iOS cross compilation support.`
   - Adds iOS README and build wiring; upgrades `glog` pointer.
3. `1a4a80e3cc2af9e61cfc62126028199599f41cfe`
   `Disable dict recompilation on iOS.`
   - Avoids table/prism rebuilds on iOS device builds.
4. `52b09e22fd5a32a989fb49fb71b1a986d2b729c3`
   `Change ReverseLookupFilter to reverse lookup word by word if the phrase doesn't have a reverse mapping.`
   - Adds per-character fallback comments for phrases.
5. `733eedc8ea61d73509425d06d6a9ebbf53f8711f`
   `Fix correction candidates preceding normal candidates.`
   - Restricts correction to normal spellings and carries credibility into table
     access.
6. `d02f26fcced5d9b3b426b394f5cbb868693bdd1c`
   `Fix Script Translator to not prefer correction candidate from user dict.`
   - Adds weighted `PreferUserPhrase()` logic.
7. `21e502a6db84c2f15eaafd4ff54a08bf31419c7c`
   `Sort the first N entries returning from dictionary to improve correction candidate ordering.`
   - Partial-sorts top 30 chunks.
8. `f58b94a0d36e16fdf3f2ab582731f997698f230a`
   `Change autocorrect to not correct digits.`
   - Removes digit correction and adjusts adjacency.
9. `ac4bddd5f65d44e98c05b6520d3923a8073626ce`
   `Change corrector keymap to better fit iOS keyboard.`
   - Replaces correction adjacency with a mobile keyboard map.
10. `d833c88a0f6ea26c268e434b2523fe01c4b90463`
    `Add missing include header.`
    - Adds missing platform header for iOS guard.
11. `29bab99103151803b55d9325871b5575475a8f99`
    `Sort reverse lookup by weight in decreasing order.`
    - Sorts reverse lookup codes by dictionary entry weight.
12. `b261736f99ff7960c5af4e21860425d23f1e1657`
    `Fix iOS cross compilation.`
    - Small `thirdparty.mk`/`xcode.mk` follow-up.
13. `84c8799b387489db1c690394fb9670417eabd4f8`
    `Revert "Sort the first N entries returning from dictionary to improve correction candidate ordering."`
    - Reverts the top-30 partial sort.
14. `fb080d5fb9768e1759a3ed487e5819eb3424b374`
    `Another attempt to fix DictEntryIterator sorting issue.`
    - Sorts after advancing a non-exhausted chunk.
15. `820c4ddfd77951b1aeca7c4f39c0d2023c6fc554`
    `Revert iOS related build changes. I moved them to another repo.`
    - Removes most iOS CMake build wiring.
16. `c865fdfe1f3a74cb67f32fd8ebac6f36b2995bc1`
    `Optimization: Use fixed size runtime allocation free IndexCode struct to speed up Table::Query.`
    - Introduces fixed-size `IndexCode`.
17. `8adf7ec7cd34010267b9bc48096b56ea38415e19`
    `Fix slow reverse db build.`
    - Captures reverse lookup sort data by reference.
18. `a7b7148c31ce515768cebeb06322e0ff479730a4`
    `Add quick start API to start Rime without updating schema and config.`
    - Adds `RimeStartQuick()` and finalizes the iOS no-compile guard.

## 8. Changes Picked or Adapted into TypeDuck

### 8.1 Explicit cherry-picks with trailers

- Cantoboard `733eedc8ea61d73509425d06d6a9ebbf53f8711f`
  -> TypeDuck `2f79c3abfbe6f485b5f0dfc151d864af60ef6f58`
  - Fixes correction candidates preceding normal candidates.
  - TypeDuck keeps the normal-spelling-only correction gate and per-spelling
    credibility table access.

- Cantoboard `d02f26fcced5d9b3b426b394f5cbb868693bdd1c`
  -> TypeDuck `76da593b97a02abbbc6cba2e626dd0f3d56cbc6d`
  - Adds `PreferUserPhrase()` and prevents user dictionary correction candidates
    from outranking better system candidates only by code length.
  - TypeDuck also keeps the "make sentence if first candidate is correction"
    behavior.

- Cantoboard side-branch `5c3c7ba080cce39a1119f16affd56d35ef2144c9`
  -> TypeDuck `34e706e27811ecf75febbd586cc5b465b3b1ddca`
  - Converts `SpellingIndices` to a vector and updates direct-index call sites
    for faster syllable graph lookup.
  - TypeDuck adapts the patch to its own completion/prediction code and omits
    Cantoboard's temporary debug prints.

### 8.2 Adapted/manual picks without cherry-pick trailers

- Cantoboard `ac4bddd5f65d44e98c05b6520d3923a8073626ce`
  -> TypeDuck `8dc9e9c4fe917e1618b9a32ec0b5d2237f233bee`
  - TypeDuck ports the mobile keyboard correction adjacency map.
  - TypeDuck improves the integration by gating it to iOS and Android while
    keeping the original desktop adjacency map for desktop builds.

- Cantoboard `a7b7148c31ce515768cebeb06322e0ff479730a4`
  -> TypeDuck `02627c08c27aa825565958f6e713a37ae59b1f3d`
  - TypeDuck ports `RimeStartQuick()` and the public header declaration.
  - TypeDuck does not port the Cantoboard iOS `DictCompiler::Compile()` early
    return from the same source commit.
  - TypeDuck later wires the function into the API struct in
    `70b91220e29492848c816a7228988ceee39661f6` and preserves direct exports in
    `980074cbce4e29c0a90839ba1872b0a27e6da81d`.

### 8.3 Not picked or only conceptually overlapping

- Cantoboard iOS cross-compilation files (`README-iOS.md`, `xcode.mk`,
  `thirdparty.mk`, iOS CMake wiring): not picked.
- Cantoboard iOS dictionary compiler early return: not picked directly.
- Cantoboard word-by-word reverse lookup fallback: not picked.
- Cantoboard reverse lookup weight ordering: not picked.
- Cantoboard fixed-size `IndexCode`: not picked.
- Cantoboard top-30 `DictEntryIterator` partial sorting: not picked; also
  reverted on Cantoboard master.
- Cantoboard abbreviation/index encoding side branch: not picked.
- Cantoboard incremental-search/edge-finding side branches: not picked, except
  for the vector-backed `SpellingIndices` optimization noted above.

## 9. Rebase Notes

- The safest direct TypeDuck carry-over set from Cantoboard is already small:
  `2f79c3ab`, `76da593b`, `34e706e2`, `8dc9e9c4`, `02627c08`, plus later API
  surface updates `70b91220` and `980074cb`.
- The most conflict-prone Cantoboard areas are `script_translator.cc`,
  `syllabifier.cc`, `table.cc`, `dictionary.cc`, `user_dictionary.cc`,
  `corrector.cc`, and `rime_api.*`; these are also heavily modified by both
  modern upstream and TypeDuck.
- Do not blindly replay Cantoboard `master`: it contains obsolete iOS build
  scaffolding, reverted experiments, and changes that TypeDuck intentionally
  adapted differently.
- For a modern upstream rebase, treat Cantoboard as a source of ideas/patch
  origins, not as a branch to merge.
