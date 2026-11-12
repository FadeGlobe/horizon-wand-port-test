// Not ported yet: the Java @Inject(HEAD) on GameRenderer.renderItemInHand (GameRendererMixin.godwand$atomicHideHand).
// Why it's off: No Horizon function mapped for GameRenderer.renderItemInHand (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for GameRenderer.renderItemInHand: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PoseStack.rotate -> (not mapped yet)
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   Minecraft.level -> (not mapped yet)
//   ClientLevel.getGameTime -> (not mapped yet)
//   GameRenderer.bobHurt -> horizon_client_render::camera::bob_hurt (suggested, high)
//   GameRenderer.gameRenderState -> (not mapped yet)
//   GameRenderer.render3dHud -> (not mapped yet)
//   GameRenderer.renderItemInHand -> (not mapped yet)
//   GameRenderer.renderLevel -> (not mapped yet)
//   ProjectionMatrixBuffer.getBuffer -> (not mapped yet)
//   GameRenderState.levelRenderState -> (not mapped yet)
//   CameraRenderState.cameraEntityPartialTicks -> (not mapped yet)
//   CameraRenderState.viewRotationMatrix -> (not mapped yet)
//   LevelRenderState.cameraRenderState -> (not mapped yet)
//   LevelRenderState.worldPartialTicks -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/GameRendererMixin.java:70:
// | @Inject(method = "renderItemInHand", at = @At("HEAD"), cancellable = true)
// | private void godwand$atomicHideHand(CameraRenderState camera, PlayerRenderState player, GpuTextureView texture, CallbackInfo ci) {
// |    if (AtomicCinematic.active() || FugaCinematic.active()) {
// |       ci.cancel();
// |    }
// | }
