// Not ported yet: the Java @Inject(INVOKE) on GameRenderer.renderLevel (GameRendererMixin.godwand$afterWorldComposite).
// Why it's off: No Horizon function mapped for GameRenderer.renderLevel (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for GameRenderer.renderLevel: run `match`.
// Note: The call it hooks (GameRenderer.render3dHud) isn't mapped to a Rust function yet.
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
// Decompiled Java, net/godwand/mixin/GameRendererMixin.java:43:
// | @Inject(
// |    method = "renderLevel",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/GameRenderer;render3dHud(Lnet/minecraft/client/renderer/state/level/CameraRenderState;Lnet/minecraft/client/renderer/state/level/PlayerRenderState;Lnet/minecraft/client/renderer/state/OptionsRenderState;Z)V"
// |    )
// | )
// | private void godwand$afterWorldComposite(CallbackInfo ci) {
// |    LevelRenderState state = ((GameRenderer)this).gameRenderState().levelRenderState;
// |    CameraRenderState camera = state.cameraRenderState;
// |    GodWandPostFxManager.onWorldRender(camera.viewRotationMatrix, this.godwand$worldProjection, state.worldPartialTicks);
// |    AtomicPostFx.capture(camera.viewRotationMatrix, this.godwand$worldProjection, state.worldPartialTicks);
// |    GodWandGeometry.flushDeferred();
// |    GodWandPostFxManager.renderAfterWorld();
// |    AtomicPostFx.renderAfterWorld();
// | }
