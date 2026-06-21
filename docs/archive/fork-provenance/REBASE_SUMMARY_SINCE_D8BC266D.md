# TypeDuck librime changes since d8bc266d56a8bfbcd8ee832bdeb5f3b10d2099c7

This file summarizes the work in this branch after base commit
`d8bc266d56a8bfbcd8ee832bdeb5f3b10d2099c7` so the changes can be
recovered or rebased onto a newer upstream librime.

The range reviewed was:

```text
d8bc266d56a8bfbcd8ee832bdeb5f3b10d2099c7..74cb52b78fb2411137a7643f6c8bc6517acfde69
```

There are also uncommitted local changes in the checkout at the time this file
was written. Those are listed separately near the end.

## High-level payload

- Cantonese/Jyutping-oriented syllabification fixes, including the important
  `m` initial/standalone syllable case where typing `m` must still match the
  word `唔` and must not be disqualified merely because `m` can also be an
  abbreviation.
- Candidate completion and iOS-style long-word prediction: after enough complete
  syllables are typed, the translator can show the best longer dictionary word
  above a frequency threshold, e.g. `santai` can predict `身體` or `身體健康`,
  and code matching can find `市建局` from `sigin` even when there is no `市建`
  word entry.
- Configurable sentence making / automatic composition, including the ability
  to disable it and logic that avoids using auto-composed sentences when a true
  exact phrase/user phrase already exists.
- User dictionary / input memory stores the component elements and full code
  information needed to keep pronunciations available after automatic
  composition or prediction candidates are committed.
- Reverse lookup comments can preserve both the reverse code and original
  pronunciation/comment. Reverse lookup pronunciations are separated with `; `.
- Multiple pronunciations for the same candidate text can be shown and adjacent
  candidates with the same text can be grouped, e.g. `斷 dyun6; dyun3`.
- Fullwidth/halfwidth UI labels were changed from `全角` / `半角` to the
  Traditional Chinese convention `全形` / `半形`.
- New RIME API and API-console helpers were added for TypeDuck clients,
  debugging, and mobile/Android setup.
- Build, CI, schema data, and release packaging were adjusted for TypeDuck,
  including the switch from checked-in minimal schemas to a `build/bin` schema
  submodule.
- A large upstream librime merge brought this branch up to librime 1.11.0-era
  code, with TypeDuck changes carried through conflicts.

## 1. Syllabification, correction, completion, and the `m` case

Relevant commits:

- `41684211` - Fix: Perfect Matches Disqualify Abbreviations
- `2f79c3ab` - Fix correction candidates preceding normal candidates
- `76da593b` - Fix Script Translator to not prefer correction candidate from user dict
- `34e706e2` - Use vector to speed up
- `3aa87595` - Fix penalty not applied to `m` as abbreviation when final letter is also abbreviation
- `c4b75c10` - Apply more completion; fine tune candidate orders
- `a802dd98` - Fix syllabification sometimes incorrect
- `68665e83` - Do not construct sentences with correction or completion syllables
- `071afb37` - fix syllabifier.cc access violation
- `b15c026e` - Fix infinite loop in syllabifier
- `b30fa281` - Fix infinite loop in syllabifier with `enable_correction`
- `585f4656` - Separate `enable_correction` from `enable_completion`
- `dd85cb9c` - Fix again: new prediction syllabification
- `adbbe0d8` - Revert "Increase Correction Penalty"
- `520a2474` - Revert "Apply more completion; Fine tune candidate orders"
- `c77d5375` - Improve correction of monosyllabic candidates
- `81e13724` - Disqualify corrections with non-minimal distance

Final behavior and implementation notes:

- `src/rime/algo/syllabifier.cc` now keeps several distinct penalty constants:
  - `kCompletionPenalty = log(0.5)`
  - `kPenaltyForAmbiguousSyllable = log(0.1)`
  - `kCorrectionCredibility = log(1e-7)`
  - `kPenaltyForDisfavoredType = log(1e-14)`
