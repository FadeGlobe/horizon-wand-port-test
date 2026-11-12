// Not ported yet: the Java @Inject(HEAD) on Hud.extractTabList (AtomicGuiMixin.godwand$atomicHideCrosshair).
// Why it's off: No Horizon function mapped for Hud.extractTabList (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Hud.extractTabList: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Hud.extractChat -> (not mapped yet)
//   Hud.extractCrosshair -> (not mapped yet)
//   Hud.extractEffects -> (not mapped yet)
//   Hud.extractHotbarAndDecorations -> (not mapped yet)
//   Hud.extractOverlayMessage -> (not mapped yet)
//   Hud.extractScoreboardSidebar -> (not mapped yet)
//   Hud.extractTabList -> (not mapped yet)
//   Hud.extractTitle -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/AtomicGuiMixin.java:15:
// |          "extractCrosshair",
// |          "extractHotbarAndDecorations",
// |          "extractChat",
// |          "extractScoreboardSidebar",
// |          "extractTabList",
// |          "extractEffects",
// |          "extractOverlayMessage",
// |          "extractTitle"
// |    },
// |    at = @At("HEAD"),
// |    cancellable = true
// | )
// | private void godwand$atomicHideCrosshair(CallbackInfo ci) {
// |    if (AtomicCinematic.active() || FugaCinematic.active()) {
// |       ci.cancel();
// |    }
// | }
