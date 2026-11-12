// Not ported yet: the Java @Accessor on PostChain.passes (PostEffectProcessorAccessor.getPasses).
// Why it's off: No Horizon function mapped for PostChain.passes (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for PostChain.passes: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PostChain.passes -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/PostEffectProcessorAccessor.java:11:
// | @Accessor("passes")
// | List<PostPass> getPasses();