- `src/rime/algo/spelling.h` adds `kCorrection` as a spelling type between
  `kNormalSpelling` and `kFuzzySpelling`. Corrections are no longer only tracked
  by `is_correction`; they also get their own type so stale-edge pruning can
  compare them correctly against normal/fuzzy/abbreviation/completion paths.
- `Prism::Match` in `src/rime/dict/prism.h` now carries an edit `distance`.
  `Syllabifier::BuildSyllableGraph()` copies `Corrector::ToleranceSearch()`
  distances into matches and multiplies the correction credibility penalty by
  distance.
- Corrections are accepted only when:
  - the match was not already an exact spelling match,
  - the spelling properties are `kNormalSpelling`,
  - the correction distance is the minimal distance among correction candidates
    for that input position.
- Correction tolerance is currently called with tolerance `4`.
- Non-minimal-distance corrections are discarded. This avoids farther correction
  candidates competing with nearer corrections.
- The graph-pruning pass was changed from deleting all less-favored paths to
  penalizing some disfavored paths and preserving cases that still matter. This
  is the important part for the `m` case: an exact/normal match should not erase
  an abbreviation path that is still needed to match standalone or initial `m`.
- The pruning logic computes `min_edge_type` and `max_edge_type` for viable
  outgoing edges. Edges with worse types than the best available type are:
  - removed when they are correction edges or shorter than the farthest edge
    from the same start,
  - otherwise preserved with `kPenaltyForDisfavoredType`.
- `CheckOverlappedSpellings()` now applies a softer ambiguity penalty
  `log(0.1)` instead of the old much stronger `log(1e-10)`.
- `SyllableGraph::indices` changed from `map<size_t, SpellingIndex>` to
  `vector<SpellingIndex>` for faster lookup by input position. `Transpose()`
  now clears and resizes this vector to `interpreted_length`.
- Completion uses `Prism::ExpandSearch()` from the farthest interpreted
  position when `enable_completion_` is true and the graph did not consume the
  entire input.
- Completion only inserts spellings whose original type is better than
  `kAbbreviation`; those inserted spellings are marked `kCompletion` and receive
  `kCompletionPenalty`.
- `strict_spelling_` continues to reject fuzzy/abbreviation spellings when the
  whole input is a single exact candidate.
- `Table::Query()` can now be called with `with_correction = false`; when false,
  it skips correction and completion syllables. This is used by sentence making
  so automatic composition is not built from typo corrections or incomplete
  syllables.
- `ScriptTranslation` treats both corrections and completions as non-normal
  candidates when deciding correction limits and sentence-making fallback.
- `enable_correction` is independent from `enable_completion`. Completion no
  longer silently enables correction.

## 2. Candidate completion and long-word prediction

Relevant commits:

- `a01dd1af` - Candidate Completion & Prediction
- `c4b75c10` - Apply more completion; Fine tune candidate orders
- `a802dd98` - Fix syllabification sometimes incorrect
- `68665e83` - Do not construct sentences with correction/completion syllables
- `77337c2d` - Temporarily fix excessive predictions
- `245543ec` - Increase prediction threshold to frequency >= 100
- `e3137a75` - Fix duplicated prediction candidate after commit
- `585f4656` - Separate correction from completion
- `dd85cb9c` - Fix prediction syllabification regression
- `520a2474` - Partial revert of overly aggressive completion/order tuning
- `0754e5bf` - Revise user candidate order calculation formula

Final behavior and implementation notes:

- Prediction is exposed through existing `enable_completion`. It only runs when
  the current segment reaches the end of the full context input:
  `translator_->enable_completion() && start_ + consumed == end_of_input_`.
- Dictionary prediction threshold is `kPredictionThreshold = log(100)`, i.e.
  only entries with frequency/weight equivalent to at least 100 are eligible.
- `Table::Query()` now has signature:
  `Query(syll_graph, start_pos, result, predict_word, with_correction)`.
- `TableQuery::AccessAll()` recursively enumerates all longer entries under the
  current code prefix so prediction can retrieve words beyond the typed code.
- `Table::Query()` uses negative result keys for predictions:
  - at end of input, if `predict_word` is true and the traversed code used
    regular spellings with `query.level() >= 2`, it stores predictive accessors
    under `(*result)[-query.level()]`;
  - the negative key records how many syllables of the entry's code matched the
    typed input.
