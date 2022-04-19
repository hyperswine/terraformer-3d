# TODO

## MVC - Controller

The 'controller' in model-view-control framework

SHOULD HAVE ACCESS TO THE CURRENT `<MAP>/<LEVEL>` when 'in game', i.e. completely loaded.

UPDATE THE VIEW WHEN SOMETHING CHANGES. Wait actually we can have 'caller prompted observer pattern" to update on demand at the next tick.

So the updates are queued for each entity. On the next tick, game controller checks entity update queue, goes to those entities and fetches the new state.

Then makes a render call using the new state.
