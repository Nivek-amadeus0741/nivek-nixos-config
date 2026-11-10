use nivek_wasm_abi::SystemEvent;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::debug;

pub struct EventBus {
    sender: broadcast::Sender<Arc<SystemEvent>>,
    total_events: AtomicU64,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self {
            sender,
            total_events: AtomicU64::new(0),
        }
    }

    pub fn publish(&self, event: SystemEvent) {
        let event_arc = Arc::new(event);
        self.total_events.fetch_add(1, Ordering::Relaxed);
        let count = self.sender.receiver_count();
        debug!(
            "Publicando evento: {:?} (suscriptores activos: {})",
            event_arc, count
        );
        let _ = self.sender.send(event_arc);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Arc<SystemEvent>> {
        self.sender.subscribe()
    }

    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }

    pub fn total_events(&self) -> u64 {
        self.total_events.load(Ordering::Relaxed)
    }
}