- `Dictionary::lookup_table()` interprets negative end positions as prediction
  buckets, keeps only the best predicted entry, and stores it in collector key
  `-1` if it passes `log(100)`.
- `dictionary::Chunk` now stores `matching_code_size`. Exact matches sort before
  predictive matches. Predictive matches carry a shorter matching code than the
  full dictionary code.
- `DictEntry` now stores `matching_code_size`; `DictEntryIterator::Peek()`
  copies the predictive `matching_code_size` to the emitted entry.
- `Phrase` exposes:
  - `matching_code_size()`
  - `is_exact_match()`
  - `is_predicitve_match()` (typo in method name preserved in code)
  - `matching_code()`
- `ScriptSyllabifier::GetPreeditString()` syllabifies `cand.matching_code()`
  instead of the full word code. This is essential for predictions: the preedit
  should correspond to the typed part, not the full predicted word.
- `ScriptTranslation::Evaluate()` converts dictionary collector key `-1` into a
  `Phrase` of type `predicted_phrase`, spanning the typed/interpreted input.
- The predicted phrase is deliberately not first when there is another candidate
  to show first. `PrepareCandidate()` only chooses `prediction_` after
  `cand_count_` is nonzero and either:
  - there is no current normal candidate,
  - the prediction has higher weight,
  - or the current phrase/user phrase covers less than the interpreted input.
- This matches the intended mobile/iOS-like behavior: after enough complete
  syllables, a longer frequent phrase can be suggested, but ordinary exact
  matches get the first chance.
- Because matching is done against numeric code paths, not against Chinese text,
  a longer word such as `市建局` can be found from the typed code for `sigin`
  even if there is no separate `市建` dictionary word.
- User dictionary prediction exists too:
  `user_dict->Lookup(..., predict_word ? kNumSyllablesToPredictWord : 0)`.
  The final constant is `kNumSyllablesToPredictWord = 4`, so user-dictionary
  predictive lookup starts only after that configured depth.
- `Memory::CommitEntry::AppendPhrase()` uses `phrase->full_code()` rather than
  `phrase->code()` when committing a prediction. This prevents a committed
  predicted phrase from being saved with only the partial typed code, which was
  causing duplicate candidates with mismatched comments/pronunciations.

## 3. Automatic composition / sentence making controls

Relevant commits:

- `5e50fcdb` - Allow disabling sentence-making
- `68665e83` - Do not construct sentences with correction/completion syllables
- `2ea5f56f` - Always prefer composition with fewer syllables
- `c1938644` - Decrease composition word penalty
- `520a2474` - Partial revert of previous completion/order tuning
- `0754e5bf` - Revise user candidate order calculation formula

Final behavior and implementation notes:

- `ScriptTranslator` has `enable_sentence_ = true` and reads
  `translator/enable_sentence`.
- `ScriptTranslator::Memorize()` also reads `translator/encode_commit_history`;
  if disabled, commit history is not encoded into the user dictionary.
- Sentence making is skipped completely when `enable_sentence` is false.
- Sentence making requires at least two syllable-graph edges.
- Sentence making happens when there is no exact dictionary phrase and no exact
  user phrase covering the consumed input, or when the first candidate is a
  correction.
- Exactness is checked with `DictEntry::matching_code_size`; predictive matches
  are not treated as exact matches.
- `MakeSentence()` queries user phrases up to `kMaxSyllablesForUserPhraseQuery =
  5` and dictionary entries at every graph edge, then calls `Poet::MakeSentence`.
- Sentence entries are offset to the segment start and receive the same
  `ScriptSyllabifier` so comments/preedit can be recovered.
- `grammar.h` / `poet.cc` / contextual translation received tuning so
  compositions with fewer syllables are preferred, then later the composition
  word penalty was reduced from the first attempt.
- Candidate ordering between normal dictionary entries and user dictionary
  entries now uses `cand_count_` and:
  `cand_count_ >= exp(2) - exp(user_phrase_weight + exp(2))`.
  This replaced the earlier simpler user-weight comparison.

