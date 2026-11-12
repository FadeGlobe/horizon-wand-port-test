// Not ported yet: the Java @Inject(HEAD) on Entity.move (AtomicLocalMovementMixin.godwand$atomicStill).
// Why it's off: No Horizon function mapped for Entity.move (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Entity.move: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LocalPlayer.fallDistance -> (not mapped yet)
//   LocalPlayer.setDeltaMovement -> (not mapped yet)
//   Entity.move -> (not mapped yet)
//   Entity.turn -> horizon_client_input::mouse::Rotation::turn (suggested, high)
//   Vec3.ZERO -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/AtomicLocalMovementMixin.java:16:
// | @Inject(method = "move", at = @At("HEAD"), cancellable = true)
// | private void godwand$atomicStill(MoverType type, Vec3 movement, CallbackInfo ci) {
// |    if (this instanceof LocalPlayer player && (AtomicCinematic.movementLocked() || FugaCinematic.movementLocked())) {
// |       player.setDeltaMovement(Vec3.ZERO);
// |       player.fallDistance = 0.0;
// |       ci.cancel();
// |    }
// | }
