// Not ported yet: the Java @Accessor on ExperienceOrb.count (ExperienceOrbAccessor.godwand$getCount).
// Why it's off: No Horizon function mapped for ExperienceOrb.count (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for ExperienceOrb.count: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ExperienceOrb.count -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/ExperienceOrbAccessor.java:9:
// | @Accessor("count")
// | int godwand$getCount();
