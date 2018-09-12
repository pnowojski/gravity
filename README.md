Simple gravity engine/game. Written in Rust.

My first code ever written in Rust - it aint pretty.

There is some bug in close approaches, which shoots out objects. Two objects 
with same mass falling towards themselves can be shoot out to almost infinity.
Probably this is because gravity force is calculated in discrete samples and
the faster the object is moving, the less samples over the same distance it will
collect/trigger/calculate. Thus when initially objects are falling towards themselves,
they start slowly wtih good accuracy, but when they are passing themselves and they should
start braking, they are moving so fast, that braking is less accurate (has less samples)
compared to acceleration period.
 
Based on https://github.com/a5huynh/defender-game/
