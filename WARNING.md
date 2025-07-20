warning: unexpected `cfg` condition value: `strum`
 --> src/cli/enum_support.rs:7:7
  |
7 | #[cfg(feature = "strum")]
  |       ^^^^^^^^^^^^^^^^^
  |
  = note: expected values for `feature` are: `chrono`, `console`, `crossterm`, `date`, `default`, `editor`, `fuzzy`, `fuzzy-matcher`, `macros`, `one-liners`, `tempfile`, and `termion`
  = help: consider adding `strum` as a feature in `Cargo.toml`
  = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
  = note: `#[warn(unexpected_cfgs)]` on by default

warning: trait `InquireEnumVariants` is never used
 --> src/cli/enum_support.rs:3:11
  |
3 | pub trait InquireEnumVariants {
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
   --> src/cli/prompts/confirm/mod.rs:143:12
    |
104 | impl<'a> Confirm<'a> {
    | -------------------- methods in this implementation
...
143 |     pub fn with_starting_input(mut self, message: &'a str) -> Self {
    |            ^^^^^^^^^^^^^^^^^^^
...
149 |     pub fn with_default(mut self, default: bool) -> Self {
    |            ^^^^^^^^^^^^
...
155 |     pub fn with_placeholder(mut self, placeholder: &'a str) -> Self {
    |            ^^^^^^^^^^^^^^^^
...
161 |     pub fn with_help_message(mut self, message: &'a str) -> Self {
    |            ^^^^^^^^^^^^^^^^^
...
167 |     pub fn with_formatter(mut self, formatter: BoolFormatter<'a>) -> Self {
    |            ^^^^^^^^^^^^^^
...
173 |     pub fn with_parser(mut self, parser: BoolParser<'a>) -> Self {
    |            ^^^^^^^^^^^
...
179 |     pub fn with_error_message(mut self, error_message: &'a str) -> Self {
    |            ^^^^^^^^^^^^^^^^^^
...
185 |     pub fn with_default_value_formatter(mut self, formatter: BoolFormatter<'a>) -> Self {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
198 |     pub fn with_render_config(mut self, render_config: RenderConfig<'a>) -> Self {
    |            ^^^^^^^^^^^^^^^^^^
...
212 |     pub fn prompt_skippable(self) -> InquireResult<Option<bool>> {
    |            ^^^^^^^^^^^^^^^^
...
222 |     pub fn prompt(self) -> InquireResult<bool> {
    |            ^^^^^^
...
228 |     pub(crate) fn prompt_with_backend<B: CustomTypeBackend>(
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
  --> src/cli/prompts/multiselect/mod.rs:30:8
   |
30 | static DEFAULT_MATCHER: Lazy<SkimMatcherV2> = Lazy::new(|| SkimMatcherV2::default().ignore_case());
   |        ^^^^^^^^^^^^^^^

warning: multiple fields are never read
   --> src/cli/prompts/multiselect/mod.rs:65:9
    |
63  | pub struct MultiSelect<'a, T> {
    |            ----------- fields in this struct
64  |     /// Message to be presented to the user.
65  |     pub message: &'a str,
    |         ^^^^^^^
...
68  |     pub options: Vec<T>,
    |         ^^^^^^^
...
71  |     pub default: Option<Vec<usize>>,
    |         ^^^^^^^
...
74  |     pub help_message: Option<&'a str>,
    |         ^^^^^^^^^^^^
...
84  |     pub starting_cursor: usize,
    |         ^^^^^^^^^^^^^^^
...
87  |     pub starting_filter_input: Option<&'a str>,
    |         ^^^^^^^^^^^^^^^^^^^^^
...
96  |     pub filter_input_enabled: bool,
    |         ^^^^^^^^^^^^^^^^^^^^
...
101 |     pub scorer: Scorer<'a, T>,
    |         ^^^^^^
...
107 |     pub formatter: MultiOptionFormatter<'a, T>,
    |         ^^^^^^^^^
...
112 |     pub validator: Option<Box<dyn MultiOptionValidator<T>>>,
    |         ^^^^^^^^^
...
122 |     pub render_config: RenderConfig<'a>,
    |         ^^^^^^^^^^^^^
    |
    = note: `MultiSelect` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: multiple associated items are never used
   --> src/cli/prompts/multiselect/mod.rs:149:15
    |
125 | / impl<'a, T> MultiSelect<'a, T>
126 | | where
127 | |     T: Display,
    | |_______________- associated items in this implementation
...
149 |       pub const DEFAULT_FORMATTER: MultiOptionFormatter<'a, T> = &|ans| {
    |                 ^^^^^^^^^^^^^^^^^
...
180 |       pub const DEFAULT_SCORER: Scorer<'a, T> =
    |                 ^^^^^^^^^^^^^^
...
196 |       pub const DEFAULT_PAGE_SIZE: usize = crate::cli::config::DEFAULT_PAGE_SIZE;
    |                 ^^^^^^^^^^^^^^^^^
...
199 |       pub const DEFAULT_VIM_MODE: bool = crate::cli::config::DEFAULT_VIM_MODE;
    |                 ^^^^^^^^^^^^^^^^
...
202 |       pub const DEFAULT_STARTING_CURSOR: usize = 0;
    |                 ^^^^^^^^^^^^^^^^^^^^^^^
...
206 |       pub const DEFAULT_RESET_CURSOR: bool = true;
    |                 ^^^^^^^^^^^^^^^^^^^^
...
210 |       pub const DEFAULT_FILTER_INPUT_ENABLED: bool = true;
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
213 |       pub const DEFAULT_KEEP_FILTER: bool = true;
    |                 ^^^^^^^^^^^^^^^^^^^
...
216 |       pub const DEFAULT_HELP_MESSAGE: Option<&'a str> =
    |                 ^^^^^^^^^^^^^^^^^^^^
...
220 |       pub fn new(message: &'a str, options: Vec<T>) -> Self {
    |              ^^^
...
241 |       pub fn with_help_message(mut self, message: &'a str) -> Self {
    |              ^^^^^^^^^^^^^^^^^
...
247 |       pub fn without_help_message(mut self) -> Self {
    |              ^^^^^^^^^^^^^^^^^^^^
...
253 |       pub fn with_page_size(mut self, page_size: usize) -> Self {
    |              ^^^^^^^^^^^^^^
...
259 |       pub fn with_vim_mode(mut self, vim_mode: bool) -> Self {
    |              ^^^^^^^^^^^^^
...
265 |       pub fn with_keep_filter(mut self, keep_filter: bool) -> Self {
    |              ^^^^^^^^^^^^^^^^
...
271 |       pub fn with_scorer(mut self, scorer: Scorer<'a, T>) -> Self {
    |              ^^^^^^^^^^^
...
277 |       pub fn with_formatter(mut self, formatter: MultiOptionFormatter<'a, T>) -> Self {
    |              ^^^^^^^^^^^^^^
...
287 |       pub fn with_validator<V>(mut self, validator: V) -> Self
    |              ^^^^^^^^^^^^^^
...
299 |       pub fn with_default(mut self, default: &'a [usize]) -> Self {
    |              ^^^^^^^^^^^^
...
308 |       pub fn with_all_selected_by_default(mut self) -> Self {
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
317 |       pub fn with_starting_cursor(mut self, starting_cursor: usize) -> Self {
    |              ^^^^^^^^^^^^^^^^^^^^
...
323 |       pub fn with_starting_filter_input(mut self, starting_filter_input: &'a str) -> Self {
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^
...
332 |       pub fn with_reset_cursor(mut self, reset_cursor: bool) -> Self {
    |              ^^^^^^^^^^^^^^^^^
...
342 |       pub fn without_filtering(mut self) -> Self {
    |              ^^^^^^^^^^^^^^^^^
...
355 |       pub fn with_render_config(mut self, render_config: RenderConfig<'a>) -> Self {
    |              ^^^^^^^^^^^^^^^^^^
...
371 |       pub fn prompt_skippable(self) -> InquireResult<Option<Vec<T>>> {
    |              ^^^^^^^^^^^^^^^^
...
383 |       pub fn prompt(self) -> InquireResult<Vec<T>> {
    |              ^^^^^^
...
400 |       pub fn raw_prompt_skippable(self) -> InquireResult<Option<Vec<ListOption<T>>>> {
    |              ^^^^^^^^^^^^^^^^^^^^
...
413 |       pub fn raw_prompt(self) -> InquireResult<Vec<ListOption<T>>> {
    |              ^^^^^^^^^^
...
419 |       pub(crate) fn prompt_with_backend<B: MultiSelectBackend>(
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
  --> src/cli/prompts/select/mod.rs:28:8
   |
28 | static DEFAULT_MATCHER: Lazy<SkimMatcherV2> = Lazy::new(|| SkimMatcherV2::default().ignore_case());
   |        ^^^^^^^^^^^^^^^

warning: multiple fields are never read
   --> src/cli/prompts/select/mod.rs:74:9
    |
72  | pub struct Select<'a, T> {
    |            ------ fields in this struct
73  |     /// Message to be presented to the user.
74  |     pub message: &'a str,
    |         ^^^^^^^
...
77  |     pub options: Vec<T>,
    |         ^^^^^^^
...
80  |     pub help_message: Option<&'a str>,
    |         ^^^^^^^^^^^^
...
90  |     pub starting_cursor: usize,
    |         ^^^^^^^^^^^^^^^
...
93  |     pub starting_filter_input: Option<&'a str>,
    |         ^^^^^^^^^^^^^^^^^^^^^
...
102 |     pub filter_input_enabled: bool,
    |         ^^^^^^^^^^^^^^^^^^^^
...
106 |     pub scorer: Scorer<'a, T>,
    |         ^^^^^^
...
109 |     pub formatter: OptionFormatter<'a, T>,
    |         ^^^^^^^^^
...
119 |     pub render_config: RenderConfig<'a>,
    |         ^^^^^^^^^^^^^
    |
    = note: `Select` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: multiple associated items are never used
   --> src/cli/prompts/select/mod.rs:139:15
    |
122 | / impl<'a, T> Select<'a, T>
123 | | where
124 | |     T: Display,
    | |_______________- associated items in this implementation
...
139 |       pub const DEFAULT_FORMATTER: OptionFormatter<'a, T> = &|ans| ans.to_string();
    |                 ^^^^^^^^^^^^^^^^^
...
165 |       pub const DEFAULT_SCORER: Scorer<'a, T> =
    |                 ^^^^^^^^^^^^^^
...
181 |       pub const DEFAULT_PAGE_SIZE: usize = crate::cli::config::DEFAULT_PAGE_SIZE;
    |                 ^^^^^^^^^^^^^^^^^
...
184 |       pub const DEFAULT_VIM_MODE: bool = crate::cli::config::DEFAULT_VIM_MODE;
    |                 ^^^^^^^^^^^^^^^^
...
187 |       pub const DEFAULT_STARTING_CURSOR: usize = 0;
    |                 ^^^^^^^^^^^^^^^^^^^^^^^
...
191 |       pub const DEFAULT_RESET_CURSOR: bool = true;
    |                 ^^^^^^^^^^^^^^^^^^^^
...
195 |       pub const DEFAULT_FILTER_INPUT_ENABLED: bool = true;
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
198 |       pub const DEFAULT_HELP_MESSAGE: Option<&'a str> =
    |                 ^^^^^^^^^^^^^^^^^^^^
...
202 |       pub fn new(message: &'a str, options: Vec<T>) -> Self {
    |              ^^^
...
220 |       pub fn with_help_message(mut self, message: &'a str) -> Self {
    |              ^^^^^^^^^^^^^^^^^
...
226 |       pub fn without_help_message(mut self) -> Self {
    |              ^^^^^^^^^^^^^^^^^^^^
...
232 |       pub fn with_page_size(mut self, page_size: usize) -> Self {
    |              ^^^^^^^^^^^^^^
...
238 |       pub fn with_vim_mode(mut self, vim_mode: bool) -> Self {
    |              ^^^^^^^^^^^^^
...
244 |       pub fn with_scorer(mut self, scorer: Scorer<'a, T>) -> Self {
    |              ^^^^^^^^^^^
...
250 |       pub fn with_formatter(mut self, formatter: OptionFormatter<'a, T>) -> Self {
    |              ^^^^^^^^^^^^^^
...
259 |       pub fn with_starting_cursor(mut self, starting_cursor: usize) -> Self {
    |              ^^^^^^^^^^^^^^^^^^^^
...
265 |       pub fn with_starting_filter_input(mut self, starting_filter_input: &'a str) -> Self {
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^
...
274 |       pub fn with_reset_cursor(mut self, reset_cursor: bool) -> Self {
    |              ^^^^^^^^^^^^^^^^^
...
284 |       pub fn without_filtering(mut self) -> Self {
    |              ^^^^^^^^^^^^^^^^^
...
297 |       pub fn with_render_config(mut self, render_config: RenderConfig<'a>) -> Self {
    |              ^^^^^^^^^^^^^^^^^^
...
306 |       pub fn prompt(self) -> InquireResult<T> {
    |              ^^^^^^
...
319 |       pub fn prompt_skippable(self) -> InquireResult<Option<T>> {
    |              ^^^^^^^^^^^^^^^^
...
332 |       pub fn raw_prompt(self) -> InquireResult<ListOption<T>> {
    |              ^^^^^^^^^^
...
338 |       pub(crate) fn prompt_with_backend<B: SelectBackend>(
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
   --> src/cli/prompts/text/mod.rs:154:12
    |
124 | impl<'a> Text<'a> {
    | ----------------- methods in this implementation
...
154 |     pub fn with_help_message(mut self, message: &'a str) -> Self {
    |            ^^^^^^^^^^^^^^^^^
...
164 |     pub fn with_initial_value(mut self, message: &'a str) -> Self {
    |            ^^^^^^^^^^^^^^^^^^
...
170 |     pub fn with_default(mut self, message: &'a str) -> Self {
    |            ^^^^^^^^^^^^
...
176 |     pub fn with_placeholder(mut self, placeholder: &'a str) -> Self {
    |            ^^^^^^^^^^^^^^^^
...
182 |     pub fn with_autocomplete<AC>(mut self, ac: AC) -> Self
    |            ^^^^^^^^^^^^^^^^^
...
191 |     pub fn with_formatter(mut self, formatter: StringFormatter<'a>) -> Self {
    |            ^^^^^^^^^^^^^^
...
197 |     pub fn with_page_size(mut self, page_size: usize) -> Self {
    |            ^^^^^^^^^^^^^^
...
210 |     pub fn with_validator<V>(mut self, validator: V) -> Self
    |            ^^^^^^^^^^^^^^
...
226 |     pub fn with_validators(mut self, validators: &[Box<dyn StringValidator>]) -> Self {
    |            ^^^^^^^^^^^^^^^
...
242 |     pub fn with_render_config(mut self, render_config: RenderConfig<'a>) -> Self {
    |            ^^^^^^^^^^^^^^^^^^
...
256 |     pub fn prompt_skippable(self) -> InquireResult<Option<String>> {
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
