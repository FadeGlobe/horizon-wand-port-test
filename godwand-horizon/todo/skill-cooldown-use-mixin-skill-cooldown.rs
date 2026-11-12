// Not ported yet: the Java @Inject(HEAD) on MultiPlayerGameMode.useItem (SkillCooldownUseMixin.godwand$skillCooldown).
// Why it's off: No Horizon function mapped for MultiPlayerGameMode.useItem (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for MultiPlayerGameMode.useItem: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MultiPlayerGameMode.useItem -> (not mapped yet)
//   InteractionResult.FAIL -> (not mapped yet)
//   Player.getItemInHand -> (not mapped yet)
//   ItemStack.getItem -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/SkillCooldownUseMixin.java:17:
// | @Inject(method = "useItem", at = @At("HEAD"), cancellable = true)
// | private void godwand$skillCooldown(Player player, InteractionHand hand, CallbackInfoReturnable<InteractionResult> ci) {
// |    ItemStack stack = player.getItemInHand(hand);
// |    if (stack.getItem() instanceof GodWandItem && !SkillCooldownClient.canUse(GodWandItem.getSkillMode(stack))) {
// |       ci.setReturnValue(InteractionResult.FAIL);
// |    }
// | }
