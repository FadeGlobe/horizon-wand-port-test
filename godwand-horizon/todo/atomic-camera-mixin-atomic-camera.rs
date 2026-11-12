// Not ported yet: the Java @Inject(INVOKE) on Camera.update (AtomicCameraMixin.godwand$atomicCamera).
// Why it's off: No Horizon function mapped for Camera.update (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for Camera.update: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Camera.alignWithEntity -> horizon_client_render::camera::Camera::align_with_entity (suggested, high)
//   Camera.calculateFov -> horizon_client_render::camera::Camera::calculate_fov (suggested, high)
//   Camera.update -> (not mapped yet)
//   CameraType.isFirstPerson -> (not mapped yet)
//   DeltaTracker.getGameTimeDeltaPartialTick -> (not mapped yet)
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   Minecraft.options -> (not mapped yet)
//   Options.getCameraType -> (not mapped yet)
//   Entity.getEyePosition -> (not mapped yet)
//   Vec3.distanceToSqr -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/AtomicCameraMixin.java:42:
// | @Inject(method = "update", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/Camera;alignWithEntity(F)V", shift = Shift.AFTER))
// | private void godwand$atomicCamera(DeltaTracker timer, CallbackInfo ci) {
// |    float delta = timer.getGameTimeDeltaPartialTick(false);
// |    AtomicCinematic.Pose pose = AtomicCinematic.camera(delta, this.position(), this.yRot(), this.xRot());
// |    if (pose == null) {
// |       pose = FugaCinematic.camera(delta, this.position(), this.yRot(), this.xRot());
// |    }
// |
// |    if (pose != null && this.entity() != null) {
// |       this.detached = FugaCinematic.showCaster(delta)
// |          || !Minecraft.getInstance().options.getCameraType().isFirstPerson()
// |          || pose.position().distanceToSqr(this.entity().getEyePosition(delta)) > 1.0;
// |       this.setPosition(pose.position());
// |       this.setRotation(pose.yaw(), pose.pitch());
// |    }
// | }