## 4. Multiple pronunciations and candidate grouping

Relevant commits:

- `0b5dd737` - Fix: only a single pronunciation is shown for each candidate
- `3f7b9a36` - Separate reverse lookup pronunciations by `; `
- `97b193f7` - Make `combine_candidates` an option
- `c44ab537` - Fix: last candidate is missing

Final behavior and implementation notes:

- `DistinctTranslation` no longer deduplicates solely by candidate text. It
  deduplicates by `(text, comment)`, so `斷 dyun6` and `斷 dyun3` can both exist
  before grouping.
- When `combine_candidates` is true, `DistinctTranslation` scans adjacent
  candidates with the same text and merges their comments with `; `.
- Result example: adjacent entries can display as `斷 dyun6; dyun3`.
- When `combine_candidates` is false, the translator still removes exact
  `(text, comment)` duplicates but does not merge same-text candidates.
- `combine_candidates` is read by both `ScriptTranslator` and `TableTranslator`
  and defaults to true.
- The `c44ab537` follow-up removed logic that marked the translation exhausted
  too early, fixing a regression where the last candidate was skipped.

## 5. Reverse lookup comments

Relevant commits:

- `3e90bf97` - Show both reverse code and original comment
- `3f7b9a36` - Separate reverse lookup pronunciations by `; `
- `578a55c2` - Always show schema name in reverse lookup
- upstream merge `7573b57e` also touched reverse lookup code

Final behavior and implementation notes:

- `ReverseLookupFilter` now supports `append_comment` in addition to
  `overwrite_comment`.
- If a candidate already has a comment:
  - normal behavior still leaves it untouched,
  - `overwrite_comment` replaces it with reverse lookup codes,
  - `append_comment` appends reverse lookup codes after the existing comment.
- This enables reverse lookup to show both the reverse input code and the
  original pronunciation/comment, e.g. Cangjie code plus Jyutping.
- `ReverseLookupDictionary` joins multiple reverse lookup pronunciations/codes
  with `; ` rather than a less readable separator.
- `composition.cc` was adjusted so reverse lookup always shows the schema name.

## 6. Traditional Chinese fullwidth/halfwidth labels

Relevant commit:

- `5fe09db5` - Use `全形` and `半形` labels instead of `全角` and `半角`

Final behavior:

- `src/rime/gear/punctuator.cc` changed state labels from Mainland/Japanese
  convention `全角` / `半角` to Traditional Chinese `全形` / `半形`.

## 7. Table translator full-code comments

Relevant commit:

- `d8667c92` - Add `show_full_code` option for Table Translator

Final behavior and implementation notes:

- `TranslatorOptions` reads `translator/show_full_code`, default false.
- `TableTranslation::Peek()` normally shows the entry comment, or the unity
  symbol for constructed entries.
- If `show_full_code` is true and the options object is a `TableTranslator`, it
  replaces the comment with `TableTranslator::Spell(e->code)`.
- `TableTranslator::Spell()` decodes the numeric code through the dictionary and
  joins syllables using the first configured delimiter. `comment_formatter` is
  still applied in `TableTranslation::Peek()`.

## 8. User dictionary, input memory, and pronunciation recovery

Relevant commits:

- `d057fb75` - Lookup User Dictionary
- `e2c8c4f0` - Recursively extract entry elements, fix snapshot import
- `ae2e49cc` - Fix incorrect/missing pronunciation of sentences mixed with user dict
- `124b6836` - Fix user dictionary candidates not broken into individual entries
- `e3137a75` - Fix duplicated prediction candidate
- `0754e5bf` - Revise user candidate order calculation formula

Final behavior and implementation notes:

- `DictEntry` now has `vector<string> elements` and `int matching_code_size`.
- `UserDbValue` stores optional elements in packed user-db values as `e=...`
  with tab-separated element text.
- `UserDbValue::AppendElements()` recursively extracts elements from
  `CommitEntry` and stores constituent entries for composed candidates.
- `UserDictionary::UpdateEntry()` ensures `v.AppendElements(entry)` is called
  when elements are missing, so older-style entries still get component data
  when updated.
