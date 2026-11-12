// Not ported yet: the Java @Inject(INVOKE) on ServerGamePacketListenerImpl.handleMovePlayer (AtomicMovementMixin.godwand$atomicAnchor).
// Why it's off: No Horizon function mapped for ServerGamePacketListenerImpl.handleMovePlayer (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for ServerGamePacketListenerImpl.handleMovePlayer: run `match`.
// Note: The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PacketUtils.ensureRunningOnSameThread -> (not mapped yet)
//   ServerboundMovePlayerPacket.getXRot -> (not mapped yet)
//   ServerboundMovePlayerPacket.getYRot -> (not mapped yet)
//   ServerboundMovePlayerPacket.hasRotation -> (not mapped yet)
//   ServerPlayer.getXRot -> (not mapped yet)
//   ServerPlayer.getYRot -> (not mapped yet)
//   ServerPlayer.setXRot -> (not mapped yet)
//   ServerPlayer.setYRot -> (not mapped yet)
//   ServerGamePacketListenerImpl.handleMovePlayer -> (not mapped yet)
//   Mth.clamp -> horizon_client_render::chat::clamp (suggested, high)
//   Mth.wrapDegrees -> horizon_world::stepped_interpolation::wrap_degrees (suggested, high)
//
// Decompiled Java, net/godwand/mixin/AtomicMovementMixin.java:21:
// | @Inject(
// |    method = "handleMovePlayer",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/server/level/ServerLevel;)V",
// |       shift = Shift.AFTER
// |    ),
// |    cancellable = true
// | )
// | private void godwand$atomicAnchor(ServerboundMovePlayerPacket packet, CallbackInfo ci) {
// |    if (FugaManager.movementLocked(this.player)) {
// |       if (packet.hasRotation()) {
// |          float yaw = packet.getYRot(this.player.getYRot());
// |          float pitch = packet.getXRot(this.player.getXRot());
// |          if (Float.isFinite(yaw) && Float.isFinite(pitch)) {
// |             this.player.setYRot(Mth.wrapDegrees(yaw));
// |             this.player.setXRot(Mth.clamp(pitch, -90.0F, 90.0F));
// |          }
// |       }
// |
// |       ci.cancel();
// |    } else {
// |       if (AtomicManager.movementLocked(this.player)) {
// |          ci.cancel();
// |       }
// |    }
// | }
