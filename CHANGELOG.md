# CHANGELOG

<!-- generated from cargo-changelog -->

## v0.1.0

### "Feature"

#### Make all types serde::Serialize

All types derive `serde::Serialize` now because a user might want to serialize
again them at some point.
#### Initial implementation

The first implementation of the crate as added.
### "Misc"

#### All values are deserialized as String

For now, all values are deserialized as String, because that is what
systemd-journal-gatewayd returns (unfortunately).

`Priority` and `Transport` are an exception to this.

This will change in later versions of this library, but for now this is a good
first step before we implement more sophisticated deserializing.
