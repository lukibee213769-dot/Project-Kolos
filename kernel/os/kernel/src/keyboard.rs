/// PS/2 keyboard driver: ISR → scancode queue → decoded char stream.
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::{Lazy, Mutex};

// ── Scancode ring buffer (interrupt-safe, no heap) ────────────────────────────

const QUEUE_CAP: usize = 256;

struct RingBuf {
    buf: [u8; QUEUE_CAP],
    head: usize,
    tail: usize,
}

impl RingBuf {
    const fn new() -> Self {
        Self {
            buf: [0; QUEUE_CAP],
            head: 0,
            tail: 0,
        }
    }

    fn push(&mut self, byte: u8) -> bool {
        let next = (self.tail + 1) % QUEUE_CAP;
        if next == self.head {
            return false; // full — drop scancode
        }
        self.buf[self.tail] = byte;
        self.tail = next;
        true
    }

    fn pop(&mut self) -> Option<u8> {
        if self.head == self.tail {
            return None; // empty
        }
        let byte = self.buf[self.head];
        self.head = (self.head + 1) % QUEUE_CAP;
        Some(byte)
    }
}

static SCANCODES: Mutex<RingBuf> = Mutex::new(RingBuf::new());

/// Called from the IRQ1 interrupt handler.
pub fn push_scancode(byte: u8) {
    SCANCODES.lock().push(byte);
}

// ── Keyboard decoder (US layout, ScancodeSet1) ────────────────────────────────

static KB: Lazy<Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>>> = Lazy::new(|| {
    Mutex::new(Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    ))
});

/// Poll for the next decoded character. Returns `None` if nothing pending.
pub fn next_char() -> Option<char> {
    loop {
        let scancode = x86_64::instructions::interrupts::without_interrupts(|| {
            SCANCODES.lock().pop()
        });
        let Some(scancode) = scancode else {
            return None;
        };
        let mut kb = KB.lock();
        if let Ok(Some(event)) = kb.add_byte(scancode) {
            if let Some(key) = kb.process_keyevent(event) {
                match key {
                    DecodedKey::Unicode(c) => return Some(c),
                    DecodedKey::RawKey(_) => {}
                }
            }
        }
    }
}
