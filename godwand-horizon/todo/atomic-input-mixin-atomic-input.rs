// Not ported yet: the Java @Inject(TAIL) on KeyboardInput.tick (AtomicInputMixin.godwand$atomicInput).
// Why it's off: No Horizon function mapped for KeyboardInput.tick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for KeyboardInput.tick: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientInput.<init> -> (not mapped yet)
//   KeyboardInput.tick -> (not mapped yet)
//   Input.EMPTY -> (not mapped yet)
//   Vec2.ZERO -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/AtomicInputMixin.java:16:
// | @Inject(method = "tick", at = @At("TAIL"))
// | private void godwand$atomicInput(CallbackInfo ci) {
// |    if (AtomicCinematic.movementLocked() || FugaCinematic.movementLocked()) {
// |       this.keyPresses = Input.EMPTY;
// |       this.moveVector = Vec2.ZERO;
// |    }
// | }
