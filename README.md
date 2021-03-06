# No callback execution when using no function argument


## Discription
 callback gets not executed when handle function does not use call_by_value.
To illustrate the problem i created the same function twice
``` 
fn render_call_by_value(call_value: i32) -> sixtyfps::Image {...}

fn render_call_void() -> sixtyfps::Image {...}

```

## Expectation
 Both images should get updated each time the callback gets executed.

## Behaviour
Only the function with call_by_value gets updated

For bug reproduction use the following minimal repository:
https://github.com/Vollbrecht/callback-bug

For demonstration purpose both run at the same time, but commenting one function out and only let one run does not change the behavior.



## Workings of the provided example
There is a separate thread that updates an "phase" value into a global mutexed variable. Its used to create the animation effect. The thread also executes `     sixtyfps::invoke_from_event_loop` to update the context.
The appwindow.60 defines both needed callbacks and the two Images that should be drawn.
The main function only  bootstrap the thread,create both callbacks and than run ui.run()



### Software version

sixtyfps 0.1.5

### Operating system

Linux 5.10.0-8-amd64

### Cargo version
```
> cargo --version 
cargo 1.57.0 (b2e52d7ca 2021-10-21)
```