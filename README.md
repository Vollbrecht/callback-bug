# callback problem

Discription: callback gets not executed whenn handle function does not use call_by_value using sixtyfps
To illustrate the problem i createt the same function twice
``` 
fn render_call_by_value(call_value: i32) -> sixtyfps::Image {...}

fn render_call_void() -> sixtyfps::Image {...}

```

## Expectation
 Both images should get updated each time the callback gets executed.

## Behaviour
Only the function with call_by_value gets updated

For bug reproduction use the following minimal repository:

For demonstatrion purpose both run at the same time, but commenting one function out and only let one run does not change the behaviour.

## Working of the provided example
There is a seperate thread that updates an "phase" value into a global mutexed variable. Its used to create the animation effetct. The thread also executes `     sixtyfps::invoke_from_event_loop` to update the context.
The appwindow.60 defines both needed callbacks and the two Images that should be drawn.
The main function only  bootstrep the thread,create both callbacks and than run ui.run()

