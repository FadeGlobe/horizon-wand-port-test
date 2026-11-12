// Not ported yet: the Java @Inject(HEAD) on Entity.isInvulnerableToBase (EntityMixin.godwand$atomicProtection).
// Why it's off: No Horizon function mapped for Entity.isInvulnerableToBase (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Entity.isInvulnerableToBase: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Entity.entityTags -> (not mapped yet)
//   Entity.hasGlowingTag -> (not mapped yet)
//   Entity.isCurrentlyGlowing -> (not mapped yet)
//   Entity.isInvulnerableToBase -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/EntityMixin.java:15:
// | @Inject(method = "isInvulnerableToBase", at = @At("HEAD"), cancellable = true)
// | private void godwand$atomicProtection(DamageSource source, CallbackInfoReturnable<Boolean> cir) {
// |    if (this instanceof ServerPlayer player && (AtomicManager.invulnerable(player) || FugaManager.invulnerable(player))) {
// |       cir.setReturnValue(true);
// |    }
// | }
