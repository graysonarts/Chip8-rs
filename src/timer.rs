use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

fn decrement(t: &Arc<AtomicU8>) -> Result<(), u8> {
    t.fetch_update(
        |t| {
            Some(match t {
                0 => 0,
                _ => t - 1,
            })
        },
        Ordering::SeqCst,
        Ordering::SeqCst,
    )?;

    Ok(())
}
pub(crate) struct Timer {
    pub delay: Arc<AtomicU8>,
    pub sound: Arc<AtomicU8>,

    thread: Option<JoinHandle<()>>,
}

impl Timer {
    pub fn new() -> Self {
        Self::with_values(0, 0)
    }

    pub fn with_values(delay: u8, sound: u8) -> Self {
        Self {
            delay: Arc::new(AtomicU8::new(delay)),
            sound: Arc::new(AtomicU8::new(sound)),
            thread: None,
        }
    }

    pub fn start(&mut self) {
        let delay = self.delay.clone();
        let sound = self.sound.clone();
        self.thread = Some(std::thread::spawn(move || {
					loop {
						std::thread::sleep(std::time::Duration::from_millis(16));
						decrement(&delay).unwrap_or_default();
						decrement(&sound).unwrap_or_default();
					}
				}));
    }

}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_timers_do_not_go_below_zero() {
        let t = Timer::new();
        decrement(&t.delay).expect("fetch update failed");
        assert_eq!(t.delay.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_timers_above_zero_decrement() {
        let t = Timer::with_values(1, 2);
				decrement(&t.delay).expect("fetch update failed");
				decrement(&t.sound).expect("fetch update failed");
        assert_eq!(t.delay.load(Ordering::SeqCst), 0);
        assert_eq!(t.sound.load(Ordering::SeqCst), 1);
    }
}
