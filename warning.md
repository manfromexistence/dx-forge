warning: deny(unused_crate_dependencies) is ignored unless specified at crate level
 --> src/cli/mod.rs:2:9
  |
2 | #![deny(unused_crate_dependencies)]
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_attributes)]` on by default

warning: unexpected `cfg` condition value: `strum`
  --> src/cli/enum_support.rs:13:7
   |
13 | #[cfg(feature = "strum")]
   |       ^^^^^^^^^^^^^^^^^
   |
   = note: expected values for `feature` are: `chrono`, `console`, `crossterm`, `date`, `default`, `editor`, `fuzzy`, `fuzzy-matcher`, `macros`, `one-liners`, `tempfile`, and `termion`
   = help: consider adding `strum` as a feature in `Cargo.toml`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
   = note: `#[warn(unexpected_cfgs)]` on by default

warning: unused import: `std::str::FromStr`
 --> src/cli/enum_support.rs:1:5
  |
1 | use std::str::FromStr;
  |     ^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `ListOption` and `error::InquireResult`
 --> src/cli/enum_support.rs:4:5
  |
4 |     error::InquireResult,
  |     ^^^^^^^^^^^^^^^^^^^^
5 |     // list_option::{FromListOption, ListOption},
6 |     list_option::{ListOption},
  |                   ^^^^^^^^^^

warning: unused import: `action::*`
 --> src/cli/prompts/confirm/mod.rs:5:9
  |
5 | pub use action::*;
  |         ^^^^^^^^^

warning: unused import: `action::*`
 --> src/cli/prompts/multiselect/mod.rs:8:9
  |
8 | pub use action::*;
  |         ^^^^^^^^^

warning: unused import: `action::*`
 --> src/cli/prompts/select/mod.rs:8:9
  |
8 | pub use action::*;
  |         ^^^^^^^^^

warning: unused import: `action::*`
 --> src/cli/prompts/text/mod.rs:8:9
  |
8 | pub use action::*;
  |         ^^^^^^^^^

warning: unused import: `one_liners::*`
  --> src/cli/prompts/mod.rs:26:9
   |
26 | pub use one_liners::*;
   |         ^^^^^^^^^^^^^

warning: unused import: `self::config::set_global_render_config`
  --> src/cli/mod.rs:27:9
   |
27 | pub use self::config::set_global_render_config;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: trait `InquireEnumVariants` is never used
 --> src/cli/enum_support.rs:9:11
  |
