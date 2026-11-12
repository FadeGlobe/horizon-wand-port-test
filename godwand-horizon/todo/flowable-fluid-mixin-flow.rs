// Not ported yet: the Java @Inject(HEAD) on FlowingFluid.canPassThrough (FlowableFluidMixin.godwand$flow).
// Why it's off: No Horizon function mapped for FlowingFluid.canPassThrough (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for FlowingFluid.canPassThrough: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ServerLevel.dimension -> (not mapped yet)
//   FlowingFluid.canMaybePassThrough -> (not mapped yet)
//   FlowingFluid.canPassThrough -> (not mapped yet)
//   FlowingFluid.spreadTo -> horizon_world::level::fluids::Sim<'_, L>::spread_to (suggested, high)
//
// Decompiled Java, net/godwand/mixin/FlowableFluidMixin.java:25:
// | @Inject(method = "canPassThrough", at = @At("HEAD"), cancellable = true)
// | private void godwand$flow(
// |    BlockGetter world,
// |    Fluid fluid,
// |    BlockPos from,
// |    BlockState old,
// |    Direction dir,
// |    BlockPos to,
// |    BlockState state,
// |    FluidState liquid,
// |    CallbackInfoReturnable<Boolean> ci
// | ) {
// |    if (protectedPosition(world, to)) {
// |       ci.setReturnValue(false);
// |    }
// | }
