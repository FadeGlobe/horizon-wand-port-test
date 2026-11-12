// Not ported yet: the Java @Inject(TAIL) on HumanoidMobRenderer.extractHumanoidRenderState (AtomicPoseCaptureMixin.godwand$extractAtomicPose).
// Why it's off: No Horizon function mapped for HumanoidMobRenderer.extractHumanoidRenderState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for HumanoidMobRenderer.extractHumanoidRenderState: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   HumanoidMobRenderer.extractHumanoidRenderState -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/AtomicPoseCaptureMixin.java:15:
// | @Inject(method = "extractHumanoidRenderState", at = @At("TAIL"))
// | private static void godwand$extractAtomicPose(LivingEntity entity, HumanoidRenderState state, float delta, ItemModelResolver resolver, CallbackInfo ci) {
// |    AtomicPoseFrames.capture(entity, state, delta);
// | }