- `UserDictionary::CreateDictEntry()` restores `e->elements` from the packed
  user-db value.
- `CommitEntry::AppendPhrase()` appends phrase text and full numeric code; for
  sentences it appends each sentence component as an element, otherwise it stores
  the phrase's own entry.
- This is the main fix for input memory remembering pronunciation: if an
  automatically composed phrase is later recalled from the user dictionary, its
  elements and full code are still available for comments/pronunciation display.
- User dictionary DFS lookup can now predict beyond the typed input when
  `predict_word_from_depth` is nonzero:
  - when the DFS reaches the end of input and depth is high enough, it scans
    prefix matches in the user DB,
  - lazily builds a syllabary map from `table_->GetSyllabary()`,
  - converts stored full-code syllable strings back into numeric code,
  - sets `matching_code_size` to the typed code depth.
- `UserDictionary::UpdateTickCount()` / `FetchTickCount()` were moved from
  `boost::lexical_cast` to `std::to_string` / `std::stoul`.

## 9. Correction UX and mobile keyboard tweaks

Relevant commits:

- `8dc9e9c4` - Change corrector keymap to better fit mobile keyboard
- `5bdb5c78` - Increase correction penalty
- `adbbe0d8` - Revert correction penalty increase
- `c77d5375` - Improve correction of monosyllabic candidates
- `81e13724` - Disqualify corrections with non-minimal distance

Final behavior and implementation notes:

- The final correction penalty is back to `log(1e-7)` after the `5bdb5c78`
  increase was reverted.
- Correction cost scales by edit distance.
- Corrections with larger edit distance than the minimal correction at the same
  position are discarded.
- The corrector keymap was extended/tuned for mobile keyboard adjacency in
  `src/rime/dict/corrector.cc`.

## 10. RIME API and API-console changes

Relevant commits:

- `02627c08` - Add quick start API
- `2944f7d1` - Add config-list append string API
- `70b91220` - RIME API: append bool/int/double, include quick start in API struct
- `b83a2ab4` - Update API Console
- `90c4bb1c` - API Console: build `trime.yaml` for Android
- `980074cb` - Export all APIs exported in `rime_get_api` individually
- `93159863` - Print candidate weights in API Console

Final behavior and implementation notes:

- `RimeStartQuick()` starts Rime without full schema/config update:
  - loads deployer modules,
  - runs `clean_old_log_files`,
  - runs `installation_update`,
  - schedules `user_dict_upgrade`,
  - starts maintenance.
- `RimeApi` includes `start_quick`.
- Config-list append APIs were added:
  - `RimeConfigListAppendBool`
  - `RimeConfigListAppendInt`
  - `RimeConfigListAppendDouble`
  - `RimeConfigListAppendString`
- These append APIs create a `ConfigValue`, set the typed value, and append it
  to the existing `ConfigList`.
- Many APIs that were reachable through `rime_get_api()` are now also exported
  with `RIME_API` as individual symbols, including secure data-dir getters,
  raw input/caret APIs, candidate page/highlight APIs, version, and state-label
  APIs.
- `RimeCandidate` now includes `double quality`; `rime_candidate_copy()` fills
  it from `Candidate::quality()`.
- `tools/rime_api_console.cc` prints candidate quality/weight for debugging.
- The API console was adjusted to deploy/build schemata and generate extra
  debugging config such as `common.yaml` and Android `trime.yaml`.

## 11. Schema menu/mobile UI options

Relevant commits:

- `838e3d41` - Add option to hide lone schema in schema menu
- `83924c37` - Hide caret in schema menu

Final behavior and implementation notes:

- `switcher/hide_lone_schema` can hide the schema switcher candidate when only
  one schema exists.
- `SchemaListTranslation::LoadSchemaList()` checks `candies_.size() == 1`,
  reads `switcher/hide_lone_schema`, then advances past the only candidate.
- `Switcher::RefreshMenu()` tags the switcher segment with `switcher`.
- `Context::GetSoftCursor()` suppresses the soft cursor when the active segment
  has the `switcher` tag.

