use std::{
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex, OnceLock,
    },
    time::Instant,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StartupTraceMemorySample {
    pub working_set_bytes: Option<u64>,
    pub peak_working_set_bytes: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StartupTraceEvent {
    pub name: &'static str,
    pub micros: u128,
    pub working_set_before: Option<u64>,
    pub working_set_after: Option<u64>,
    pub peak_working_set_after: Option<u64>,
}

type MemorySampler = fn() -> StartupTraceMemorySample;

struct TraceState {
    events: Vec<StartupTraceEvent>,
    memory_sampler: Option<MemorySampler>,
}

static TRACE_ENABLED: AtomicBool = AtomicBool::new(false);
static TRACE_STATE: OnceLock<Mutex<TraceState>> = OnceLock::new();

pub fn begin_startup_trace(memory_sampler: Option<MemorySampler>) {
    let state = TRACE_STATE.get_or_init(|| {
        Mutex::new(TraceState {
            events: Vec::new(),
            memory_sampler: None,
        })
    });
    let mut state = state
        .lock()
        .expect("startup trace state should not be poisoned");
    state.events.clear();
    state.memory_sampler = memory_sampler;
    TRACE_ENABLED.store(true, Ordering::Release);
}

pub fn finish_startup_trace() -> Vec<StartupTraceEvent> {
    TRACE_ENABLED.store(false, Ordering::Release);
    let Some(state) = TRACE_STATE.get() else {
        return Vec::new();
    };
    let mut state = state
        .lock()
        .expect("startup trace state should not be poisoned");
    state.memory_sampler = None;
    mem::take(&mut state.events)
}

pub(crate) fn span(name: &'static str) -> StartupTraceSpan {
    if !TRACE_ENABLED.load(Ordering::Acquire) {
        return StartupTraceSpan::disabled();
    }
    let memory_before = sample_memory();
    StartupTraceSpan {
        name,
        start: Some(Instant::now()),
        working_set_before: memory_before.and_then(|sample| sample.working_set_bytes),
    }
}

fn sample_memory() -> Option<StartupTraceMemorySample> {
    if !TRACE_ENABLED.load(Ordering::Acquire) {
        return None;
    }
    let state = TRACE_STATE.get()?;
    let sampler = {
        state
            .lock()
            .expect("startup trace state should not be poisoned")
            .memory_sampler
    }?;
    Some(sampler())
}

pub(crate) struct StartupTraceSpan {
    name: &'static str,
    start: Option<Instant>,
    working_set_before: Option<u64>,
}

impl StartupTraceSpan {
    const fn disabled() -> Self {
        Self {
            name: "",
            start: None,
            working_set_before: None,
        }
    }
}

impl Drop for StartupTraceSpan {
    fn drop(&mut self) {
        let Some(start) = self.start else {
            return;
        };
        let elapsed = start.elapsed();
        let memory_after = sample_memory();
        let event = StartupTraceEvent {
            name: self.name,
            micros: elapsed.as_micros(),
            working_set_before: self.working_set_before,
            working_set_after: memory_after.and_then(|sample| sample.working_set_bytes),
            peak_working_set_after: memory_after.and_then(|sample| sample.peak_working_set_bytes),
        };
        if let Some(state) = TRACE_STATE.get() {
            state
                .lock()
                .expect("startup trace state should not be poisoned")
                .events
                .push(event);
        }
    }
}
