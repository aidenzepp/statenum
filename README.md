# statenum
`statenum`, short for "state-enum", is a procedural macro attribute that generates a trait and a struct for each variant of an enum.

## Installation

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
statenum = "1.0"
```

## Usage

To use `statenum`, add the `#[statenum]` attribute to your enum definition. 
By default, the generated trait will be named `State`. 
You can override this by providing a name as an argument to the macro.

```rust
use statenum::statenum;
#[statenum]
enum RocketStage {
    Grounded,
    Launched,
    // and so on...
}

#[statenum("PayloadState")]
enum PayloadStage {
    Vacant,
    Packed,
    // and so on...
}
```
## Visibility

This will generate a trait named `State` and a struct for each variant of the enum. 
The visibility of the enum carries through to the implementation of `statenum`. 
If the enum is marked as `pub`, the resulting struct variants and trait will also be marked as `pub`. 
However, if the enum isn't public then neither will the resulting components.

```rust
mod hidden {
    use statenum::statenum;
    
    #[statenum]
    enum RocketStage {
        Grounded,
        Launched,
    }

    pub struct Rocket<Stage: State = Grounded> {
        stage: Stage,
    }
}

// This would cause an error because the enum `RocketStage` is marked as private.
// let grounded: hidden::Rocket<hidden::Grounded>;

// The same applies to the trait.
// trait PayloadState: hidden::State {}
```

## Examples

Normally, the state pattern in Rust requires the use of multiple struct definitions. 
The example below demonstrates the typical method of implementing the state pattern and is taken from the book 'Rust for Rustaceans' by Jon Gjengset.

```rust
use std::marker::PhantomData;

struct Grounded;
struct Launched;
// and so on...

pub struct Rocket<Stage = Grounded> {
    stage: PhantomData<Stage>,
}

impl<Stage> Rocket<Stage> {
    // ...
}

impl Rocket<Grounded> {
    pub fn launch(self) -> Rocket<Launched> {
        Rocket::<Launched> { stage: PhantomData::<Launched> }
    }
}

impl Rocket<Launched> {
    pub fn accelerate(&mut self) {
        // ...
    }

    pub fn decelerate(&mut self) {
        // ...
    }
}
```


Using the macro without any trait name specified will create a trait under the name of `State`, like so (intentionally ignoring the `PhantomData` aspect):

```rust
use statenum::statenum;

#[statenum]
enum RocketStage {
    Grounded,
    Launched,
    // and so on...
}

pub struct Rocket<Stage: State = Grounded> {
    stage: Stage,
}

impl<Stage: State> Rocket<Stage> {
    // ...
}

impl Rocket<Grounded> {
    pub fn launch(self) -> Rocket<Launched> {
        Rocket::<Launched> { stage: Launched }
    }
}

impl Rocket<Launched> {
    pub fn accelerate(&mut self) {
        // ...
    }

    pub fn decelerate(&mut self) {
        // ...
    }
}
```


Otherwise, if you would prefer to name the trait something other than `State`, you can accomplish that by specifying a name:

```rust
use statenum::statenum;

#[statenum("RocketState")]
enum RocketStage {
    Grounded,
    Launched,
    // and so on...
}

pub struct Rocket<Stage: RocketState = Grounded> {
    stage: Stage,
}

impl<Stage: RocketState> Rocket<Stage> {
    // ...
}

impl Rocket<Grounded> {
    pub fn launch(self) -> Rocket<Launched> {
        Rocket::<Launched> { stage: Launched }
    }
}

impl Rocket<Launched> {
    pub fn accelerate(&mut self) {
        // ...
    }

    pub fn decelerate(&mut self) {
        // ...
    }
}
```

# License

`statenum` is distributed under the terms of the MIT license.