## 12. Build, packaging, schemas, CI, and docs

Relevant commits:

- `160e7948` - Ignore Visual Studio files
- `4ffc1c8f` - Add CMake flag
- `7a1245fe` - Use `schema` submodule
- `22dc02e4` - Update workflow for TypeDuck
- `424fd5cd` - Make workflow happy
- `b5173b54` - Temporarily drop tests in workflow
- `24f4b381` - Prevent updating schemata/configs on startup
- `df41bc9a` - Prevent dictionary building on startup
- `e8be4403` - Fix API console should deploy schemata on startup
- `5d85fb8b` - Revert "Prevent Updating Schemata & Configs on Startup"
- `ee689010` - Fix Windows common settings patch not applied
- `3a752341` - Revert Windows common settings patch
- `b96f2f3b` - Fix dead Boost download link
- `d9a9ddd0` - Fix workflow
- `92f6fade` - Remove `env.vs2017_xp.bat`
- `0cf04ab5` - Update README & development guide
- `b35c0c8f` - Fix duplicated workflow run
- release commits `3822779b`, `1564d4a2`, `d69618d5`, `3bee2a62`, `74cb52b7`

Final behavior and implementation notes:

- The huge checked-in `data/minimal` schema files were removed.
- `.gitmodules` gained/uses the `build/bin` schema submodule.
- `build/bin` was repeatedly updated by schema submodule commits:
  `61db0b50`, `dcaab281`, `b31a32bc`, `44b4e667`, `09ecab87`,
  `84df68e5`, `9213e306`, `838e3d41`, `67439a61`.
- `build.bat` and CMake options were adjusted for TypeDuck builds.
- Workflows were simplified for TypeDuck, plugin install scripts were removed,
  and duplicated workflow triggers were fixed.
- Boost download URLs were updated.
- `env.vs2017_xp.bat` was removed; docs and env templates now focus on newer
  supported Visual Studio setups.
- Version bumps:
  - `3822779b`: release build 1.0.0
  - `1564d4a2`: release build 1.0.1
  - `d69618d5`: release build 1.1.0
  - `3bee2a62`: release build 1.1.1
  - `74cb52b7`: release build 1.1.2

## 13. Upstream librime merge

Relevant commit:

