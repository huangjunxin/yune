use std::{
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex, OnceLock,
    },
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MemoryProbeSample {
    pub working_set_bytes: Option<u64>,
    pub peak_working_set_bytes: Option<u64>,
    pub private_bytes: Option<u64>,
    pub allocator_live_bytes: Option<u64>,
    pub allocator_high_water_bytes: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryProbeEvent {
    pub phase: String,
    pub working_set_bytes: Option<u64>,
    pub peak_working_set_bytes: Option<u64>,
    pub private_bytes: Option<u64>,
    pub allocator_live_bytes: Option<u64>,
    pub allocator_high_water_bytes: Option<u64>,
}

type MemorySampler = fn() -> MemoryProbeSample;

struct ProbeState {
    events: Vec<MemoryProbeEvent>,
    sampler: Option<MemorySampler>,
}

static PROBE_ENABLED: AtomicBool = AtomicBool::new(false);
static PROBE_STATE: OnceLock<Mutex<ProbeState>> = OnceLock::new();

pub fn begin_memory_probe(sampler: Option<MemorySampler>) {
    let state = PROBE_STATE.get_or_init(|| {
        Mutex::new(ProbeState {
            events: Vec::new(),
            sampler: None,
        })
    });
    let mut state = state
        .lock()
        .expect("memory probe state should not be poisoned");
    state.events.clear();
    state.sampler = sampler;
    PROBE_ENABLED.store(true, Ordering::Release);
}

pub fn finish_memory_probe() -> Vec<MemoryProbeEvent> {
    PROBE_ENABLED.store(false, Ordering::Release);
    let Some(state) = PROBE_STATE.get() else {
        return Vec::new();
    };
    let mut state = state
        .lock()
        .expect("memory probe state should not be poisoned");
    state.sampler = None;
    mem::take(&mut state.events)
}

pub fn memory_probe_mark(phase: impl Into<String>) {
    if !PROBE_ENABLED.load(Ordering::Acquire) {
        return;
    }
    let phase = phase.into();
    let sample = memory_probe_sample();
    if let Some(state) = PROBE_STATE.get() {
        state
            .lock()
            .expect("memory probe state should not be poisoned")
            .events
            .push(MemoryProbeEvent {
                phase,
                working_set_bytes: sample.and_then(|sample| sample.working_set_bytes),
                peak_working_set_bytes: sample.and_then(|sample| sample.peak_working_set_bytes),
                private_bytes: sample.and_then(|sample| sample.private_bytes),
                allocator_live_bytes: sample.and_then(|sample| sample.allocator_live_bytes),
                allocator_high_water_bytes: sample
                    .and_then(|sample| sample.allocator_high_water_bytes),
            });
    }
}

fn memory_probe_sample() -> Option<MemoryProbeSample> {
    if !PROBE_ENABLED.load(Ordering::Acquire) {
        return None;
    }
    let state = PROBE_STATE.get()?;
    let sampler = {
        state
            .lock()
            .expect("memory probe state should not be poisoned")
            .sampler
    }?;
    Some(sampler())
}
