# R.I.B.S battle system

The R.I.B.S battle system is a simple version of a AFK grid based battle system used to learn and show some capabilities of the RUST language and how to get it running!

# System overview

## Data Storage

The data for this application stored in [SurrealDb](https://surrealdb.com/).

## API layer 

As any "modern" application we have an API. This one is powered by [Rocket](https://rocket.rs/).

# Domain objects

To better understand the system we first have to present some domain objects that can be found in the system

## Combatant

The _combatant_ object represents an instance of a "player" that is participating in the battle. Each combatant has the following properties associated with him:

- A humanly readable name used for identification
- A total number of hit points
- A damage output
- The number of squares it can traverse in a single round 

## Battlefield

A "field" with a width and height on which the battle takes place.

Each combatant takes up 1 field on the battlefield.

A minimum size of 4x4 is imposed! 

# Game rules

- All combatants know the location of all other combatants

## Game start

- A fixed number of combatants is placed randomly on the battle field, no overlap is allowed.

## Rounds

The battle is carried out in rounds. Each round has a 

- Movement phase
- Attack phase

In each round every combatant can execute both phases if possible. 

Each phase can be skipped!

### Movement phase

Each combatant has a predetermined maximum distance it can travel.

The closest _alive_ combatant is targeted and moved towards. Do not overshoot the target!

If the distance to the _target_ is less than the maximal movement distance then only the __needed__ distance will be traveled.

### Attack phase

Each combatant has a damage output. This damage can be applied to any target within a 1 block radius. 

If the target is in _reach_ then the whole damage output is deducted from the targets hit points.

## Game end

There are two ways to end an battle:

- All combatants except one are dead
- A total number of a 1000 rounds was played

If multiple combatants are alive after the final round then a draw is achieved