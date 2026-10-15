# Additional Types

This file documents additional types from the zenoh crate that weren't covered in other reference files.

## KeyExpr Types

### KeyExprUndeclaration

Undeclaration result for KeyExpr.

### OwnedKeyExpr

Owned key expression (non-reference).

### OwnedNonWildKeyExpr

Owned non-wildcard key expression.

### nonwild_keyexpr

Non-wildcard key expression.

## KeyExpr Tree

### keyexpr_tree::KeBoxTree

Boxed key expression tree.

### keyexpr_tree::KeyedSetProvider

Provider for keyed sets.

### keyexpr_tree::NonWild

Non-wildcard key expression tree node.

### keyexpr_tree::UnknownWildness

Unknown wildness indicator.

## KeyExpr Format

### key_expr::format::Iter

Iterator for key expression format.

### key_expr::format::KeFormat

Key expression format.

### key_expr::format::KeFormatter

Key expression formatter.

### key_expr::format::OwnedKeFormat

Owned key expression format.

### key_expr::format::Parsed

Parsed key expression.

### key_expr::format::Segment

Format segment.

## Set Intersection

### SetIntersectionLevel

Set intersection level for key expressions.

### FormatSetError

Error when setting key expression format.

## Handler Types (additional)

### CallbackDrop

Callback with drop handler.

### FifoChannelHandler

FIFO channel handler.

### RingChannelHandler

Ring channel handler.

### handlers::fifo::Drain

FIFO drain.

### handlers::fifo::IntoIter

FIFO into iterator.

### handlers::fifo::Iter

FIFO iterator.

### handlers::fifo::RecvFut

FIFO receive future.

### handlers::fifo::RecvStream

FIFO receive stream.

### handlers::fifo::TryIter

FIFO try iterator.

## ZBytes Types

### OptionZBytes

Optional ZBytes.

### ZBytesReader

Reader for ZBytes.

### ZBytesSliceIterator

Iterator over ZBytes slices.

### ZBytesWriter

Writer for ZBytes.

## Session Types (additional)

### SessionClosedError

Error when session is closed.

### LinkEvent

Link event (opened, closed, etc.).

### LinkEventsListener

Listener for link events.

### LinkEventsListenerBuilder

Builder for link events listener.

### LinkEventsListenerUndeclaration

Undeclaration for link events listener.

### LinksBuilder

Builder to get all links.

### PeersZenohIdBuilder

Builder for peers ZenohId.

### RoutersZenohIdBuilder

Builder for routers ZenohId.

### SessionGetBuilder

Session get builder.

### TransportEvent

Transport event.

### TransportEventsListener

Listener for transport events.

### TransportEventsListenerBuilder

Builder for transport events listener.

### TransportEventsListenerUndeclaration

Undeclaration for transport events listener.

### TransportsBuilder

Builder to get all transports.

### ZenohIdBuilder

Builder for ZenohId.

## Pub/Sub Undeclarations

### PublisherUndeclaration

Undeclare a publisher.

### SubscriberUndeclaration

Undeclare a subscriber.

## Query Undeclarations

### QuerierUndeclaration

Undeclare a querier.

### QueryableUndeclaration

Undeclare a queryable.

## Liveliness

### LivelinessTokenUndeclaration

Undeclare a liveliness token.

### MatchingListenerUndeclaration

Undeclare a matching listener.

## Time Types

### TimestampId

Type alias for timestamp ID.
