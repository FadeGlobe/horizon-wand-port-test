// Not ported yet: the Java @Inject(HEAD) on Minecraft.shouldEntityAppearGlowing (MinecraftClientMixin.godwand$suppressBlockDisplayOutline).
// Why it's off: No Horizon function mapped for Minecraft.shouldEntityAppearGlowing (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Minecraft.shouldEntityAppearGlowing: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.shouldEntityAppearGlowing -> (not mapped yet)
//   Entity.entityTags -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/MinecraftClientMixin.java:12:
// | @Inject(method = "shouldEntityAppearGlowing", at = @At("HEAD"), cancellable = true)
// | private void godwand$suppressBlockDisplayOutline(Entity entity, CallbackInfoReturnable<Boolean> cir) {
// |    if (entity != null && entity.entityTags().contains("godwand_displaced")) {
// |       cir.setReturnValue(false);
// |    }
// | }
