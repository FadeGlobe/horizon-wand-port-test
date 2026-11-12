// Not ported yet: the Java @ModifyArg(INVOKE) on GameRenderer.renderLevel (GameRendererMixin.godwand$captureProjection).
// Why it's off: No Horizon function mapped for GameRenderer.renderLevel (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for GameRenderer.renderLevel: run `match`.
// Note: The call it hooks (ProjectionMatrixBuffer.getBuffer) isn't mapped to a Rust function yet.
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
// Decompiled Java, net/godwand/mixin/GameRendererMixin.java:30:
// | @ModifyArg(
// |    method = "renderLevel",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/ProjectionMatrixBuffer;getBuffer(Lorg/joml/Matrix4f;)Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;"
// |    ),
// |    index = 0
// | )
// | private Matrix4f godwand$captureProjection(Matrix4f matrix) {
// |    this.godwand$worldProjection = new Matrix4f(matrix);
// |    return matrix;
// | }
