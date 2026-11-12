// Not ported yet: the Java @Inject(HEAD) on Entity.hasGlowingTag (EntityMixin.godwand$preventDisplacedHasGlowingTag).
// Why it's off: No Horizon function mapped for Entity.hasGlowingTag (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Entity.hasGlowingTag: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Entity.entityTags -> (not mapped yet)
//   Entity.hasGlowingTag -> (not mapped yet)
//   Entity.isCurrentlyGlowing -> (not mapped yet)
//   Entity.isInvulnerableToBase -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/EntityMixin.java:30:
// | @Inject(method = "hasGlowingTag", at = @At("HEAD"), cancellable = true)
// | private void godwand$preventDisplacedHasGlowingTag(CallbackInfoReturnable<Boolean> cir) {
// |    Entity self = (Entity)this;
// |    if (self.entityTags().contains("godwand_displaced")) {
// |       cir.setReturnValue(false);
// |    }
// | }
