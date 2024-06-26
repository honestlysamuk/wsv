A Moore machine is a form of state machine which is defined by six variables.

For educational purposes, I am going to write code in such a way as to highlight those six things as clearly as possible. 

A state machine, as defined by wikipedia, is composed of five parts, which together help define the states in which your machine can be, and which states follow which other states for a given input. It says nothing of computation: only of transitioning through states. This is equivalent to an object with a single field called `state` which is mutated every time you call `transition`, but nothing else happens. If you want something to happen, you are now looking at a type of state machine called a transducer. There are two versions of a transducer: a Moore Machine and a Mealy Machine. Here, I explore the differences between the two, and how those differences manifest in code. Moreover, I also provide an example of how I would use a state machine in a real use case, rather than ones contrived for educational purposes. Having said that, writing code for educational purposes might actually be the quality you want to optimise for. That's not for me to judge. 

Let us assume that you have a state machine in your programme, and that in a given state, you want something to happen elsewhere in your programme. Then look no further than the implementation below. I have deliberately written code in a way which mirrors the theory as closely as I can. That is, this is not my first implementation which doesn't do as good a job of highlighting which bit is the state machine. It is a lot faster though. 

Astute readers may notice that there are a few details which do not line up nicely My favourite is the while loop. You might forgive me for using a finish state when it's not defined in the spec. The more fun one is that the input list is actually unbounded, since inputs.next() eventually just returns None forever. I need the states to transition more times than there are inputs in order to reach the Finished state.

The choice of output set here, called `Transform`s, is not opinionated. The crux is that the output set must be finite! Let us consider some alternatives. It is reasonable at first, to desire the output list of the state machine to be a `Vec<WsvValue>`. Indeed, that is the output of the tool I am building. It is not, however, the output of the state machine. I am using the pattern of a state machine to get there, but the state machine isn't doing everything. I want to be very explicit on the boundary between the state machine and the rest of my code. I have. I want you to think of this machine as a black box, which takes a list of things and produces a list of things. You could try to make the list of things a `Vec<WsvValue>`. However, WsvValue is an infinite set! each one can have an unlimited number of characters inside, so it is impossible to define a finite set of

```rs
/// Imagine for this one that I actually have a `PushChar` variant for every `char`,
/// and an `AddError` variant for every `kind`. Each variant represents a
/// transformation on the `Data` struct.
/// 
/// The data struct is what contains everything relevant to the features I want from my parser.
/// The state machine is just a means to an end. It is a tool I have used to produce a list of
///  simple transformations on this struct. That's it.
///
/// 
```