- `7573b57e` - Merge From Upstream (#1, #2)

The merge was assembled through GitHub PRs:

- TypeDuck-HK/librime#1: `Merge From Upstream`, head
  `90c1cd306e61bceec1669a7890a090835d0f648c`, 144 commits.
- TypeDuck-HK/librime#2: `Merge From Upstream`, head
  `31631b4e3c7704b8449f10dda81871780fcff431`, 6 commits.

The merge commit states it includes upstream changes from
`53ce306e6be1bc73016cf7de22435f75eb6f67e4` through
`4d8bb870ff56b4976aedcaa6f56051a97a6c80ef`, corresponding to librime 1.11.0,
with some commits deliberately excluded.

Comparing upstream commit subjects in that range against PR #1 and PR #2, after
normalizing BOM-prefixed subjects used by a few TypeDuck cherry-picks, shows
these deliberately excluded upstream commits:

- `a60876745af20ecc8489ec6997c6c195949b99ac` -
  `chore(release): 1.9.0 :tada:`. This only bumps release metadata and updates
  upstream `CHANGELOG.md`; TypeDuck has its own release sequence.
- `295cb2ab68f89ee9d3237c7d4b8033bda3f3b635` -
  `chore(release): 1.10.0 :tada:`. This likewise only updates upstream release
  metadata/`CHANGELOG.md` and package versioning, so it was not carried into the
  TypeDuck release line.
- `5c7fb64be01f4f43f62c8d7dc4bee5d0ac34fed5` -
  `feat(script_translator): word completion from 2nd place (#848)`. The merge
  message explicitly says this one was temporarily excluded. It changes
  `src/rime/dict/user_dictionary.cc`, `src/rime/dict/vocabulary.cc`,
  `src/rime/dict/vocabulary.h`, `src/rime/gear/contextual_translation.cc`, and
  `src/rime/gear/script_translator.cc`, with upstream intent to prefer exact
  match phrases on top and set a word-completion type.

PR #1 also contains TypeDuck-only carry/fix commits around the upstream import,
including reverts of TypeDuck workflow/docs/comment changes, `Format Code`,
`Fix New Prediction Syllabification`, and `Fix Workflow`. PR #2 carries the
late upstream path/logging/resource-info commits after PR #1.

Important upstream/platform effects in the merged tree:

- Modernized CMake/build files, Dockerfile, Makefile, Windows resource file,
  dependency handling, and workflow files.
- Added `.dockerignore` and `.git-blame-ignore-revs`.
- Removed old bundled `deps/CMakeLists.txt`, `cmake/FindKyotoCabinet.cmake`,
  `include/msvc/stdint.h`, and `xcode.mk`.
- Updated utf8 headers and added `include/utf8/cpp17.h`.
- Added `src/rime/algo/fs.h`.
- Added DB pool headers.
- Migrated many filesystem paths and resource resolver APIs toward
  `std::filesystem`/`path` style.
- Added/updated deployment task infrastructure, user dictionary manager,
  config APIs, simplifier behavior, chord composer behavior, key binder,
  sample tools, tests, and `tools/rime_table_decompiler.cc`.
- Many TypeDuck changes above were carried through this merge, so rebasing onto
  an even newer upstream should compare both pre-merge TypeDuck feature commits
  and the post-merge final implementations.

## 14. Uncommitted local changes present while writing this summary

These were not committed in the `d8bc266d..HEAD` range, but they were present in
the working tree and may be part of the real rebase payload.

- `include/darts.h`: `DARTS_THROW` reports `__FILE_NAME__` instead of `__FILE__`,
  reducing embedded absolute path leakage in WebAssembly/browser builds.
- `deps/glog`: CMake adds `-ffile-prefix-map=${CMAKE_CURRENT_SOURCE_DIR}=.` and
  logging macros use `__FILE_NAME__` in multiple places, also reducing absolute
  path leakage.
- `deps/leveldb/util/env_posix.cc`: `PosixEnv::Schedule()` directly calls the
  background work function and returns before the normal background thread queue,
  which changes async scheduling semantics in this checkout.
- `deps/marisa-trie/include/marisa/exception.h`: `MARISA_THROW` reports
  `__FILE_NAME__` instead of `__FILE__`.
- `deps/opencc`: CMake changes add `-ffile-prefix-map`, adjust Emscripten include
  paths, avoid `-pthread` under Emscripten, remove doc/data/test subdirectories
  from the build, and stop adding the `tools` subdirectory from `src/CMakeLists.txt`.
- `deps/opencc` vendored RapidJSON: `GenericStringRef::operator=` is removed.

## 15. Chronological commit inventory

- `61db0b50` - Update submodule.
- `160e7948` - Ignore Visual Studio files.
- `0b5dd737` - Preserve multiple pronunciations by deduplicating on
  `(text, comment)` and grouping same-text adjacent candidates.
- `41684211` - Rework syllable graph pruning so perfect matches do not wrongly
  disqualify abbreviation paths.
- `5fe09db5` - Change fullwidth/halfwidth labels to `全形` / `半形`.
- `3f7b9a36` - Join reverse lookup pronunciations with `; `.
- `3e90bf97` - Let reverse lookup show both reverse code and existing comment.
- `a01dd1af` - Add dictionary completion/prediction path.
- `2f79c3ab` - Keep correction candidates behind normal candidates.
- `76da593b` - Avoid preferring user-dictionary correction candidates.
- `34e706e2` - Change syllable graph indices to vector-backed lookup.
- `dcaab281` - Update submodule.
- `3aa87595` - Fix `m` abbreviation penalty edge case.
- `c4b75c10` - Tune completion penalties and candidate order; later partially
  reverted by `520a2474`.
- `a802dd98` - Track matching/original code length to fix syllabification for
  predictions.
- `68665e83` - Prevent sentence construction from correction/completion
  syllables.
- `d057fb75` - Store and lookup user dictionary elements.
- `5e50fcdb` - Add `enable_sentence` and `encode_commit_history` controls.
- `88e36264` - Make `always_show_comments` override `spelling_hints`.
- `97b193f7` - Add `combine_candidates` option for script/table translators.
- `578a55c2` - Always show schema name in reverse lookup.
- `4ffc1c8f` - Adjust Windows build CMake flag.
- `7a1245fe` - Replace checked-in minimal schema data with schema submodule.
- `22dc02e4` - Update CI/workflows for TypeDuck.
- `424fd5cd` - Small fixes to satisfy workflow/build.
- `e2c8c4f0` - Recursively extract commit entry elements for snapshots.
- `77337c2d` - Temporary prediction over-generation fix.
- `071afb37` - Fix syllabifier access violation.
- `b15c026e` - Fix syllabifier infinite loop.
- `b5173b54` - Temporarily remove tests from CI workflow.
- `b31a32bc` - Update submodules.
- `8dc9e9c4` - Mobile-keyboard corrector keymap.
- `24f4b381` - Attempt to prevent schema/config startup update; later reverted.
- `df41bc9a` - Avoid dictionary build on startup.
- `2092da35` - Improve API console debugging.
- `44b4e667` - Update schema submodule.
- `e8be4403` - Make API console deploy schemata on startup.
- `5d85fb8b` - Revert `24f4b381`.
- `02627c08` - Add `RimeStartQuick`.
- `b30fa281` - Fix correction-enabled syllabifier infinite loop.
- `d8667c92` - Add table translator `show_full_code`.
- `ae2e49cc` - Preserve pronunciation for mixed sentence/user-dict entries.
- `2ea5f56f` - Prefer compositions with fewer syllables.
- `09ecab87` - Update schema submodule.
- `2944f7d1` - Add config-list append string API.
- `ee689010` - Attempt Windows common settings patch; later reverted.
- `124b6836` - Keep user dictionary candidates breakable into individual
  elements while preserving element data on `DictEntry`.
- `5bdb5c78` - Increase correction penalty; later reverted.
- `245543ec` - Raise prediction threshold to frequency >= 100.
- `b96f2f3b` - Fix Boost download URL.
- `d9a9ddd0` - Workflow cleanup.
- `92f6fade` - Remove VS2017 XP env file.
- `0cf04ab5` - README/development guide update.
- `e3137a75` - Store full code for committed prediction candidates.
- `84df68e5` - Update schema submodule.
- `3822779b` - Release 1.0.0.
- `3a752341` - Revert Windows common settings patch.
- `70b91220` - Expand RIME API struct and config-list append APIs.
- `b83a2ab4` - API console common.yaml/dictionary-build fixes.
- `585f4656` - Separate correction from completion.
- `9213e306` - Update schema submodule.
- `ee4775f9` - README wording update.
- `b35c0c8f` - Fix duplicated workflow run.
- `1564d4a2` - Release 1.0.1.
- `7573b57e` - Merge upstream through librime 1.11.0-era code.
- `c44ab537` - Fix last candidate missing after distinct/grouping change.
- `d69618d5` - Release 1.1.0.
- `90c4bb1c` - API console builds Android `trime.yaml`.
- `980074cb` - Export individually all APIs available through `rime_get_api`.
- `838e3d41` - Add `switcher/hide_lone_schema`.
- `83924c37` - Hide soft cursor in schema menu.
- `10bd5ade` - clang-format 18.1 formatting pass.
- `dd85cb9c` - Restore correct prediction syllabification DFS termination.
- `adbbe0d8` - Revert correction penalty increase.
- `c1938644` - Reduce composition word penalty.
- `520a2474` - Partially revert aggressive completion/order tuning.
- `93159863` - Add candidate `quality` to API and console output.
- `0754e5bf` - Revise user candidate ordering formula.
- `67439a61` - Update schema submodule.
- `3bee2a62` - Release 1.1.1.
- `c77d5375` - Scale correction penalty by edit distance.
- `81e13724` - Add `kCorrection` and discard non-minimal corrections.
- `74cb52b7` - Release 1.1.2.
