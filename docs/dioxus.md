# Dioxus Reactivity and SSR Guide

This guide covers common pitfalls and best practices for working with Dioxus reactivity and Server-Side Rendering (SSR) in this codebase.

## Table of Contents
1. [Core Concepts](#core-concepts)
2. [Common Pitfalls](#common-pitfalls)
3. [SSR vs Client-Side](#ssr-vs-client-side)
4. [Debugging Guide](#debugging-guide)
5. [Code Patterns](#code-patterns)
6. [Troubleshooting](#troubleshooting)
7. [React vs Dioxus](#react-vs-dioxus)

## Core Concepts {#core-concepts}

### Dioxus Reactivity {#dioxus-reactivity}

1. **Signals** (`use_signal()`): Reactive state that implements `Copy`
   - Reading: `signal()` creates subscriptions
   - Writing: `signal.set()` triggers re-renders
   - **No cloning needed** - signals are already `Copy`

2. **Memos** (`use_memo()`): Derived values that auto-update
3. **Effects** (`use_effect()`): Side effects, **runs client-side only**
4. **Futures** (`use_future()`): One-time background tasks, **non-reactive**

### 🚨 Critical Hook Rules {#critical-hook-rules}

- **Hooks MUST be in same order every render**
- **Hooks MUST be unconditional** - no conditionals, loops, or closures
- **Hooks MUST be at top level only**

### Common Pitfalls {#common-pitfalls}

#### 1. Reactive Cycles {#reactive-cycles}

❌ **BAD**: Writing to signals you read in the same reactive context
```rust
let memo = use_memo({
    move || {
        let current = count_signal(); // Reads count
        if current > 10 {
            count_signal.set(0); // ❌ Writes back → infinite loop
        }
        current
    }
});
```

✅ **GOOD**: Separate reading and writing
```rust
let is_over_limit = use_memo(move || count_signal() > 10); // Only read
use_effect({
    move || {
        if is_over_limit() {
            count_signal.set(0); // Write in effect, not memo
        }
    }
});
```

#### 2. SSR Violations {#ssr-violations}

❌ **BAD**: Client-side code during render
```rust
// Runs during SSR - causes problems
versions_signal.write().entry(slide_id).or_insert(1);
spawn(async move { /* client work */ });
```

✅ **GOOD**: Move client code to effects
```rust
use_effect({
    move || {
        web! {
            // Only runs on client
            versions_signal.write().entry(slide_id).or_insert(1);
        }
    }
});
```

## SSR vs Client-Side {#ssr-vs-client-side}

### SSR-Safe Patterns {#ssr-safe-patterns}

1. **Use `web! {}` macro** for client-only code
2. **Type aliases** instead of `#[cfg]` guards for DOM types
3. **Never read/write signals during render** - only in hooks

### Handling Browser-Specific Events

When working with events like `onmounted`, `onclick`, or `oninput`, the event data type is different on the server versus the client. On the client, it's a browser-specific type that gives you access to the DOM element and event details. On the server, it's a generic type with no browser information.

To handle this, use the `as_web_event()` method, which is available on Dioxus events. This method returns an `Option<&web_sys::Event>`, which will be `Some` on the client and `None` on the server. You can then use `dyn_into` to downcast the event to its specific type.

✅ **GOOD**: Safely downcasting an event
```rust
use dioxus_web::WebEventExt; // Trait that provides as_web_event()

rsx! {
    div {
        onmounted: move |event| {
            if let Some(mounted_event) = event.as_web_event() {
                if let Ok(element) = mounted_event.dyn_into::<web_sys::HtmlElement>() {
                    // This code only runs on the client
                    tracing::info!("Element mounted: {:?}", element);
                }
            }
        }
    }
}
```

This pattern allows you to write a single component that compiles for both targets but gracefully handles browser-specific logic on the client.

### Example: Clean Component Structure
```rust
#[component]
fn GoodComponent() -> Element {
    let state = use_signal(|| "initial".to_string());
    
    use_effect({
        move || {
            web! {
                // Client-side only
                setup_client_stuff();
            }
        }
    });
    
    rsx! {
        div { "{state()}" } // SSR-safe rendering
    }
}
```

## Signal Lifetimes {#signal-lifetimes}

### 🚨 Lifetime Dangers

- ❌ Moving signals into external closures (wasm-bindgen callbacks, `spawn_forever` tasks, long-lived listeners) that outlive the component scope
- ❌ No cleanup for external resources
- ❌ Signals in long-running async tasks that outlive component

### ✅ Safe Patterns

```rust
// Store cleanup handles in signals
let cleanup_handle = use_signal(|| None::<CleanupHandle>);

use_effect({
    move || {
        web! {
            let handle = register_resource();
            cleanup_handle.set(Some(handle));
        }
    }
});

// Clean up with use_drop
use_drop({
    move || {
        if let Some(handle) = cleanup_handle() {
            cleanup_resource(handle);
        }
    }
});
```

**Note**: Capturing signals inside `spawn(async move { ... })` that originates from the current component is safe—the task is tied to the component scope and is cancelled automatically on unmount. Only promote the task to `spawn_forever` or store the handle yourself if it truly needs to outlive the component, and pair that with explicit cleanup (`use_drop`).

## Debugging Guide {#debugging-guide}

### Infinite Loop Debugging {#infinite-loop-debugging}

1. **Add tracing**: Log component lifecycle, memo execution, signal reads/writes
2. **Look for patterns**: 
   - Component re-renders immediately after "Starting rsx"
   - Memo runs on every render
   - Effect runs on every render
3. **Fix cycles**: Move signal reads out of render, break reactive chains

### Common Symptoms

- **Hook ordering issues**: Different behavior server vs client
- **Lifetime issues**: Use-after-free, memory leaks
- **SSR problems**: Different hook order between builds

## Code Patterns {#code-patterns}

### State Management
```rust
#[component]
fn StateComponent() -> Element {
    let items = use_signal(|| Vec::<String>::new());
    let filter = use_signal(|| String::new());
    
    // Derived memo - no side effects
    let filtered = use_memo({
        move || {
            let f = filter().to_lowercase();
            items().iter()
                .filter(|item| item.to_lowercase().contains(&f))
                .cloned()
                .collect::<Vec<_>>()
        }
    });
    
    // Use with_mut for efficient updates
    let add_item = move |item: String| {
        items.with_mut(|v| v.push(item));
    };
    
    rsx! {
        div {
            input {
                value: "{filter()}",
                oninput: move |evt| filter.set(evt.value())
            }
            button { onclick: move |_| add_item("new".to_string()), "Add" }
            for item in filtered() { div { "{item}" } }
        }
    }
}
```

### Data Loading
```rust
#[component]
fn DataLoader(id: String) -> Element {
    let data = use_signal(|| None::<String>);
    
    use_effect({
        let id = id.clone();
        let mut data = data.clone();
        move || {
            web! {
                spawn(async move {
                    if let Some(d) = fetch_data(&id).await {
                        data.set(Some(d));
                    }
                });
            }
        }
    });
    
    rsx! {
        div {
            match data() {
                Some(d) => "{d}",
                None => "Loading..."
            }
        }
    }
}
```

### Background Workers with use_future {#use_future-pattern}

`use_future` is designed for long-lived background tasks that run once and handle continuous async operations. Unlike `use_effect` which runs reactively, `use_future` runs exactly once when the component mounts.

#### Channel Worker Pattern

The most common pattern combines `use_future` with MPSC channels:

```rust
#[component]
fn WebSocketComponent() -> Element {
    let messages = use_signal(|| Vec::<ServerMsg>::new());
    let (ws_tx, mut ws_rx) = use_channel::<ServerMsg>(100);
    
    // Background worker runs exactly once
    use_future(move || async move {
        while let Some(msg) = ws_rx.next().await {
            match msg {
                ServerMsg::Update(data) => {
                    messages.with_mut(|v| v.push(data));
                }
                ServerMsg::Error(e) => {
                    tracing::error!("WebSocket error: {}", e);
                }
            }
        }
    });
    
    // Send messages to worker
    let send_message = move |msg: ClientMsg| {
        if let Err(e) = ws_tx.try_send(msg) {
            tracing::warn!("Failed to send message: {}", e);
        }
    };
    
    rsx! {
        div {
            button { 
                onclick: move |_| send_message(ClientMsg::RequestData),
                "Request Data" 
            }
            for msg in messages() { div { "{msg:?}" } }
        }
    }
}
```

#### Time-based Background Tasks

```rust
#[component]
fn TimerComponent() -> Element {
    let elapsed = use_signal(|| 0u64);
    
    use_future(move || async move {
        let mut interval = interval(Duration::from_secs(1));
        let start = Instant::now();
        
        loop {
            interval.tick().await;
            elapsed.set(start.elapsed().as_secs());
        }
    });
    
    rsx! {
        div { "Elapsed: {elapsed()} seconds" }
    }
}
```

#### Key use_future Characteristics

- **Runs Once**: Executes exactly once when component mounts
- **Non-reactive**: Doesn't re-run when dependencies change
- **Long-lived**: Perfect for continuous background operations
- **Auto-cleanup**: Future is cancelled when component unmounts
- **Channel-based**: Commonly used with MPSC channels for communication

#### Common Use Cases

- WebSocket message handling
- Background timers/intervals
- Long-running async operations
- Channel workers for processing streams
- One-time async initialization

## Troubleshooting {#troubleshooting}

### Quick Checklist {#troubleshooting-checklist}

- [ ] No SSR violations (DOM calls, async tasks during render)?
- [ ] No reactive cycles (reading signals during render)?
- [ ] Hooks in consistent order, unconditional?
- [ ] Proper cleanup with `use_drop`?
- [ ] Signals used correctly (no cloning, proper lifetimes)?

### Common Errors {#common-errors}

| Error | Cause | Solution |
|-------|--------|--------|
| "Infinite loop detected" | Reactive cycles | Move signal reads out of render |
| "web_sys not available during SSR" | Browser APIs on server | Use `web!` macro |
| "Hook called on wrong component" | Inconsistent hook order | Ensure hooks are unconditional |
| "Use-after-free" | Signals accessed after unmount | Add cleanup with `use_drop` |

## React vs Dioxus {#react-vs-dioxus}

### Key Differences

| Feature | React | Dioxus |
|--------|-------|--------|
| State | `useState()` returns `[value, setter]` | `use_signal()` returns single signal |
| Copy trait | No copy on state | **Signals are `Copy`** |
| Cleanup | `useEffect(() => setup; return cleanup)` | `use_effect(() => setup); use_drop(cleanup)` |
| Client-side code | Manual guards | **`web! {}` macro** |
| Hooks | Can be conditional | **Must be unconditional** |
