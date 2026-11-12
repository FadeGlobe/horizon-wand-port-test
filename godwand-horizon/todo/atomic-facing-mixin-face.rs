// Not ported yet: the Java @Inject(TAIL) on LivingEntityRenderer.setupRotations (AtomicFacingMixin.godwand$face).
// Why it's off: No Horizon function mapped for LivingEntityRenderer.setupRotations (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for LivingEntityRenderer.setupRotations: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PoseStack.rotate -> (not mapped yet)
//   LivingEntityRenderer.setupRotations -> (not mapped yet)
//   Mth.wrapDegrees -> horizon_world::stepped_interpolation::wrap_degrees (suggested, high)
//
// Decompiled Java, net/godwand/mixin/AtomicFacingMixin.java:19:
// | @Inject(method = "setupRotations", at = @At("TAIL"))
// | private void godwand$face(LivingEntityRenderState state, PoseStack pose, float yaw, float scale, CallbackInfo ci) {
// |    if (state instanceof HumanoidRenderState humanoid) {
// |       AtomicPoseFrames.Pose extracted = AtomicPoseFrames.get(humanoid);
// |       if (extracted != null && extracted.frame() != null) {
// |          AtomicRenderer.Frame frame = extracted.frame();
// |          pose.rotate(Axis.YP.rotationDegrees(Mth.wrapDegrees(yaw - frame.yaw()) * AtomicSequence.facing(frame.age())));
// |       }
// |    }
// | }