9 | pub trait InquireEnumVariants {
  |           ^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: associated function `new` is never used
  --> src/cli/input/mod.rs:35:12
   |
34 | impl Input {
   | ---------- associated function in this implementation
35 |     pub fn new() -> Self {
   |            ^^^

warning: function `set_global_render_config` is never used
  --> src/cli/config.rs:18:8
   |
18 | pub fn set_global_render_config(config: RenderConfig<'static>) {
   |        ^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `DEFAULT_VIM_MODE` is never used
  --> src/cli/config.rs:27:11
   |
27 | pub const DEFAULT_VIM_MODE: bool = false;
   |           ^^^^^^^^^^^^^^^^

warning: variant `InvalidConfiguration` is never constructed
  --> src/cli/error.rs:19:5
   |
12 | pub enum InquireError {
   |          ------------ variant in this enum
...
19 |     InvalidConfiguration(String),
   |     ^^^^^^^^^^^^^^^^^^^^
   |
   = note: `InquireError` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

warning: multiple methods are never used
   --> src/cli/prompts/confirm/mod.rs:144:12
    |
105 | impl<'a> Confirm<'a> {
    | -------------------- methods in this implementation
...
144 |     pub fn with_starting_input(mut self, message: &'a str) -> Self {
    |            ^^^^^^^^^^^^^^^^^^^
...
150 |     pub fn with_default(mut self, default: bool) -> Self {
    |            ^^^^^^^^^^^^
...
156 |     pub fn with_placeholder(mut self, placeholder: &'a str) -> Self {
    |            ^^^^^^^^^^^^^^^^
...
162 |     pub fn with_help_message(mut self, message: &'a str) -> Self {
    |            ^^^^^^^^^^^^^^^^^
...
168 |     pub fn with_formatter(mut self, formatter: BoolFormatter<'a>) -> Self {
    |            ^^^^^^^^^^^^^^
...
174 |     pub fn with_parser(mut self, parser: BoolParser<'a>) -> Self {
    |            ^^^^^^^^^^^
...
180 |     pub fn with_error_message(mut self, error_message: &'a str) -> Self {
    |            ^^^^^^^^^^^^^^^^^^
...
186 |     pub fn with_default_value_formatter(mut self, formatter: BoolFormatter<'a>) -> Self {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
199 |     pub fn with_render_config(mut self, render_config: RenderConfig<'a>) -> Self {
    |            ^^^^^^^^^^^^^^^^^^
...
213 |     pub fn prompt_skippable(self) -> InquireResult<Option<bool>> {
    |            ^^^^^^^^^^^^^^^^
...
223 |     pub fn prompt(self) -> InquireResult<bool> {
    |            ^^^^^^
...
229 |     pub(crate) fn prompt_with_backend<B: CustomTypeBackend>(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: type alias `ConfirmPromptAction` is never used
 --> src/cli/prompts/confirm/action.rs:4:10
  |
4 | pub type ConfirmPromptAction = CustomTypePromptAction;
  |          ^^^^^^^^^^^^^^^^^^^

warning: field `render_config` is never read
   --> src/cli/prompts/custom_type/mod.rs:128:9
    |
80  | pub struct CustomType<'a, T> {
    |            ---------- field in this struct
...
128 |     pub render_config: RenderConfig<'a>,
    |         ^^^^^^^^^^^^^
    |
    = note: `CustomType` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: multiple associated items are never used
   --> src/cli/prompts/custom_type/mod.rs:136:15
    |
131 | / impl<'a, T> CustomType<'a, T>
132 | | where
133 | |     T: Clone,
    | |_____________- associated items in this implementation
...
136 |       pub const DEFAULT_VALIDATORS: Vec<Box<dyn CustomTypeValidator<T>>> = vec![];
    |                 ^^^^^^^^^^^^^^^^^^
...
139 |       pub fn new(message: &'a str) -> Self
    |              ^^^
...
163 |       pub fn with_starting_input(mut self, message: &'a str) -> Self {
    |              ^^^^^^^^^^^^^^^^^^^
...
169 |       pub fn with_default(mut self, default: T) -> Self {
    |              ^^^^^^^^^^^^
...
175 |       pub fn with_placeholder(mut self, placeholder: &'a str) -> Self {
    |              ^^^^^^^^^^^^^^^^
...
181 |       pub fn with_help_message(mut self, message: &'a str) -> Self {
    |              ^^^^^^^^^^^^^^^^^
...
187 |       pub fn with_formatter(mut self, formatter: CustomTypeFormatter<'a, T>) -> Self {
    |              ^^^^^^^^^^^^^^
...
196 |       pub fn with_default_value_formatter(mut self, formatter: CustomTypeFormatter<'a, T>) -> Self {
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
202 |       pub fn with_parser(mut self, parser: CustomTypeParser<'a, T>) -> Self {
    |              ^^^^^^^^^^^
...
214 |       pub fn with_validator<V>(mut self, validator: V) -> Self
    |              ^^^^^^^^^^^^^^
...
230 |       pub fn with_validators(mut self, validators: &[Box<dyn CustomTypeValidator<T>>]) -> Self {
    |              ^^^^^^^^^^^^^^^
...
239 |       pub fn with_error_message(mut self, error_message: &'a str) -> Self {
    |              ^^^^^^^^^^^^^^^^^^
...
252 |       pub fn with_render_config(mut self, render_config: RenderConfig<'a>) -> Self {
    |              ^^^^^^^^^^^^^^^^^^
...
266 |       pub fn prompt_skippable(self) -> InquireResult<Option<T>> {
    |              ^^^^^^^^^^^^^^^^
...
276 |       pub fn prompt(self) -> InquireResult<T> {
    |              ^^^^^^
...
282 |       pub(crate) fn prompt_with_backend<B: CustomTypeBackend>(
    |                     ^^^^^^^^^^^^^^^^^^^

warning: static `DEFAULT_MATCHER` is never used
  --> src/cli/prompts/multiselect/mod.rs:31:8
   |
31 | static DEFAULT_MATCHER: Lazy<SkimMatcherV2> = Lazy::new(|| SkimMatcherV2::default().ignore_case());
   |        ^^^^^^^^^^^^^^^

warning: multiple fields are never read
   --> src/cli/prompts/multiselect/mod.rs:66:9
    |
64  | pub struct MultiSelect<'a, T> {
    |            ----------- fields in this struct
65  |     /// Message to be presented to the user.
66  |     pub message: &'a str,
    |         ^^^^^^^
...
69  |     pub options: Vec<T>,
    |         ^^^^^^^
...
72  |     pub default: Option<Vec<usize>>,
    |         ^^^^^^^
...
75  |     pub help_message: Option<&'a str>,
    |         ^^^^^^^^^^^^
...
85  |     pub starting_cursor: usize,
    |         ^^^^^^^^^^^^^^^
...
88  |     pub starting_filter_input: Option<&'a str>,
    |         ^^^^^^^^^^^^^^^^^^^^^
...
97  |     pub filter_input_enabled: bool,
    |         ^^^^^^^^^^^^^^^^^^^^
...
102 |     pub scorer: Scorer<'a, T>,
    |         ^^^^^^
...
108 |     pub formatter: MultiOptionFormatter<'a, T>,
    |         ^^^^^^^^^
...
113 |     pub validator: Option<Box<dyn MultiOptionValidator<T>>>,
    |         ^^^^^^^^^
...
123 |     pub render_config: RenderConfig<'a>,
    |         ^^^^^^^^^^^^^
    |
    = note: `MultiSelect` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: multiple associated items are never used
   --> src/cli/prompts/multiselect/mod.rs:150:15
    |
126 | / impl<'a, T> MultiSelect<'a, T>
127 | | where
128 | |     T: Display,
    | |_______________- associated items in this implementation
...
150 |       pub const DEFAULT_FORMATTER: MultiOptionFormatter<'a, T> = &|ans| {
    |                 ^^^^^^^^^^^^^^^^^
...
181 |       pub const DEFAULT_SCORER: Scorer<'a, T> =
    |                 ^^^^^^^^^^^^^^
...
197 |       pub const DEFAULT_PAGE_SIZE: usize = crate::cli::config::DEFAULT_PAGE_SIZE;
    |                 ^^^^^^^^^^^^^^^^^
...
200 |       pub const DEFAULT_VIM_MODE: bool = crate::cli::config::DEFAULT_VIM_MODE;
    |                 ^^^^^^^^^^^^^^^^
...
203 |       pub const DEFAULT_STARTING_CURSOR: usize = 0;
    |                 ^^^^^^^^^^^^^^^^^^^^^^^
...
207 |       pub const DEFAULT_RESET_CURSOR: bool = true;
    |                 ^^^^^^^^^^^^^^^^^^^^
...
211 |       pub const DEFAULT_FILTER_INPUT_ENABLED: bool = true;
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
214 |       pub const DEFAULT_KEEP_FILTER: bool = true;
    |                 ^^^^^^^^^^^^^^^^^^^
...
217 |       pub const DEFAULT_HELP_MESSAGE: Option<&'a str> =
    |                 ^^^^^^^^^^^^^^^^^^^^
...
221 |       pub fn new(message: &'a str, options: Vec<T>) -> Self {
    |              ^^^
...
242 |       pub fn with_help_message(mut self, message: &'a str) -> Self {
    |              ^^^^^^^^^^^^^^^^^
...
248 |       pub fn without_help_message(mut self) -> Self {
    |              ^^^^^^^^^^^^^^^^^^^^
...
254 |       pub fn with_page_size(mut self, page_size: usize) -> Self {
    |              ^^^^^^^^^^^^^^
...
260 |       pub fn with_vim_mode(mut self, vim_mode: bool) -> Self {
    |              ^^^^^^^^^^^^^
...
266 |       pub fn with_keep_filter(mut self, keep_filter: bool) -> Self {
    |              ^^^^^^^^^^^^^^^^
...
272 |       pub fn with_scorer(mut self, scorer: Scorer<'a, T>) -> Self {
    |              ^^^^^^^^^^^
...
278 |       pub fn with_formatter(mut self, formatter: MultiOptionFormatter<'a, T>) -> Self {
    |              ^^^^^^^^^^^^^^
...
288 |       pub fn with_validator<V>(mut self, validator: V) -> Self
    |              ^^^^^^^^^^^^^^
...
300 |       pub fn with_default(mut self, default: &'a [usize]) -> Self {
    |              ^^^^^^^^^^^^
...
309 |       pub fn with_all_selected_by_default(mut self) -> Self {
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
318 |       pub fn with_starting_cursor(mut self, starting_cursor: usize) -> Self {
    |              ^^^^^^^^^^^^^^^^^^^^
...
324 |       pub fn with_starting_filter_input(mut self, starting_filter_input: &'a str) -> Self {
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^
...
333 |       pub fn with_reset_cursor(mut self, reset_cursor: bool) -> Self {
    |              ^^^^^^^^^^^^^^^^^
...
343 |       pub fn without_filtering(mut self) -> Self {
    |              ^^^^^^^^^^^^^^^^^
...
356 |       pub fn with_render_config(mut self, render_config: RenderConfig<'a>) -> Self {
    |              ^^^^^^^^^^^^^^^^^^
...
372 |       pub fn prompt_skippable(self) -> InquireResult<Option<Vec<T>>> {
    |              ^^^^^^^^^^^^^^^^
...
384 |       pub fn prompt(self) -> InquireResult<Vec<T>> {
    |              ^^^^^^
...
401 |       pub fn raw_prompt_skippable(self) -> InquireResult<Option<Vec<ListOption<T>>>> {
    |              ^^^^^^^^^^^^^^^^^^^^
...
414 |       pub fn raw_prompt(self) -> InquireResult<Vec<ListOption<T>>> {
    |              ^^^^^^^^^^
...
420 |       pub(crate) fn prompt_with_backend<B: MultiSelectBackend>(
    |                     ^^^^^^^^^^^^^^^^^^^

warning: associated function `new` is never used
  --> src/cli/prompts/multiselect/prompt.rs:38:12
   |
34 | / impl<'a, T> MultiSelectPrompt<'a, T>
35 | | where
36 | |     T: Display,
   | |_______________- associated function in this implementation
37 |   {
38 |       pub fn new(mso: MultiSelect<'a, T>) -> InquireResult<Self> {
   |              ^^^

warning: function `prompt_confirmation` is never used
  --> src/cli/prompts/one_liners.rs:36:8
   |
36 | pub fn prompt_confirmation<M>(message: M) -> InquireResult<bool>
   |        ^^^^^^^^^^^^^^^^^^^

warning: function `prompt_text` is never used
  --> src/cli/prompts/one_liners.rs:71:8
   |
71 | pub fn prompt_text<M>(message: M) -> InquireResult<String>
   |        ^^^^^^^^^^^

warning: function `prompt_f64` is never used
   --> src/cli/prompts/one_liners.rs:178:8
    |
178 | pub fn prompt_f64<M>(message: M) -> InquireResult<f64>
    |        ^^^^^^^^^^

warning: function `prompt_f32` is never used
   --> src/cli/prompts/one_liners.rs:213:8
    |
213 | pub fn prompt_f32<M>(message: M) -> InquireResult<f32>
    |        ^^^^^^^^^^

warning: function `prompt_u64` is never used
   --> src/cli/prompts/one_liners.rs:248:8
    |
248 | pub fn prompt_u64<M>(message: M) -> InquireResult<u64>
    |        ^^^^^^^^^^

warning: function `prompt_u32` is never used
   --> src/cli/prompts/one_liners.rs:283:8
    |
283 | pub fn prompt_u32<M>(message: M) -> InquireResult<u32>
    |        ^^^^^^^^^^

warning: function `prompt_usize` is never used
   --> src/cli/prompts/one_liners.rs:318:8
    |
318 | pub fn prompt_usize<M>(message: M) -> InquireResult<usize>
    |        ^^^^^^^^^^^^

warning: function `prompt_u128` is never used
   --> src/cli/prompts/one_liners.rs:353:8
    |
353 | pub fn prompt_u128<M>(message: M) -> InquireResult<u128>
    |        ^^^^^^^^^^^

warning: static `DEFAULT_MATCHER` is never used
  --> src/cli/prompts/select/mod.rs:29:8
   |
29 | static DEFAULT_MATCHER: Lazy<SkimMatcherV2> = Lazy::new(|| SkimMatcherV2::default().ignore_case());
   |        ^^^^^^^^^^^^^^^

warning: multiple fields are never read
   --> src/cli/prompts/select/mod.rs:75:9
    |
73  | pub struct Select<'a, T> {
    |            ------ fields in this struct
74  |     /// Message to be presented to the user.
75  |     pub message: &'a str,
    |         ^^^^^^^
...
78  |     pub options: Vec<T>,
    |         ^^^^^^^
...
81  |     pub help_message: Option<&'a str>,
    |         ^^^^^^^^^^^^
...
91  |     pub starting_cursor: usize,
    |         ^^^^^^^^^^^^^^^
...
94  |     pub starting_filter_input: Option<&'a str>,
    |         ^^^^^^^^^^^^^^^^^^^^^
...
103 |     pub filter_input_enabled: bool,
    |         ^^^^^^^^^^^^^^^^^^^^
...
107 |     pub scorer: Scorer<'a, T>,
    |         ^^^^^^
...
110 |     pub formatter: OptionFormatter<'a, T>,
    |         ^^^^^^^^^
...
120 |     pub render_config: RenderConfig<'a>,
    |         ^^^^^^^^^^^^^
    |
    = note: `Select` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: multiple associated items are never used
   --> src/cli/prompts/select/mod.rs:140:15
    |
123 | / impl<'a, T> Select<'a, T>
124 | | where
125 | |     T: Display,
    | |_______________- associated items in this implementation
...
140 |       pub const DEFAULT_FORMATTER: OptionFormatter<'a, T> = &|ans| ans.to_string();
    |                 ^^^^^^^^^^^^^^^^^
...
166 |       pub const DEFAULT_SCORER: Scorer<'a, T> =
    |                 ^^^^^^^^^^^^^^
...
182 |       pub const DEFAULT_PAGE_SIZE: usize = crate::cli::config::DEFAULT_PAGE_SIZE;
    |                 ^^^^^^^^^^^^^^^^^
...
185 |       pub const DEFAULT_VIM_MODE: bool = crate::cli::config::DEFAULT_VIM_MODE;
    |                 ^^^^^^^^^^^^^^^^
...
188 |       pub const DEFAULT_STARTING_CURSOR: usize = 0;
    |                 ^^^^^^^^^^^^^^^^^^^^^^^
...
192 |       pub const DEFAULT_RESET_CURSOR: bool = true;
    |                 ^^^^^^^^^^^^^^^^^^^^
...
196 |       pub const DEFAULT_FILTER_INPUT_ENABLED: bool = true;
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
199 |       pub const DEFAULT_HELP_MESSAGE: Option<&'a str> =
    |                 ^^^^^^^^^^^^^^^^^^^^
...
203 |       pub fn new(message: &'a str, options: Vec<T>) -> Self {
    |              ^^^
...
221 |       pub fn with_help_message(mut self, message: &'a str) -> Self {
    |              ^^^^^^^^^^^^^^^^^
...
227 |       pub fn without_help_message(mut self) -> Self {
    |              ^^^^^^^^^^^^^^^^^^^^
...
233 |       pub fn with_page_size(mut self, page_size: usize) -> Self {
    |              ^^^^^^^^^^^^^^
...
239 |       pub fn with_vim_mode(mut self, vim_mode: bool) -> Self {
    |              ^^^^^^^^^^^^^
...
245 |       pub fn with_scorer(mut self, scorer: Scorer<'a, T>) -> Self {
    |              ^^^^^^^^^^^
...
251 |       pub fn with_formatter(mut self, formatter: OptionFormatter<'a, T>) -> Self {
    |              ^^^^^^^^^^^^^^
...
260 |       pub fn with_starting_cursor(mut self, starting_cursor: usize) -> Self {
    |              ^^^^^^^^^^^^^^^^^^^^
...
266 |       pub fn with_starting_filter_input(mut self, starting_filter_input: &'a str) -> Self {
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^
...
275 |       pub fn with_reset_cursor(mut self, reset_cursor: bool) -> Self {
    |              ^^^^^^^^^^^^^^^^^
...
285 |       pub fn without_filtering(mut self) -> Self {
    |              ^^^^^^^^^^^^^^^^^
...
298 |       pub fn with_render_config(mut self, render_config: RenderConfig<'a>) -> Self {
    |              ^^^^^^^^^^^^^^^^^^
...
307 |       pub fn prompt(self) -> InquireResult<T> {
    |              ^^^^^^
...
320 |       pub fn prompt_skippable(self) -> InquireResult<Option<T>> {
    |              ^^^^^^^^^^^^^^^^
...
333 |       pub fn raw_prompt(self) -> InquireResult<ListOption<T>> {
    |              ^^^^^^^^^^
...
339 |       pub(crate) fn prompt_with_backend<B: SelectBackend>(
    |                     ^^^^^^^^^^^^^^^^^^^

warning: associated function `new` is never used
  --> src/cli/prompts/select/prompt.rs:34:12
   |
30 | / impl<'a, T> SelectPrompt<'a, T>
31 | | where
32 | |     T: Display,
   | |_______________- associated function in this implementation
33 |   {
34 |       pub fn new(so: Select<'a, T>) -> InquireResult<Self> {
   |              ^^^

warning: multiple methods are never used
   --> src/cli/prompts/text/mod.rs:155:12
    |
125 | impl<'a> Text<'a> {
    | ----------------- methods in this implementation
...
155 |     pub fn with_help_message(mut self, message: &'a str) -> Self {
    |            ^^^^^^^^^^^^^^^^^
...
165 |     pub fn with_initial_value(mut self, message: &'a str) -> Self {
    |            ^^^^^^^^^^^^^^^^^^
...
171 |     pub fn with_default(mut self, message: &'a str) -> Self {
    |            ^^^^^^^^^^^^
...
177 |     pub fn with_placeholder(mut self, placeholder: &'a str) -> Self {
    |            ^^^^^^^^^^^^^^^^
...
183 |     pub fn with_autocomplete<AC>(mut self, ac: AC) -> Self
    |            ^^^^^^^^^^^^^^^^^
...
192 |     pub fn with_formatter(mut self, formatter: StringFormatter<'a>) -> Self {
    |            ^^^^^^^^^^^^^^
...
198 |     pub fn with_page_size(mut self, page_size: usize) -> Self {
    |            ^^^^^^^^^^^^^^
...
211 |     pub fn with_validator<V>(mut self, validator: V) -> Self
    |            ^^^^^^^^^^^^^^
...
227 |     pub fn with_validators(mut self, validators: &[Box<dyn StringValidator>]) -> Self {
    |            ^^^^^^^^^^^^^^^
...
243 |     pub fn with_render_config(mut self, render_config: RenderConfig<'a>) -> Self {
    |            ^^^^^^^^^^^^^^^^^^
...
257 |     pub fn prompt_skippable(self) -> InquireResult<Option<String>> {
    |            ^^^^^^^^^^^^^^^^

warning: type alias `Suggester` is never used
  --> src/cli/type_aliases.rs:79:10
   |
79 | pub type Suggester<'a> = &'a dyn Fn(&str) -> Result<Vec<String>, CustomUserError>;
   |          ^^^^^^^^^

warning: type alias `Completer` is never used
  --> src/cli/type_aliases.rs:84:10
   |
84 | pub type Completer<'a> = &'a dyn Fn(&str) -> Result<Option<String>, CustomUserError>;
   |          ^^^^^^^^^

warning: multiple methods are never used
   --> src/cli/ui/api/render_config.rs:215:12
    |
151 | impl<'a> RenderConfig<'a> {
    | ------------------------- methods in this implementation
...
215 |     pub fn with_prompt_prefix(mut self, prompt_prefix: Styled<&'a str>) -> Self {
    |            ^^^^^^^^^^^^^^^^^^
...
221 |     pub fn with_answered_prompt_prefix(mut self, answered_prompt_prefix: Styled<&'a str>) -> Self {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
227 |     pub fn with_text_input(mut self, text_input: StyleSheet) -> Self {
    |            ^^^^^^^^^^^^^^^
...
233 |     pub fn with_default_value(mut self, default_value: StyleSheet) -> Self {
    |            ^^^^^^^^^^^^^^^^^^
...
239 |     pub fn with_help_message(mut self, help_message: StyleSheet) -> Self {
    |            ^^^^^^^^^^^^^^^^^
...
245 |     pub fn with_answer(mut self, answer: StyleSheet) -> Self {
    |            ^^^^^^^^^^^
...
251 |     pub fn with_error_message(mut self, error_message: ErrorMessageRenderConfig<'a>) -> Self {
    |            ^^^^^^^^^^^^^^^^^^
...
257 |     pub fn with_highlighted_option_prefix(
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
266 |     pub fn with_scroll_up_prefix(mut self, scroll_up_prefix: Styled<&'a str>) -> Self {
    |            ^^^^^^^^^^^^^^^^^^^^^
...
272 |     pub fn with_scroll_down_prefix(mut self, scroll_down_prefix: Styled<&'a str>) -> Self {
    |            ^^^^^^^^^^^^^^^^^^^^^^^
...
278 |     pub fn with_selected_checkbox(mut self, selected_checkbox: Styled<&'a str>) -> Self {
    |            ^^^^^^^^^^^^^^^^^^^^^^
...
284 |     pub fn with_unselected_checkbox(mut self, unselected_checkbox: Styled<&'a str>) -> Self {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^
...
290 |     pub fn with_option_index_prefix(mut self, index_prefix: IndexPrefix) -> Self {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^
...
296 |     pub fn with_option(mut self, option: StyleSheet) -> Self {
    |            ^^^^^^^^^^^
...
302 |     pub fn with_selected_option(mut self, selected_option: Option<StyleSheet>) -> Self {
    |            ^^^^^^^^^^^^^^^^^^^^
...
308 |     pub fn with_canceled_prompt_indicator(
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: variants `Simple`, `SpacePadded`, and `ZeroPadded` are never constructed
   --> src/cli/ui/api/render_config.rs:349:5
    |
342 | pub enum IndexPrefix {
    |          ----------- variants in this enum
...
349 |     Simple,
    |     ^^^^^^
...
361 |     SpacePadded,
    |     ^^^^^^^^^^^
...
373 |     ZeroPadded,
    |     ^^^^^^^^^^
    |
    = note: `IndexPrefix` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: methods `with_prefix`, `with_separator`, and `with_message` are never used
   --> src/cli/ui/api/render_config.rs:417:12
    |
395 | impl<'a> ErrorMessageRenderConfig<'a> {
    | ------------------------------------- methods in this implementation
...
417 |     pub fn with_prefix(mut self, prefix: Styled<&'a str>) -> Self {
    |            ^^^^^^^^^^^
...
426 |     pub fn with_separator(mut self, separator: StyleSheet) -> Self {
    |            ^^^^^^^^^^^^^^
...
432 |     pub fn with_message(mut self, message: StyleSheet) -> Self {
    |            ^^^^^^^^^^^^

warning: methods `with_bg` and `with_attr` are never used
   --> src/cli/ui/api/style.rs:88:12
    |
61  | impl StyleSheet {
    | --------------- methods in this implementation
...
88  |     pub fn with_bg(mut self, bg: Color) -> Self {
    |            ^^^^^^^
...
116 |     pub fn with_attr(mut self, attributes: Attributes) -> Self {
    |            ^^^^^^^^^

warning: methods `with_bg` and `with_attr` are never used
   --> src/cli/ui/api/style.rs:170:12
    |
143 | / impl<T> Styled<T>
144 | | where
145 | |     T: Display,
    | |_______________- methods in this implementation
...
170 |       pub fn with_bg(mut self, bg: Color) -> Self {
    |              ^^^^^^^
...
180 |       pub fn with_attr(mut self, attributes: Attributes) -> Self {
    |              ^^^^^^^^^

warning: methods `render_prompt`, `render_prompt_with_masked_input`, and `render_prompt_with_full_input` are never used
  --> src/cli/ui/backend.rs:67:8
   |
66 | pub trait PasswordBackend: CommonBackend {
   |           --------------- methods in this trait
67 |     fn render_prompt(&mut self, prompt: &str) -> Result<()>;
   |        ^^^^^^^^^^^^^
68 |     fn render_prompt_with_masked_input(&mut self, prompt: &str, cur_input: &Input) -> Result<()>;
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
69 |     fn render_prompt_with_full_input(&mut self, prompt: &str, cur_input: &Input) -> Result<()>;
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: associated function `new` is never used
   --> src/cli/validator.rs:315:12
    |
313 | impl ValueRequiredValidator {
    | --------------------------- associated function in this implementation
314 |     /// Create a new instance of this validator with given error message.
315 |     pub fn new(message: impl Into<String>) -> Self {
    |            ^^^

warning: associated items `new` and `with_message` are never used
   --> src/cli/validator.rs:409:12
    |
406 | impl MaxLengthValidator {
    | ----------------------- associated items in this implementation
...
409 |     pub fn new(limit: usize) -> Self {
    |            ^^^
...
418 |     pub fn with_message(mut self, message: impl Into<String>) -> Self {
    |            ^^^^^^^^^^^^

warning: associated items `new` and `with_message` are never used
   --> src/cli/validator.rs:518:12
    |
515 | impl MinLengthValidator {
    | ----------------------- associated items in this implementation
...
518 |     pub fn new(limit: usize) -> Self {
    |            ^^^
...
527 |     pub fn with_message(mut self, message: impl Into<String>) -> Self {
    |            ^^^^^^^^^^^^

warning: associated items `new` and `with_message` are never used
   --> src/cli/validator.rs:624:12
    |
621 | impl ExactLengthValidator {
    | ------------------------- associated items in this implementation
...
624 |     pub fn new(length: usize) -> Self {
    |            ^^^
...
633 |     pub fn with_message(mut self, message: impl Into<String>) -> Self {
    |            ^^^^^^^^^^^^