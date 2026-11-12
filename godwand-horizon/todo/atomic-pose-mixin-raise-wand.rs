// Not ported yet: the Java @Inject(TAIL) on HumanoidModel.setupAnim (AtomicPoseMixin.godwand$raiseWand).
// Why it's off: No Horizon function mapped for HumanoidModel.setupAnim (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for HumanoidModel.setupAnim: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   HumanoidModel.setupAnim -> (not mapped yet)
//   ModelPart.loadPose -> (not mapped yet)
//   ModelPart.storePose -> (not mapped yet)
//   ModelPart.xRot -> (not mapped yet)
//   ModelPart.yRot -> (not mapped yet)
//   ModelPart.zRot -> (not mapped yet)
//   Mth.lerp -> horizon_world::stepped_interpolation::lerp (suggested, high)
//   Vec3.x -> (not mapped yet)
//   Vec3.y -> (not mapped yet)
//   Vec3.z -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/AtomicPoseMixin.java:43:
// | @Inject(method = "setupAnim(Lnet/minecraft/client/renderer/entity/state/HumanoidRenderState;)V", at = @At("TAIL"))
// | private void godwand$raiseWand(HumanoidRenderState state, CallbackInfo ci) {
// |    if (FugaPose.apply(AtomicPoseFrames.entityId(state), AtomicPoseFrames.partial(state), this.rightArm, this.leftArm, this.body, this.head)) {
// |       FugaPose.stance(AtomicPoseFrames.entityId(state), AtomicPoseFrames.partial(state), this.rightLeg, this.leftLeg);
// |       this.hat.loadPose(this.head.storePose());
// |    } else {
// |       AtomicPoseFrames.Pose pose = AtomicPoseFrames.get(state);
// |       if (pose != null) {
// |          AtomicRenderer.Frame frame = pose.frame();
// |          double age = pose.age();
// |          float raise = AtomicSequence.armRaise(age);
// |          float spread = AtomicSequence.armSpread(age);
// |          float start = AtomicSequence.activation(age);
// |          float windup = AtomicSequence.armWindup(age);
// |          float facing = AtomicSequence.facing(age);
// |          boolean right = pose.rightHand();
// |          ModelPart grip = right ? this.rightArm : this.leftArm;
// |          ModelPart support = right ? this.leftArm : this.rightArm;
// |          float weight = 1.0F - AtomicSequence.phase(age, 774.0, 802.0);
// |          float style = AtomicSequence.phase(age, 320.0, 360.0) * weight;
// |          float oldGrip = grip.xRot;
// |          float oldSupport = support.xRot;
// |          grip.xRot = AtomicSequence.armPitch(age);
// |          grip.yRot = (right ? 1 : -1) * (-0.64F * raise * (1.0F - windup) * (1.0F - spread) + 0.55F * windup) * weight;
// |          grip.zRot = (right ? 1 : -1) * (0.02F * raise * (1.0F - spread) * (1.0F - windup) - 0.72F * windup + 1.15F * spread);
// |          float supportUp = Mth.lerp(raise, -0.12F - 1.3F * start, -2.16F);
// |          supportUp += (-1.05F - supportUp) * windup;
// |          support.xRot = supportUp * (1.0F - spread) - 0.12F * spread;
// |          support.zRot = (right ? -1 : 1) * (0.02F * raise * (1.0F - spread) * (1.0F - windup) - 0.88F * windup + 1.32F * spread) * weight;
// |          support.yRot = (right ? 1 : -1) * (0.64F * raise * (1.0F - spread) * (1.0F - windup) - 0.525F * windup) * weight;
// |          grip.xRot = Mth.lerp(weight, oldGrip, grip.xRot);
// |          support.xRot = Mth.lerp(weight, oldSupport, support.xRot);
// |          grip.zRot *= weight;
// |          this.body.yRot = this.body.yRot * (1.0F - facing) - 0.12F * style * (1.0F - facing) + 0.2F * windup;
// |          this.body.zRot = -0.035F * style * (1.0F - facing);
// |          float idle = frame == null ? 0.0F : AtomicSequence.holdIdle(frame.waitTicks()) * (1.0F - AtomicSequence.phase(age, 406.0, 410.0));
// |          grip.xRot += idle;
// |          support.xRot -= idle * 0.8F;
// |          this.body.zRot += idle * 0.2F;
// |          Vec3 look = AtomicSequence.headLook(age, pose.entityId() + (frame == null ? 0.0 : frame.startedAt() * 0.01));
// |          float glance = AtomicSequence.phase(age, 0.0, 14.0) * weight;
// |          this.head.xRot = Mth.lerp(glance, this.head.xRot, (float)look.x) * (1.0F - facing);
// |          this.head.yRot = AtomicSequence.blendHeadYaw(this.head.yRot, (float)look.y, glance) * (1.0F - facing);
// |          this.head.zRot = Mth.lerp(glance, this.head.zRot, (float)look.z) * (1.0F - facing);
// |          this.hat.loadPose(this.head.storePose());
// |       }
// |    }
// | }
