const LAYER_COUNT: usize = 5; // adjust to the number of layers defined below

pub const COLS: usize = 12;
pub const ROWS: usize = 4;

use keyberon::action::{Action, HoldTapAction, HoldTapConfig, m};
use keyberon::key_code::KeyCode;
use keyberon::layout;

pub type QuackenLayout = layout::Layout<COLS, ROWS, LAYER_COUNT, ()>;

// common shortcuts -- adapt to your OS layout if necessary, e.g. for Ergol:
// const CLOSE: Action<()> = m(&[KeyCode::LCtrl, KeyCode::T].as_slice());
// const COPY:  Action<()> = m(&[KeyCode::LCtrl, KeyCode::W].as_slice());
const CTRL: KeyCode = KeyCode::LCtrl; // would give CMD on macOS
// const SUPER: KeyCode = KeyCode::LGui; // would give CMD on macOS
// const UNDO: Action<()> = m(&[CTRL, KeyCode::Z].as_slice());
// const CUT: Action<()> = m(&[CTRL, KeyCode::X].as_slice());
const COPY: Action<()> = m(&[CTRL, KeyCode::C].as_slice());
const PASTE: Action<()> = m(&[CTRL, KeyCode::V].as_slice());
// const SAVE: Action<()> = m(&[CTRL, KeyCode::S].as_slice());
// const FIRST: Action<()> = m(&[SUPER, KeyCode::Kb1].as_slice()); // first windows focus
// const SECOND: Action<()> = m(&[SUPER, KeyCode::Kb2].as_slice()); // second windows focus   s

const ACCENTS_LAYER: Action<()> = Action::Layer(2);

static O_TAP_ACCENTS: Action<()> = Action::HoldTap(&HoldTapAction {
    timeout: 200,         // tweak to taste (ticks, usually ms)
    tap_hold_interval: 0, // set to desired value, e.g. 0 for default behavior
    hold: Action::NoOp,
    tap: ACCENTS_LAYER,
    config: HoldTapConfig::Default,
});

// On French AZERTY: Kb2 -> é, Kb7 -> è, Kb0 -> à
const E_ACUTE: Action<()> = Action::KeyCode(KeyCode::Kb2); // é
const E_GRAVE: Action<()> = Action::KeyCode(KeyCode::Kb7); // è
const A_GRAVE: Action<()> = Action::KeyCode(KeyCode::Kb0); // à

// chiffres
const ZERO: Action<()> = m(&[KeyCode::LShift, KeyCode::Kb0].as_slice());
const UN: Action<()> = m(&[KeyCode::LShift, KeyCode::Kb1].as_slice());
const DEUX: Action<()> = m(&[KeyCode::LShift, KeyCode::Kb2].as_slice());
const TROIS: Action<()> = m(&[KeyCode::LShift, KeyCode::Kb3].as_slice());
const QUATRE: Action<()> = m(&[KeyCode::LShift, KeyCode::Kb4].as_slice());
const CINQ: Action<()> = m(&[KeyCode::LShift, KeyCode::Kb5].as_slice());
const SIX: Action<()> = m(&[KeyCode::LShift, KeyCode::Kb6].as_slice());
const SEPT: Action<()> = m(&[KeyCode::LShift, KeyCode::Kb7].as_slice());
const HUIT: Action<()> = m(&[KeyCode::LShift, KeyCode::Kb8].as_slice());
const NEUF: Action<()> = m(&[KeyCode::LShift, KeyCode::Kb9].as_slice());

// other shortcuts
const STB: Action<()> = m(&[KeyCode::RShift, KeyCode::Tab].as_slice());

// Médias
// const BCK: Action<()> = Action::KeyCode(KeyCode::MediaBack);
// const FWD: Action<()> = Action::KeyCode(KeyCode::MediaForward);

#[rustfmt::skip]
pub static LAYERS: layout::Layers<COLS, ROWS, LAYER_COUNT, ()> = layout::layout! {

    { // base layer -- don't worry about lhe key names, this will reflect your OS keyboard layout
        [ Escape  Q    W    E    R    T           Y    U    I    {O_TAP_ACCENTS}    P    BSpace ],
        [ LAlt    A    S    D    F    G           H    J    K    L                  ;     Enter ],
        [ LShift  Z    X    C    V    B           N    M    ,    .                  /    RShift ],
        [ n n n          LCtrl Space (1)          LGui Space (3)                            n n n ],
    }
    { // 1 NumNav
        [ t       Tab      Tab  Up   End  PgUp      n    {SEPT}   {HUIT}   {NEUF}    n    Delete ],
        [ t    CapsLock    Left Down Right PgDown   n    {QUATRE} {CINQ}   {SIX}    {ZERO}    t ],
        [ t        n       n    {COPY}{PASTE}{STB}  n    {UN}     {DEUX}   {TROIS}   n    t ],
        [ n n n            LCtrl  Space     t       (4)  LAlt  t        n n n ],
    }
    { // 2 QWERTY éèê letters..
        [ t         n    n    n    n    n                   n    n    n    n    n        t ],
        [ t     {A_GRAVE}{E_ACUTE} {E_GRAVE}  n    n        K    n    n    n    n        t ],
        [ t         n    n    n    n    n                   n    n    n    n    n        t ],
        [ n n n              t     t    t                   t    t    t               n n n ],
    }    
    { // 3 QWERTY alt gr replacement
        [ t         ^    <    >    $    %         @    &    *  '\''  '`'        t ],
        [ t        '{'  '('  ')'  '}'   =       '\\'   +    -    /   '"'        t ],
        [ t         ~   '['  ']'  '_'   #         |    !    ;    :    ?         t ],
        [ n n n              t  Space  t         t    t    t               n n n ],
    }
    //  4 Function Keys
    {
        [ t        F1   F2   F3   F4    n         n             MediaPlayPause  PScreen             t               n    t  ],
        [ t        F5   F6   F7   F8    n         MediaVolDown  MediaVolUp      MediaPreviousSong   MediaNextSong   Mute t  ],   
        [ t        F9   F10  F11  F12   n         n             n               n           n                       n    t  ],
        [ n n n               t    t    t         t             t               t           n                       n    n  ],
    }
};
