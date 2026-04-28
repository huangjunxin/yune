#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub lock: bool,
    pub control: bool,
    pub alt: bool,
    pub super_key: bool,
    pub hyper: bool,
    pub meta: bool,
    pub release: bool,
}

impl KeyModifiers {
    #[must_use]
    pub const fn is_empty(self) -> bool {
        !self.shift
            && !self.lock
            && !self.control
            && !self.alt
            && !self.super_key
            && !self.hyper
            && !self.meta
            && !self.release
    }
}

const fn is_exact_control_modifier(modifiers: KeyModifiers) -> bool {
    modifiers.control
        && !modifiers.shift
        && !modifiers.alt
        && !modifiers.super_key
        && !modifiers.hyper
        && !modifiers.meta
        && !modifiers.release
}

const fn is_exact_shift_modifier(modifiers: KeyModifiers) -> bool {
    modifiers.shift
        && !modifiers.control
        && !modifiers.alt
        && !modifiers.super_key
        && !modifiers.hyper
        && !modifiers.meta
        && !modifiers.release
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KeyCode {
    Character(char),
    KeypadDigit(char),
    Tab,
    Ignored,
    Backspace,
    Delete,
    Escape,
    MoveCaretLeft,
    MoveCaretRight,
    MoveCaretLeftByChar,
    MoveCaretRightByChar,
    MoveCaretLeftBySyllable,
    MoveCaretRightBySyllable,
    Home,
    End,
    PreviousCandidate,
    NextCandidate,
    FirstCandidate,
    PreviousPage,
    NextPage,
    Return,
    KeypadEnter,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl KeyEvent {
    #[must_use]
    pub const fn character(ch: char) -> Self {
        Self {
            code: KeyCode::Character(ch),
            modifiers: KeyModifiers {
                shift: false,
                lock: false,
                control: false,
                alt: false,
                super_key: false,
                hyper: false,
                meta: false,
                release: false,
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeySequenceParseError {
    message: String,
}

impl KeySequenceParseError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for KeySequenceParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for KeySequenceParseError {}

pub fn parse_key_sequence(input: &str) -> Result<Vec<KeyEvent>, KeySequenceParseError> {
    let mut events = Vec::new();
    let mut index = 0;

    while index < input.len() {
        let ch = input[index..]
            .chars()
            .next()
            .expect("index should be at a character boundary");
        if ch == '{' && index + ch.len_utf8() < input.len() {
            let start = index + ch.len_utf8();
            let end = input[start..].find('}').map(|offset| start + offset);
            let end = end.ok_or_else(|| {
                KeySequenceParseError::new(format!(
                    "unmatched '{{' in key sequence at byte offset {index}"
                ))
            })?;
            let repr = &input[start..end];
            events.push(parse_key_event_repr(repr)?);
            index = end + '}'.len_utf8();
        } else {
            events.push(KeyEvent::character(ch));
            index += ch.len_utf8();
        }
    }

    Ok(events)
}

fn parse_key_event_repr(repr: &str) -> Result<KeyEvent, KeySequenceParseError> {
    if repr.is_empty() {
        return Err(KeySequenceParseError::new("empty key name in key sequence"));
    }
    if repr.chars().count() == 1 {
        return Ok(KeyEvent::character(repr.chars().next().expect(
            "single-character key representation should contain a char",
        )));
    }

    let mut tokens = repr.split('+').peekable();
    let mut modifiers = KeyModifiers::default();
    while let Some(token) = tokens.next() {
        if tokens.peek().is_none() {
            let code = if is_exact_control_modifier(modifiers) || is_exact_shift_modifier(modifiers)
            {
                match token {
                    "Up" => KeyCode::MoveCaretLeftBySyllable,
                    "Down" => KeyCode::MoveCaretRightBySyllable,
                    _ => key_code_from_name(token)?,
                }
            } else {
                key_code_from_name(token)?
            };
            return Ok(KeyEvent { code, modifiers });
        }
        apply_modifier(&mut modifiers, token)?;
    }

    Err(KeySequenceParseError::new("empty key representation"))
}

fn apply_modifier(modifiers: &mut KeyModifiers, token: &str) -> Result<(), KeySequenceParseError> {
    match token {
        "Shift" => modifiers.shift = true,
        "Lock" => modifiers.lock = true,
        "Control" => modifiers.control = true,
        "Alt" => modifiers.alt = true,
        "Super" => modifiers.super_key = true,
        "Hyper" => modifiers.hyper = true,
        "Meta" => modifiers.meta = true,
        "Release" => modifiers.release = true,
        _ => {
            return Err(KeySequenceParseError::new(format!(
                "unrecognized key modifier: {token}"
            )));
        }
    }
    Ok(())
}

fn key_code_from_name(name: &str) -> Result<KeyCode, KeySequenceParseError> {
    if name.chars().count() == 1 {
        return Ok(KeyCode::Character(
            name.chars()
                .next()
                .expect("single-character key name should contain a char"),
        ));
    }

    let code = match name {
        "space" => KeyCode::Character(' '),
        _ if let Some(ch) = ascii_named_key_char(name) => KeyCode::Character(ch),
        "Tab" => KeyCode::Tab,
        "Linefeed" | "Clear" | "Pause" | "Scroll_Lock" | "Sys_Req" | "Begin" | "Select"
        | "Print" | "Execute" | "Insert" | "Undo" | "Redo" | "Menu" | "Find" | "Cancel"
        | "Help" | "Break" | "Arabic_switch" | "Greek_switch" | "Hangul_switch"
        | "Hebrew_switch" | "ISO_Group_Shift" | "Mode_switch" | "kana_switch" | "script_switch"
        | "Num_Lock" | "F1" | "F2" | "F3" | "F4" | "F5" | "F6" | "F7" | "F8" | "F9" | "F10"
        | "F11" | "F12" | "F13" | "F14" | "F15" | "F16" | "F17" | "F18" | "F19" | "F20" | "F21"
        | "F22" | "F23" | "F24" | "F25" | "F26" | "F27" | "F28" | "F29" | "F30" | "F31" | "F32"
        | "F33" | "F34" | "F35" | "Shift_L" | "Shift_R" | "Control_L" | "Control_R"
        | "Caps_Lock" | "Shift_Lock" | "Meta_L" | "Meta_R" | "Alt_L" | "Alt_R" | "Super_L"
        | "Super_R" | "Hyper_L" | "Hyper_R" => KeyCode::Ignored,
        _ if is_librime_iso_key_name(name)
            || is_librime_xkb_key_name(name)
            || is_librime_input_method_key_name(name)
            || is_librime_keypad_noop_key_name(name)
            || is_librime_latin1_key_name(name)
            || is_librime_latin2_key_name(name)
            || is_librime_latin3_key_name(name)
            || is_librime_latin4_key_name(name)
            || is_librime_kana_key_name(name)
            || is_librime_arabic_key_name(name)
            || is_librime_cyrillic_key_name(name)
            || is_librime_greek_key_name(name)
            || is_librime_technical_key_name(name)
            || is_librime_publishing_key_name(name)
            || is_librime_hebrew_key_name(name) =>
        {
            KeyCode::Ignored
        }
        "BackSpace" => KeyCode::Backspace,
        "Delete" => KeyCode::Delete,
        "Escape" => KeyCode::Escape,
        "Left" => KeyCode::MoveCaretLeft,
        "Right" => KeyCode::MoveCaretRight,
        "KP_Left" => KeyCode::MoveCaretLeftByChar,
        "KP_Right" => KeyCode::MoveCaretRightByChar,
        "Up" | "KP_Up" => KeyCode::PreviousCandidate,
        "Down" | "KP_Down" => KeyCode::NextCandidate,
        "Home" | "KP_Home" => KeyCode::Home,
        "End" | "KP_End" => KeyCode::End,
        "Page_Up" | "Prior" | "KP_Page_Up" | "KP_Prior" => KeyCode::PreviousPage,
        "Page_Down" | "Next" | "KP_Page_Down" | "KP_Next" => KeyCode::NextPage,
        "Return" => KeyCode::Return,
        "KP_Enter" => KeyCode::KeypadEnter,
        "KP_0" => KeyCode::KeypadDigit('0'),
        "KP_1" => KeyCode::KeypadDigit('1'),
        "KP_2" => KeyCode::KeypadDigit('2'),
        "KP_3" => KeyCode::KeypadDigit('3'),
        "KP_4" => KeyCode::KeypadDigit('4'),
        "KP_5" => KeyCode::KeypadDigit('5'),
        "KP_6" => KeyCode::KeypadDigit('6'),
        "KP_7" => KeyCode::KeypadDigit('7'),
        "KP_8" => KeyCode::KeypadDigit('8'),
        "KP_9" => KeyCode::KeypadDigit('9'),
        _ => {
            return Err(KeySequenceParseError::new(format!(
                "unrecognized key name: {name}"
            )));
        }
    };
    Ok(code)
}

fn ascii_named_key_char(name: &str) -> Option<char> {
    Some(match name {
        "exclam" => '!',
        "quotedbl" => '"',
        "numbersign" => '#',
        "dollar" => '$',
        "percent" => '%',
        "ampersand" => '&',
        "apostrophe" | "quoteright" => '\'',
        "parenleft" => '(',
        "parenright" => ')',
        "asterisk" => '*',
        "plus" => '+',
        "comma" => ',',
        "minus" => '-',
        "period" => '.',
        "slash" => '/',
        "colon" => ':',
        "semicolon" => ';',
        "less" => '<',
        "equal" => '=',
        "greater" => '>',
        "question" => '?',
        "at" => '@',
        "bracketleft" => '[',
        "backslash" => '\\',
        "bracketright" => ']',
        "asciicircum" => '^',
        "underscore" => '_',
        "grave" | "quoteleft" => '`',
        "braceleft" => '{',
        "bar" => '|',
        "braceright" => '}',
        "asciitilde" => '~',
        _ => return None,
    })
}

fn is_librime_iso_key_name(name: &str) -> bool {
    matches!(
        name,
        "ISO_Lock"
            | "ISO_Level2_Latch"
            | "ISO_Level3_Shift"
            | "ISO_Level3_Latch"
            | "ISO_Level3_Lock"
            | "ISO_Group_Latch"
            | "ISO_Group_Lock"
            | "ISO_Next_Group"
            | "ISO_Next_Group_Lock"
            | "ISO_Prev_Group"
            | "ISO_Prev_Group_Lock"
            | "ISO_First_Group"
            | "ISO_First_Group_Lock"
            | "ISO_Last_Group"
            | "ISO_Last_Group_Lock"
            | "ISO_Left_Tab"
            | "ISO_Move_Line_Up"
            | "ISO_Move_Line_Down"
            | "ISO_Partial_Line_Up"
            | "ISO_Partial_Line_Down"
            | "ISO_Partial_Space_Left"
            | "ISO_Partial_Space_Right"
            | "ISO_Set_Margin_Left"
            | "ISO_Set_Margin_Right"
            | "ISO_Release_Margin_Left"
            | "ISO_Release_Margin_Right"
            | "ISO_Release_Both_Margins"
            | "ISO_Fast_Cursor_Left"
            | "ISO_Fast_Cursor_Right"
            | "ISO_Fast_Cursor_Up"
            | "ISO_Fast_Cursor_Down"
            | "ISO_Continuous_Underline"
            | "ISO_Discontinuous_Underline"
            | "ISO_Emphasize"
            | "ISO_Center_Object"
            | "ISO_Enter"
    )
}

fn is_librime_xkb_key_name(name: &str) -> bool {
    matches!(
        name,
        "dead_grave"
            | "dead_acute"
            | "dead_circumflex"
            | "dead_tilde"
            | "dead_macron"
            | "dead_breve"
            | "dead_abovedot"
            | "dead_diaeresis"
            | "dead_abovering"
            | "dead_doubleacute"
            | "dead_caron"
            | "dead_cedilla"
            | "dead_ogonek"
            | "dead_iota"
            | "dead_voiced_sound"
            | "dead_semivoiced_sound"
            | "dead_belowdot"
            | "dead_hook"
            | "dead_horn"
            | "AccessX_Enable"
            | "AccessX_Feedback_Enable"
            | "RepeatKeys_Enable"
            | "SlowKeys_Enable"
            | "BounceKeys_Enable"
            | "StickyKeys_Enable"
            | "MouseKeys_Enable"
            | "MouseKeys_Accel_Enable"
            | "Overlay1_Enable"
            | "Overlay2_Enable"
            | "AudibleBell_Enable"
            | "First_Virtual_Screen"
            | "Prev_Virtual_Screen"
            | "Next_Virtual_Screen"
            | "Last_Virtual_Screen"
            | "Terminate_Server"
            | "Pointer_Left"
            | "Pointer_Right"
            | "Pointer_Up"
            | "Pointer_Down"
            | "Pointer_UpLeft"
            | "Pointer_UpRight"
            | "Pointer_DownLeft"
            | "Pointer_DownRight"
            | "Pointer_Button_Dflt"
            | "Pointer_Button1"
            | "Pointer_Button2"
            | "Pointer_Button3"
            | "Pointer_Button4"
            | "Pointer_Button5"
            | "Pointer_DblClick_Dflt"
            | "Pointer_DblClick1"
            | "Pointer_DblClick2"
            | "Pointer_DblClick3"
            | "Pointer_DblClick4"
            | "Pointer_DblClick5"
            | "Pointer_Drag_Dflt"
            | "Pointer_Drag1"
            | "Pointer_Drag2"
            | "Pointer_Drag3"
            | "Pointer_Drag4"
            | "Pointer_EnableKeys"
            | "Pointer_Accelerate"
            | "Pointer_DfltBtnNext"
            | "Pointer_DfltBtnPrev"
            | "Pointer_Drag5"
    )
}

fn is_librime_input_method_key_name(name: &str) -> bool {
    matches!(
        name,
        "Multi_key"
            | "Kanji"
            | "Muhenkan"
            | "Henkan"
            | "Henkan_Mode"
            | "Romaji"
            | "Hiragana"
            | "Katakana"
            | "Hiragana_Katakana"
            | "Zenkaku"
            | "Hankaku"
            | "Zenkaku_Hankaku"
            | "Touroku"
            | "Massyo"
            | "Kana_Lock"
            | "Kana_Shift"
            | "Eisu_Shift"
            | "Eisu_toggle"
            | "Hangul"
            | "Hangul_Start"
            | "Hangul_End"
            | "Hangul_Hanja"
            | "Hangul_Jamo"
            | "Hangul_Romaja"
            | "Codeinput"
            | "Hangul_Jeonja"
            | "Hangul_Banja"
            | "Hangul_PreHanja"
            | "Hangul_PostHanja"
            | "SingleCandidate"
            | "MultipleCandidate"
            | "PreviousCandidate"
            | "Hangul_Special"
    )
}

fn is_librime_keypad_noop_key_name(name: &str) -> bool {
    matches!(
        name,
        "KP_Space"
            | "KP_Tab"
            | "KP_F1"
            | "KP_F2"
            | "KP_F3"
            | "KP_F4"
            | "KP_Begin"
            | "KP_Insert"
            | "KP_Delete"
            | "KP_Multiply"
            | "KP_Add"
            | "KP_Separator"
            | "KP_Subtract"
            | "KP_Decimal"
            | "KP_Divide"
            | "KP_Equal"
    )
}

fn is_librime_latin1_key_name(name: &str) -> bool {
    matches!(
        name,
        "nobreakspace"
            | "exclamdown"
            | "cent"
            | "sterling"
            | "currency"
            | "yen"
            | "brokenbar"
            | "section"
            | "diaeresis"
            | "copyright"
            | "ordfeminine"
            | "guillemotleft"
            | "notsign"
            | "hyphen"
            | "registered"
            | "macron"
            | "degree"
            | "plusminus"
            | "twosuperior"
            | "threesuperior"
            | "acute"
            | "mu"
            | "paragraph"
            | "periodcentered"
            | "cedilla"
            | "onesuperior"
            | "masculine"
            | "guillemotright"
            | "onequarter"
            | "onehalf"
            | "threequarters"
            | "questiondown"
            | "Agrave"
            | "Aacute"
            | "Acircumflex"
            | "Atilde"
            | "Adiaeresis"
            | "Aring"
            | "AE"
            | "Ccedilla"
            | "Egrave"
            | "Eacute"
            | "Ecircumflex"
            | "Ediaeresis"
            | "Igrave"
            | "Iacute"
            | "Icircumflex"
            | "Idiaeresis"
            | "ETH"
            | "Eth"
            | "Ntilde"
            | "Ograve"
            | "Oacute"
            | "Ocircumflex"
            | "Otilde"
            | "Odiaeresis"
            | "multiply"
            | "Ooblique"
            | "Ugrave"
            | "Uacute"
            | "Ucircumflex"
            | "Udiaeresis"
            | "Yacute"
            | "THORN"
            | "Thorn"
            | "ssharp"
            | "agrave"
            | "aacute"
            | "acircumflex"
            | "atilde"
            | "adiaeresis"
            | "aring"
            | "ae"
            | "ccedilla"
            | "egrave"
            | "eacute"
            | "ecircumflex"
            | "ediaeresis"
            | "igrave"
            | "iacute"
            | "icircumflex"
            | "idiaeresis"
            | "eth"
            | "ntilde"
            | "ograve"
            | "oacute"
            | "ocircumflex"
            | "otilde"
            | "odiaeresis"
            | "division"
            | "oslash"
            | "ugrave"
            | "uacute"
            | "ucircumflex"
            | "udiaeresis"
            | "yacute"
            | "thorn"
            | "ydiaeresis"
    )
}

fn is_librime_latin2_key_name(name: &str) -> bool {
    matches!(
        name,
        "Aogonek"
            | "breve"
            | "Lstroke"
            | "Lcaron"
            | "Sacute"
            | "Scaron"
            | "Scedilla"
            | "Tcaron"
            | "Zacute"
            | "Zcaron"
            | "Zabovedot"
            | "aogonek"
            | "ogonek"
            | "lstroke"
            | "lcaron"
            | "sacute"
            | "caron"
            | "scaron"
            | "scedilla"
            | "tcaron"
            | "zacute"
            | "doubleacute"
            | "zcaron"
            | "zabovedot"
            | "Racute"
            | "Abreve"
            | "Lacute"
            | "Cacute"
            | "Ccaron"
            | "Eogonek"
            | "Ecaron"
            | "Dcaron"
            | "Dstroke"
            | "Nacute"
            | "Ncaron"
            | "Odoubleacute"
            | "Rcaron"
            | "Uring"
            | "Udoubleacute"
            | "Tcedilla"
            | "racute"
            | "abreve"
            | "lacute"
            | "cacute"
            | "ccaron"
            | "eogonek"
            | "ecaron"
            | "dcaron"
            | "dstroke"
            | "nacute"
            | "ncaron"
            | "odoubleacute"
            | "udoubleacute"
            | "rcaron"
            | "uring"
            | "tcedilla"
            | "abovedot"
    )
}

fn is_librime_latin3_key_name(name: &str) -> bool {
    matches!(
        name,
        "Hstroke"
            | "Hcircumflex"
            | "Iabovedot"
            | "Gbreve"
            | "Jcircumflex"
            | "hstroke"
            | "hcircumflex"
            | "idotless"
            | "gbreve"
            | "jcircumflex"
            | "Cabovedot"
            | "Ccircumflex"
            | "Gabovedot"
            | "Gcircumflex"
            | "Ubreve"
            | "Scircumflex"
            | "cabovedot"
            | "ccircumflex"
            | "gabovedot"
            | "gcircumflex"
            | "ubreve"
            | "scircumflex"
    )
}

fn is_librime_latin4_key_name(name: &str) -> bool {
    matches!(
        name,
        "kappa"
            | "kra"
            | "Rcedilla"
            | "Itilde"
            | "Lcedilla"
            | "Emacron"
            | "Gcedilla"
            | "Tslash"
            | "rcedilla"
            | "itilde"
            | "lcedilla"
            | "emacron"
            | "gcedilla"
            | "tslash"
            | "ENG"
            | "eng"
            | "Amacron"
            | "Iogonek"
            | "Eabovedot"
            | "Imacron"
            | "Ncedilla"
            | "Omacron"
            | "Kcedilla"
            | "Uogonek"
            | "Utilde"
            | "Umacron"
            | "amacron"
            | "iogonek"
            | "eabovedot"
            | "imacron"
            | "ncedilla"
            | "omacron"
            | "kcedilla"
            | "uogonek"
            | "utilde"
            | "umacron"
    )
}

fn is_librime_kana_key_name(name: &str) -> bool {
    matches!(
        name,
        "overline"
            | "kana_fullstop"
            | "kana_openingbracket"
            | "kana_closingbracket"
            | "kana_comma"
            | "kana_conjunctive"
            | "kana_middledot"
            | "kana_WO"
            | "kana_a"
            | "kana_i"
            | "kana_u"
            | "kana_e"
            | "kana_o"
            | "kana_ya"
            | "kana_yu"
            | "kana_yo"
            | "kana_tsu"
            | "kana_tu"
            | "prolongedsound"
            | "kana_A"
            | "kana_I"
            | "kana_U"
            | "kana_E"
            | "kana_O"
            | "kana_KA"
            | "kana_KI"
            | "kana_KU"
            | "kana_KE"
            | "kana_KO"
            | "kana_SA"
            | "kana_SHI"
            | "kana_SU"
            | "kana_SE"
            | "kana_SO"
            | "kana_TA"
            | "kana_CHI"
            | "kana_TI"
            | "kana_TSU"
            | "kana_TU"
            | "kana_TE"
            | "kana_TO"
            | "kana_NA"
            | "kana_NI"
            | "kana_NU"
            | "kana_NE"
            | "kana_NO"
            | "kana_HA"
            | "kana_HI"
            | "kana_FU"
            | "kana_HU"
            | "kana_HE"
            | "kana_HO"
            | "kana_MA"
            | "kana_MI"
            | "kana_MU"
            | "kana_ME"
            | "kana_MO"
            | "kana_YA"
            | "kana_YU"
            | "kana_YO"
            | "kana_RA"
            | "kana_RI"
            | "kana_RU"
            | "kana_RE"
            | "kana_RO"
            | "kana_WA"
            | "kana_N"
            | "voicedsound"
            | "semivoicedsound"
    )
}

fn is_librime_arabic_key_name(name: &str) -> bool {
    matches!(
        name,
        "Arabic_comma"
            | "Arabic_semicolon"
            | "Arabic_question_mark"
            | "Arabic_hamza"
            | "Arabic_maddaonalef"
            | "Arabic_hamzaonalef"
            | "Arabic_hamzaonwaw"
            | "Arabic_hamzaunderalef"
            | "Arabic_hamzaonyeh"
            | "Arabic_alef"
            | "Arabic_beh"
            | "Arabic_tehmarbuta"
            | "Arabic_teh"
            | "Arabic_theh"
            | "Arabic_jeem"
            | "Arabic_hah"
            | "Arabic_khah"
            | "Arabic_dal"
            | "Arabic_thal"
            | "Arabic_ra"
            | "Arabic_zain"
            | "Arabic_seen"
            | "Arabic_sheen"
            | "Arabic_sad"
            | "Arabic_dad"
            | "Arabic_tah"
            | "Arabic_zah"
            | "Arabic_ain"
            | "Arabic_ghain"
            | "Arabic_tatweel"
            | "Arabic_feh"
            | "Arabic_qaf"
            | "Arabic_kaf"
            | "Arabic_lam"
            | "Arabic_meem"
            | "Arabic_noon"
            | "Arabic_ha"
            | "Arabic_heh"
            | "Arabic_waw"
            | "Arabic_alefmaksura"
            | "Arabic_yeh"
            | "Arabic_fathatan"
            | "Arabic_dammatan"
            | "Arabic_kasratan"
            | "Arabic_fatha"
            | "Arabic_damma"
            | "Arabic_kasra"
            | "Arabic_shadda"
            | "Arabic_sukun"
    )
}

fn is_librime_cyrillic_key_name(name: &str) -> bool {
    matches!(
        name,
        "Serbian_dje"
            | "Macedonia_gje"
            | "Cyrillic_io"
            | "Ukrainian_ie"
            | "Ukranian_je"
            | "Macedonia_dse"
            | "Ukrainian_i"
            | "Ukranian_i"
            | "Ukrainian_yi"
            | "Ukranian_yi"
            | "Cyrillic_je"
            | "Serbian_je"
            | "Cyrillic_lje"
            | "Serbian_lje"
            | "Cyrillic_nje"
            | "Serbian_nje"
            | "Serbian_tshe"
            | "Macedonia_kje"
            | "Byelorussian_shortu"
            | "Cyrillic_dzhe"
            | "Serbian_dze"
            | "numerosign"
            | "Serbian_DJE"
            | "Macedonia_GJE"
            | "Cyrillic_IO"
            | "Ukrainian_IE"
            | "Ukranian_JE"
            | "Macedonia_DSE"
            | "Ukrainian_I"
            | "Ukranian_I"
            | "Ukrainian_YI"
            | "Ukranian_YI"
            | "Cyrillic_JE"
            | "Serbian_JE"
            | "Cyrillic_LJE"
            | "Serbian_LJE"
            | "Cyrillic_NJE"
            | "Serbian_NJE"
            | "Serbian_TSHE"
            | "Macedonia_KJE"
            | "Byelorussian_SHORTU"
            | "Cyrillic_DZHE"
            | "Serbian_DZE"
            | "Cyrillic_yu"
            | "Cyrillic_a"
            | "Cyrillic_be"
            | "Cyrillic_tse"
            | "Cyrillic_de"
            | "Cyrillic_ie"
            | "Cyrillic_ef"
            | "Cyrillic_ghe"
            | "Cyrillic_ha"
            | "Cyrillic_i"
            | "Cyrillic_shorti"
            | "Cyrillic_ka"
            | "Cyrillic_el"
            | "Cyrillic_em"
            | "Cyrillic_en"
            | "Cyrillic_o"
            | "Cyrillic_pe"
            | "Cyrillic_ya"
            | "Cyrillic_er"
            | "Cyrillic_es"
            | "Cyrillic_te"
            | "Cyrillic_u"
            | "Cyrillic_zhe"
            | "Cyrillic_ve"
            | "Cyrillic_softsign"
            | "Cyrillic_yeru"
            | "Cyrillic_ze"
            | "Cyrillic_sha"
            | "Cyrillic_e"
            | "Cyrillic_shcha"
            | "Cyrillic_che"
            | "Cyrillic_hardsign"
            | "Cyrillic_YU"
            | "Cyrillic_A"
            | "Cyrillic_BE"
            | "Cyrillic_TSE"
            | "Cyrillic_DE"
            | "Cyrillic_IE"
            | "Cyrillic_EF"
            | "Cyrillic_GHE"
            | "Cyrillic_HA"
            | "Cyrillic_I"
            | "Cyrillic_SHORTI"
            | "Cyrillic_KA"
            | "Cyrillic_EL"
            | "Cyrillic_EM"
            | "Cyrillic_EN"
            | "Cyrillic_O"
            | "Cyrillic_PE"
            | "Cyrillic_YA"
            | "Cyrillic_ER"
            | "Cyrillic_ES"
            | "Cyrillic_TE"
            | "Cyrillic_U"
            | "Cyrillic_ZHE"
            | "Cyrillic_VE"
            | "Cyrillic_SOFTSIGN"
            | "Cyrillic_YERU"
            | "Cyrillic_ZE"
            | "Cyrillic_SHA"
            | "Cyrillic_E"
            | "Cyrillic_SHCHA"
            | "Cyrillic_CHE"
            | "Cyrillic_HARDSIGN"
    )
}

fn is_librime_greek_key_name(name: &str) -> bool {
    matches!(
        name,
        "Greek_ALPHAaccent"
            | "Greek_EPSILONaccent"
            | "Greek_ETAaccent"
            | "Greek_IOTAaccent"
            | "Greek_IOTAdieresis"
            | "Greek_IOTAdiaeresis"
            | "Greek_OMICRONaccent"
            | "Greek_UPSILONaccent"
            | "Greek_UPSILONdieresis"
            | "Greek_OMEGAaccent"
            | "Greek_accentdieresis"
            | "Greek_horizbar"
            | "Greek_alphaaccent"
            | "Greek_epsilonaccent"
            | "Greek_etaaccent"
            | "Greek_iotaaccent"
            | "Greek_iotadieresis"
            | "Greek_iotaaccentdieresis"
            | "Greek_omicronaccent"
            | "Greek_upsilonaccent"
            | "Greek_upsilondieresis"
            | "Greek_upsilonaccentdieresis"
            | "Greek_omegaaccent"
            | "Greek_ALPHA"
            | "Greek_BETA"
            | "Greek_GAMMA"
            | "Greek_DELTA"
            | "Greek_EPSILON"
            | "Greek_ZETA"
            | "Greek_ETA"
            | "Greek_THETA"
            | "Greek_IOTA"
            | "Greek_KAPPA"
            | "Greek_LAMBDA"
            | "Greek_LAMDA"
            | "Greek_MU"
            | "Greek_NU"
            | "Greek_XI"
            | "Greek_OMICRON"
            | "Greek_PI"
            | "Greek_RHO"
            | "Greek_SIGMA"
            | "Greek_TAU"
            | "Greek_UPSILON"
            | "Greek_PHI"
            | "Greek_CHI"
            | "Greek_PSI"
            | "Greek_OMEGA"
            | "Greek_alpha"
            | "Greek_beta"
            | "Greek_gamma"
            | "Greek_delta"
            | "Greek_epsilon"
            | "Greek_zeta"
            | "Greek_eta"
            | "Greek_theta"
            | "Greek_iota"
            | "Greek_kappa"
            | "Greek_lambda"
            | "Greek_lamda"
            | "Greek_mu"
            | "Greek_nu"
            | "Greek_xi"
            | "Greek_omicron"
            | "Greek_pi"
            | "Greek_rho"
            | "Greek_sigma"
            | "Greek_finalsmallsigma"
            | "Greek_tau"
            | "Greek_upsilon"
            | "Greek_phi"
            | "Greek_chi"
            | "Greek_psi"
            | "Greek_omega"
    )
}

fn is_librime_technical_key_name(name: &str) -> bool {
    matches!(
        name,
        "leftradical"
            | "topleftradical"
            | "horizconnector"
            | "topintegral"
            | "botintegral"
            | "vertconnector"
            | "topleftsqbracket"
            | "botleftsqbracket"
            | "toprightsqbracket"
            | "botrightsqbracket"
            | "topleftparens"
            | "botleftparens"
            | "toprightparens"
            | "botrightparens"
            | "leftmiddlecurlybrace"
            | "rightmiddlecurlybrace"
            | "topleftsummation"
            | "botleftsummation"
            | "topvertsummationconnector"
            | "botvertsummationconnector"
            | "toprightsummation"
            | "botrightsummation"
            | "rightmiddlesummation"
            | "lessthanequal"
            | "notequal"
            | "greaterthanequal"
            | "integral"
            | "therefore"
            | "variation"
            | "infinity"
            | "nabla"
            | "approximate"
            | "similarequal"
            | "ifonlyif"
            | "implies"
            | "identical"
            | "radical"
            | "includedin"
            | "includes"
            | "intersection"
            | "union"
            | "logicaland"
            | "logicalor"
            | "partialderivative"
            | "function"
            | "leftarrow"
            | "uparrow"
            | "rightarrow"
            | "downarrow"
            | "blank"
            | "soliddiamond"
            | "checkerboard"
            | "ht"
            | "ff"
            | "cr"
            | "lf"
            | "nl"
            | "vt"
            | "lowrightcorner"
            | "uprightcorner"
            | "upleftcorner"
            | "lowleftcorner"
            | "crossinglines"
            | "horizlinescan1"
            | "horizlinescan3"
            | "horizlinescan5"
            | "horizlinescan7"
            | "horizlinescan9"
            | "leftt"
            | "rightt"
            | "bott"
            | "topt"
            | "vertbar"
    )
}

fn is_librime_publishing_key_name(name: &str) -> bool {
    matches!(
        name,
        "emspace"
            | "enspace"
            | "em3space"
            | "em4space"
            | "digitspace"
            | "punctspace"
            | "thinspace"
            | "hairspace"
            | "emdash"
            | "endash"
            | "signifblank"
            | "ellipsis"
            | "doubbaselinedot"
            | "onethird"
            | "twothirds"
            | "onefifth"
            | "twofifths"
            | "threefifths"
            | "fourfifths"
            | "onesixth"
            | "fivesixths"
            | "careof"
            | "figdash"
            | "leftanglebracket"
            | "decimalpoint"
            | "rightanglebracket"
            | "marker"
            | "oneeighth"
            | "threeeighths"
            | "fiveeighths"
            | "seveneighths"
            | "trademark"
            | "signaturemark"
            | "trademarkincircle"
            | "leftopentriangle"
            | "rightopentriangle"
            | "emopencircle"
            | "emopenrectangle"
            | "leftsinglequotemark"
            | "rightsinglequotemark"
            | "leftdoublequotemark"
            | "rightdoublequotemark"
            | "prescription"
            | "minutes"
            | "seconds"
            | "latincross"
            | "hexagram"
            | "filledrectbullet"
            | "filledlefttribullet"
            | "filledrighttribullet"
            | "emfilledcircle"
            | "emfilledrect"
            | "enopencircbullet"
            | "enopensquarebullet"
            | "openrectbullet"
            | "opentribulletup"
            | "opentribulletdown"
            | "openstar"
            | "enfilledcircbullet"
            | "enfilledsqbullet"
            | "filledtribulletup"
            | "filledtribulletdown"
            | "leftpointer"
            | "rightpointer"
            | "club"
            | "diamond"
            | "heart"
            | "maltesecross"
            | "dagger"
            | "doubledagger"
            | "checkmark"
            | "ballotcross"
            | "musicalsharp"
            | "musicalflat"
            | "malesymbol"
            | "femalesymbol"
            | "telephone"
            | "telephonerecorder"
            | "phonographcopyright"
            | "caret"
            | "singlelowquotemark"
            | "doublelowquotemark"
            | "cursor"
            | "leftcaret"
            | "rightcaret"
            | "downcaret"
            | "upcaret"
            | "overbar"
            | "downtack"
            | "upshoe"
            | "downstile"
            | "underbar"
            | "jot"
            | "quad"
            | "uptack"
            | "circle"
            | "upstile"
            | "downshoe"
            | "rightshoe"
            | "leftshoe"
            | "lefttack"
            | "righttack"
    )
}

fn is_librime_hebrew_key_name(name: &str) -> bool {
    matches!(
        name,
        "hebrew_doublelowline"
            | "hebrew_aleph"
            | "hebrew_bet"
            | "hebrew_beth"
            | "hebrew_gimel"
            | "hebrew_gimmel"
            | "hebrew_dalet"
            | "hebrew_daleth"
            | "hebrew_he"
            | "hebrew_waw"
            | "hebrew_zain"
            | "hebrew_zayin"
            | "hebrew_chet"
            | "hebrew_het"
            | "hebrew_tet"
            | "hebrew_teth"
            | "hebrew_yod"
            | "hebrew_finalkaph"
            | "hebrew_kaph"
            | "hebrew_lamed"
            | "hebrew_finalmem"
            | "hebrew_mem"
            | "hebrew_finalnun"
            | "hebrew_nun"
            | "hebrew_samech"
            | "hebrew_samekh"
            | "hebrew_ayin"
            | "hebrew_finalpe"
            | "hebrew_pe"
            | "hebrew_finalzade"
            | "hebrew_finalzadi"
            | "hebrew_zade"
            | "hebrew_zadi"
            | "hebrew_kuf"
            | "hebrew_qoph"
            | "hebrew_resh"
            | "hebrew_shin"
            | "hebrew_taf"
            | "hebrew_taw"
    )
}
