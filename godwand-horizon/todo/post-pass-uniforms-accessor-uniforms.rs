// Not ported yet: the Java @Accessor on PostPass.customUniforms (PostPassUniformsAccessor.godwand$uniforms).
// Why it's off: No Horizon function mapped for PostPass.customUniforms (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for PostPass.customUniforms: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PostPass.customUniforms -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/PostPassUniformsAccessor.java:11:
// | @Accessor("customUniforms")
// | Map<String, GpuBuffer> godwand$uniforms();
