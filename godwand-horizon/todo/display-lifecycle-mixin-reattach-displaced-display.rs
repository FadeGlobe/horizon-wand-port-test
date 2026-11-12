// Not ported yet: the Java @Inject(HEAD) on Display.tick (DisplayLifecycleMixin.godwand$reattachDisplacedDisplay).
// Why it's off: No Horizon function mapped for Display.tick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Display.tick: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Display.tick -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/DisplayLifecycleMixin.java:13:
// | @Inject(method = "tick", at = @At("HEAD"))
// | private void godwand$reattachDisplacedDisplay(CallbackInfo ci) {
// |    if (this instanceof BlockDisplay display) {
// |       SpatialDisplacementManager.tickManagedDisplay(display);
// |    }
// | }
