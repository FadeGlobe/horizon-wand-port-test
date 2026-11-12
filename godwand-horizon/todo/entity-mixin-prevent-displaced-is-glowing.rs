// Not ported yet: the Java @Inject(HEAD) on Entity.isCurrentlyGlowing (EntityMixin.godwand$preventDisplacedIsGlowing).
// Why it's off: No Horizon function mapped for Entity.isCurrentlyGlowing (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Entity.isCurrentlyGlowing: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Entity.entityTags -> (not mapped yet)
//   Entity.hasGlowingTag -> (not mapped yet)
//   Entity.isCurrentlyGlowing -> (not mapped yet)
//   Entity.isInvulnerableToBase -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/EntityMixin.java:22:
// | @Inject(method = "isCurrentlyGlowing", at = @At("HEAD"), cancellable = true)
// | private void godwand$preventDisplacedIsGlowing(CallbackInfoReturnable<Boolean> cir) {
// |    Entity self = (Entity)this;
// |    if (self.entityTags().contains("godwand_displaced")) {
// |       cir.setReturnValue(false);
// |    }
// | }
