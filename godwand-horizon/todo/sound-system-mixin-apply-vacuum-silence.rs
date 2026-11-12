// Not ported yet: the Java @Inject(RETURN) on SoundEngine.calculateVolume (SoundSystemMixin.godwand$applyVacuumSilence).
// Why it's off: No Horizon function mapped for SoundEngine.calculateVolume (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for SoundEngine.calculateVolume: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SoundInstance.getSource -> (not mapped yet)
//   SoundEngine.calculateVolume -> (not mapped yet)
//   SoundSource.MASTER -> (not mapped yet)
//   SoundSource.PLAYERS -> (not mapped yet)
//
// Decompiled Java, net/godwand/mixin/SoundSystemMixin.java:14:
// | @Inject(method = "calculateVolume(Lnet/minecraft/client/resources/sounds/SoundInstance;)F", at = @At("RETURN"), cancellable = true)
// | private void godwand$applyVacuumSilence(SoundInstance sound, CallbackInfoReturnable<Float> cir) {
// |    float gain = GodWandCinematicEngine.getEnvironmentGain();
// |    if (gain < 0.999F) {
// |       SoundSource category = sound.getSource();
// |       if (category != SoundSource.PLAYERS && category != SoundSource.MASTER) {
// |          cir.setReturnValue((Float)cir.getReturnValue() * gain);
// |       }
// |    }
// | }
