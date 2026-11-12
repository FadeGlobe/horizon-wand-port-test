// Not ported yet: the Java @Inject(HEAD) on ToastManager.extractRenderState (AtomicToastMixin.godwand$atomicDeferToasts).
// Why it's off: No Horizon function mapped for ToastManager.extractRenderState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ToastManager.extractRenderState: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ToastManager.extractRenderState -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/AtomicToastMixin.java:13:
// | @Inject(method = "extractRenderState", at = @At("HEAD"), cancellable = true)
// | private void godwand$atomicDeferToasts(CallbackInfo ci) {
// |    if (AtomicCinematic.active() || FugaCinematic.active()) {
// |       ci.cancel();
// |    }
// | }
