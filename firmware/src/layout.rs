const LAYER_COUNT: usize = 3; // adjust to the number of layers defined below

pub const COLS: usize = 12;
pub const ROWS: usize = 4;

use keyberon::action::{Action, m};
use keyberon::key_code::KeyCode;
use keyberon::layout;

pub type QuackenLayout = layout::Layout<COLS, ROWS, LAYER_COUNT, ()>;

// common shortcuts -- adapt to your OS layout if necessary, e.g. for Ergol:
// const CLOSE: Action<()> = m(&[KeyCode::LCtrl, KeyCode::T].as_slice());
// const COPY:  Action<()> = m(&[KeyCode::LCtrl, KeyCode::W].as_slice());
const CTRL: KeyCode = KeyCode::LCtrl; // would give CMD on macOS
const UNDO: Action<()> = m(&[CTRL, KeyCode::Z].as_slice());
const CUT: Action<()> = m(&[CTRL, KeyCode::X].as_slice());
const COPY: Action<()> = m(&[CTRL, KeyCode::C].as_slice());
const PASTE: Action<()> = m(&[CTRL, KeyCode::V].as_slice());
const ALL: Action<()> = m(&[CTRL, KeyCode::A].as_slice());
const SAVE: Action<()> = m(&[CTRL, KeyCode::S].as_slice());
const CLOSE: Action<()> = m(&[CTRL, KeyCode::W].as_slice());

// other shortcuts
const STB: Action<()> = m(&[KeyCode::RShift, KeyCode::Tab].as_slice());
const BCK: Action<()> = Action::KeyCode(KeyCode::MediaBack);
const FWD: Action<()> = Action::KeyCode(KeyCode::MediaForward);

#[rustfmt::skip]
pub static LAYERS: layout::Layers<COLS, ROWS, LAYER_COUNT, ()> = layout::layout! {

    { // base layer -- don't worry about lhe key names, this will reflect your OS keyboard layout
        [ Escape Q    W    E    R    T         Y    U    I    O    P    BSpace ],
        [ Tab    A    S    D    F    G         H    J    K    L    ;     Enter ],
        [ LShift Z    X    C    V    B         N    M    ,    .    /    RShift ],
        [ n n n         LCtrl Space (1)        LGui Space RAlt             n n n ],
    }
    { // NumNav
        [ t        Tab  Home  Up   End  PgUp      n    7    8    9    n    Delete ],
        [ t    CapsLock Left Down Right PgDown    n    4    5    6    0         t ],
        [ t    {UNDO}{CUT}{COPY}{PASTE}{STB}      n    1    2    3    n         t ],
        [ n n n               t  Space   t       (2)  LAlt  t               n n n ],
    }
    // Function Keys
    {
        [ t        F1   F2   F3   F4    n         n Pause PScreen    t     n    t  ],
        [ t        F5   F6   F7   F8    n         n  {ALL}  {BCK}  {FWD} {SAVE} t  ],
        [ t        F9   F10  F11  F12   n         n    n    n    n      {CLOSE} t  ],
        [ n n n               t    t    t         t    t    t               n n n  ],
    }
};
