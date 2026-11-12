# Java translated to Rust

Translated 239 of 1056 methods (2 more are done another way in Horizon) and 221 of 341 static fields to Rust (4 of those call the game, through Horizon's helpers); 1 of 1 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 434 | `ArcaneOrbEntityRenderer.ArcaneOrbEntityRenderer(1)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) |
| Uses Minecraft classes (needs Horizon's game API) | 254 | `ArcaneOrbChargeRenderer.render(3)`: it uses Minecraft's `Camera` |
| Other Java this translator doesn't handle yet | 56 | `AtomicCinematic.lettering(1)`: Style.withFont |
| Uses null | 17 | `AtomicPostFx.capture(3)`: a nullable object kept in a changing static |
| Calls the game where Horizon has no helper yet | 15 | `AtomicCinematic.loadedNeighborhood(2)`: it calls Minecraft's ClientLevel.getChunkSource (no Horizon helper for it yet) |
| Uses other libraries | 14 | `AtomicRenderer.receive(1)`: it uses `AtomicFxPayload` (net.godwand.network) |
| Uses try/catch | 11 | `ArcaneOrbModel.renderCharge(4)`: a try/finally |
| Uses Java collections other than List (Map, Set...) | 9 | `SpatialBridge.disable(1)`: it uses `UUID` (java.util) |
| Uses inheritance (extends, implements, instanceof) | 5 | `StoneCannonWindSound.update(1)`: `instanceof GodWandItem` on an item (no single item id for it) |
| Done another way in Horizon | 2 | `GodWandFabricClientMod.onInitializeClient(0)`: done another way in Horizon: it runs as `on_init`, from Horizon's init hook |

## Close, not the same

Where the Rust does a little less than the Java, because Horizon doesn't have something:

- GodWandItem.getSkillMode: the item's custom data is treated as absent (Horizon doesn't keep it)
- WandSkillLock.isLocked: the item's custom data is treated as absent (Horizon doesn't keep it)

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| net/godwand/fabric/GodWandFabricMod.java:74 | `init` | `on_init`, called from the hook, without: the call to `GodWandConfig.loadServer(..)`: it calls GodWandConfig.loadServer, which isn't translated |

## Every class


### `net/godwand/network/AtomicFxPayload.java`: couldn't be read: expected `;` but found `>` (line 16)


### `net/godwand/network/AtomicSkipPayload.java`: couldn't be read: expected `;` but found `>` (line 11)


### `net/godwand/network/CannonPressurePayload.java`: couldn't be read: expected `;` but found `>` (line 12)


### `net/godwand/network/DrillWindPayload.java`: couldn't be read: expected `;` but found `>` (line 11)


### `net/godwand/network/FugaFxPayload.java`: couldn't be read: expected `;` but found `>` (line 17)


### `net/godwand/network/SelectSkillPayload.java`: couldn't be read: expected `;` but found `>` (line 11)


### `net/godwand/network/SingularityThreadsPayload.java`: couldn't be read: expected `;` but found `>` (line 13)


### `net/godwand/network/SkillCooldownPayload.java`: couldn't be read: expected `;` but found `>` (line 11)


### `net/godwand/network/SpatialAuraPayload.java`: couldn't be read: expected `;` but found `>` (line 12)


### `net/godwand/spatial/DimensionPos.java`: couldn't be read: expected a name but found `{` (line 8)


### `ArcaneOrbChargeRenderer`

- [ ] `ArcaneOrbChargeRenderer.render(3)`: it uses Minecraft's `Camera` (net/godwand/client/ArcaneOrbChargeRenderer.java:12)

### `ArcaneOrbEntityRenderer`: it extends Minecraft's `EntityRenderer`

- [ ] `ArcaneOrbEntityRenderer.ArcaneOrbEntityRenderer(1)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) (net/godwand/client/ArcaneOrbEntityRenderer.java:15)
- [ ] `ArcaneOrbEntityRenderer.createRenderState(0)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) (net/godwand/client/ArcaneOrbEntityRenderer.java:19)
- [ ] `ArcaneOrbEntityRenderer.extractRenderState(3)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) (net/godwand/client/ArcaneOrbEntityRenderer.java:23)
- [ ] `ArcaneOrbEntityRenderer.submit(4)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) (net/godwand/client/ArcaneOrbEntityRenderer.java:29)

### `ArcaneOrbEntityRenderer.State`: it extends Minecraft's `EntityRenderState`


### `ArcaneOrbModel`

- [x] `ArcaneOrbModel.chargeGlowAlpha(1)` (net/godwand/client/ArcaneOrbModel.java:10)
- [ ] `ArcaneOrbModel.renderCharge(4)`: a try/finally (net/godwand/client/ArcaneOrbModel.java:15)
- [ ] `ArcaneOrbModel.render(4)`: a try/finally (net/godwand/client/ArcaneOrbModel.java:49)
- [ ] `ArcaneOrbModel.renderSphere(9)`: it uses Minecraft's `BufferBuilder` (net/godwand/client/ArcaneOrbModel.java:83)
- [ ] `ArcaneOrbModel.renderEnergyRing(10)`: it uses Minecraft's `BufferBuilder` (net/godwand/client/ArcaneOrbModel.java:112)
- [ ] `ArcaneOrbModel.renderInwardSpirals(4)`: it uses Minecraft's `BufferBuilder` (net/godwand/client/ArcaneOrbModel.java:133)

### `AtomicCinematic`

- [x] `AtomicCinematic.TITLE_FONT` (net/godwand/client/AtomicCinematic.java:23)
- [x] `AtomicCinematic.skipped` (net/godwand/client/AtomicCinematic.java:24)
- [x] `AtomicCinematic.movementReleased` (net/godwand/client/AtomicCinematic.java:25)
- [x] `AtomicCinematic.movementReleaseSent` (net/godwand/client/AtomicCinematic.java:26)
- [x] `AtomicCinematic.wideStarted` (net/godwand/client/AtomicCinematic.java:27)
- [x] `AtomicCinematic.wideAngle` (net/godwand/client/AtomicCinematic.java:28)
- [x] `AtomicCinematic.wideHeight` (net/godwand/client/AtomicCinematic.java:29)
- [x] `AtomicCinematic.environmentGain` (net/godwand/client/AtomicCinematic.java:30)
- [ ] `AtomicCinematic.movementLocked(0)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicCinematic.java:32)
- [ ] `AtomicCinematic.active(0)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicCinematic.java:38)
- [ ] `AtomicCinematic.tick(1)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicCinematic.java:50)
- [x] `AtomicCinematic.reset(0)` (net/godwand/client/AtomicCinematic.java:73)
- [x] `AtomicCinematic.onCastStart(0)` (net/godwand/client/AtomicCinematic.java:81)
- [ ] `AtomicCinematic.skip(0)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicCinematic.java:88)
- [ ] `AtomicCinematic.camera(4)`: it uses Pose, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicCinematic.java:96)
- [ ] `AtomicCinematic.wideClearance(4)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicCinematic.java:157)
- [ ] `AtomicCinematic.loadedNeighborhood(2)`: it calls Minecraft's ClientLevel.getChunkSource (no Horizon helper for it yet) (net/godwand/client/AtomicCinematic.java:185)
- [ ] `AtomicCinematic.fov(2)`: it calls AtomicCinematic.active, which isn't translated (net/godwand/client/AtomicCinematic.java:197)
- [x] `AtomicCinematic.environmentGain(0)` (net/godwand/client/AtomicCinematic.java:206)
- [ ] `AtomicCinematic.applyShake(2)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicCinematic.java:210)
- [ ] `AtomicCinematic.render(2)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicCinematic.java:232)
- [ ] `AtomicCinematic.releaseHint(2)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicCinematic.java:300)
- [ ] `AtomicCinematic.title(7)`: it calls AtomicCinematic.lettering, which isn't translated (net/godwand/client/AtomicCinematic.java:313)
- [ ] `AtomicCinematic.lettering(1)`: Style.withFont (net/godwand/client/AtomicCinematic.java:333)

### `AtomicCinematic.Pose`: it uses Minecraft's `Vec3`

- [ ] `AtomicCinematic.Pose.Pose(3)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicCinematic.java:337)
- [ ] `AtomicCinematic.Pose.position(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicCinematic.java:337)
- [ ] `AtomicCinematic.Pose.yaw(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicCinematic.java:337)
- [ ] `AtomicCinematic.Pose.pitch(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicCinematic.java:337)

### `AtomicPoseFrames`

- [ ] `AtomicPoseFrames.POSES`: it uses Minecraft's `HumanoidRenderState` (net/godwand/client/AtomicPoseFrames.java:14)
- [ ] `AtomicPoseFrames.IDS`: it uses Minecraft's `HumanoidRenderState` (net/godwand/client/AtomicPoseFrames.java:15)
- [ ] `AtomicPoseFrames.PARTIALS`: it uses Minecraft's `HumanoidRenderState` (net/godwand/client/AtomicPoseFrames.java:16)
- [ ] `AtomicPoseFrames.entityId(1)`: it uses Minecraft's `HumanoidRenderState` (net/godwand/client/AtomicPoseFrames.java:18)
- [ ] `AtomicPoseFrames.partial(1)`: it uses Minecraft's `HumanoidRenderState` (net/godwand/client/AtomicPoseFrames.java:22)
- [ ] `AtomicPoseFrames.capture(3)`: it uses Minecraft's `LivingEntity` (net/godwand/client/AtomicPoseFrames.java:26)
- [ ] `AtomicPoseFrames.get(1)`: it uses Pose, which isn't translated (it keeps an atomic value in an object's field) (net/godwand/client/AtomicPoseFrames.java:40)

### `AtomicPoseFrames.Pose`: it keeps an atomic value in an object's field

- [ ] `AtomicPoseFrames.Pose.Pose(5)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/client/AtomicPoseFrames.java:44)
- [ ] `AtomicPoseFrames.Pose.entityId(0)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/client/AtomicPoseFrames.java:44)
- [ ] `AtomicPoseFrames.Pose.charging(0)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/client/AtomicPoseFrames.java:44)
- [ ] `AtomicPoseFrames.Pose.rightHand(0)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/client/AtomicPoseFrames.java:44)
- [ ] `AtomicPoseFrames.Pose.age(0)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/client/AtomicPoseFrames.java:44)
- [ ] `AtomicPoseFrames.Pose.frame(0)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/client/AtomicPoseFrames.java:44)

### `AtomicPostFx`

- [x] `AtomicPostFx.pending` (net/godwand/client/AtomicPostFx.java:14)
- [ ] `AtomicPostFx.chain`: it uses GodWandPostChain, which isn't translated (it implements `AutoCloseable`) (net/godwand/client/AtomicPostFx.java:15)
- [x] `AtomicPostFx.width` (net/godwand/client/AtomicPostFx.java:16)
- [x] `AtomicPostFx.height` (net/godwand/client/AtomicPostFx.java:17)
- [x] `AtomicPostFx.downsample` (net/godwand/client/AtomicPostFx.java:18)
- [x] `AtomicPostFx.failed` (net/godwand/client/AtomicPostFx.java:19)
- [ ] `AtomicPostFx.irisInUse`: it uses `Method` (java.lang.reflect) (net/godwand/client/AtomicPostFx.java:20)
- [ ] `AtomicPostFx.iris`: it uses `Object` (net/godwand/client/AtomicPostFx.java:21)
- [x] `AtomicPostFx.irisChecked` (net/godwand/client/AtomicPostFx.java:22)
- [ ] `AtomicPostFx.capture(3)`: a nullable object kept in a changing static (net/godwand/client/AtomicPostFx.java:24)
- [ ] `AtomicPostFx.available(0)`: a nullable object kept in a changing static (net/godwand/client/AtomicPostFx.java:30)
- [ ] `AtomicPostFx.reset(0)`: it calls AtomicPostFx.reload, which isn't translated (net/godwand/client/AtomicPostFx.java:34)
- [ ] `AtomicPostFx.reload(0)`: it uses AtomicPostFx.chain, which isn't translated (it uses GodWandPostChain, which isn't translated (it implements `AutoCloseable`)) (net/godwand/client/AtomicPostFx.java:39)
- [ ] `AtomicPostFx.enabled(0)`: a nullable object kept in a changing static (net/godwand/client/AtomicPostFx.java:51)
- [ ] `AtomicPostFx.shaderPackActive(0)`: a try/catch (ReflectiveOperationException, LinkageError) (net/godwand/client/AtomicPostFx.java:55)
- [ ] `AtomicPostFx.create(1)`: it uses AtomicPostFx.chain, which isn't translated (it uses GodWandPostChain, which isn't translated (it implements `AutoCloseable`)) (net/godwand/client/AtomicPostFx.java:71)
- [ ] `AtomicPostFx.prewarm(0)`: it uses AtomicPostFx.chain, which isn't translated (it uses GodWandPostChain, which isn't translated (it implements `AutoCloseable`)) (net/godwand/client/AtomicPostFx.java:75)
- [x] `AtomicPostFx.warm(0)` (net/godwand/client/AtomicPostFx.java:94)
- [ ] `AtomicPostFx.renderAfterWorld(0)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicPostFx.java:97)

### `AtomicPostFx.Frame`

- [x] `AtomicPostFx.Frame.Frame(3)` (net/godwand/client/AtomicPostFx.java:211)
- [x] `AtomicPostFx.Frame.view(0)` (net/godwand/client/AtomicPostFx.java:211)
- [x] `AtomicPostFx.Frame.projection(0)` (net/godwand/client/AtomicPostFx.java:211)
- [x] `AtomicPostFx.Frame.delta(0)` (net/godwand/client/AtomicPostFx.java:211)

### `AtomicRenderer`

- [ ] `AtomicRenderer.ACTIVE`: it uses State, which isn't translated (it keeps an atomic value in an object's field) (net/godwand/client/AtomicRenderer.java:43)
- [ ] `AtomicRenderer.AUDIO_TAILS`: it uses State, which isn't translated (it keeps an atomic value in an object's field) (net/godwand/client/AtomicRenderer.java:44)
- [ ] `AtomicRenderer.SCARS`: it uses Scar, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:45)
- [x] `AtomicRenderer.ENERGY_TEXTURE` (net/godwand/client/AtomicRenderer.java:46)
- [x] `AtomicRenderer.textureReady` (net/godwand/client/AtomicRenderer.java:47)
- [ ] `AtomicRenderer.world`: it uses Minecraft's `ClientLevel` (net/godwand/client/AtomicRenderer.java:48)
- [x] `AtomicRenderer.scarHeightTick` (net/godwand/client/AtomicRenderer.java:49)
- [x] `AtomicRenderer.SCAR_HEIGHTS` (net/godwand/client/AtomicRenderer.java:50)
- [ ] `AtomicRenderer.receive(1)`: it uses `AtomicFxPayload` (net.godwand.network) (net/godwand/client/AtomicRenderer.java:52)
- [ ] `AtomicRenderer.sound(3)`: it uses Minecraft's `SimpleSoundInstance` (net/godwand/client/AtomicRenderer.java:135)
- [x] `AtomicRenderer.distanceGain(1)` (net/godwand/client/AtomicRenderer.java:156)
- [ ] `AtomicRenderer.stop(1)`: it uses State, which isn't translated (it keeps an atomic value in an object's field) (net/godwand/client/AtomicRenderer.java:160)
- [ ] `AtomicRenderer.tick(1)`: it uses AtomicRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`) (net/godwand/client/AtomicRenderer.java:175)
- [ ] `AtomicRenderer.frame(2)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:234)
- [ ] `AtomicRenderer.localFrame(1)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:251)
- [ ] `AtomicRenderer.frameFor(2)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:256)
- [ ] `AtomicRenderer.heldFrame(2)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:265)
- [ ] `AtomicRenderer.captureSwordTip(3)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:280)
- [ ] `AtomicRenderer.sampleSwordTip(3)`: it uses TipSample, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:289)
- [ ] `AtomicRenderer.swordTrail(5)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:311)
- [ ] `AtomicRenderer.nearestFrame(1)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:328)
- [ ] `AtomicRenderer.reset(0)`: it uses AtomicRenderer.ACTIVE, which isn't translated (it uses State, which isn't translated (it keeps an atomic value in an object's field)) (net/godwand/client/AtomicRenderer.java:354)
- [ ] `AtomicRenderer.render(3)`: it uses Minecraft's `Camera` (net/godwand/client/AtomicRenderer.java:374)
- [ ] `AtomicRenderer.emit(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:463)
- [ ] `AtomicRenderer.charge(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:506)
- [ ] `AtomicRenderer.bladeConvergence(6)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:599)
- [x] `AtomicRenderer.chargeAlpha(1)` (net/godwand/client/AtomicRenderer.java:624)
- [ ] `AtomicRenderer.motes(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:628)
- [ ] `AtomicRenderer.release(3)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:672)
- [ ] `AtomicRenderer.fortress(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:682)
- [ ] `AtomicRenderer.fortressBand(6)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:787)
- [ ] `AtomicRenderer.trace(6)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:793)
- [ ] `AtomicRenderer.blast(5)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:809)
- [ ] `AtomicRenderer.pressureAura(5)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:900)
- [ ] `AtomicRenderer.scarHeight(2)`: it uses AtomicRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`) (net/godwand/client/AtomicRenderer.java:941)
- [ ] `AtomicRenderer.scars(3)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:958)
- [ ] `AtomicRenderer.polar(3)`: it creates Minecraft's `Vec3` (net/godwand/client/AtomicRenderer.java:990)
- [x] `AtomicRenderer.fract(1)` (net/godwand/client/AtomicRenderer.java:994)
- [ ] `AtomicRenderer.ring(6)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:998)
- [ ] `AtomicRenderer.ringProgress(7)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:1002)
- [ ] `AtomicRenderer.stroke(6)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:1021)
- [ ] `AtomicRenderer.cylinder(7)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:1040)
- [ ] `AtomicRenderer.shard(5)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:1054)
- [ ] `AtomicRenderer.mote(6)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:1059)
- [x] `AtomicRenderer.energyAlpha(2)` (net/godwand/client/AtomicRenderer.java:1087)
- [x] `AtomicRenderer.circuitAlpha(2)` (net/godwand/client/AtomicRenderer.java:1092)
- [ ] `AtomicRenderer.circuitPanel(7)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:1112)
- [ ] `AtomicRenderer.ensureEnergyTexture(0)`: it uses Minecraft's `NativeImage` (net/godwand/client/AtomicRenderer.java:1120)
- [ ] `AtomicRenderer.disc(8)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:1136)
- [ ] `AtomicRenderer.quad(8)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:1144)
- [ ] `AtomicRenderer.energyVertex(7)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/AtomicRenderer.java:1152)

### `AtomicRenderer.EnergyTexture`: it extends Minecraft's `DynamicTexture`

- [ ] `AtomicRenderer.EnergyTexture.EnergyTexture(1)`: its class isn't translated (it extends Minecraft's `DynamicTexture`) (net/godwand/client/AtomicRenderer.java:1157)

### `AtomicRenderer.Frame`: it uses Minecraft's `Vec3`

- [ ] `AtomicRenderer.Frame.Frame(7)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:1163)
- [ ] `AtomicRenderer.Frame.casterId(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:1163)
- [ ] `AtomicRenderer.Frame.startedAt(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:1163)
- [ ] `AtomicRenderer.Frame.center(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:1163)
- [ ] `AtomicRenderer.Frame.age(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:1163)
- [ ] `AtomicRenderer.Frame.yaw(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:1163)
- [ ] `AtomicRenderer.Frame.waitTicks(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:1163)
- [ ] `AtomicRenderer.Frame.waiting(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:1163)

### `AtomicRenderer.Scar`: it uses Minecraft's `Vec3`

- [ ] `AtomicRenderer.Scar.Scar(2)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:1166)
- [ ] `AtomicRenderer.Scar.center(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:1166)
- [ ] `AtomicRenderer.Scar.started(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:1166)

### `AtomicRenderer.State`: it keeps an atomic value in an object's field

- [ ] `AtomicRenderer.State.State(2)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/client/AtomicRenderer.java:1184)

### `AtomicRenderer.TipSample`: it uses Minecraft's `Vec3`

- [ ] `AtomicRenderer.TipSample.TipSample(2)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:1190)
- [ ] `AtomicRenderer.TipSample.position(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:1190)
- [ ] `AtomicRenderer.TipSample.time(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/AtomicRenderer.java:1190)

### `CannonPressureRenderer`

- [ ] `CannonPressureRenderer.BURSTS`: it uses Burst, which isn't translated (it uses `CannonPressurePayload` (net.godwand.network)) (net/godwand/client/CannonPressureRenderer.java:18)
- [ ] `CannonPressureRenderer.world`: it uses Minecraft's `ClientLevel` (net/godwand/client/CannonPressureRenderer.java:19)
- [ ] `CannonPressureRenderer.reset(0)`: it uses CannonPressureRenderer.BURSTS, which isn't translated (it uses Burst, which isn't translated (it uses `CannonPressurePayload` (net.godwand.network))) (net/godwand/client/CannonPressureRenderer.java:21)
- [ ] `CannonPressureRenderer.receive(1)`: it uses `CannonPressurePayload` (net.godwand.network) (net/godwand/client/CannonPressureRenderer.java:26)
- [ ] `CannonPressureRenderer.conePoint(4)`: it creates Minecraft's `Vec3` (net/godwand/client/CannonPressureRenderer.java:42)
- [ ] `CannonPressureRenderer.render(3)`: it uses Minecraft's `Camera` (net/godwand/client/CannonPressureRenderer.java:47)

### `CannonPressureRenderer.Burst`: it uses `CannonPressurePayload` (net.godwand.network)

- [ ] `CannonPressureRenderer.Burst.Burst(2)`: its class isn't translated (it uses `CannonPressurePayload` (net.godwand.network)) (net/godwand/client/CannonPressureRenderer.java:104)
- [ ] `CannonPressureRenderer.Burst.payload(0)`: its class isn't translated (it uses `CannonPressurePayload` (net.godwand.network)) (net/godwand/client/CannonPressureRenderer.java:104)
- [ ] `CannonPressureRenderer.Burst.tick(0)`: its class isn't translated (it uses `CannonPressurePayload` (net.godwand.network)) (net/godwand/client/CannonPressureRenderer.java:104)

### `ExtractedModelVertices`: it implements Minecraft's `VertexConsumer`

- [ ] `ExtractedModelVertices.addVertex(3)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (net/godwand/client/compat/ExtractedModelVertices.java:14)
- [ ] `ExtractedModelVertices.setColor(4)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (net/godwand/client/compat/ExtractedModelVertices.java:23)
- [ ] `ExtractedModelVertices.setColor(1)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (net/godwand/client/compat/ExtractedModelVertices.java:27)
- [ ] `ExtractedModelVertices.setUv(2)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (net/godwand/client/compat/ExtractedModelVertices.java:32)
- [ ] `ExtractedModelVertices.setUv1(2)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (net/godwand/client/compat/ExtractedModelVertices.java:38)
- [ ] `ExtractedModelVertices.setUv2(2)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (net/godwand/client/compat/ExtractedModelVertices.java:43)
- [ ] `ExtractedModelVertices.setUv3(2)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (net/godwand/client/compat/ExtractedModelVertices.java:48)
- [ ] `ExtractedModelVertices.setNormal(3)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (net/godwand/client/compat/ExtractedModelVertices.java:54)
- [ ] `ExtractedModelVertices.setLineWidth(1)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (net/godwand/client/compat/ExtractedModelVertices.java:61)
- [ ] `ExtractedModelVertices.submit(2)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (net/godwand/client/compat/ExtractedModelVertices.java:66)

### `ExtractedModelVertices.Vertex`


### `GodWandGeometry`

- [ ] `GodWandGeometry.ARENA`: it uses Minecraft's `ByteBufferBuilder` (net/godwand/client/compat/GodWandGeometry.java:48)
- [ ] `GodWandGeometry.PENDING`: it uses Draw, which isn't translated (it uses Minecraft's `MeshData`) (net/godwand/client/compat/GodWandGeometry.java:49)
- [ ] `GodWandGeometry.PIPELINES`: it uses Key, which isn't translated (it uses Minecraft's `PrimitiveTopology`) (net/godwand/client/compat/GodWandGeometry.java:50)
- [x] `GodWandGeometry.blend` (net/godwand/client/compat/GodWandGeometry.java:51)
- [x] `GodWandGeometry.additive` (net/godwand/client/compat/GodWandGeometry.java:52)
- [x] `GodWandGeometry.depth` (net/godwand/client/compat/GodWandGeometry.java:53)
- [x] `GodWandGeometry.writeDepth` (net/godwand/client/compat/GodWandGeometry.java:54)
- [x] `GodWandGeometry.cull` (net/godwand/client/compat/GodWandGeometry.java:55)
- [x] `GodWandGeometry.texture` (net/godwand/client/compat/GodWandGeometry.java:56)
- [x] `GodWandGeometry.textureId` (net/godwand/client/compat/GodWandGeometry.java:57)
- [ ] `GodWandGeometry.color`: it uses `Vector4f` (org.joml) (net/godwand/client/compat/GodWandGeometry.java:58)
- [x] `GodWandGeometry.deferredView` (net/godwand/client/compat/GodWandGeometry.java:59)
- [ ] `GodWandGeometry.begin(2)`: it uses Minecraft's `BufferBuilder` (net/godwand/client/compat/GodWandGeometry.java:61)
- [x] `GodWandGeometry.enableBlend(0)` (net/godwand/client/compat/GodWandGeometry.java:65)
- [x] `GodWandGeometry.disableBlend(0)` (net/godwand/client/compat/GodWandGeometry.java:69)
- [x] `GodWandGeometry.defaultBlendFunc(0)` (net/godwand/client/compat/GodWandGeometry.java:73)
- [x] `GodWandGeometry.additiveBlend(0)` (net/godwand/client/compat/GodWandGeometry.java:77)
- [x] `GodWandGeometry.enableDepthTest(0)` (net/godwand/client/compat/GodWandGeometry.java:81)
- [x] `GodWandGeometry.disableDepthTest(0)` (net/godwand/client/compat/GodWandGeometry.java:85)
- [x] `GodWandGeometry.depthMask(1)` (net/godwand/client/compat/GodWandGeometry.java:89)
- [x] `GodWandGeometry.enableCull(0)` (net/godwand/client/compat/GodWandGeometry.java:93)
- [x] `GodWandGeometry.disableCull(0)` (net/godwand/client/compat/GodWandGeometry.java:97)
- [x] `GodWandGeometry.colored(0)` (net/godwand/client/compat/GodWandGeometry.java:101)
- [x] `GodWandGeometry.textured(0)` (net/godwand/client/compat/GodWandGeometry.java:105)
- [x] `GodWandGeometry.setShaderTexture(2)` (net/godwand/client/compat/GodWandGeometry.java:109)
- [ ] `GodWandGeometry.setShaderColor(4)`: it uses GodWandGeometry.color, which isn't translated (it uses `Vector4f` (org.joml)) (net/godwand/client/compat/GodWandGeometry.java:117)
- [ ] `GodWandGeometry.pipeline(1)`: it uses Minecraft's `RenderPipeline` (net/godwand/client/compat/GodWandGeometry.java:121)
- [ ] `GodWandGeometry.initialize(0)`: it calls GodWandGeometry.initialize, which isn't translated (net/godwand/client/compat/GodWandGeometry.java:152)
- [ ] `GodWandGeometry.initialize(1)`: it uses Minecraft's `RenderPipeline` (net/godwand/client/compat/GodWandGeometry.java:156)
- [ ] `GodWandGeometry.draw(1)`: it uses Minecraft's `MeshData` (net/godwand/client/compat/GodWandGeometry.java:176)
- [ ] `GodWandGeometry.reset(0)`: it uses GodWandGeometry.PENDING, which isn't translated (it uses Draw, which isn't translated (it uses Minecraft's `MeshData`)) (net/godwand/client/compat/GodWandGeometry.java:183)
- [ ] `GodWandGeometry.deferFlush(0)`: it calls Minecraft's `RenderSystem`.getModelViewMatrixCopy (net/godwand/client/compat/GodWandGeometry.java:193)
- [ ] `GodWandGeometry.flushDeferred(0)`: it calls GodWandGeometry.flush, which isn't translated (net/godwand/client/compat/GodWandGeometry.java:197)
- [ ] `GodWandGeometry.flush(0)`: it calls GodWandGeometry.flush, which isn't translated (net/godwand/client/compat/GodWandGeometry.java:205)
- [ ] `GodWandGeometry.flush(1)`: it uses GodWandGeometry.PENDING, which isn't translated (it uses Draw, which isn't translated (it uses Minecraft's `MeshData`)) (net/godwand/client/compat/GodWandGeometry.java:209)

### `GodWandGeometry.Draw`: it uses Minecraft's `MeshData`

- [ ] `GodWandGeometry.Draw.Draw(4)`: its class isn't translated (it uses Minecraft's `MeshData`) (net/godwand/client/compat/GodWandGeometry.java:282)
- [ ] `GodWandGeometry.Draw.mesh(0)`: its class isn't translated (it uses Minecraft's `MeshData`) (net/godwand/client/compat/GodWandGeometry.java:282)
- [ ] `GodWandGeometry.Draw.key(0)`: its class isn't translated (it uses Minecraft's `MeshData`) (net/godwand/client/compat/GodWandGeometry.java:282)
- [ ] `GodWandGeometry.Draw.texture(0)`: its class isn't translated (it uses Minecraft's `MeshData`) (net/godwand/client/compat/GodWandGeometry.java:282)
- [ ] `GodWandGeometry.Draw.color(0)`: its class isn't translated (it uses Minecraft's `MeshData`) (net/godwand/client/compat/GodWandGeometry.java:282)

### `GodWandGeometry.Key`: it uses Minecraft's `PrimitiveTopology`

- [ ] `GodWandGeometry.Key.Key(7)`: its class isn't translated (it uses Minecraft's `PrimitiveTopology`) (net/godwand/client/compat/GodWandGeometry.java:285)
- [ ] `GodWandGeometry.Key.topology(0)`: its class isn't translated (it uses Minecraft's `PrimitiveTopology`) (net/godwand/client/compat/GodWandGeometry.java:285)
- [ ] `GodWandGeometry.Key.texture(0)`: its class isn't translated (it uses Minecraft's `PrimitiveTopology`) (net/godwand/client/compat/GodWandGeometry.java:285)
- [ ] `GodWandGeometry.Key.blend(0)`: its class isn't translated (it uses Minecraft's `PrimitiveTopology`) (net/godwand/client/compat/GodWandGeometry.java:285)
- [ ] `GodWandGeometry.Key.additive(0)`: its class isn't translated (it uses Minecraft's `PrimitiveTopology`) (net/godwand/client/compat/GodWandGeometry.java:285)
- [ ] `GodWandGeometry.Key.depth(0)`: its class isn't translated (it uses Minecraft's `PrimitiveTopology`) (net/godwand/client/compat/GodWandGeometry.java:285)
- [ ] `GodWandGeometry.Key.writeDepth(0)`: its class isn't translated (it uses Minecraft's `PrimitiveTopology`) (net/godwand/client/compat/GodWandGeometry.java:285)
- [ ] `GodWandGeometry.Key.cull(0)`: its class isn't translated (it uses Minecraft's `PrimitiveTopology`) (net/godwand/client/compat/GodWandGeometry.java:285)

### `CustomImpactFrameOverlay`

- [x] `CustomImpactFrameOverlay.FRAMES` (net/godwand/client/CustomImpactFrameOverlay.java:12)
- [x] `CustomImpactFrameOverlay.IMPACT_FRAME_TICKS` (net/godwand/client/CustomImpactFrameOverlay.java:18)
- [x] `CustomImpactFrameOverlay.FADEOUT_TICKS` (net/godwand/client/CustomImpactFrameOverlay.java:19)
- [x] `CustomImpactFrameOverlay.TOTAL_DURATION` (net/godwand/client/CustomImpactFrameOverlay.java:20)
- [x] `CustomImpactFrameOverlay.currentTick` (net/godwand/client/CustomImpactFrameOverlay.java:21)
- [x] `CustomImpactFrameOverlay.active` (net/godwand/client/CustomImpactFrameOverlay.java:22)
- [x] `CustomImpactFrameOverlay.trigger(1)` (net/godwand/client/CustomImpactFrameOverlay.java:24)
- [x] `CustomImpactFrameOverlay.trigger(0)` (net/godwand/client/CustomImpactFrameOverlay.java:29)
- [x] `CustomImpactFrameOverlay.reset(0)` (net/godwand/client/CustomImpactFrameOverlay.java:33)
- [x] `CustomImpactFrameOverlay.onClientTick(0)` (net/godwand/client/CustomImpactFrameOverlay.java:38)
- [ ] `CustomImpactFrameOverlay.render(2)`: it uses GodWandConfig.CLIENT, which isn't translated (it uses Client, which isn't translated (it uses WandSkin, which isn't translated (its constants carry values))) (net/godwand/client/CustomImpactFrameOverlay.java:48)

### `DrillWindRenderer`

- [ ] `DrillWindRenderer.WINDS`: it uses Wind, which isn't translated (it uses `DrillWindPayload` (net.godwand.network)) (net/godwand/client/DrillWindRenderer.java:20)
- [ ] `DrillWindRenderer.world`: it uses Minecraft's `ClientLevel` (net/godwand/client/DrillWindRenderer.java:21)
- [ ] `DrillWindRenderer.receive(1)`: it uses `DrillWindPayload` (net.godwand.network) (net/godwand/client/DrillWindRenderer.java:23)
- [ ] `DrillWindRenderer.reset(0)`: it uses DrillWindRenderer.WINDS, which isn't translated (it uses Wind, which isn't translated (it uses `DrillWindPayload` (net.godwand.network))) (net/godwand/client/DrillWindRenderer.java:35)
- [ ] `DrillWindRenderer.render(3)`: it uses Minecraft's `Camera` (net/godwand/client/DrillWindRenderer.java:40)
- [ ] `DrillWindRenderer.drawCloud(5)`: a try/finally (net/godwand/client/DrillWindRenderer.java:80)
- [ ] `DrillWindRenderer.draw(5)`: it calls DrillWindRenderer.draw, which isn't translated (net/godwand/client/DrillWindRenderer.java:113)
- [ ] `DrillWindRenderer.draw(6)`: a try/finally (net/godwand/client/DrillWindRenderer.java:117)

### `DrillWindRenderer.Wind`: it uses `DrillWindPayload` (net.godwand.network)

- [ ] `DrillWindRenderer.Wind.Wind(2)`: its class isn't translated (it uses `DrillWindPayload` (net.godwand.network)) (net/godwand/client/DrillWindRenderer.java:219)
- [ ] `DrillWindRenderer.Wind.payload(0)`: its class isn't translated (it uses `DrillWindPayload` (net.godwand.network)) (net/godwand/client/DrillWindRenderer.java:219)
- [ ] `DrillWindRenderer.Wind.tick(0)`: its class isn't translated (it uses `DrillWindPayload` (net.godwand.network)) (net/godwand/client/DrillWindRenderer.java:219)

### `EarthDrillSoundscape`

- [x] `EarthDrillSoundscape.AUDIO` (net/godwand/client/EarthDrillSoundscape.java:25)
- [x] `EarthDrillSoundscape.PLACEHOLDER_SHA` (net/godwand/client/EarthDrillSoundscape.java:26)
- [ ] `EarthDrillSoundscape.voices`: it uses Minecraft's `SoundInstance` (net/godwand/client/EarthDrillSoundscape.java:27)
- [ ] `EarthDrillSoundscape.world`: it uses Minecraft's `ClientLevel` (net/godwand/client/EarthDrillSoundscape.java:28)
- [x] `EarthDrillSoundscape.start` (net/godwand/client/EarthDrillSoundscape.java:29)
- [x] `EarthDrillSoundscape.lastAge` (net/godwand/client/EarthDrillSoundscape.java:30)
- [x] `EarthDrillSoundscape.gain` (net/godwand/client/EarthDrillSoundscape.java:31)
- [x] `EarthDrillSoundscape.custom` (net/godwand/client/EarthDrillSoundscape.java:32)
- [ ] `EarthDrillSoundscape.isCustomAudio(1)`: a try/catch (NoSuchAlgorithmException) (net/godwand/client/EarthDrillSoundscape.java:34)
- [x] `EarthDrillSoundscape.distanceGain(1)` (net/godwand/client/EarthDrillSoundscape.java:48)
- [ ] `EarthDrillSoundscape.start(1)`: it calls EarthDrillSoundscape.reset, which isn't translated (net/godwand/client/EarthDrillSoundscape.java:52)
- [ ] `EarthDrillSoundscape.tick(1)`: it uses EarthDrillSoundscape.world, which isn't translated (it uses Minecraft's `ClientLevel`) (net/godwand/client/EarthDrillSoundscape.java:79)
- [ ] `EarthDrillSoundscape.cue(1)`: it's overloaded with the same number of parameters (net/godwand/client/EarthDrillSoundscape.java:109)
- [ ] `EarthDrillSoundscape.play(3)`: it's overloaded with the same number of parameters (net/godwand/client/EarthDrillSoundscape.java:137)
- [ ] `EarthDrillSoundscape.play(3)`: it's overloaded with the same number of parameters (net/godwand/client/EarthDrillSoundscape.java:141)
- [ ] `EarthDrillSoundscape.playAt(4)`: it uses Minecraft's `SimpleSoundInstance` (net/godwand/client/EarthDrillSoundscape.java:149)
- [ ] `EarthDrillSoundscape.reset(0)`: `null` (Rust has no null: this needs an Option) (net/godwand/client/EarthDrillSoundscape.java:157)

### `FugaAimRig`

- [ ] `FugaAimRig.availableFor`: a function kept in a variable or field (only parameters can be) (net/godwand/client/FugaAimRig.java:7)
- [x] `FugaAimRig.FugaAimRig(0)` (net/godwand/client/FugaAimRig.java:9)
- [x] `FugaAimRig.weight(1)` (net/godwand/client/FugaAimRig.java:12)
- [ ] `FugaAimRig.joints(2)`: it uses Joints, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaAimRig.java:16)
- [ ] `FugaAimRig.hand(2)`: it uses Joints, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaAimRig.java:31)

### `FugaAimRig.Joints`: it uses Minecraft's `Vec3`

- [ ] `FugaAimRig.Joints.Joints(3)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaAimRig.java:37)
- [ ] `FugaAimRig.Joints.shoulder(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaAimRig.java:37)
- [ ] `FugaAimRig.Joints.elbow(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaAimRig.java:37)
- [ ] `FugaAimRig.Joints.tip(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaAimRig.java:37)

### `FugaCinematic`

- [x] `FugaCinematic.cast` (net/godwand/client/FugaCinematic.java:23)
- [x] `FugaCinematic.skipped` (net/godwand/client/FugaCinematic.java:24)
- [ ] `FugaCinematic.world`: it uses Minecraft's `ClientLevel` (net/godwand/client/FugaCinematic.java:25)
- [x] `FugaCinematic.renderedFov` (net/godwand/client/FugaCinematic.java:26)
- [x] `FugaCinematic.cameraPartial` (net/godwand/client/FugaCinematic.java:27)
- [x] `FugaCinematic.END_CUT` (net/godwand/client/FugaCinematic.java:28)
- [x] `FugaCinematic.PILLAR_CUT` (net/godwand/client/FugaCinematic.java:29)
- [x] `FugaCinematic.IMPACT_KEYS` (net/godwand/client/FugaCinematic.java:30)
- [x] `FugaCinematic.IMPACT_OUT` (net/godwand/client/FugaCinematic.java:31)
- [x] `FugaCinematic.IMPACT_SIDE` (net/godwand/client/FugaCinematic.java:32)
- [x] `FugaCinematic.CHARGE_KEYS` (net/godwand/client/FugaCinematic.java:33)
- [x] `FugaCinematic.ORBIT` (net/godwand/client/FugaCinematic.java:34)
- [x] `FugaCinematic.DISTANCE` (net/godwand/client/FugaCinematic.java:35)
- [x] `FugaCinematic.HEIGHT` (net/godwand/client/FugaCinematic.java:36)
- [x] `FugaCinematic.FugaCinematic(0)` (net/godwand/client/FugaCinematic.java:38)
- [ ] `FugaCinematic.present(2)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaCinematic.java:41)
- [ ] `FugaCinematic.openingFaceVisible(1)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaCinematic.java:57)
- [ ] `FugaCinematic.showCaster(1)`: it calls FugaCinematic.openingFaceVisible, which isn't translated (net/godwand/client/FugaCinematic.java:61)
- [ ] `FugaCinematic.active(0)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaCinematic.java:65)
- [ ] `FugaCinematic.movementLocked(0)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaCinematic.java:74)
- [ ] `FugaCinematic.rotationLocked(0)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaCinematic.java:80)
- [ ] `FugaCinematic.tick(1)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaCinematic.java:85)
- [x] `FugaCinematic.skip(0)` (net/godwand/client/FugaCinematic.java:96)
- [ ] `FugaCinematic.reset(0)`: it uses FugaCinematic.world, which isn't translated (it uses Minecraft's `ClientLevel`) (net/godwand/client/FugaCinematic.java:100)
- [ ] `FugaCinematic.camera(4)`: it uses Pose, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaCinematic.java:107)
- [ ] `FugaCinematic.cameraFor(8)`: it uses Pose, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaCinematic.java:154)
- [ ] `FugaCinematic.blend(3)`: it uses Pose, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaCinematic.java:235)
- [x] `FugaCinematic.spline(3)` (net/godwand/client/FugaCinematic.java:239)
- [ ] `FugaCinematic.chargeCamera(7)`: it uses Pose, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaCinematic.java:257)
- [x] `FugaCinematic.sample(3)` (net/godwand/client/FugaCinematic.java:276)
- [ ] `FugaCinematic.shoulder(3)`: it uses Pose, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaCinematic.java:298)
- [ ] `FugaCinematic.look(2)`: it uses Pose, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaCinematic.java:305)
- [x] `FugaCinematic.smooth(3)` (net/godwand/client/FugaCinematic.java:310)
- [ ] `FugaCinematic.environmentGain(0)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaCinematic.java:315)
- [x] `FugaCinematic.darkness(1)` (net/godwand/client/FugaCinematic.java:326)
- [x] `FugaCinematic.silence(1)` (net/godwand/client/FugaCinematic.java:330)
- [ ] `FugaCinematic.fov(2)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaCinematic.java:334)
- [ ] `FugaCinematic.shake(1)`: a nullable object kept in a changing static (net/godwand/client/FugaCinematic.java:348)
- [x] `FugaCinematic.ringKick(1)` (net/godwand/client/FugaCinematic.java:384)
- [x] `FugaCinematic.letterbox(2)` (net/godwand/client/FugaCinematic.java:399)
- [x] `FugaCinematic.recordFov(1)` (net/godwand/client/FugaCinematic.java:417)
- [ ] `FugaCinematic.project(7)`: it calls Minecraft's `Vec3`.directionFromRotation (net/godwand/client/FugaCinematic.java:423)
- [ ] `FugaCinematic.aimTarget(2)`: it calls Minecraft's LocalPlayer.getEyePosition (no Horizon helper for it yet) (net/godwand/client/FugaCinematic.java:443)
- [ ] `FugaCinematic.reticle(3)`: it uses Minecraft's `Camera` (net/godwand/client/FugaCinematic.java:465)
- [x] `FugaCinematic.overlayFlash(2)` (net/godwand/client/FugaCinematic.java:483)
- [x] `FugaCinematic.flash(1)` (net/godwand/client/FugaCinematic.java:487)
- [x] `FugaCinematic.tint(7)` in game_side.rs (net/godwand/client/FugaCinematic.java:491)
- [ ] `FugaCinematic.grade(5)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaCinematic.java:503)
- [ ] `FugaCinematic.render(2)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaCinematic.java:526)

### `FugaPose`

- [x] `FugaPose.TWIST` (net/godwand/client/FugaPose.java:10)
- [x] `FugaPose.CLAP` (net/godwand/client/FugaPose.java:11)
- [x] `FugaPose.FugaPose(0)` (net/godwand/client/FugaPose.java:13)
- [ ] `FugaPose.apply(6)`: it uses Minecraft's `ModelPart` (net/godwand/client/FugaPose.java:16)
- [x] `FugaPose.arms(4)` (net/godwand/client/FugaPose.java:43)
- [ ] `FugaPose.stance(4)`: it uses Minecraft's `ModelPart` (net/godwand/client/FugaPose.java:81)
- [ ] `FugaPose.handFocus(2)`: it calls FugaPose.handFocus, which isn't translated (net/godwand/client/FugaPose.java:95)
- [ ] `FugaPose.handFocus(3)`: it calls FugaPose.hand, which isn't translated (net/godwand/client/FugaPose.java:99)
- [ ] `FugaPose.hand(3)`: it calls FugaPose.hand, which isn't translated (net/godwand/client/FugaPose.java:103)
- [ ] `FugaPose.hand(4)`: it calls FugaPose.wrist, which isn't translated (net/godwand/client/FugaPose.java:107)
- [ ] `FugaPose.wrist(4)`: JOML's Quaternionf.rotationZYX (net/godwand/client/FugaPose.java:115)

### `FugaPose.Arms`

- [x] `FugaPose.Arms.Arms(8)` (net/godwand/client/FugaPose.java:120)
- [x] `FugaPose.Arms.rightPitch(0)` (net/godwand/client/FugaPose.java:120)
- [x] `FugaPose.Arms.rightYaw(0)` (net/godwand/client/FugaPose.java:120)
- [x] `FugaPose.Arms.rightRoll(0)` (net/godwand/client/FugaPose.java:120)
- [x] `FugaPose.Arms.leftPitch(0)` (net/godwand/client/FugaPose.java:120)
- [x] `FugaPose.Arms.leftYaw(0)` (net/godwand/client/FugaPose.java:120)
- [x] `FugaPose.Arms.leftRoll(0)` (net/godwand/client/FugaPose.java:120)
- [x] `FugaPose.Arms.bodyYaw(0)` (net/godwand/client/FugaPose.java:120)
- [x] `FugaPose.Arms.weight(0)` (net/godwand/client/FugaPose.java:120)

### `FugaRenderer`

- [ ] `FugaRenderer.ACTIVE`: it uses `FugaFxPayload` (net.godwand.network) (net/godwand/client/FugaRenderer.java:38)
- [x] `FugaRenderer.RECEIVED` (net/godwand/client/FugaRenderer.java:39)
- [x] `FugaRenderer.PHASE_SEEN` (net/godwand/client/FugaRenderer.java:40)
- [ ] `FugaRenderer.LATEST`: it uses `FugaFxPayload` (net.godwand.network) (net/godwand/client/FugaRenderer.java:41)
- [ ] `FugaRenderer.world`: it uses Minecraft's `ClientLevel` (net/godwand/client/FugaRenderer.java:42)
- [ ] `FugaRenderer.AFTERMATH`: it uses Aftermath, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:43)
- [x] `FugaRenderer.AFTERMATH_TICKS` (net/godwand/client/FugaRenderer.java:44)
- [x] `FugaRenderer.TAU` (net/godwand/client/FugaRenderer.java:45)
- [ ] `FugaRenderer.glow`: it uses Glow, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:46)
- [x] `FugaRenderer.GLOW_TICKS` (net/godwand/client/FugaRenderer.java:47)
- [x] `FugaRenderer.clientTicks` (net/godwand/client/FugaRenderer.java:48)
- [x] `FugaRenderer.FUTURE_TOLERANCE` (net/godwand/client/FugaRenderer.java:49)
- [x] `FugaRenderer.CLOCKS` (net/godwand/client/FugaRenderer.java:50)
- [x] `FugaRenderer.IMPACT_BEATS` (net/godwand/client/FugaRenderer.java:51)
- [x] `FugaRenderer.HIT_STOP` (net/godwand/client/FugaRenderer.java:52)
- [x] `FugaRenderer.FREEZE_AGE` (net/godwand/client/FugaRenderer.java:53)
- [x] `FugaRenderer.renderPartial` (net/godwand/client/FugaRenderer.java:54)
- [x] `FugaRenderer.LAUNCH` in game_side.rs (net/godwand/client/FugaRenderer.java:55)
- [x] `FugaRenderer.shaderCaster` (net/godwand/client/FugaRenderer.java:56)
- [x] `FugaRenderer.pass` (net/godwand/client/FugaRenderer.java:57)
- [x] `FugaRenderer.FLAME_TEXTURE` (net/godwand/client/FugaRenderer.java:58)
- [x] `FugaRenderer.textureReady` (net/godwand/client/FugaRenderer.java:59)
- [x] `FugaRenderer.SOLID_U` (net/godwand/client/FugaRenderer.java:60)
- [x] `FugaRenderer.SOLID_V` (net/godwand/client/FugaRenderer.java:61)
- [x] `FugaRenderer.EDGE_V` (net/godwand/client/FugaRenderer.java:62)
- [x] `FugaRenderer.SPINE_V` (net/godwand/client/FugaRenderer.java:63)
- [x] `FugaRenderer.FAR_V` (net/godwand/client/FugaRenderer.java:64)
- [x] `FugaRenderer.flow` (net/godwand/client/FugaRenderer.java:65)
- [ ] `FugaRenderer.deferred`: it uses Deferred, which isn't translated (it uses Minecraft's `Camera`) (net/godwand/client/FugaRenderer.java:66)
- [x] `FugaRenderer.PULL_BALL` (net/godwand/client/FugaRenderer.java:67)
- [x] `FugaRenderer.STREAMS` (net/godwand/client/FugaRenderer.java:68)
- [x] `FugaRenderer.PUFF_COS` (net/godwand/client/FugaRenderer.java:69)
- [x] `FugaRenderer.PUFF_SIN` (net/godwand/client/FugaRenderer.java:70)
- [x] `FugaRenderer.FugaRenderer(0)` (net/godwand/client/FugaRenderer.java:72)
- [ ] `FugaRenderer.newer(2)`: it uses `FugaFxPayload` (net.godwand.network) (net/godwand/client/FugaRenderer.java:75)
- [ ] `FugaRenderer.receive(1)`: it uses `FugaFxPayload` (net.godwand.network) (net/godwand/client/FugaRenderer.java:81)
- [ ] `FugaRenderer.reset(0)`: it calls FugaSoundscape.reset, which isn't translated (net/godwand/client/FugaRenderer.java:123)
- [ ] `FugaRenderer.aftermath(1)`: it uses FugaRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`) (net/godwand/client/FugaRenderer.java:136)
- [ ] `FugaRenderer.phaseSeen(2)`: it uses FugaRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`) (net/godwand/client/FugaRenderer.java:154)
- [ ] `FugaRenderer.tick(1)`: it uses FugaRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`) (net/godwand/client/FugaRenderer.java:159)
- [ ] `FugaRenderer.particles(1)`: it uses FugaRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`) (net/godwand/client/FugaRenderer.java:216)
- [ ] `FugaRenderer.frameFor(2)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:277)
- [ ] `FugaRenderer.steadyAge(4)`: it uses `FugaFxPayload` (net.godwand.network) (net/godwand/client/FugaRenderer.java:318)
- [ ] `FugaRenderer.hitStop(1)`: it calls FugaRenderer.hitStop, which isn't translated (net/godwand/client/FugaRenderer.java:326)
- [ ] `FugaRenderer.hitStop(2)`: it uses `FugaFxPayload` (net.godwand.network) (net/godwand/client/FugaRenderer.java:330)
- [ ] `FugaRenderer.impactBeat(2)`: it uses `FugaFxPayload` (net.godwand.network) (net/godwand/client/FugaRenderer.java:340)
- [x] `FugaRenderer.flightTime(1)` (net/godwand/client/FugaRenderer.java:356)
- [ ] `FugaRenderer.localFrame(1)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:372)
- [ ] `FugaRenderer.shaderFrame(1)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:382)
- [ ] `FugaRenderer.afterglow(2)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:406)
- [ ] `FugaRenderer.ensureFlameTexture(0)`: it uses Minecraft's `NativeImage` (net/godwand/client/FugaRenderer.java:415)
- [x] `FugaRenderer.flameAlpha(2)` (net/godwand/client/FugaRenderer.java:431)
- [x] `FugaRenderer.fibre(3)` (net/godwand/client/FugaRenderer.java:445)
- [ ] `FugaRenderer.render(3)`: it uses Minecraft's `Camera` (net/godwand/client/FugaRenderer.java:461)
- [ ] `FugaRenderer.deferForShaderPack(3)`: it uses Minecraft's `Camera` (net/godwand/client/FugaRenderer.java:531)
- [x] `FugaRenderer.flushDeferred(0)` (net/godwand/client/FugaRenderer.java:535)
- [ ] `FugaRenderer.valid(2)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:538)
- [ ] `FugaRenderer.accents(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:547)
- [ ] `FugaRenderer.groundRings(5)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:565)
- [ ] `FugaRenderer.groundRing(8)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:589)
- [ ] `FugaRenderer.emitBackdrop(5)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:610)
- [ ] `FugaRenderer.emit(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:636)
- [ ] `FugaRenderer.emit(5)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:640)
- [x] `FugaRenderer.ribbonVolume(1)` (net/godwand/client/FugaRenderer.java:660)
- [ ] `FugaRenderer.shaderAxis(1)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:664)
- [ ] `FugaRenderer.chargeHand(3)`: it uses Frame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:675)
- [ ] `FugaRenderer.ritualRibbon(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:684)
- [x] `FugaRenderer.crackle(1)` (net/godwand/client/FugaRenderer.java:787)
- [ ] `FugaRenderer.clap(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:801)
- [ ] `FugaRenderer.stream(6)`: it calls Minecraft's Vec3.add (no Horizon helper for it yet) (net/godwand/client/FugaRenderer.java:817)
- [ ] `FugaRenderer.vortex(7)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:832)
- [ ] `FugaRenderer.flame(7)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:872)
- [ ] `FugaRenderer.flame(8)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:876)
- [ ] `FugaRenderer.ground(6)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:903)
- [ ] `FugaRenderer.cinders(6)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:947)
- [ ] `FugaRenderer.storms(3)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:991)
- [ ] `FugaRenderer.storm(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:997)
- [ ] `FugaRenderer.puff(7)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1064)
- [ ] `FugaRenderer.light(9)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1088)
- [ ] `FugaRenderer.loop(8)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1099)
- [ ] `FugaRenderer.glowDisc(6)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1118)
- [ ] `FugaRenderer.streaks(8)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1131)
- [x] `FugaRenderer.mixRgb(3)` (net/godwand/client/FugaRenderer.java:1156)
- [ ] `FugaRenderer.ember(7)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1163)
- [ ] `FugaRenderer.dot(7)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1168)
- [ ] `FugaRenderer.glint(6)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1184)
- [ ] `FugaRenderer.flowArrow(8)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1193)
- [ ] `FugaRenderer.charging(6)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1297)
- [ ] `FugaRenderer.flight(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1334)
- [ ] `FugaRenderer.streamers(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1392)
- [ ] `FugaRenderer.airRush(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1417)
- [ ] `FugaRenderer.impactPoint(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1469)
- [ ] `FugaRenderer.skyBeam(5)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1480)
- [ ] `FugaRenderer.lingeringTrail(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1492)
- [ ] `FugaRenderer.launch(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1515)
- [x] `FugaRenderer.orbRadius(1)` (net/godwand/client/FugaRenderer.java:1550)
- [x] `FugaRenderer.orbScale(1)` (net/godwand/client/FugaRenderer.java:1554)
- [x] `FugaRenderer.orbGap(1)` (net/godwand/client/FugaRenderer.java:1558)
- [x] `FugaRenderer.orbCenter(1)` (net/godwand/client/FugaRenderer.java:1562)
- [ ] `FugaRenderer.impactVolume(5)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1566)
- [x] `FugaRenderer.taper(3)` (net/godwand/client/FugaRenderer.java:1638)
- [ ] `FugaRenderer.impactAccents(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1642)
- [x] `FugaRenderer.layer(1)` (net/godwand/client/FugaRenderer.java:1977)
- [ ] `FugaRenderer.strand(8)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1981)
- [ ] `FugaRenderer.strand(9)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:1985)
- [ ] `FugaRenderer.line(9)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:2027)
- [ ] `FugaRenderer.spark(5)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:2031)
- [ ] `FugaRenderer.fireBody(9)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:2035)
- [ ] `FugaRenderer.licks(10)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:2068)
- [ ] `FugaRenderer.licks(13)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:2072)
- [ ] `FugaRenderer.ellipsoid(9)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:2124)
- [ ] `FugaRenderer.ring(8)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:2147)
- [ ] `FugaRenderer.ring(9)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:2151)
- [ ] `FugaRenderer.ring(10)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:2155)
- [ ] `FugaRenderer.quad2(10)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:2205)
- [x] `FugaRenderer.hot(1)` (net/godwand/client/FugaRenderer.java:2212)
- [ ] `FugaRenderer.quad(8)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:2216)
- [ ] `FugaRenderer.vertex(5)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:2223)
- [ ] `FugaRenderer.vertex(7)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/FugaRenderer.java:2227)
- [x] `FugaRenderer.hash(2)` (net/godwand/client/FugaRenderer.java:2232)
- [x] `FugaRenderer.noise(2)` (net/godwand/client/FugaRenderer.java:2238)
- [x] `FugaRenderer.easeOut(1)` (net/godwand/client/FugaRenderer.java:2252)
- [ ] `FugaRenderer.direction(1)`: it creates Minecraft's `Vec3` (net/godwand/client/FugaRenderer.java:2257)
- [ ] `FugaRenderer.side(1)`: it calls FugaRenderer.direction, which isn't translated (net/godwand/client/FugaRenderer.java:2261)
- [x] `FugaRenderer.smooth(1)` (net/godwand/client/FugaRenderer.java:2265)
- [x] `FugaRenderer.finite(1)` (net/godwand/client/FugaRenderer.java:2270)

### `FugaRenderer.Aftermath`: it uses Minecraft's `Vec3`

- [ ] `FugaRenderer.Aftermath.Aftermath(3)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2287)
- [ ] `FugaRenderer.Aftermath.center(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2287)
- [ ] `FugaRenderer.Aftermath.at(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2287)
- [ ] `FugaRenderer.Aftermath.cast(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2287)

### `FugaRenderer.Clock`

- [x] `FugaRenderer.Clock.Clock(4)` (net/godwand/client/FugaRenderer.java:2290)
- [x] `FugaRenderer.Clock.token(0)` (net/godwand/client/FugaRenderer.java:2290)
- [x] `FugaRenderer.Clock.phase(0)` (net/godwand/client/FugaRenderer.java:2290)
- [x] `FugaRenderer.Clock.age(0)` (net/godwand/client/FugaRenderer.java:2290)
- [x] `FugaRenderer.Clock.at(0)` (net/godwand/client/FugaRenderer.java:2290)

### `FugaRenderer.Deferred`: it uses Minecraft's `Camera`

- [ ] `FugaRenderer.Deferred.Deferred(5)`: its class isn't translated (it uses Minecraft's `Camera`) (net/godwand/client/FugaRenderer.java:2293)
- [ ] `FugaRenderer.Deferred.pose(0)`: its class isn't translated (it uses Minecraft's `Camera`) (net/godwand/client/FugaRenderer.java:2293)
- [ ] `FugaRenderer.Deferred.modelView(0)`: its class isn't translated (it uses Minecraft's `Camera`) (net/godwand/client/FugaRenderer.java:2293)
- [ ] `FugaRenderer.Deferred.projection(0)`: its class isn't translated (it uses Minecraft's `Camera`) (net/godwand/client/FugaRenderer.java:2293)
- [ ] `FugaRenderer.Deferred.camera(0)`: its class isn't translated (it uses Minecraft's `Camera`) (net/godwand/client/FugaRenderer.java:2293)
- [ ] `FugaRenderer.Deferred.partial(0)`: its class isn't translated (it uses Minecraft's `Camera`) (net/godwand/client/FugaRenderer.java:2293)

### `FugaRenderer.FlameTexture`: it extends Minecraft's `DynamicTexture`

- [ ] `FugaRenderer.FlameTexture.FlameTexture(1)`: its class isn't translated (it extends Minecraft's `DynamicTexture`) (net/godwand/client/FugaRenderer.java:2297)

### `FugaRenderer.Frame`: it uses Minecraft's `Vec3`

- [ ] `FugaRenderer.Frame.Frame(6)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2303)
- [ ] `FugaRenderer.Frame.center(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2304)
- [ ] `FugaRenderer.Frame.arrowPosition(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2308)
- [ ] `FugaRenderer.Frame.casterId(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2303)
- [ ] `FugaRenderer.Frame.startedAt(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2303)
- [ ] `FugaRenderer.Frame.phase(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2303)
- [ ] `FugaRenderer.Frame.age(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2303)
- [ ] `FugaRenderer.Frame.origin(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2303)
- [ ] `FugaRenderer.Frame.target(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2303)

### `FugaRenderer.Glow`: it uses Minecraft's `Vec3`

- [ ] `FugaRenderer.Glow.Glow(4)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2315)
- [ ] `FugaRenderer.Glow.caster(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2315)
- [ ] `FugaRenderer.Glow.token(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2315)
- [ ] `FugaRenderer.Glow.phaseAt(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2315)
- [ ] `FugaRenderer.Glow.center(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/FugaRenderer.java:2315)

### `FugaSoundscape`

- [x] `FugaSoundscape.CHARGE` (net/godwand/client/FugaSoundscape.java:27)
- [x] `FugaSoundscape.HOLD` (net/godwand/client/FugaSoundscape.java:28)
- [x] `FugaSoundscape.RELEASE` (net/godwand/client/FugaSoundscape.java:29)
- [x] `FugaSoundscape.END` (net/godwand/client/FugaSoundscape.java:30)
- [x] `FugaSoundscape.PLACEHOLDER_SHA` (net/godwand/client/FugaSoundscape.java:31)
- [ ] `FugaSoundscape.CASTS`: it uses Cast, which isn't translated (it uses Minecraft's `SimpleSoundInstance`) (net/godwand/client/FugaSoundscape.java:32)
- [x] `FugaSoundscape.CUSTOM` (net/godwand/client/FugaSoundscape.java:33)
- [ ] `FugaSoundscape.world`: it uses Minecraft's `ClientLevel` (net/godwand/client/FugaSoundscape.java:34)
- [x] `FugaSoundscape.FugaSoundscape(0)` (net/godwand/client/FugaSoundscape.java:36)
- [x] `FugaSoundscape.id(2)` (net/godwand/client/FugaSoundscape.java:39)
- [ ] `FugaSoundscape.isCustomAudio(1)`: a try/catch (NoSuchAlgorithmException) (net/godwand/client/FugaSoundscape.java:43)
- [ ] `FugaSoundscape.custom(2)`: it uses Minecraft's `Resource` (net/godwand/client/FugaSoundscape.java:57)
- [ ] `FugaSoundscape.tick(3)`: it uses Minecraft's `ClientLevel` (net/godwand/client/FugaSoundscape.java:72)
- [ ] `FugaSoundscape.gain(3)`: it uses `FugaFxPayload` (net.godwand.network) (net/godwand/client/FugaSoundscape.java:163)
- [ ] `FugaSoundscape.play(3)`: it uses Minecraft's `SimpleSoundInstance` (net/godwand/client/FugaSoundscape.java:177)
- [ ] `FugaSoundscape.cue(5)`: it uses `FugaFxPayload` (net.godwand.network) (net/godwand/client/FugaSoundscape.java:189)
- [ ] `FugaSoundscape.fadeHold(1)`: it uses Cast, which isn't translated (it uses Minecraft's `SimpleSoundInstance`) (net/godwand/client/FugaSoundscape.java:198)
- [ ] `FugaSoundscape.stop(2)`: it uses Cast, which isn't translated (it uses Minecraft's `SimpleSoundInstance`) (net/godwand/client/FugaSoundscape.java:205)
- [ ] `FugaSoundscape.reset(0)`: it uses FugaSoundscape.CASTS, which isn't translated (it uses Cast, which isn't translated (it uses Minecraft's `SimpleSoundInstance`)) (net/godwand/client/FugaSoundscape.java:229)

### `FugaSoundscape.Cast`: it uses Minecraft's `SimpleSoundInstance`

- [ ] `FugaSoundscape.Cast.Cast(1)`: its class isn't translated (it uses Minecraft's `SimpleSoundInstance`) (net/godwand/client/FugaSoundscape.java:251)

### `FugaSoundscape.Hold`: it extends Minecraft's `AbstractTickableSoundInstance`

- [ ] `FugaSoundscape.Hold.Hold(2)`: its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`) (net/godwand/client/FugaSoundscape.java:260)
- [ ] `FugaSoundscape.Hold.tick(0)`: its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`) (net/godwand/client/FugaSoundscape.java:270)

### `GodWandCinematicEngine`

- [x] `GodWandCinematicEngine.trauma` (net/godwand/client/GodWandCinematicEngine.java:14)
- [x] `GodWandCinematicEngine.traumaDecay` (net/godwand/client/GodWandCinematicEngine.java:15)
- [x] `GodWandCinematicEngine.shakeTime` (net/godwand/client/GodWandCinematicEngine.java:16)
- [x] `GodWandCinematicEngine.targetFovMultiplier` (net/godwand/client/GodWandCinematicEngine.java:17)
- [x] `GodWandCinematicEngine.currentFovMultiplier` (net/godwand/client/GodWandCinematicEngine.java:18)
- [x] `GodWandCinematicEngine.prevFovMultiplier` (net/godwand/client/GodWandCinematicEngine.java:19)
- [x] `GodWandCinematicEngine.fovHoldTicks` (net/godwand/client/GodWandCinematicEngine.java:20)
- [x] `GodWandCinematicEngine.fovRecoverySpeed` (net/godwand/client/GodWandCinematicEngine.java:21)
- [x] `GodWandCinematicEngine.vacuumSilenceTicks` (net/godwand/client/GodWandCinematicEngine.java:22)
- [x] `GodWandCinematicEngine.environmentGain` (net/godwand/client/GodWandCinematicEngine.java:23)
- [ ] `GodWandCinematicEngine.onCinematicEvent(1)`: it uses CinematicFxPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/client/GodWandCinematicEngine.java:25)
- [x] `GodWandCinematicEngine.distanceFactor(2)` (net/godwand/client/GodWandCinematicEngine.java:69)
- [x] `GodWandCinematicEngine.setFovFocus(3)` (net/godwand/client/GodWandCinematicEngine.java:76)
- [x] `GodWandCinematicEngine.addTrauma(2)` (net/godwand/client/GodWandCinematicEngine.java:82)
- [x] `GodWandCinematicEngine.setFovPunch(2)` (net/godwand/client/GodWandCinematicEngine.java:89)
- [x] `GodWandCinematicEngine.setVacuumSilence(1)` (net/godwand/client/GodWandCinematicEngine.java:96)
- [ ] `GodWandCinematicEngine.onClientTick(0)`: it calls SupernovaAccretionRenderer.age, which isn't translated (net/godwand/client/GodWandCinematicEngine.java:100)
- [ ] `GodWandCinematicEngine.applyScreenShake(2)`: a nullable object kept in a changing static (net/godwand/client/GodWandCinematicEngine.java:159)
- [ ] `GodWandCinematicEngine.getFovMultiplier(1)`: a nullable object kept in a changing static (net/godwand/client/GodWandCinematicEngine.java:176)
- [ ] `GodWandCinematicEngine.getEnvironmentGain(0)`: it calls FugaCinematic.environmentGain, which isn't translated (net/godwand/client/GodWandCinematicEngine.java:181)
- [x] `GodWandCinematicEngine.isVacuumSilenceActive(0)` (net/godwand/client/GodWandCinematicEngine.java:185)
- [ ] `GodWandCinematicEngine.reset(0)`: it calls SupernovaSoundscape.reset, which isn't translated (net/godwand/client/GodWandCinematicEngine.java:189)

### `GodWandPostFxManager`

- [x] `GodWandPostFxManager.pendingFrame` (net/godwand/client/GodWandPostFxManager.java:14)
- [ ] `GodWandPostFxManager.postProcessor`: it uses GodWandPostChain, which isn't translated (it implements `AutoCloseable`) (net/godwand/client/GodWandPostFxManager.java:15)
- [ ] `GodWandPostFxManager.novaProcessor`: it uses GodWandPostChain, which isn't translated (it implements `AutoCloseable`) (net/godwand/client/GodWandPostFxManager.java:16)
- [x] `GodWandPostFxManager.active` (net/godwand/client/GodWandPostFxManager.java:17)
- [x] `GodWandPostFxManager.currentTick` (net/godwand/client/GodWandPostFxManager.java:18)
- [x] `GodWandPostFxManager.waitingForBurstTicks` (net/godwand/client/GodWandPostFxManager.java:19)
- [x] `GodWandPostFxManager.maxTicks` (net/godwand/client/GodWandPostFxManager.java:20)
- [x] `GodWandPostFxManager.fxType` (net/godwand/client/GodWandPostFxManager.java:21)
- [x] `GodWandPostFxManager.baseIntensity` (net/godwand/client/GodWandPostFxManager.java:22)
- [ ] `GodWandPostFxManager.blastCenter`: it uses Minecraft's `Vec3` (net/godwand/client/GodWandPostFxManager.java:23)
- [x] `GodWandPostFxManager.recallCoreStart` (net/godwand/client/GodWandPostFxManager.java:24)
- [x] `GodWandPostFxManager.singularityRadius(1)` (net/godwand/client/GodWandPostFxManager.java:26)
- [x] `GodWandPostFxManager.singularityRecallRadius(2)` (net/godwand/client/GodWandPostFxManager.java:32)
- [x] `GodWandPostFxManager.supernovaRadius(1)` (net/godwand/client/GodWandPostFxManager.java:37)
- [x] `GodWandPostFxManager.blackHoleRadius(1)` (net/godwand/client/GodWandPostFxManager.java:44)
- [x] `GodWandPostFxManager.supernovaAftermath(1)` (net/godwand/client/GodWandPostFxManager.java:48)
- [ ] `GodWandPostFxManager.triggerImpact(6)`: it uses GodWandPostFxManager.blastCenter, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/GodWandPostFxManager.java:53)
- [ ] `GodWandPostFxManager.onClientTick(0)`: `++` inside an expression (net/godwand/client/GodWandPostFxManager.java:75)
- [ ] `GodWandPostFxManager.reset(0)`: it calls GodWandPostFxManager.reloadShaders, which isn't translated (net/godwand/client/GodWandPostFxManager.java:88)
- [ ] `GodWandPostFxManager.reloadShaders(0)`: it uses GodWandPostFxManager.postProcessor, which isn't translated (it uses GodWandPostChain, which isn't translated (it implements `AutoCloseable`)) (net/godwand/client/GodWandPostFxManager.java:95)
- [ ] `GodWandPostFxManager.bloomDimension(1)`: a nullable object kept in a changing static (net/godwand/client/GodWandPostFxManager.java:107)
- [ ] `GodWandPostFxManager.onWorldRender(3)`: a nullable object kept in a changing static (net/godwand/client/GodWandPostFxManager.java:112)
- [ ] `GodWandPostFxManager.renderAfterWorld(0)`: it calls GodWandPostFxManager.renderFrame, which isn't translated (net/godwand/client/GodWandPostFxManager.java:120)
- [ ] `GodWandPostFxManager.renderFrame(1)`: it uses SkyFrame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/GodWandPostFxManager.java:128)

### `GodWandPostFxManager.Frame`

- [x] `GodWandPostFxManager.Frame.Frame(3)` (net/godwand/client/GodWandPostFxManager.java:369)
- [x] `GodWandPostFxManager.Frame.positionMatrix(0)` (net/godwand/client/GodWandPostFxManager.java:369)
- [x] `GodWandPostFxManager.Frame.projectionMatrix(0)` (net/godwand/client/GodWandPostFxManager.java:369)
- [x] `GodWandPostFxManager.Frame.tickDelta(0)` (net/godwand/client/GodWandPostFxManager.java:369)

### `GodWandConfigScreen`: it extends Minecraft's `Screen`

- [ ] `GodWandConfigScreen.GodWandConfigScreen(1)`: its class isn't translated (it extends Minecraft's `Screen`) (net/godwand/client/gui/GodWandConfigScreen.java:16)
- [ ] `GodWandConfigScreen.init(0)`: its body couldn't be read: an anonymous class (net/godwand/client/gui/GodWandConfigScreen.java:21)
- [ ] `GodWandConfigScreen.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `Screen`) (net/godwand/client/gui/GodWandConfigScreen.java:126)
- [ ] `GodWandConfigScreen.onClose(0)`: its class isn't translated (it extends Minecraft's `Screen`) (net/godwand/client/gui/GodWandConfigScreen.java:131)

### `ImpactDebrisRenderer`

- [x] `ImpactDebrisRenderer.SEGMENTS` (net/godwand/client/ImpactDebrisRenderer.java:28)
- [x] `ImpactDebrisRenderer.RINGS` (net/godwand/client/ImpactDebrisRenderer.java:29)
- [ ] `ImpactDebrisRenderer.BURSTS`: it uses Burst, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/ImpactDebrisRenderer.java:30)
- [ ] `ImpactDebrisRenderer.world`: it uses Minecraft's `ClientLevel` (net/godwand/client/ImpactDebrisRenderer.java:31)
- [ ] `ImpactDebrisRenderer.reset(0)`: it uses ImpactDebrisRenderer.BURSTS, which isn't translated (it uses Burst, which isn't translated (it uses Minecraft's `Vec3`)) (net/godwand/client/ImpactDebrisRenderer.java:33)
- [ ] `ImpactDebrisRenderer.add(4)`: it uses Minecraft's `ClientLevel` (net/godwand/client/ImpactDebrisRenderer.java:38)
- [ ] `ImpactDebrisRenderer.receive(1)`: it's overloaded with the same number of parameters (net/godwand/client/ImpactDebrisRenderer.java:56)
- [ ] `ImpactDebrisRenderer.receive(1)`: it's overloaded with the same number of parameters (net/godwand/client/ImpactDebrisRenderer.java:60)
- [x] `ImpactDebrisRenderer.waveRadius(3)` (net/godwand/client/ImpactDebrisRenderer.java:71)
- [ ] `ImpactDebrisRenderer.fragmentOffset(4)`: it calls Minecraft's Vec3.scale (no Horizon helper for it yet) (net/godwand/client/ImpactDebrisRenderer.java:76)
- [ ] `ImpactDebrisRenderer.render(3)`: it uses Minecraft's `Camera` (net/godwand/client/ImpactDebrisRenderer.java:80)
- [ ] `ImpactDebrisRenderer.emit(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/ImpactDebrisRenderer.java:141)
- [ ] `ImpactDebrisRenderer.emitNovaPressure(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/ImpactDebrisRenderer.java:225)
- [ ] `ImpactDebrisRenderer.emitNovaEmbers(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/ImpactDebrisRenderer.java:277)
- [ ] `ImpactDebrisRenderer.vertex(7)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/ImpactDebrisRenderer.java:297)
- [ ] `ImpactDebrisRenderer.quad(10)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/ImpactDebrisRenderer.java:301)

### `ImpactDebrisRenderer.Burst`: it uses Minecraft's `Vec3`

- [ ] `ImpactDebrisRenderer.Burst.Burst(5)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/ImpactDebrisRenderer.java:319)
- [ ] `ImpactDebrisRenderer.Burst.sampleSurface(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/ImpactDebrisRenderer.java:329)
- [ ] `ImpactDebrisRenderer.Burst.point(3)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/ImpactDebrisRenderer.java:360)
- [ ] `ImpactDebrisRenderer.Burst.height(2)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/ImpactDebrisRenderer.java:364)

### `AquaHeartiaModel`: it uses Minecraft's `ModelPart`

- [ ] `AquaHeartiaModel.CHASSIS_QUADS`: it calls AquaHeartiaModel.createChassisQuads, which isn't translated (net/godwand/client/model/AquaHeartiaModel.java:29)
- [ ] `AquaHeartiaModel.SILVER_QUADS`: it calls AquaHeartiaModel.createSilverQuads, which isn't translated (net/godwand/client/model/AquaHeartiaModel.java:30)
- [ ] `AquaHeartiaModel.VENT_QUADS`: it calls AquaHeartiaModel.createVentQuads, which isn't translated (net/godwand/client/model/AquaHeartiaModel.java:31)
- [ ] `AquaHeartiaModel.SPHERE_QUADS`: it calls AquaHeartiaModel.createSphereQuads, which isn't translated (net/godwand/client/model/AquaHeartiaModel.java:32)
- [ ] `AquaHeartiaModel.CORE_SPHERE_QUADS`: it calls AquaHeartiaModel.createSphereQuads, which isn't translated (net/godwand/client/model/AquaHeartiaModel.java:33)
- [ ] `AquaHeartiaModel.AURA_SPHERE_QUADS`: it calls AquaHeartiaModel.createSphereQuads, which isn't translated (net/godwand/client/model/AquaHeartiaModel.java:34)
- [ ] `AquaHeartiaModel.AquaHeartiaModel(1)`: its class isn't translated (it uses Minecraft's `ModelPart`) (net/godwand/client/model/AquaHeartiaModel.java:36)
- [ ] `AquaHeartiaModel.getTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (net/godwand/client/model/AquaHeartiaModel.java:44)
- [ ] `AquaHeartiaModel.setupProceduralAnimations(3)`: its class isn't translated (it uses Minecraft's `ModelPart`) (net/godwand/client/model/AquaHeartiaModel.java:54)
- [ ] `AquaHeartiaModel.renderToBuffer(5)`: its class isn't translated (it uses Minecraft's `ModelPart`) (net/godwand/client/model/AquaHeartiaModel.java:65)
- [ ] `AquaHeartiaModel.renderWand(6)`: its class isn't translated (it uses Minecraft's `ModelPart`) (net/godwand/client/model/AquaHeartiaModel.java:71)
- [ ] `AquaHeartiaModel.renderQuads(6)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/model/AquaHeartiaModel.java:88)
- [ ] `AquaHeartiaModel.quad(9)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/model/AquaHeartiaModel.java:94)
- [ ] `AquaHeartiaModel.enlargeHead(1)`: new HashMap<>() without its types (net/godwand/client/model/AquaHeartiaModel.java:131)
- [x] `AquaHeartiaModel.addMiteredTube(3)` (net/godwand/client/model/AquaHeartiaModel.java:145)
- [x] `AquaHeartiaModel.addBeveledBox(7)` (net/godwand/client/model/AquaHeartiaModel.java:206)
- [ ] `AquaHeartiaModel.createSphereQuads(3)`: List.toArray (net/godwand/client/model/AquaHeartiaModel.java:229)
- [ ] `AquaHeartiaModel.createChassisQuads(0)`: it calls AquaHeartiaModel.enlargeHead, which isn't translated (net/godwand/client/model/AquaHeartiaModel.java:255)
- [ ] `AquaHeartiaModel.createSilverQuads(0)`: it calls AquaHeartiaModel.enlargeHead, which isn't translated (net/godwand/client/model/AquaHeartiaModel.java:309)
- [x] `AquaHeartiaModel.getBandCorners(9)` (net/godwand/client/model/AquaHeartiaModel.java:368)
- [ ] `AquaHeartiaModel.createVentQuads(0)`: it calls AquaHeartiaModel.enlargeHead, which isn't translated (net/godwand/client/model/AquaHeartiaModel.java:382)

### `GodWandModel`: it uses Minecraft's `ModelPart`

- [x] `GodWandModel.CX` (net/godwand/client/model/GodWandModel.java:30)
- [x] `GodWandModel.CY` (net/godwand/client/model/GodWandModel.java:31)
- [x] `GodWandModel.C_RADIUS` (net/godwand/client/model/GodWandModel.java:32)
- [x] `GodWandModel.NUM_SEGMENTS` (net/godwand/client/model/GodWandModel.java:33)
- [ ] `GodWandModel.SHAFT_QUADS`: it calls GodWandModel.createShaftQuads, which isn't translated (net/godwand/client/model/GodWandModel.java:34)
- [ ] `GodWandModel.GOLD_COLLARS`: it calls GodWandModel.createGoldCollars, which isn't translated (net/godwand/client/model/GodWandModel.java:35)
- [ ] `GodWandModel.LEATHER_QUADS`: it calls GodWandModel.createLeatherQuads, which isn't translated (net/godwand/client/model/GodWandModel.java:36)
- [ ] `GodWandModel.WRAPPING_QUADS`: it calls GodWandModel.createWrappingQuads, which isn't translated (net/godwand/client/model/GodWandModel.java:37)
- [ ] `GodWandModel.REACTOR_QUADS`: it calls GodWandModel.createReactorQuads, which isn't translated (net/godwand/client/model/GodWandModel.java:38)
- [ ] `GodWandModel.CRESCENT_QUADS`: it calls GodWandModel.createCrescentQuads, which isn't translated (net/godwand/client/model/GodWandModel.java:39)
- [ ] `GodWandModel.GOLD_RAIL`: it calls GodWandModel.createGoldRail, which isn't translated (net/godwand/client/model/GodWandModel.java:40)
- [ ] `GodWandModel.RED_VENTS`: it calls GodWandModel.createRedVents, which isn't translated (net/godwand/client/model/GodWandModel.java:41)
- [ ] `GodWandModel.GOLD_BRACKETS`: it calls GodWandModel.createGoldBrackets, which isn't translated (net/godwand/client/model/GodWandModel.java:42)
- [ ] `GodWandModel.TOP_TALON`: it calls GodWandModel.createTopTalon, which isn't translated (net/godwand/client/model/GodWandModel.java:43)
- [ ] `GodWandModel.POMMEL_BLADE`: it calls GodWandModel.createPommelBlade, which isn't translated (net/godwand/client/model/GodWandModel.java:44)
- [ ] `GodWandModel.GodWandModel(1)`: its class isn't translated (it uses Minecraft's `ModelPart`) (net/godwand/client/model/GodWandModel.java:46)
- [ ] `GodWandModel.getTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (net/godwand/client/model/GodWandModel.java:59)
- [ ] `GodWandModel.setupProceduralAnimations(3)`: its class isn't translated (it uses Minecraft's `ModelPart`) (net/godwand/client/model/GodWandModel.java:74)
- [ ] `GodWandModel.renderWand(6)`: its class isn't translated (it uses Minecraft's `ModelPart`) (net/godwand/client/model/GodWandModel.java:97)
- [ ] `GodWandModel.renderQuads(6)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/model/GodWandModel.java:143)
- [x] `GodWandModel.addOctagonalPrism(7)` (net/godwand/client/model/GodWandModel.java:149)
- [x] `GodWandModel.addBeveledRing(7)` (net/godwand/client/model/GodWandModel.java:176)
- [ ] `GodWandModel.createShaftQuads(0)`: List.toArray (net/godwand/client/model/GodWandModel.java:212)
- [ ] `GodWandModel.createGoldCollars(0)`: List.toArray (net/godwand/client/model/GodWandModel.java:220)
- [ ] `GodWandModel.createLeatherQuads(0)`: List.toArray (net/godwand/client/model/GodWandModel.java:244)
- [ ] `GodWandModel.createWrappingQuads(0)`: List.toArray (net/godwand/client/model/GodWandModel.java:250)
- [ ] `GodWandModel.createReactorQuads(0)`: List.toArray (net/godwand/client/model/GodWandModel.java:274)
- [ ] `GodWandModel.createCrescentQuads(0)`: List.toArray (net/godwand/client/model/GodWandModel.java:292)
- [ ] `GodWandModel.renderCrescent(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/model/GodWandModel.java:353)
- [ ] `GodWandModel.createGoldRail(0)`: List.toArray (net/godwand/client/model/GodWandModel.java:357)
- [ ] `GodWandModel.createRedVents(0)`: List.toArray (net/godwand/client/model/GodWandModel.java:395)
- [ ] `GodWandModel.createGoldBrackets(0)`: List.toArray (net/godwand/client/model/GodWandModel.java:426)
- [ ] `GodWandModel.createTopTalon(0)`: List.toArray (net/godwand/client/model/GodWandModel.java:460)
- [ ] `GodWandModel.createPommelBlade(0)`: List.toArray (net/godwand/client/model/GodWandModel.java:505)
- [ ] `GodWandModel.renderPommelBlade(4)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/model/GodWandModel.java:531)
- [ ] `GodWandModel.renderDiamondBezel(4)`: its class isn't translated (it uses Minecraft's `ModelPart`) (net/godwand/client/model/GodWandModel.java:535)
- [ ] `GodWandModel.renderFacetedGem(8)`: its class isn't translated (it uses Minecraft's `ModelPart`) (net/godwand/client/model/GodWandModel.java:580)
- [ ] `GodWandModel.renderEmberMotes(4)`: its class isn't translated (it uses Minecraft's `ModelPart`) (net/godwand/client/model/GodWandModel.java:598)
- [ ] `GodWandModel.quad(9)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/model/GodWandModel.java:609)
- [ ] `GodWandModel.vertex(9)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/model/GodWandModel.java:621)
- [ ] `GodWandModel.renderToBuffer(5)`: its class isn't translated (it uses Minecraft's `ModelPart`) (net/godwand/client/model/GodWandModel.java:630)

### `ShadowBladeModel`: it uses Minecraft's `ModelPart`

- [ ] `ShadowBladeModel.POMMEL_QUADS`: it calls ShadowBladeModel.createPommelQuads, which isn't translated (net/godwand/client/model/ShadowBladeModel.java:24)
- [ ] `ShadowBladeModel.HILT_QUADS`: it calls ShadowBladeModel.createHiltQuads, which isn't translated (net/godwand/client/model/ShadowBladeModel.java:25)
- [ ] `ShadowBladeModel.TSUBA_QUADS`: it calls ShadowBladeModel.createTsubaQuads, which isn't translated (net/godwand/client/model/ShadowBladeModel.java:26)
- [ ] `ShadowBladeModel.BLADE_QUADS`: it calls ShadowBladeModel.createBladeQuads, which isn't translated (net/godwand/client/model/ShadowBladeModel.java:27)
- [ ] `ShadowBladeModel.ShadowBladeModel(1)`: its class isn't translated (it uses Minecraft's `ModelPart`) (net/godwand/client/model/ShadowBladeModel.java:29)
- [ ] `ShadowBladeModel.getTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (net/godwand/client/model/ShadowBladeModel.java:33)
- [ ] `ShadowBladeModel.setupProceduralAnimations(3)`: its class isn't translated (it uses Minecraft's `ModelPart`) (net/godwand/client/model/ShadowBladeModel.java:40)
- [ ] `ShadowBladeModel.renderToBuffer(5)`: its class isn't translated (it uses Minecraft's `ModelPart`) (net/godwand/client/model/ShadowBladeModel.java:45)
- [ ] `ShadowBladeModel.renderBlade(6)`: its class isn't translated (it uses Minecraft's `ModelPart`) (net/godwand/client/model/ShadowBladeModel.java:49)
- [ ] `ShadowBladeModel.renderQuads(6)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/model/ShadowBladeModel.java:70)
- [ ] `ShadowBladeModel.quad(9)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/model/ShadowBladeModel.java:86)
- [ ] `ShadowBladeModel.createHiltQuads(0)`: List.toArray (net/godwand/client/model/ShadowBladeModel.java:142)
- [ ] `ShadowBladeModel.createTsubaQuads(0)`: List.toArray (net/godwand/client/model/ShadowBladeModel.java:174)
- [ ] `ShadowBladeModel.createPommelQuads(0)`: List.toArray (net/godwand/client/model/ShadowBladeModel.java:186)
- [x] `ShadowBladeModel.profile(3)` (net/godwand/client/model/ShadowBladeModel.java:198)
- [ ] `ShadowBladeModel.createBladeQuads(0)`: List.toArray (net/godwand/client/model/ShadowBladeModel.java:231)
- [x] `ShadowBladeModel.point(3)` (net/godwand/client/model/ShadowBladeModel.java:262)

### `GodWandPostChain`: it implements `AutoCloseable`

- [ ] `GodWandPostChain.GodWandPostChain(1)`: its class isn't translated (it implements `AutoCloseable`) (net/godwand/client/postfx/GodWandPostChain.java:45)
- [ ] `GodWandPostChain.setUniform(2)`: its class isn't translated (it implements `AutoCloseable`) (net/godwand/client/postfx/GodWandPostChain.java:49)
- [ ] `GodWandPostChain.setUniform(2)`: its class isn't translated (it implements `AutoCloseable`) (net/godwand/client/postfx/GodWandPostChain.java:53)
- [ ] `GodWandPostChain.resize(2)`: its class isn't translated (it implements `AutoCloseable`) (net/godwand/client/postfx/GodWandPostChain.java:57)
- [ ] `GodWandPostChain.process(0)`: its class isn't translated (it implements `AutoCloseable`) (net/godwand/client/postfx/GodWandPostChain.java:104)
- [ ] `GodWandPostChain.close(0)`: its class isn't translated (it implements `AutoCloseable`) (net/godwand/client/postfx/GodWandPostChain.java:172)

### `GodWandItemModel`: it implements Minecraft's `ItemModel`

- [ ] `GodWandItemModel.GodWandItemModel(1)`: its class isn't translated (it implements Minecraft's `ItemModel`) (net/godwand/client/render/GodWandItemModel.java:29)
- [ ] `GodWandItemModel.update(7)`: its class isn't translated (it implements Minecraft's `ItemModel`) (net/godwand/client/render/GodWandItemModel.java:33)

### `GodWandItemModel.Renderer`: it implements Minecraft's `SpecialModelRenderer`

- [ ] `GodWandItemModel.Renderer.Renderer(1)`: its class isn't translated (it implements Minecraft's `SpecialModelRenderer`) (net/godwand/client/render/GodWandItemModel.java:49)
- [ ] `GodWandItemModel.Renderer.submit(7)`: its class isn't translated (it implements Minecraft's `SpecialModelRenderer`) (net/godwand/client/render/GodWandItemModel.java:53)
- [ ] `GodWandItemModel.Renderer.getExtents(1)`: its class isn't translated (it implements Minecraft's `SpecialModelRenderer`) (net/godwand/client/render/GodWandItemModel.java:57)
- [ ] `GodWandItemModel.Renderer.extractArgument(1)`: its class isn't translated (it implements Minecraft's `SpecialModelRenderer`) (net/godwand/client/render/GodWandItemModel.java:62)

### `GodWandItemModel.Unbaked`: it implements `Unbaked`

- [ ] `GodWandItemModel.Unbaked.CODEC`: it uses Minecraft's `MapCodec` (net/godwand/client/render/GodWandItemModel.java:68)
- [ ] `GodWandItemModel.Unbaked.Unbaked(0)`: its class isn't translated (it implements `Unbaked`) (net/godwand/client/render/GodWandItemModel.java:67)
- [ ] `GodWandItemModel.Unbaked.type(0)`: its class isn't translated (it implements `Unbaked`) (net/godwand/client/render/GodWandItemModel.java:70)
- [ ] `GodWandItemModel.Unbaked.bake(2)`: its class isn't translated (it implements `Unbaked`) (net/godwand/client/render/GodWandItemModel.java:74)
- [ ] `GodWandItemModel.Unbaked.resolveDependencies(1)`: its class isn't translated (it implements `Unbaked`) (net/godwand/client/render/GodWandItemModel.java:78)

### `GodWandItemRenderer`: it uses GodWandModel, which isn't translated (it uses Minecraft's `ModelPart`)

- [x] `GodWandItemRenderer.TEXTURE` (net/godwand/client/render/GodWandItemRenderer.java:28)
- [ ] `GodWandItemRenderer.chargingStack`: it uses Minecraft's `ItemStack` (net/godwand/client/render/GodWandItemRenderer.java:32)
- [ ] `GodWandItemRenderer.releasedStack`: it uses Minecraft's `ItemStack` (net/godwand/client/render/GodWandItemRenderer.java:33)
- [x] `GodWandItemRenderer.previousCharge` (net/godwand/client/render/GodWandItemRenderer.java:34)
- [x] `GodWandItemRenderer.releaseTick` (net/godwand/client/render/GodWandItemRenderer.java:35)
- [ ] `GodWandItemRenderer.trackedWorld`: it uses Minecraft's `ClientLevel` (net/godwand/client/render/GodWandItemRenderer.java:36)
- [ ] `GodWandItemRenderer.tick(1)`: it uses GodWandItemRenderer.trackedWorld, which isn't translated (it uses Minecraft's `ClientLevel`) (net/godwand/client/render/GodWandItemRenderer.java:38)
- [x] `GodWandItemRenderer.releaseRecoil(1)` (net/godwand/client/render/GodWandItemRenderer.java:63)
- [x] `GodWandItemRenderer.coreColor(2)` (net/godwand/client/render/GodWandItemRenderer.java:67)
- [ ] `GodWandItemRenderer.submit(6)`: its class isn't translated (it uses GodWandModel, which isn't translated (it uses Minecraft's `ModelPart`)) (net/godwand/client/render/GodWandItemRenderer.java:88)
- [ ] `GodWandItemRenderer.render(6)`: its class isn't translated (it uses GodWandModel, which isn't translated (it uses Minecraft's `ModelPart`)) (net/godwand/client/render/GodWandItemRenderer.java:94)
- [ ] `GodWandItemRenderer.applyUseTransform(3)`: PoseStack.rotate (net/godwand/client/render/GodWandItemRenderer.java:185)
- [ ] `GodWandItemRenderer.applyAtomicHoldTransform(3)`: PoseStack.rotate (net/godwand/client/render/GodWandItemRenderer.java:191)
- [ ] `GodWandItemRenderer.applySwordDisplayTransform(2)`: PoseStack.rotate (net/godwand/client/render/GodWandItemRenderer.java:195)
- [ ] `GodWandItemRenderer.applyAtomicSwordHoldTransform(2)`: PoseStack.rotate (net/godwand/client/render/GodWandItemRenderer.java:209)
- [ ] `GodWandItemRenderer.applyDisplayTransform(2)`: its class isn't translated (it uses GodWandModel, which isn't translated (it uses Minecraft's `ModelPart`)) (net/godwand/client/render/GodWandItemRenderer.java:217)
- [x] `GodWandItemRenderer.chargeRatio(2)` (net/godwand/client/render/GodWandItemRenderer.java:258)
- [ ] `GodWandItemRenderer.getChargeRatio(2)`: its class isn't translated (it uses GodWandModel, which isn't translated (it uses Minecraft's `ModelPart`)) (net/godwand/client/render/GodWandItemRenderer.java:270)

### `GodWandItemRenderer.Buffers`: it's an interface

- [ ] `GodWandItemRenderer.Buffers.getBuffer(1)`: it has no body (net/godwand/client/render/GodWandItemRenderer.java:284)

### `SingularityThreadRenderer`

- [x] `SingularityThreadRenderer.THREADS` (net/godwand/client/SingularityThreadRenderer.java:25)
- [ ] `SingularityThreadRenderer.world`: it uses Minecraft's `ClientLevel` (net/godwand/client/SingularityThreadRenderer.java:26)
- [ ] `SingularityThreadRenderer.reset(0)`: it uses SingularityThreadRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`) (net/godwand/client/SingularityThreadRenderer.java:28)
- [ ] `SingularityThreadRenderer.receive(1)`: it uses `SingularityThreadsPayload` (net.godwand.network) (net/godwand/client/SingularityThreadRenderer.java:33)
- [x] `SingularityThreadRenderer.pulseFraction(3)` (net/godwand/client/SingularityThreadRenderer.java:47)
- [ ] `SingularityThreadRenderer.render(3)`: it uses Minecraft's `Camera` (net/godwand/client/SingularityThreadRenderer.java:57)

### `SingularityThreadRenderer.Thread`

- [x] `SingularityThreadRenderer.Thread.Thread(2)` (net/godwand/client/SingularityThreadRenderer.java:144)
- [x] `SingularityThreadRenderer.Thread.received(0)` (net/godwand/client/SingularityThreadRenderer.java:144)
- [x] `SingularityThreadRenderer.Thread.recallTicks(0)` (net/godwand/client/SingularityThreadRenderer.java:144)

### `SkillCooldownClient`

- [ ] `SkillCooldownClient.observer`: it uses Minecraft's `Player` (net/godwand/client/SkillCooldownClient.java:15)
- [ ] `SkillCooldownClient.session`: it uses `Object` (net/godwand/client/SkillCooldownClient.java:16)
- [x] `SkillCooldownClient.clock` (net/godwand/client/SkillCooldownClient.java:17)
- [x] `SkillCooldownClient.timers` (net/godwand/client/SkillCooldownClient.java:18)
- [x] `SkillCooldownClient.recall` (net/godwand/client/SkillCooldownClient.java:19)
- [x] `SkillCooldownClient.SkillCooldownClient(0)` (net/godwand/client/SkillCooldownClient.java:21)
- [ ] `SkillCooldownClient.observe(1)`: it uses Minecraft's `ClientPacketListener` (net/godwand/client/SkillCooldownClient.java:24)
- [ ] `SkillCooldownClient.tick(1)`: it calls SkillCooldownClient.observe, which isn't translated (net/godwand/client/SkillCooldownClient.java:41)
- [ ] `SkillCooldownClient.receive(1)`: it uses `SkillCooldownPayload` (net.godwand.network) (net/godwand/client/SkillCooldownClient.java:48)
- [ ] `SkillCooldownClient.remaining(1)`: it calls SkillCooldownClient.observe, which isn't translated (net/godwand/client/SkillCooldownClient.java:59)
- [ ] `SkillCooldownClient.fraction(1)`: it calls SkillCooldownClient.remaining, which isn't translated (net/godwand/client/SkillCooldownClient.java:64)
- [ ] `SkillCooldownClient.canUse(1)`: it calls SkillCooldownClient.remaining, which isn't translated (net/godwand/client/SkillCooldownClient.java:68)
- [ ] `SkillCooldownClient.render(1)`: it calls SkillCooldownClient.observe, which isn't translated (net/godwand/client/SkillCooldownClient.java:72)

### `SkillWheelScreen`: it extends Minecraft's `Screen`

- [ ] `SkillWheelScreen.CATEGORY`: it uses Minecraft's `Category` (net/godwand/client/SkillWheelScreen.java:25)
- [ ] `SkillWheelScreen.OPEN`: it uses Minecraft's `KeyMapping` (net/godwand/client/SkillWheelScreen.java:26)
- [ ] `SkillWheelScreen.OPEN_CONFIG`: it uses Minecraft's `KeyMapping` (net/godwand/client/SkillWheelScreen.java:27)
- [x] `SkillWheelScreen.ART` (net/godwand/client/SkillWheelScreen.java:28)
- [x] `SkillWheelScreen.SECTORS` (net/godwand/client/SkillWheelScreen.java:29)
- [x] `SkillWheelScreen.SECTOR` (net/godwand/client/SkillWheelScreen.java:30)
- [ ] `SkillWheelScreen.SkillWheelScreen(3)`: its class isn't translated (it extends Minecraft's `Screen`) (net/godwand/client/SkillWheelScreen.java:36)
- [ ] `SkillWheelScreen.handleKeyInput(1)`: it calls AtomicCinematic.movementLocked, which isn't translated (net/godwand/client/SkillWheelScreen.java:43)
- [ ] `SkillWheelScreen.wheelScale(0)`: its class isn't translated (it extends Minecraft's `Screen`) (net/godwand/client/SkillWheelScreen.java:82)
- [ ] `SkillWheelScreen.canSelect(0)`: its class isn't translated (it extends Minecraft's `Screen`) (net/godwand/client/SkillWheelScreen.java:86)
- [ ] `SkillWheelScreen.updateSelection(0)`: its class isn't translated (it extends Minecraft's `Screen`) (net/godwand/client/SkillWheelScreen.java:97)
- [ ] `SkillWheelScreen.confirm(0)`: its class isn't translated (it extends Minecraft's `Screen`) (net/godwand/client/SkillWheelScreen.java:103)
- [ ] `SkillWheelScreen.tick(0)`: its class isn't translated (it extends Minecraft's `Screen`) (net/godwand/client/SkillWheelScreen.java:114)
- [ ] `SkillWheelScreen.keyReleased(1)`: its class isn't translated (it extends Minecraft's `Screen`) (net/godwand/client/SkillWheelScreen.java:126)
- [ ] `SkillWheelScreen.mouseReleased(1)`: its class isn't translated (it extends Minecraft's `Screen`) (net/godwand/client/SkillWheelScreen.java:135)
- [ ] `SkillWheelScreen.isPauseScreen(0)`: its class isn't translated (it extends Minecraft's `Screen`) (net/godwand/client/SkillWheelScreen.java:144)
- [ ] `SkillWheelScreen.extractBackground(4)`: its class isn't translated (it extends Minecraft's `Screen`) (net/godwand/client/SkillWheelScreen.java:148)
- [ ] `SkillWheelScreen.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `Screen`) (net/godwand/client/SkillWheelScreen.java:151)
- [ ] `SkillWheelScreen.drawWheel(4)`: it calls Minecraft's GuiGraphics.blit (no Horizon helper for it yet) (net/godwand/client/SkillWheelScreen.java:177)
- [x] `SkillWheelScreen.skillColor(1)` (net/godwand/client/SkillWheelScreen.java:198)

### `SkillWheelSelection`

- [x] `SkillWheelSelection.modeAt(2)` (net/godwand/client/SkillWheelSelection.java:4)

### `SpatialAuraRenderer`

- [x] `SpatialAuraRenderer.ACTIVE` (net/godwand/client/SpatialAuraRenderer.java:22)
- [x] `SpatialAuraRenderer.ACTIVATIONS` (net/godwand/client/SpatialAuraRenderer.java:23)
- [ ] `SpatialAuraRenderer.HITS`: it uses Hit, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/SpatialAuraRenderer.java:24)
- [ ] `SpatialAuraRenderer.world`: it uses Minecraft's `ClientLevel` (net/godwand/client/SpatialAuraRenderer.java:25)
- [x] `SpatialAuraRenderer.EDGES` (net/godwand/client/SpatialAuraRenderer.java:26)
- [ ] `SpatialAuraRenderer.reset(0)`: it uses SpatialAuraRenderer.HITS, which isn't translated (it uses Hit, which isn't translated (it uses Minecraft's `Vec3`)) (net/godwand/client/SpatialAuraRenderer.java:28)
- [ ] `SpatialAuraRenderer.receive(1)`: it uses `SpatialAuraPayload` (net.godwand.network) (net/godwand/client/SpatialAuraRenderer.java:35)
- [ ] `SpatialAuraRenderer.render(3)`: it uses Minecraft's `Camera` (net/godwand/client/SpatialAuraRenderer.java:65)
- [ ] `SpatialAuraRenderer.emitField(5)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/SpatialAuraRenderer.java:129)
- [ ] `SpatialAuraRenderer.emitRibbon(7)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/SpatialAuraRenderer.java:139)
- [ ] `SpatialAuraRenderer.emitHit(6)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/SpatialAuraRenderer.java:169)
- [x] `SpatialAuraRenderer.smooth(3)` (net/godwand/client/SpatialAuraRenderer.java:197)
- [ ] `SpatialAuraRenderer.emitActivationShell(6)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/SpatialAuraRenderer.java:202)
- [ ] `SpatialAuraRenderer.emitEquatorRing(5)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/SpatialAuraRenderer.java:252)

### `SpatialAuraRenderer.Hit`: it uses Minecraft's `Vec3`

- [ ] `SpatialAuraRenderer.Hit.Hit(3)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/SpatialAuraRenderer.java:273)
- [ ] `SpatialAuraRenderer.Hit.playerId(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/SpatialAuraRenderer.java:273)
- [ ] `SpatialAuraRenderer.Hit.normal(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/SpatialAuraRenderer.java:273)
- [ ] `SpatialAuraRenderer.Hit.tick(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/SpatialAuraRenderer.java:273)

### `StoneCannonChargeRenderer`

- [ ] `StoneCannonChargeRenderer.render(3)`: it uses Minecraft's `Camera` (net/godwand/client/StoneCannonChargeRenderer.java:13)

### `StoneCannonEntityRenderer`: it extends Minecraft's `EntityRenderer`

- [ ] `StoneCannonEntityRenderer.StoneCannonEntityRenderer(1)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) (net/godwand/client/StoneCannonEntityRenderer.java:20)
- [ ] `StoneCannonEntityRenderer.createRenderState(0)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) (net/godwand/client/StoneCannonEntityRenderer.java:24)
- [ ] `StoneCannonEntityRenderer.extractRenderState(3)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) (net/godwand/client/StoneCannonEntityRenderer.java:28)
- [ ] `StoneCannonEntityRenderer.submit(4)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) (net/godwand/client/StoneCannonEntityRenderer.java:34)

### `StoneCannonEntityRenderer.State`: it extends Minecraft's `EntityRenderState`


### `StoneCannonModel`

- [x] `StoneCannonModel.chargeWindPoint(4)` (net/godwand/client/StoneCannonModel.java:11)
- [x] `StoneCannonModel.chargeWindVisibility(1)` (net/godwand/client/StoneCannonModel.java:18)
- [x] `StoneCannonModel.chargeDrillRotation(1)` (net/godwand/client/StoneCannonModel.java:23)
- [x] `StoneCannonModel.chargeWindRotation(1)` (net/godwand/client/StoneCannonModel.java:28)
- [ ] `StoneCannonModel.renderChargeWind(4)`: a try/finally (net/godwand/client/StoneCannonModel.java:34)
- [x] `StoneCannonModel.surface(2)` (net/godwand/client/StoneCannonModel.java:76)
- [ ] `StoneCannonModel.render(4)`: a try/finally (net/godwand/client/StoneCannonModel.java:81)

### `StoneCannonWindSound`: it extends Minecraft's `AbstractTickableSoundInstance`

- [ ] `StoneCannonWindSound.instance`: it uses StoneCannonWindSound, which isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`) (net/godwand/client/StoneCannonWindSound.java:13)
- [ ] `StoneCannonWindSound.StoneCannonWindSound(1)`: its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`) (net/godwand/client/StoneCannonWindSound.java:16)
- [x] `StoneCannonWindSound.windVolume(1)` (net/godwand/client/StoneCannonWindSound.java:26)
- [ ] `StoneCannonWindSound.update(1)`: `instanceof GodWandItem` on an item (no single item id for it) (net/godwand/client/StoneCannonWindSound.java:31)
- [ ] `StoneCannonWindSound.reset(0)`: it uses StoneCannonWindSound.instance, which isn't translated (it uses StoneCannonWindSound, which isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`)) (net/godwand/client/StoneCannonWindSound.java:54)
- [ ] `StoneCannonWindSound.tick(0)`: its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`) (net/godwand/client/StoneCannonWindSound.java:66)

### `SupernovaAccretionRenderer`

- [ ] `SupernovaAccretionRenderer.world`: it uses Minecraft's `ClientLevel` (net/godwand/client/SupernovaAccretionRenderer.java:25)
- [ ] `SupernovaAccretionRenderer.center`: it uses Minecraft's `Vec3` (net/godwand/client/SupernovaAccretionRenderer.java:26)
- [x] `SupernovaAccretionRenderer.start` (net/godwand/client/SupernovaAccretionRenderer.java:27)
- [x] `SupernovaAccretionRenderer.active` (net/godwand/client/SupernovaAccretionRenderer.java:28)
- [ ] `SupernovaAccretionRenderer.chargeWorld`: it uses Minecraft's `ClientLevel` (net/godwand/client/SupernovaAccretionRenderer.java:29)
- [ ] `SupernovaAccretionRenderer.chargeCenter`: it uses Minecraft's `Vec3` (net/godwand/client/SupernovaAccretionRenderer.java:30)
- [x] `SupernovaAccretionRenderer.chargeTicks` (net/godwand/client/SupernovaAccretionRenderer.java:31)
- [x] `SupernovaAccretionRenderer.previousChargeTicks` (net/godwand/client/SupernovaAccretionRenderer.java:32)
- [x] `SupernovaAccretionRenderer.charging` (net/godwand/client/SupernovaAccretionRenderer.java:33)
- [x] `SupernovaAccretionRenderer.EDGES` (net/godwand/client/SupernovaAccretionRenderer.java:34)
- [ ] `SupernovaAccretionRenderer.currentCameraRel`: it uses Minecraft's `Vec3` (net/godwand/client/SupernovaAccretionRenderer.java:35)
- [ ] `SupernovaAccretionRenderer.reset(0)`: it uses SupernovaAccretionRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`) (net/godwand/client/SupernovaAccretionRenderer.java:37)
- [ ] `SupernovaAccretionRenderer.receive(1)`: it uses ImpactFxPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/client/SupernovaAccretionRenderer.java:47)
- [ ] `SupernovaAccretionRenderer.tickCharge(1)`: it uses SupernovaAccretionRenderer.chargeWorld, which isn't translated (it uses Minecraft's `ClientLevel`) (net/godwand/client/SupernovaAccretionRenderer.java:65)
- [ ] `SupernovaAccretionRenderer.skyFrame(1)`: it uses SkyFrame, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/SupernovaAccretionRenderer.java:97)
- [ ] `SupernovaAccretionRenderer.age(1)`: it calls SupernovaAccretionRenderer.skyAge, which isn't translated (net/godwand/client/SupernovaAccretionRenderer.java:114)
- [ ] `SupernovaAccretionRenderer.skyAge(1)`: `null` (Rust has no null: this needs an Option) (net/godwand/client/SupernovaAccretionRenderer.java:119)
- [ ] `SupernovaAccretionRenderer.center(0)`: it uses SupernovaAccretionRenderer.center, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/SupernovaAccretionRenderer.java:124)
- [ ] `SupernovaAccretionRenderer.render(3)`: it uses Minecraft's `Camera` (net/godwand/client/SupernovaAccretionRenderer.java:128)
- [ ] `SupernovaAccretionRenderer.emitChargeMarker(3)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/SupernovaAccretionRenderer.java:171)
- [x] `SupernovaAccretionRenderer.streamPhase(2)` (net/godwand/client/SupernovaAccretionRenderer.java:196)
- [ ] `SupernovaAccretionRenderer.streamPoint(3)`: it creates Minecraft's `Vec3` (net/godwand/client/SupernovaAccretionRenderer.java:201)
- [ ] `SupernovaAccretionRenderer.emit(3)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/SupernovaAccretionRenderer.java:213)
- [ ] `SupernovaAccretionRenderer.emitInfallSparks(5)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/SupernovaAccretionRenderer.java:263)
- [ ] `SupernovaAccretionRenderer.arc(11)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/SupernovaAccretionRenderer.java:304)
- [ ] `SupernovaAccretionRenderer.vertex(7)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/SupernovaAccretionRenderer.java:325)
- [ ] `SupernovaAccretionRenderer.emitPreImpactShockwave(3)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/SupernovaAccretionRenderer.java:336)
- [ ] `SupernovaAccretionRenderer.emitShockwaveRing(7)`: it uses Minecraft's `VertexConsumer` (net/godwand/client/SupernovaAccretionRenderer.java:347)

### `SupernovaAccretionRenderer.SkyFrame`: it uses Minecraft's `Vec3`

- [ ] `SupernovaAccretionRenderer.SkyFrame.SkyFrame(6)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/SupernovaAccretionRenderer.java:380)
- [ ] `SupernovaAccretionRenderer.SkyFrame.center(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/SupernovaAccretionRenderer.java:380)
- [ ] `SupernovaAccretionRenderer.SkyFrame.scale(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/SupernovaAccretionRenderer.java:380)
- [ ] `SupernovaAccretionRenderer.SkyFrame.collapse(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/SupernovaAccretionRenderer.java:380)
- [ ] `SupernovaAccretionRenderer.SkyFrame.pulseRadius(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/SupernovaAccretionRenderer.java:380)
- [ ] `SupernovaAccretionRenderer.SkyFrame.pulseStrength(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/SupernovaAccretionRenderer.java:380)
- [ ] `SupernovaAccretionRenderer.SkyFrame.time(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/client/SupernovaAccretionRenderer.java:380)

### `SupernovaSoundscape`

- [x] `SupernovaSoundscape.AUDIO` (net/godwand/client/SupernovaSoundscape.java:22)
- [x] `SupernovaSoundscape.PLACEHOLDER_SHA` (net/godwand/client/SupernovaSoundscape.java:23)
- [ ] `SupernovaSoundscape.chargeSound`: it uses Charge, which isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`) (net/godwand/client/SupernovaSoundscape.java:24)
- [ ] `SupernovaSoundscape.sequence`: it uses Minecraft's `SimpleSoundInstance` (net/godwand/client/SupernovaSoundscape.java:25)
- [ ] `SupernovaSoundscape.world`: it uses Minecraft's `ClientLevel` (net/godwand/client/SupernovaSoundscape.java:26)
- [x] `SupernovaSoundscape.custom` (net/godwand/client/SupernovaSoundscape.java:27)
- [x] `SupernovaSoundscape.customChecked` (net/godwand/client/SupernovaSoundscape.java:28)
- [x] `SupernovaSoundscape.charging` (net/godwand/client/SupernovaSoundscape.java:29)
- [x] `SupernovaSoundscape.activated` (net/godwand/client/SupernovaSoundscape.java:30)
- [x] `SupernovaSoundscape.cancelGraceTicks` (net/godwand/client/SupernovaSoundscape.java:31)
- [x] `SupernovaSoundscape.activationTime` (net/godwand/client/SupernovaSoundscape.java:32)
- [ ] `SupernovaSoundscape.isCustomAudio(1)`: a try/catch (NoSuchAlgorithmException) (net/godwand/client/SupernovaSoundscape.java:34)
- [ ] `SupernovaSoundscape.checkCustom(1)`: it uses Minecraft's `Resource` (net/godwand/client/SupernovaSoundscape.java:48)
- [x] `SupernovaSoundscape.volume(2)` (net/godwand/client/SupernovaSoundscape.java:66)
- [ ] `SupernovaSoundscape.start(1)`: `null` (Rust has no null: this needs an Option) (net/godwand/client/SupernovaSoundscape.java:70)
- [ ] `SupernovaSoundscape.tick(3)`: `null` (Rust has no null: this needs an Option) (net/godwand/client/SupernovaSoundscape.java:103)
- [ ] `SupernovaSoundscape.reset(0)`: `null` (Rust has no null: this needs an Option) (net/godwand/client/SupernovaSoundscape.java:178)

### `SupernovaSoundscape.Charge`: it extends Minecraft's `AbstractTickableSoundInstance`

- [ ] `SupernovaSoundscape.Charge.Charge(1)`: its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`) (net/godwand/client/SupernovaSoundscape.java:203)
- [ ] `SupernovaSoundscape.Charge.tick(0)`: its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`) (net/godwand/client/SupernovaSoundscape.java:213)

### `GodWandCommands`

- [x] `GodWandCommands.SKILLS` (net/godwand/command/GodWandCommands.java:20)
- [x] `GodWandCommands.GodWandCommands(0)` (net/godwand/command/GodWandCommands.java:22)
- [ ] `GodWandCommands.register(1)`: it uses Minecraft's `CommandDispatcher` (net/godwand/command/GodWandCommands.java:25)
- [ ] `GodWandCommands.resetCooldowns(2)`: it uses Minecraft's `ServerPlayer` (net/godwand/command/GodWandCommands.java:49)
- [ ] `GodWandCommands.change(2)`: it uses Minecraft's `ServerPlayer` (net/godwand/command/GodWandCommands.java:58)

### `GodWandConfig`

- [x] `GodWandConfig.JSON` (net/godwand/config/GodWandConfig.java:22)
- [ ] `GodWandConfig.CLIENT`: it uses Client, which isn't translated (it uses WandSkin, which isn't translated (its constants carry values)) (net/godwand/config/GodWandConfig.java:23)
- [x] `GodWandConfig.SERVER` (net/godwand/config/GodWandConfig.java:24)
- [ ] `GodWandConfig.loadClient(1)`: it uses `Class` (net/godwand/config/GodWandConfig.java:26)
- [ ] `GodWandConfig.loadServer(1)`: it uses `Class` (net/godwand/config/GodWandConfig.java:31)
- [ ] `GodWandConfig.saveClient(0)`: it calls GodWandConfig.saveClient, which isn't translated (net/godwand/config/GodWandConfig.java:36)
- [ ] `GodWandConfig.saveClient(1)`: it uses `T` (net/godwand/config/GodWandConfig.java:40)
- [ ] `GodWandConfig.saveServer(0)`: it calls GodWandConfig.saveServer, which isn't translated (net/godwand/config/GodWandConfig.java:44)
- [ ] `GodWandConfig.saveServer(1)`: it uses `T` (net/godwand/config/GodWandConfig.java:48)
- [ ] `GodWandConfig.upgradeCooldownDefaults(1)`: it uses `Object` (net/godwand/config/GodWandConfig.java:52)
- [ ] `GodWandConfig.read(3)`: it's generic (net/godwand/config/GodWandConfig.java:74)
- [ ] `GodWandConfig.write(2)`: it's generic (net/godwand/config/GodWandConfig.java:125)
- [x] `GodWandConfig.validatedCooldowns(2)` (net/godwand/config/GodWandConfig.java:134)
- [x] `GodWandConfig.validatedDamage(2)` (net/godwand/config/GodWandConfig.java:145)
- [x] `GodWandConfig.tierIndex(2)` (net/godwand/config/GodWandConfig.java:157)
- [ ] `GodWandConfig.skillCooldownTicks(2)`: a nullable object kept in a changing static (net/godwand/config/GodWandConfig.java:161)
- [ ] `GodWandConfig.skillDamageMultiplier(2)`: a nullable object kept in a changing static (net/godwand/config/GodWandConfig.java:175)
- [ ] `GodWandConfig.cooldownTicks(1)`: a nullable object kept in a changing static (net/godwand/config/GodWandConfig.java:186)
- [x] `GodWandConfig.finiteClamp(4)` (net/godwand/config/GodWandConfig.java:190)

### `GodWandConfig.Client`: it uses WandSkin, which isn't translated (its constants carry values)

- [ ] `GodWandConfig.Client.getActiveSkin(1)`: its class isn't translated (it uses WandSkin, which isn't translated (its constants carry values)) (net/godwand/config/GodWandConfig.java:207)
- [ ] `GodWandConfig.Client.validate(0)`: its class isn't translated (it uses WandSkin, which isn't translated (its constants carry values)) (net/godwand/config/GodWandConfig.java:219)

### `GodWandConfig.Server`

- [x] `GodWandConfig.Server.isDimensionBlacklisted(1)` (net/godwand/config/GodWandConfig.java:262)
- [x] `GodWandConfig.Server.isDimensionAllowed(1)` (net/godwand/config/GodWandConfig.java:266)
- [x] `GodWandConfig.Server.validate(0)` (net/godwand/config/GodWandConfig.java:270)

### `WandSkin`: its constants carry values

- [ ] `WandSkin.WandSkin(2)`: its class isn't translated (its constants carry values) (net/godwand/config/WandSkin.java:14)
- [ ] `WandSkin.getId(0)`: its class isn't translated (its constants carry values) (net/godwand/config/WandSkin.java:19)
- [ ] `WandSkin.getDisplayName(0)`: its class isn't translated (its constants carry values) (net/godwand/config/WandSkin.java:23)
- [ ] `WandSkin.fromId(1)`: it uses WandSkin, which isn't translated (its constants carry values) (net/godwand/config/WandSkin.java:27)

### `ArcaneOrbEntity`: it extends Minecraft's `Projectile`

- [ ] `ArcaneOrbEntity.CHARGE_LEVEL`: it uses Minecraft's `EntityDataAccessor` (net/godwand/entity/ArcaneOrbEntity.java:30)
- [ ] `ArcaneOrbEntity.ArcaneOrbEntity(2)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/ArcaneOrbEntity.java:32)
- [ ] `ArcaneOrbEntity.ArcaneOrbEntity(3)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/ArcaneOrbEntity.java:37)
- [ ] `ArcaneOrbEntity.setChargeLevel(1)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/ArcaneOrbEntity.java:44)
- [ ] `ArcaneOrbEntity.getChargeLevel(0)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/ArcaneOrbEntity.java:48)
- [ ] `ArcaneOrbEntity.defineSynchedData(1)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/ArcaneOrbEntity.java:52)
- [ ] `ArcaneOrbEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/ArcaneOrbEntity.java:56)
- [ ] `ArcaneOrbEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/ArcaneOrbEntity.java:62)
- [ ] `ArcaneOrbEntity.tick(0)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/ArcaneOrbEntity.java:68)
- [ ] `ArcaneOrbEntity.canHitEntity(1)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/ArcaneOrbEntity.java:104)
- [ ] `ArcaneOrbEntity.onHit(1)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/ArcaneOrbEntity.java:108)

### `StoneCannonProjectileEntity`: it extends Minecraft's `Projectile`

- [ ] `StoneCannonProjectileEntity.CHARGE_LEVEL`: it uses Minecraft's `EntityDataAccessor` (net/godwand/entity/StoneCannonProjectileEntity.java:29)
- [ ] `StoneCannonProjectileEntity.markHit(1)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/StoneCannonProjectileEntity.java:32)
- [ ] `StoneCannonProjectileEntity.StoneCannonProjectileEntity(2)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/StoneCannonProjectileEntity.java:36)
- [ ] `StoneCannonProjectileEntity.StoneCannonProjectileEntity(3)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/StoneCannonProjectileEntity.java:41)
- [ ] `StoneCannonProjectileEntity.setChargeLevel(1)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/StoneCannonProjectileEntity.java:48)
- [ ] `StoneCannonProjectileEntity.getChargeLevel(0)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/StoneCannonProjectileEntity.java:52)
- [ ] `StoneCannonProjectileEntity.defineSynchedData(1)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/StoneCannonProjectileEntity.java:56)
- [ ] `StoneCannonProjectileEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/StoneCannonProjectileEntity.java:60)
- [ ] `StoneCannonProjectileEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/StoneCannonProjectileEntity.java:66)
- [ ] `StoneCannonProjectileEntity.tick(0)`: its class isn't translated (it extends Minecraft's `Projectile`) (net/godwand/entity/StoneCannonProjectileEntity.java:72)

### `GodWandFabricClientMod`: it implements Fabric's `ClientModInitializer`

- [ ] `GodWandFabricClientMod.onInitializeClient(0)`: done another way in Horizon: it runs as `on_init`, from Horizon's init hook (net/godwand/fabric/client/GodWandFabricClientMod.java:69)

### `GodWandModMenuApi`: it implements `ModMenuApi` (com.terraformersmc.modmenu.api)

- [ ] `GodWandModMenuApi.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) (net/godwand/fabric/client/GodWandModMenuApi.java:8)

### `GodWandFabricMod`: it implements Fabric's `ModInitializer`

- [ ] `GodWandFabricMod.onInitialize(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (net/godwand/fabric/GodWandFabricMod.java:74)

### `FabricNetworkHelper`: it implements `INetworkHelper` (net.godwand.platform)

- [ ] `FabricNetworkHelper.sendToPlayer(2)`: its class isn't translated (it implements `INetworkHelper` (net.godwand.platform)) (net/godwand/fabric/platform/FabricNetworkHelper.java:13)
- [ ] `FabricNetworkHelper.sendToNearby(4)`: its class isn't translated (it implements `INetworkHelper` (net.godwand.platform)) (net/godwand/fabric/platform/FabricNetworkHelper.java:18)
- [ ] `FabricNetworkHelper.sendToServer(1)`: its class isn't translated (it implements `INetworkHelper` (net.godwand.platform)) (net/godwand/fabric/platform/FabricNetworkHelper.java:25)

### `FabricPlatformHelper`: it implements `IPlatformHelper` (net.godwand.platform)

- [ ] `FabricPlatformHelper.getPlatformName(0)`: its class isn't translated (it implements `IPlatformHelper` (net.godwand.platform)) (net/godwand/fabric/platform/FabricPlatformHelper.java:8)
- [ ] `FabricPlatformHelper.isModLoaded(1)`: its class isn't translated (it implements `IPlatformHelper` (net.godwand.platform)) (net/godwand/fabric/platform/FabricPlatformHelper.java:13)
- [ ] `FabricPlatformHelper.isDevelopmentEnvironment(0)`: its class isn't translated (it implements `IPlatformHelper` (net.godwand.platform)) (net/godwand/fabric/platform/FabricPlatformHelper.java:18)
- [ ] `FabricPlatformHelper.getConfigDirectory(0)`: its class isn't translated (it implements `IPlatformHelper` (net.godwand.platform)) (net/godwand/fabric/platform/FabricPlatformHelper.java:23)

### `GodWandMod`

- [x] `GodWandMod.MOD_ID` (net/godwand/GodWandMod.java:15)
- [x] `GodWandMod.LOGGER` (net/godwand/GodWandMod.java:16)
- [ ] `GodWandMod.ARCANE_ORB_ENTITY_TYPE`: it uses Minecraft's `EntityType` (net/godwand/GodWandMod.java:17)
- [ ] `GodWandMod.STONE_CANNON_ENTITY_TYPE`: it uses Minecraft's `EntityType` (net/godwand/GodWandMod.java:18)
- [ ] `GodWandMod.GOD_WAND_ITEM`: it uses Minecraft's `Item` (net/godwand/GodWandMod.java:19)
- [ ] `GodWandMod.GODWAND_ITEM_GROUP_KEY`: it uses Minecraft's `ResourceKey` (net/godwand/GodWandMod.java:20)
- [ ] `GodWandMod.GODWAND_ITEM_GROUP`: it uses Minecraft's `CreativeModeTab` (net/godwand/GodWandMod.java:23)

### `GodWandItem`: it extends Minecraft's `Item`

- [x] `GodWandItem.MODE_SPATIAL_PATH` (net/godwand/item/GodWandItem.java:53)
- [x] `GodWandItem.MODE_ARCANE_ORB` (net/godwand/item/GodWandItem.java:54)
- [x] `GodWandItem.MODE_RATIO_COLLAPSE` (net/godwand/item/GodWandItem.java:55)
- [x] `GodWandItem.MODE_SUPERNOVA` (net/godwand/item/GodWandItem.java:56)
- [x] `GodWandItem.MODE_STONE_CANNON` (net/godwand/item/GodWandItem.java:57)
- [x] `GodWandItem.MODE_ATOMIC` (net/godwand/item/GodWandItem.java:58)
- [x] `GodWandItem.MODE_FUGA` (net/godwand/item/GodWandItem.java:59)
- [x] `GodWandItem.TOTAL_MODES` (net/godwand/item/GodWandItem.java:60)
- [x] `GodWandItem.chargeMeter(2)` (net/godwand/item/GodWandItem.java:62)
- [ ] `GodWandItem.GodWandItem(1)`: its class isn't translated (it extends Minecraft's `Item`) (net/godwand/item/GodWandItem.java:67)
- [ ] `GodWandItem.createMeleeAttributes(0)`: it uses Minecraft's `ItemAttributeModifiers` (net/godwand/item/GodWandItem.java:71)
- [x] `GodWandItem.getSkillMode(1)` in game_side.rs (net/godwand/item/GodWandItem.java:77)
- [ ] `GodWandItem.setSkillMode(2)`: it uses Minecraft's `DataComponents`.CUSTOM_DATA (net/godwand/item/GodWandItem.java:82)
- [ ] `GodWandItem.selectSkill(3)`: `instanceof GodWandItem` on an item (no single item id for it) (net/godwand/item/GodWandItem.java:88)
- [ ] `GodWandItem.use(3)`: its class isn't translated (it extends Minecraft's `Item`) (net/godwand/item/GodWandItem.java:134)
- [ ] `GodWandItem.getUseDuration(2)`: its class isn't translated (it extends Minecraft's `Item`) (net/godwand/item/GodWandItem.java:200)
- [ ] `GodWandItem.getUseAnimation(1)`: its class isn't translated (it extends Minecraft's `Item`) (net/godwand/item/GodWandItem.java:205)
- [ ] `GodWandItem.onUseTick(4)`: its class isn't translated (it extends Minecraft's `Item`) (net/godwand/item/GodWandItem.java:210)
- [ ] `GodWandItem.releaseUsing(4)`: its class isn't translated (it extends Minecraft's `Item`) (net/godwand/item/GodWandItem.java:342)
- [ ] `GodWandItem.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `Item`) (net/godwand/item/GodWandItem.java:452)

### `WandSkillLock`

- [x] `WandSkillLock.WandSkillLock(0)` (net/godwand/item/WandSkillLock.java:8)
- [x] `WandSkillLock.isLocked(1)` in game_side.rs (net/godwand/item/WandSkillLock.java:11)
- [ ] `WandSkillLock.lock(2)`: `instanceof GodWandItem` on an item (no single item id for it) (net/godwand/item/WandSkillLock.java:16)
- [ ] `WandSkillLock.unlock(1)`: it uses Minecraft's `DataComponents`.CUSTOM_DATA (net/godwand/item/WandSkillLock.java:26)
- [ ] `AtomicCameraMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:20)
- [ ] `AtomicFacingMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:18)
- [ ] `AtomicGuiMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:12)
- [ ] `AtomicInputMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:15)
- [ ] `AtomicKeyboardMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:14)
- [ ] `AtomicLocalMovementMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:15)
- [ ] `AtomicMovementMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:17)
- [ ] `AtomicPoseCaptureMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:14)
- [ ] `AtomicPoseMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:20)
- [ ] `AtomicToastMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:12)
- [ ] `DisplayLifecycleMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:12)
- [ ] `EntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:14)
- [ ] `ExperienceOrbAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:8)
- [ ] `FlowableFluidMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:20)
- [ ] `GameRendererMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:26)
- [ ] `MinecraftClientMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:11)
- [ ] `PostEffectProcessorAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:10)
- [ ] `PostPassUniformsAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:10)
- [ ] `SkillCooldownUseMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:16)
- [ ] `SoundSystemMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (net/godwand/item/WandSkillLock.java:13)

### `CinematicFxPayload`: it implements Minecraft's `CustomPacketPayload`

- [ ] `CinematicFxPayload.ID`: it uses Minecraft's `Type` (net/godwand/network/CinematicFxPayload.java:11)
- [x] `CinematicFxPayload.EVENT_TRAUMA_SHAKE` (net/godwand/network/CinematicFxPayload.java:12)
- [x] `CinematicFxPayload.EVENT_FOV_PUNCH` (net/godwand/network/CinematicFxPayload.java:13)
- [x] `CinematicFxPayload.EVENT_VACUUM_SILENCE` (net/godwand/network/CinematicFxPayload.java:14)
- [x] `CinematicFxPayload.EVENT_METEOR_FOCUS` (net/godwand/network/CinematicFxPayload.java:15)
- [x] `CinematicFxPayload.EVENT_METEOR_SLAM` (net/godwand/network/CinematicFxPayload.java:16)
- [x] `CinematicFxPayload.EVENT_SUPERNOVA_COMPRESS` (net/godwand/network/CinematicFxPayload.java:17)
- [x] `CinematicFxPayload.EVENT_SUPERNOVA_BURST` (net/godwand/network/CinematicFxPayload.java:18)
- [ ] `CinematicFxPayload.CODEC`: it uses Minecraft's `StreamCodec` (net/godwand/network/CinematicFxPayload.java:19)
- [ ] `CinematicFxPayload.CinematicFxPayload(6)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/network/CinematicFxPayload.java:10)
- [ ] `CinematicFxPayload.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/network/CinematicFxPayload.java:35)
- [ ] `CinematicFxPayload.eventType(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/network/CinematicFxPayload.java:10)
- [ ] `CinematicFxPayload.x(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/network/CinematicFxPayload.java:10)
- [ ] `CinematicFxPayload.y(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/network/CinematicFxPayload.java:10)
- [ ] `CinematicFxPayload.z(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/network/CinematicFxPayload.java:10)
- [ ] `CinematicFxPayload.intensity(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/network/CinematicFxPayload.java:10)
- [ ] `CinematicFxPayload.durationTicks(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/network/CinematicFxPayload.java:10)

### `ImpactFxPayload`: it implements Minecraft's `CustomPacketPayload`

- [ ] `ImpactFxPayload.ID`: it uses Minecraft's `Type` (net/godwand/network/ImpactFxPayload.java:11)
- [x] `ImpactFxPayload.TYPE_DETONATION` (net/godwand/network/ImpactFxPayload.java:12)
- [x] `ImpactFxPayload.TYPE_IMPLOSION` (net/godwand/network/ImpactFxPayload.java:13)
- [x] `ImpactFxPayload.TYPE_COLLAPSE` (net/godwand/network/ImpactFxPayload.java:14)
- [x] `ImpactFxPayload.TYPE_SUPERNOVA` (net/godwand/network/ImpactFxPayload.java:15)
- [x] `ImpactFxPayload.TYPE_BLACK_HOLE` (net/godwand/network/ImpactFxPayload.java:16)
- [x] `ImpactFxPayload.TYPE_SEISMIC_SLAM` (net/godwand/network/ImpactFxPayload.java:17)
- [x] `ImpactFxPayload.TYPE_GRAVITY_CORE` (net/godwand/network/ImpactFxPayload.java:18)
- [x] `ImpactFxPayload.TYPE_REASSEMBLY` (net/godwand/network/ImpactFxPayload.java:19)
- [x] `ImpactFxPayload.TYPE_SUPERNOVA_IMPACT` (net/godwand/network/ImpactFxPayload.java:20)
- [x] `ImpactFxPayload.SUPERNOVA_IMPACT_TICKS` (net/godwand/network/ImpactFxPayload.java:21)
- [ ] `ImpactFxPayload.CODEC`: it uses Minecraft's `StreamCodec` (net/godwand/network/ImpactFxPayload.java:22)
- [ ] `ImpactFxPayload.ImpactFxPayload(6)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/network/ImpactFxPayload.java:10)
- [ ] `ImpactFxPayload.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/network/ImpactFxPayload.java:38)
- [ ] `ImpactFxPayload.x(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/network/ImpactFxPayload.java:10)
- [ ] `ImpactFxPayload.y(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/network/ImpactFxPayload.java:10)
- [ ] `ImpactFxPayload.z(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/network/ImpactFxPayload.java:10)
- [ ] `ImpactFxPayload.fxType(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/network/ImpactFxPayload.java:10)
- [ ] `ImpactFxPayload.durationTicks(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/network/ImpactFxPayload.java:10)
- [ ] `ImpactFxPayload.intensity(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (net/godwand/network/ImpactFxPayload.java:10)

### `INetworkHelper`: it's an interface

- [ ] `INetworkHelper.sendToPlayer(2)`: it has no body (net/godwand/platform/INetworkHelper.java:9)
- [ ] `INetworkHelper.sendToNearby(4)`: it has no body (net/godwand/platform/INetworkHelper.java:11)
- [ ] `INetworkHelper.sendToServer(1)`: it has no body (net/godwand/platform/INetworkHelper.java:13)

### `IPlatformHelper`: it's an interface

- [ ] `IPlatformHelper.getPlatformName(0)`: it has no body (net/godwand/platform/IPlatformHelper.java:6)
- [ ] `IPlatformHelper.isModLoaded(1)`: it has no body (net/godwand/platform/IPlatformHelper.java:8)
- [ ] `IPlatformHelper.isDevelopmentEnvironment(0)`: it has no body (net/godwand/platform/IPlatformHelper.java:10)
- [ ] `IPlatformHelper.getConfigDirectory(0)`: it has no body (net/godwand/platform/IPlatformHelper.java:12)

### `Services`

- [ ] `Services.PLATFORM`: it uses IPlatformHelper, which isn't translated (it's an interface) (net/godwand/platform/Services.java:6)
- [ ] `Services.NETWORK`: it uses INetworkHelper, which isn't translated (it's an interface) (net/godwand/platform/Services.java:7)
- [ ] `Services.load(1)`: it's generic (net/godwand/platform/Services.java:9)

### `AtomicCrater`: it keeps an atomic value in an object's field

- [ ] `AtomicCrater.AtomicCrater(2)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/skill/AtomicCrater.java:24)
- [x] `AtomicCrater.terrainRadius(1)` (net/godwand/skill/AtomicCrater.java:32)
- [ ] `AtomicCrater.finished(0)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/skill/AtomicCrater.java:36)
- [ ] `AtomicCrater.carve(2)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/skill/AtomicCrater.java:40)
- [ ] `AtomicCrater.carve(3)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/skill/AtomicCrater.java:44)
- [ ] `AtomicCrater.carve(4)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/skill/AtomicCrater.java:48)
- [ ] `AtomicCrater.loadedChunk(4)`: it uses Minecraft's `LevelChunk` (net/godwand/skill/AtomicCrater.java:129)

### `AtomicManager`

- [ ] `AtomicManager.TASKS`: it uses `UUID` (java.util) (net/godwand/skill/AtomicManager.java:27)
- [ ] `AtomicManager.invulnerable(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/AtomicManager.java:29)
- [ ] `AtomicManager.movementLocked(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/AtomicManager.java:37)
- [ ] `AtomicManager.releaseMovement(2)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/AtomicManager.java:47)
- [ ] `AtomicManager.beginCharge(2)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/AtomicManager.java:62)
- [ ] `AtomicManager.arm(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/AtomicManager.java:75)
- [ ] `AtomicManager.cancelCharge(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/AtomicManager.java:92)
- [ ] `AtomicManager.releaseCharge(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/AtomicManager.java:100)
- [ ] `AtomicManager.tick(1)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/AtomicManager.java:112)
- [ ] `AtomicManager.damage(1)`: a nullable object kept in a changing static (net/godwand/skill/AtomicManager.java:259)
- [ ] `AtomicManager.broadcast(2)`: it uses Task, which isn't translated (it keeps an atomic value in an object's field) (net/godwand/skill/AtomicManager.java:263)
- [ ] `AtomicManager.clearAllTasks(0)`: it uses AtomicManager.TASKS, which isn't translated (it uses `UUID` (java.util)) (net/godwand/skill/AtomicManager.java:273)

### `AtomicManager.Task`: it keeps an atomic value in an object's field

- [ ] `AtomicManager.Task.Task(2)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/skill/AtomicManager.java:292)

### `AtomicSequence`

- [x] `AtomicSequence.BEAM_RETRACTION` (net/godwand/skill/AtomicSequence.java:7)
- [x] `AtomicSequence.CHARGE_TICKS` (net/godwand/skill/AtomicSequence.java:8)
- [x] `AtomicSequence.WAIT_TICK` (net/godwand/skill/AtomicSequence.java:9)
- [x] `AtomicSequence.DETONATION` (net/godwand/skill/AtomicSequence.java:10)
- [x] `AtomicSequence.END` (net/godwand/skill/AtomicSequence.java:11)
- [x] `AtomicSequence.MAX_RADIUS` (net/godwand/skill/AtomicSequence.java:12)
- [x] `AtomicSequence.CAGE_RADIUS` (net/godwand/skill/AtomicSequence.java:13)
- [x] `AtomicSequence.ATOMIC_WORD` (net/godwand/skill/AtomicSequence.java:14)
- [x] `AtomicSequence.BEAM_HEIGHT` (net/godwand/skill/AtomicSequence.java:15)
- [x] `AtomicSequence.BEAM_DEPTH` (net/godwand/skill/AtomicSequence.java:16)
- [x] `AtomicSequence.CRATER_DEPTH` (net/godwand/skill/AtomicSequence.java:17)
- [x] `AtomicSequence.SCAR_FADE_START` (net/godwand/skill/AtomicSequence.java:18)
- [x] `AtomicSequence.SCAR_END` (net/godwand/skill/AtomicSequence.java:19)
- [x] `AtomicSequence.phase(3)` (net/godwand/skill/AtomicSequence.java:21)
- [x] `AtomicSequence.castAge(3)` (net/godwand/skill/AtomicSequence.java:26)
- [x] `AtomicSequence.releaseTime(2)` (net/godwand/skill/AtomicSequence.java:30)
- [x] `AtomicSequence.heartbeat(1)` (net/godwand/skill/AtomicSequence.java:34)
- [x] `AtomicSequence.holdIdle(1)` (net/godwand/skill/AtomicSequence.java:45)
- [x] `AtomicSequence.releaseFlash(1)` (net/godwand/skill/AtomicSequence.java:49)
- [x] `AtomicSequence.charge(1)` (net/godwand/skill/AtomicSequence.java:53)
- [x] `AtomicSequence.visibility(1)` (net/godwand/skill/AtomicSequence.java:57)
- [x] `AtomicSequence.compression(1)` (net/godwand/skill/AtomicSequence.java:61)
- [x] `AtomicSequence.blackout(1)` (net/godwand/skill/AtomicSequence.java:66)
- [x] `AtomicSequence.circuitProgress(2)` (net/godwand/skill/AtomicSequence.java:70)
- [x] `AtomicSequence.junctionWaveRadius(1)` (net/godwand/skill/AtomicSequence.java:74)
- [x] `AtomicSequence.armRaise(1)` (net/godwand/skill/AtomicSequence.java:78)
- [x] `AtomicSequence.armSpread(1)` (net/godwand/skill/AtomicSequence.java:82)
- [x] `AtomicSequence.armWindup(1)` (net/godwand/skill/AtomicSequence.java:86)
- [x] `AtomicSequence.facing(1)` (net/godwand/skill/AtomicSequence.java:90)
- [ ] `AtomicSequence.blendHeadYaw(3)`: it calls Minecraft's `Mth`.rotLerp (net/godwand/skill/AtomicSequence.java:94)
- [ ] `AtomicSequence.headLook(2)`: it creates Minecraft's `Vec3` (net/godwand/skill/AtomicSequence.java:100)
- [ ] `AtomicSequence.wideFocus(2)`: it creates Minecraft's `Vec3` (net/godwand/skill/AtomicSequence.java:112)
- [x] `AtomicSequence.transitionOpacity(1)` (net/godwand/skill/AtomicSequence.java:120)
- [x] `AtomicSequence.armPitch(1)` (net/godwand/skill/AtomicSequence.java:130)
- [x] `AtomicSequence.holoRadius(1)` (net/godwand/skill/AtomicSequence.java:137)
- [x] `AtomicSequence.shockRadius(1)` (net/godwand/skill/AtomicSequence.java:141)
- [x] `AtomicSequence.beamRadius(1)` (net/godwand/skill/AtomicSequence.java:145)
- [x] `AtomicSequence.beamHeight(1)` (net/godwand/skill/AtomicSequence.java:149)
- [x] `AtomicSequence.blastPulse(1)` (net/godwand/skill/AtomicSequence.java:153)
- [x] `AtomicSequence.scarOpacity(1)` (net/godwand/skill/AtomicSequence.java:157)
- [x] `AtomicSequence.protectedDuringCast(1)` (net/godwand/skill/AtomicSequence.java:161)
- [ ] `AtomicSequence.columnOccludes(3)`: it calls Minecraft's Vec3.subtract (no Horizon helper for it yet) (net/godwand/skill/AtomicSequence.java:165)
- [x] `AtomicSequence.letterbox(1)` (net/godwand/skill/AtomicSequence.java:207)
- [x] `AtomicSequence.returnBlend(1)` (net/godwand/skill/AtomicSequence.java:211)
- [x] `AtomicSequence.movementLocked(1)` (net/godwand/skill/AtomicSequence.java:215)
- [x] `AtomicSequence.activation(1)` (net/godwand/skill/AtomicSequence.java:219)
- [x] `AtomicSequence.impact(1)` (net/godwand/skill/AtomicSequence.java:223)
- [ ] `AtomicSequence.groundTracePoint(2)`: it calls AtomicSequence.polar, which isn't translated (net/godwand/skill/AtomicSequence.java:227)
- [ ] `AtomicSequence.polar(3)`: it creates Minecraft's `Vec3` (net/godwand/skill/AtomicSequence.java:235)
- [ ] `AtomicSequence.convergencePoint(4)`: it calls AtomicSequence.polar, which isn't translated (net/godwand/skill/AtomicSequence.java:239)
- [x] `AtomicSequence.moteNoise(2)` (net/godwand/skill/AtomicSequence.java:251)
- [ ] `AtomicSequence.risingMote(2)`: it calls AtomicSequence.polar, which isn't translated (net/godwand/skill/AtomicSequence.java:256)
- [x] `AtomicSequence.moteClock(1)` (net/godwand/skill/AtomicSequence.java:262)
- [ ] `AtomicSequence.releaseMote(2)`: it calls Minecraft's Vec3.add (no Horizon helper for it yet) (net/godwand/skill/AtomicSequence.java:266)
- [x] `AtomicSequence.fortressWaveHeight(1)` (net/godwand/skill/AtomicSequence.java:272)
- [x] `AtomicSequence.releaseRadius(1)` (net/godwand/skill/AtomicSequence.java:281)
- [x] `AtomicSequence.cameraBudget(1)` (net/godwand/skill/AtomicSequence.java:285)
- [x] `AtomicSequence.cameraBudget(2)` (net/godwand/skill/AtomicSequence.java:289)
- [x] `AtomicSequence.pressure(1)` (net/godwand/skill/AtomicSequence.java:293)
- [x] `AtomicSequence.pressureWaveRadius(2)` (net/godwand/skill/AtomicSequence.java:297)
- [x] `AtomicSequence.pressureCrosses(3)` (net/godwand/skill/AtomicSequence.java:306)
- [x] `AtomicSequence.pressureShake(2)` (net/godwand/skill/AtomicSequence.java:316)
- [x] `AtomicSequence.shake(1)` (net/godwand/skill/AtomicSequence.java:329)
- [ ] `AtomicSequence.shot(2)`: it uses Shot, which isn't translated (it uses Minecraft's `Vec3`) (net/godwand/skill/AtomicSequence.java:347)

### `AtomicSequence.Shot`: it uses Minecraft's `Vec3`

- [ ] `AtomicSequence.Shot.Shot(3)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/skill/AtomicSequence.java:433)
- [ ] `AtomicSequence.Shot.offset(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/skill/AtomicSequence.java:433)
- [ ] `AtomicSequence.Shot.focus(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/skill/AtomicSequence.java:433)
- [ ] `AtomicSequence.Shot.fov(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (net/godwand/skill/AtomicSequence.java:433)

### `AtomicTerrainPlanner`

- [ ] `AtomicTerrainPlanner.WORKER`: it uses `ExecutorService` (java.util.concurrent) (net/godwand/skill/AtomicTerrainPlanner.java:14)
- [ ] `AtomicTerrainPlanner.request(1)`: it calls AtomicTerrainPlanner.request, which isn't translated (net/godwand/skill/AtomicTerrainPlanner.java:20)
- [ ] `AtomicTerrainPlanner.request(2)`: a try/catch (RejectedExecutionException) (net/godwand/skill/AtomicTerrainPlanner.java:24)
- [ ] `AtomicTerrainPlanner.plan(1)`: it calls AtomicTerrainPlanner.plan, which isn't translated (net/godwand/skill/AtomicTerrainPlanner.java:32)
- [ ] `AtomicTerrainPlanner.plan(2)`: it uses `IntArrayList` (it.unimi.dsi.fastutil.ints) (net/godwand/skill/AtomicTerrainPlanner.java:36)

### `AtomicTerrainPlanner.Columns`

- [x] `AtomicTerrainPlanner.Columns.Columns(2)` (net/godwand/skill/AtomicTerrainPlanner.java:60)
- [x] `AtomicTerrainPlanner.Columns.offsets(0)` (net/godwand/skill/AtomicTerrainPlanner.java:60)
- [x] `AtomicTerrainPlanner.Columns.distances(0)` (net/godwand/skill/AtomicTerrainPlanner.java:60)

### `FugaCrater`: it keeps an atomic value in an object's field

- [x] `FugaCrater.SPIRES` (net/godwand/skill/FugaCrater.java:26)
- [x] `FugaCrater.PRESS_SINK` (net/godwand/skill/FugaCrater.java:28)
- [x] `FugaCrater.PRESS_LIP` (net/godwand/skill/FugaCrater.java:29)
- [ ] `FugaCrater.FugaCrater(2)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/skill/FugaCrater.java:34)
- [ ] `FugaCrater.finished(0)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/skill/FugaCrater.java:42)
- [x] `FugaCrater.depth(3)` (net/godwand/skill/FugaCrater.java:46)
- [x] `FugaCrater.reach(1)` (net/godwand/skill/FugaCrater.java:58)
- [x] `FugaCrater.blast(3)` (net/godwand/skill/FugaCrater.java:62)
- [x] `FugaCrater.scour(3)` (net/godwand/skill/FugaCrater.java:66)
- [ ] `FugaCrater.carve(4)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/skill/FugaCrater.java:72)
- [x] `FugaCrater.spire(2)` (net/godwand/skill/FugaCrater.java:175)
- [ ] `FugaCrater.upheave(3)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/skill/FugaCrater.java:186)
- [x] `FugaCrater.pressShape(3)` (net/godwand/skill/FugaCrater.java:242)
- [ ] `FugaCrater.press(3)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/skill/FugaCrater.java:255)
- [ ] `FugaCrater.scorch(5)`: its class isn't translated (it keeps an atomic value in an object's field) (net/godwand/skill/FugaCrater.java:334)

### `FugaManager`

- [ ] `FugaManager.TASKS`: it uses `UUID` (java.util) (net/godwand/skill/FugaManager.java:45)
- [ ] `FugaManager.SETTLING`: it uses Settling, which isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/FugaManager.java:46)
- [x] `FugaManager.FORCE_PER_TICK` (net/godwand/skill/FugaManager.java:47)
- [x] `FugaManager.FugaManager(0)` (net/godwand/skill/FugaManager.java:49)
- [ ] `FugaManager.beginCharge(2)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/FugaManager.java:52)
- [ ] `FugaManager.movementLocked(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/FugaManager.java:70)
- [ ] `FugaManager.invulnerable(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/FugaManager.java:75)
- [ ] `FugaManager.cancelCharge(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/FugaManager.java:80)
- [ ] `FugaManager.releaseCharge(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/FugaManager.java:87)
- [ ] `FugaManager.invalid(1)`: it uses Task, which isn't translated (it uses Minecraft's `ServerPlayer`) (net/godwand/skill/FugaManager.java:105)
- [ ] `FugaManager.tick(1)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/FugaManager.java:114)
- [ ] `FugaManager.settle(3)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/FugaManager.java:181)
- [ ] `FugaManager.advance(1)`: it uses Task, which isn't translated (it uses Minecraft's `ServerPlayer`) (net/godwand/skill/FugaManager.java:198)
- [ ] `FugaManager.loadedEndpoint(3)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/FugaManager.java:242)
- [ ] `FugaManager.loaded(2)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/FugaManager.java:283)
- [ ] `FugaManager.damage(2)`: it uses Task, which isn't translated (it uses Minecraft's `ServerPlayer`) (net/godwand/skill/FugaManager.java:288)
- [ ] `FugaManager.slow(1)`: it uses Task, which isn't translated (it uses Minecraft's `ServerPlayer`) (net/godwand/skill/FugaManager.java:307)
- [ ] `FugaManager.hit(3)`: it uses Task, which isn't translated (it uses Minecraft's `ServerPlayer`) (net/godwand/skill/FugaManager.java:321)
- [ ] `FugaManager.broadcast(2)`: it uses Task, which isn't translated (it uses Minecraft's `ServerPlayer`) (net/godwand/skill/FugaManager.java:331)
- [ ] `FugaManager.clearAllTasks(0)`: it uses FugaManager.TASKS, which isn't translated (it uses `UUID` (java.util)) (net/godwand/skill/FugaManager.java:353)
- [ ] `FugaManager.forceOrder(2)`: it uses Minecraft's `BlockPos` (net/godwand/skill/FugaManager.java:366)
- [ ] `FugaManager.forceSome(4)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/FugaManager.java:390)
- [ ] `FugaManager.release(2)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/FugaManager.java:399)
- [ ] `FugaManager.outflow(2)`: it uses Task, which isn't translated (it uses Minecraft's `ServerPlayer`) (net/godwand/skill/FugaManager.java:407)

### `FugaManager.Settling`: it uses Minecraft's `ServerLevel`

- [ ] `FugaManager.Settling.Settling(4)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/FugaManager.java:461)
- [ ] `FugaManager.Settling.world(0)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/FugaManager.java:461)
- [ ] `FugaManager.Settling.crater(0)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/FugaManager.java:461)
- [ ] `FugaManager.Settling.impactAt(0)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/FugaManager.java:461)
- [ ] `FugaManager.Settling.forced(0)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/FugaManager.java:461)

### `FugaManager.Task`: it uses Minecraft's `ServerPlayer`

- [ ] `FugaManager.Task.Task(2)`: its class isn't translated (it uses Minecraft's `ServerPlayer`) (net/godwand/skill/FugaManager.java:481)

### `FugaSequence`

- [x] `FugaSequence.CHARGE_TICKS` (net/godwand/skill/FugaSequence.java:6)
- [x] `FugaSequence.ORB_TICKS` (net/godwand/skill/FugaSequence.java:7)
- [x] `FugaSequence.RETRACT_TICK` (net/godwand/skill/FugaSequence.java:8)
- [x] `FugaSequence.END_TICK` (net/godwand/skill/FugaSequence.java:9)
- [x] `FugaSequence.COLUMN_HEIGHT` (net/godwand/skill/FugaSequence.java:10)
- [x] `FugaSequence.SETTLE_TICKS` (net/godwand/skill/FugaSequence.java:11)
- [x] `FugaSequence.SPEED` (net/godwand/skill/FugaSequence.java:12)
- [x] `FugaSequence.MAX_RANGE` (net/godwand/skill/FugaSequence.java:13)
- [x] `FugaSequence.RADIUS` (net/godwand/skill/FugaSequence.java:14)
- [x] `FugaSequence.TRANSITION_TICKS` (net/godwand/skill/FugaSequence.java:15)
- [x] `FugaSequence.FLIGHT_EASE` (net/godwand/skill/FugaSequence.java:16)
- [x] `FugaSequence.FugaSequence(0)` (net/godwand/skill/FugaSequence.java:18)
- [x] `FugaSequence.chargeAge(1)` (net/godwand/skill/FugaSequence.java:21)
- [x] `FugaSequence.visualAge(1)` (net/godwand/skill/FugaSequence.java:25)
- [x] `FugaSequence.movementLocked(1)` (net/godwand/skill/FugaSequence.java:29)
- [x] `FugaSequence.ready(2)` (net/godwand/skill/FugaSequence.java:33)
- [x] `FugaSequence.impactAge(2)` (net/godwand/skill/FugaSequence.java:37)
- [x] `FugaSequence.terrainActive(1)` (net/godwand/skill/FugaSequence.java:41)
- [x] `FugaSequence.columnRadius(1)` (net/godwand/skill/FugaSequence.java:45)
- [ ] `FugaSequence.launchTarget(2)`: it calls Minecraft's Vec3.normalize (no Horizon helper for it yet) (net/godwand/skill/FugaSequence.java:49)
- [x] `FugaSequence.flightDistance(1)` (net/godwand/skill/FugaSequence.java:64)
- [ ] `FugaSequence.arrowPosition(3)`: it calls Minecraft's Vec3.distanceTo (no Horizon helper for it yet) (net/godwand/skill/FugaSequence.java:68)

### `RatioCollapseManager`

- [x] `RatioCollapseManager.ASSEMBLY_END` (net/godwand/skill/RatioCollapseManager.java:44)
- [x] `RatioCollapseManager.SPIN_END` (net/godwand/skill/RatioCollapseManager.java:45)
- [x] `RatioCollapseManager.IMPACT_TICK` (net/godwand/skill/RatioCollapseManager.java:46)
- [x] `RatioCollapseManager.SEQUENCE_END` (net/godwand/skill/RatioCollapseManager.java:47)
- [ ] `RatioCollapseManager.ACTIVE_TASKS`: it uses `Queue` (java.util) (net/godwand/skill/RatioCollapseManager.java:48)
- [ ] `RatioCollapseManager.cast(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/RatioCollapseManager.java:50)
- [ ] `RatioCollapseManager.cast(2)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/RatioCollapseManager.java:54)
- [ ] `RatioCollapseManager.launch(4)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/RatioCollapseManager.java:119)
- [ ] `RatioCollapseManager.launch(9)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/RatioCollapseManager.java:123)
- [ ] `RatioCollapseManager.launch(10)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/RatioCollapseManager.java:137)
- [ ] `RatioCollapseManager.computeBoulderClusterOffset(3)`: it uses Minecraft's `Vec3`.ZERO (net/godwand/skill/RatioCollapseManager.java:241)
- [ ] `RatioCollapseManager.computeSpiralDrillOffset(4)`: it uses Minecraft's `Vec3`.ZERO (net/godwand/skill/RatioCollapseManager.java:252)
- [ ] `RatioCollapseManager.tick(1)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/RatioCollapseManager.java:270)
- [ ] `RatioCollapseManager.carveCrater(1)`: it uses RatioCollapseTask, which isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/RatioCollapseManager.java:642)
- [x] `RatioCollapseManager.drillCraterDepth(4)` (net/godwand/skill/RatioCollapseManager.java:697)
- [x] `RatioCollapseManager.drillHoverHeight(1)` (net/godwand/skill/RatioCollapseManager.java:703)
- [x] `RatioCollapseManager.drillWindVisibility(1)` (net/godwand/skill/RatioCollapseManager.java:707)
- [x] `RatioCollapseManager.drillHeat(1)` (net/godwand/skill/RatioCollapseManager.java:712)
- [ ] `RatioCollapseManager.sendDrillWind(3)`: it uses RatioCollapseTask, which isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/RatioCollapseManager.java:716)
- [ ] `RatioCollapseManager.getActiveTaskCount(0)`: it uses RatioCollapseManager.ACTIVE_TASKS, which isn't translated (it uses `Queue` (java.util)) (net/godwand/skill/RatioCollapseManager.java:721)
- [ ] `RatioCollapseManager.clearAllTasks(0)`: it uses RatioCollapseManager.ACTIVE_TASKS, which isn't translated (it uses `Queue` (java.util)) (net/godwand/skill/RatioCollapseManager.java:725)

### `RatioCollapseManager.RatioCollapseTask`: it uses Minecraft's `ServerLevel`

- [ ] `RatioCollapseManager.RatioCollapseTask.RatioCollapseTask(9)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/RatioCollapseManager.java:755)

### `ShockwaveSlicer`

- [ ] `ShockwaveSlicer.ACTIVE_TASKS`: it uses `Queue` (java.util) (net/godwand/skill/ShockwaveSlicer.java:51)
- [ ] `ShockwaveSlicer.isExposedSurfaceBlock(2)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/ShockwaveSlicer.java:53)
- [ ] `ShockwaveSlicer.queueCalamityDetonation(3)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/ShockwaveSlicer.java:64)
- [ ] `ShockwaveSlicer.queueCalamityDetonation(4)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/ShockwaveSlicer.java:68)
- [ ] `ShockwaveSlicer.spawnThreeOrbitalParticleRings(4)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/ShockwaveSlicer.java:201)
- [ ] `ShockwaveSlicer.hasActiveTask(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/ShockwaveSlicer.java:237)
- [ ] `ShockwaveSlicer.triggerEarlySnap(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/ShockwaveSlicer.java:247)
- [ ] `ShockwaveSlicer.snapTask(1)`: it uses SingularityTask, which isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/ShockwaveSlicer.java:258)
- [ ] `ShockwaveSlicer.tick(1)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/ShockwaveSlicer.java:316)
- [x] `ShockwaveSlicer.returnProgress(1)` (net/godwand/skill/ShockwaveSlicer.java:462)
- [ ] `ShockwaveSlicer.animateReturn(2)`: it uses SingularityTask, which isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/ShockwaveSlicer.java:467)
- [ ] `ShockwaveSlicer.executeRestoration(1)`: it uses SingularityTask, which isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/ShockwaveSlicer.java:487)
- [ ] `ShockwaveSlicer.updateBlockPositions(3)`: it uses SingularityTask, which isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/ShockwaveSlicer.java:571)
- [ ] `ShockwaveSlicer.sendThreadFx(2)`: it uses SingularityTask, which isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/ShockwaveSlicer.java:619)
- [ ] `ShockwaveSlicer.getActiveTaskCount(0)`: it uses ShockwaveSlicer.ACTIVE_TASKS, which isn't translated (it uses `Queue` (java.util)) (net/godwand/skill/ShockwaveSlicer.java:634)
- [ ] `ShockwaveSlicer.updateBeam(4)`: it uses SingularityTask, which isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/ShockwaveSlicer.java:638)
- [ ] `ShockwaveSlicer.clearBeams(1)`: it uses SingularityTask, which isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/ShockwaveSlicer.java:650)
- [ ] `ShockwaveSlicer.clearAllTasks(0)`: it uses ShockwaveSlicer.ACTIVE_TASKS, which isn't translated (it uses `Queue` (java.util)) (net/godwand/skill/ShockwaveSlicer.java:660)

### `ShockwaveSlicer.SingularityTask`: it uses Minecraft's `ServerLevel`

- [ ] `ShockwaveSlicer.SingularityTask.SingularityTask(4)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/ShockwaveSlicer.java:687)
- [ ] `ShockwaveSlicer.SingularityTask.SingularityTask(5)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/ShockwaveSlicer.java:691)

### `SkillCooldowns`

- [ ] `SkillCooldowns.STATES`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SkillCooldowns.java:13)
- [x] `SkillCooldowns.SkillCooldowns(0)` (net/godwand/skill/SkillCooldowns.java:15)
- [ ] `SkillCooldowns.state(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SkillCooldowns.java:18)
- [ ] `SkillCooldowns.reset(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SkillCooldowns.java:29)
- [ ] `SkillCooldowns.ready(2)`: `instanceof` (net/godwand/skill/SkillCooldowns.java:37)
- [ ] `SkillCooldowns.start(3)`: `instanceof` (net/godwand/skill/SkillCooldowns.java:54)
- [ ] `SkillCooldowns.sync(2)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SkillCooldowns.java:61)

### `SkillCooldowns.Entry`: it uses `WeakReference` (java.lang.ref)

- [ ] `SkillCooldowns.Entry.Entry(2)`: its class isn't translated (it uses `WeakReference` (java.lang.ref)) (net/godwand/skill/SkillCooldowns.java:72)
- [ ] `SkillCooldowns.Entry.owner(0)`: its class isn't translated (it uses `WeakReference` (java.lang.ref)) (net/godwand/skill/SkillCooldowns.java:72)
- [ ] `SkillCooldowns.Entry.timers(0)`: its class isn't translated (it uses `WeakReference` (java.lang.ref)) (net/godwand/skill/SkillCooldowns.java:72)

### `SkillCooldownState`

- [x] `SkillCooldownState.MAX_TICKS` (net/godwand/skill/SkillCooldownState.java:4)
- [x] `SkillCooldownState.set(4)` (net/godwand/skill/SkillCooldownState.java:8)
- [x] `SkillCooldownState.remaining(2)` (net/godwand/skill/SkillCooldownState.java:17)
- [x] `SkillCooldownState.duration(1)` (net/godwand/skill/SkillCooldownState.java:21)
- [x] `SkillCooldownState.fraction(2)` (net/godwand/skill/SkillCooldownState.java:25)

### `SpatialBridge`

- [ ] `SpatialBridge.ENABLED`: it uses `UUID` (java.util) (net/godwand/skill/SpatialBridge.java:32)
- [ ] `SpatialBridge.PRESSED`: it uses `UUID` (java.util) (net/godwand/skill/SpatialBridge.java:33)
- [ ] `SpatialBridge.ELIGIBLE`: it uses `UUID` (java.util) (net/godwand/skill/SpatialBridge.java:34)
- [ ] `SpatialBridge.FLIGHTS`: it uses Flight, which isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/SpatialBridge.java:35)
- [ ] `SpatialBridge.LEVEL`: it uses `UUID` (java.util) (net/godwand/skill/SpatialBridge.java:36)
- [ ] `SpatialBridge.resetRuntime(0)`: it uses SpatialBridge.ENABLED, which isn't translated (it uses `UUID` (java.util)) (net/godwand/skill/SpatialBridge.java:38)
- [ ] `SpatialBridge.disable(1)`: it uses `UUID` (java.util) (net/godwand/skill/SpatialBridge.java:46)
- [x] `SpatialBridge.raisedLevel(2)` (net/godwand/skill/SpatialBridge.java:53)
- [ ] `SpatialBridge.plan(3)`: it uses Minecraft's `BlockPos` (net/godwand/skill/SpatialBridge.java:57)
- [ ] `SpatialBridge.selected(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SpatialBridge.java:72)
- [ ] `SpatialBridge.toggle(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SpatialBridge.java:82)
- [ ] `SpatialBridge.tickPlayer(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SpatialBridge.java:97)
- [ ] `SpatialBridge.usable(3)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/SpatialBridge.java:244)
- [ ] `SpatialBridge.occupied(2)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/SpatialBridge.java:252)
- [ ] `SpatialBridge.clearVisuals(1)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/SpatialBridge.java:257)
- [ ] `SpatialBridge.tickWorld(1)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/SpatialBridge.java:268)

### `SpatialBridge.Flight`: it uses Minecraft's `ServerLevel`

- [ ] `SpatialBridge.Flight.Flight(4)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/SpatialBridge.java:321)
- [ ] `SpatialBridge.Flight.world(0)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/SpatialBridge.java:321)
- [ ] `SpatialBridge.Flight.display(0)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/SpatialBridge.java:321)
- [ ] `SpatialBridge.Flight.target(0)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/SpatialBridge.java:321)
- [ ] `SpatialBridge.Flight.born(0)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/SpatialBridge.java:321)

### `SpatialPathWalker`

- [ ] `SpatialPathWalker.LAST_SHIELD_SOUND`: it uses `UUID` (java.util) (net/godwand/skill/SpatialPathWalker.java:47)
- [ ] `SpatialPathWalker.ACTIVE_PLAYERS`: it uses `UUID` (java.util) (net/godwand/skill/SpatialPathWalker.java:48)
- [ ] `SpatialPathWalker.LAST_GROUND_Y`: it uses `UUID` (java.util) (net/godwand/skill/SpatialPathWalker.java:49)
- [ ] `SpatialPathWalker.LAST_PLAYER_POS`: it uses `UUID` (java.util) (net/godwand/skill/SpatialPathWalker.java:50)
- [ ] `SpatialPathWalker.CURRENT_FORWARD_LEAD`: it uses `UUID` (java.util) (net/godwand/skill/SpatialPathWalker.java:51)
- [ ] `SpatialPathWalker.CURRENT_PLAYER_SPEED`: it uses `UUID` (java.util) (net/godwand/skill/SpatialPathWalker.java:52)
- [ ] `SpatialPathWalker.isActive(1)`: it uses SpatialPathWalker.ACTIVE_PLAYERS, which isn't translated (it uses `UUID` (java.util)) (net/godwand/skill/SpatialPathWalker.java:54)
- [ ] `SpatialPathWalker.toggle(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SpatialPathWalker.java:58)
- [ ] `SpatialPathWalker.hasGodWand(1)`: it uses GodWandMod.GOD_WAND_ITEM, which isn't translated (it uses Minecraft's `Item`) (net/godwand/skill/SpatialPathWalker.java:114)
- [ ] `SpatialPathWalker.resetRuntime(0)`: it uses SpatialPathWalker.ACTIVE_PLAYERS, which isn't translated (it uses `UUID` (java.util)) (net/godwand/skill/SpatialPathWalker.java:128)
- [ ] `SpatialPathWalker.deactivate(1)`: it calls SpatialBridge.disable, which isn't translated (net/godwand/skill/SpatialPathWalker.java:137)
- [ ] `SpatialPathWalker.syncAura(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SpatialPathWalker.java:153)
- [ ] `SpatialPathWalker.shouldPlayShieldSound(2)`: it uses `UUID` (java.util) (net/godwand/skill/SpatialPathWalker.java:158)
- [ ] `SpatialPathWalker.sendAuraHit(2)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SpatialPathWalker.java:168)
- [ ] `SpatialPathWalker.deflectProjectiles(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SpatialPathWalker.java:173)
- [ ] `SpatialPathWalker.repelMobs(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SpatialPathWalker.java:212)
- [ ] `SpatialPathWalker.updatePlayerForwardLead(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SpatialPathWalker.java:248)
- [ ] `SpatialPathWalker.getPlayerSpeed(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SpatialPathWalker.java:265)
- [ ] `SpatialPathWalker.getPlayerForwardLead(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SpatialPathWalker.java:269)
- [ ] `SpatialPathWalker.tickPlayer(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SpatialPathWalker.java:273)

### `SpellDamage`

- [ ] `SpellDamage.TYPE`: it uses Minecraft's `ResourceKey` (net/godwand/skill/SpellDamage.java:33)
- [x] `SpellDamage.WORM` (net/godwand/skill/SpellDamage.java:34)
- [ ] `SpellDamage.WARNED`: it uses `Class` (net/godwand/skill/SpellDamage.java:35)
- [ ] `SpellDamage.KILLS`: it uses Minecraft's `ServerLevel` (net/godwand/skill/SpellDamage.java:36)
- [ ] `SpellDamage.isCaster(2)`: it uses Minecraft's `Entity` (net/godwand/skill/SpellDamage.java:38)
- [ ] `SpellDamage.hurt(3)`: it uses Minecraft's `Entity` (net/godwand/skill/SpellDamage.java:42)
- [ ] `SpellDamage.noteKill(2)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/SpellDamage.java:81)
- [ ] `SpellDamage.poolExperience(1)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/SpellDamage.java:86)
- [ ] `SpellDamage.wormPart(1)`: it uses Minecraft's `Entity` (net/godwand/skill/SpellDamage.java:127)
- [ ] `SpellDamage.wormOwner(1)`: it uses Minecraft's `Entity` (net/godwand/skill/SpellDamage.java:138)
- [ ] `SpellDamage.identity(1)`: it uses `UUID` (java.util) (net/godwand/skill/SpellDamage.java:151)
- [ ] `SpellDamage.extraTargets(2)`: it uses Minecraft's `Entity` (net/godwand/skill/SpellDamage.java:156)
- [ ] `SpellDamage.hurtExtras(4)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/SpellDamage.java:160)
- [ ] `SpellDamage.warn(2)`: it uses Minecraft's `Entity` (net/godwand/skill/SpellDamage.java:172)

### `StoneCannonManager`

- [x] `StoneCannonManager.calculateChargeLevel(1)` (net/godwand/skill/StoneCannonManager.java:40)
- [x] `StoneCannonManager.calculateDirectDamage(1)` (net/godwand/skill/StoneCannonManager.java:50)
- [x] `StoneCannonManager.calculateCraterRadius(1)` (net/godwand/skill/StoneCannonManager.java:59)
- [x] `StoneCannonManager.calculateSpeed(1)` (net/godwand/skill/StoneCannonManager.java:68)
- [x] `StoneCannonManager.calculateRecoil(1)` (net/godwand/skill/StoneCannonManager.java:77)
- [ ] `StoneCannonManager.insideDrillSweep(5)`: it calls Minecraft's Vec3.subtract (no Horizon helper for it yet) (net/godwand/skill/StoneCannonManager.java:86)
- [ ] `StoneCannonManager.excavateImpact(3)`: its body couldn't be read: an anonymous class (net/godwand/skill/StoneCannonManager.java:104)
- [x] `StoneCannonManager.chargeStoneVolume(1)` (net/godwand/skill/StoneCannonManager.java:181)
- [ ] `StoneCannonManager.tickCharge(3)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/StoneCannonManager.java:185)
- [ ] `StoneCannonManager.fire(2)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/StoneCannonManager.java:250)
- [ ] `StoneCannonManager.onProjectileImpact(2)`: it uses StoneCannonProjectileEntity, which isn't translated (it extends Minecraft's `Projectile`) (net/godwand/skill/StoneCannonManager.java:272)

### `SupernovaCraterCursor`

- [x] `SupernovaCraterCursor.SupernovaCraterCursor(1)` (net/godwand/skill/SupernovaCraterCursor.java:11)
- [x] `SupernovaCraterCursor.x(0)` (net/godwand/skill/SupernovaCraterCursor.java:15)
- [x] `SupernovaCraterCursor.z(0)` (net/godwand/skill/SupernovaCraterCursor.java:19)
- [x] `SupernovaCraterCursor.finished(0)` (net/godwand/skill/SupernovaCraterCursor.java:23)
- [x] `SupernovaCraterCursor.advance(0)` (net/godwand/skill/SupernovaCraterCursor.java:27)

### `SupernovaManager`

- [x] `SupernovaManager.DETONATION_TICK` (net/godwand/skill/SupernovaManager.java:53)
- [x] `SupernovaManager.IMPACT_TICK` (net/godwand/skill/SupernovaManager.java:54)
- [x] `SupernovaManager.TERRAIN_START_TICK` (net/godwand/skill/SupernovaManager.java:55)
- [x] `SupernovaManager.MAX_LIFETIME_TICKS` (net/godwand/skill/SupernovaManager.java:56)
- [x] `SupernovaManager.MAX_DISPLAYS_CAP` (net/godwand/skill/SupernovaManager.java:57)
- [ ] `SupernovaManager.ACTIVE_TASKS`: it uses `Queue` (java.util) (net/godwand/skill/SupernovaManager.java:58)
- [ ] `SupernovaManager.CHARGE_TARGETS`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SupernovaManager.java:59)
- [ ] `SupernovaManager.isFoliageOrNonSolid(3)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/SupernovaManager.java:61)
- [ ] `SupernovaManager.mobPullVelocity(3)`: it uses Minecraft's `Vec3`.ZERO (net/godwand/skill/SupernovaManager.java:71)
- [ ] `SupernovaManager.findTarget(1)`: it uses Minecraft's `BlockPos` (net/godwand/skill/SupernovaManager.java:81)
- [ ] `SupernovaManager.beginCharge(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SupernovaManager.java:104)
- [ ] `SupernovaManager.cancelCharge(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SupernovaManager.java:108)
- [ ] `SupernovaManager.cast(1)`: it uses Minecraft's `ServerPlayer` (net/godwand/skill/SupernovaManager.java:112)
- [ ] `SupernovaManager.prepareTerrain(1)`: it uses SupernovaTask, which isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/SupernovaManager.java:131)
- [ ] `SupernovaManager.tick(1)`: it uses Minecraft's `ServerLevel` (net/godwand/skill/SupernovaManager.java:164)
- [ ] `SupernovaManager.broadcastDetonation(1)`: it uses SupernovaTask, which isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/SupernovaManager.java:471)
- [ ] `SupernovaManager.carveSphericalCraterShell(3)`: it uses SupernovaTask, which isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/SupernovaManager.java:482)
- [ ] `SupernovaManager.getActiveTaskCount(0)`: it uses SupernovaManager.ACTIVE_TASKS, which isn't translated (it uses `Queue` (java.util)) (net/godwand/skill/SupernovaManager.java:539)
- [ ] `SupernovaManager.clearAllTasks(0)`: it uses SupernovaManager.ACTIVE_TASKS, which isn't translated (it uses `Queue` (java.util)) (net/godwand/skill/SupernovaManager.java:543)

### `SupernovaManager.SupernovaTask`: it uses Minecraft's `ServerLevel`

- [ ] `SupernovaManager.SupernovaTask.SupernovaTask(4)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (net/godwand/skill/SupernovaManager.java:573)

### `SupernovaSequence`

- [x] `SupernovaSequence.CHARGE_TICKS` (net/godwand/skill/SupernovaSequence.java:4)
- [x] `SupernovaSequence.SKY_RELEASE` (net/godwand/skill/SupernovaSequence.java:5)
- [x] `SupernovaSequence.SKY_CLOSE_START` (net/godwand/skill/SupernovaSequence.java:6)
- [x] `SupernovaSequence.SKY_CLOSE_END` (net/godwand/skill/SupernovaSequence.java:7)
- [x] `SupernovaSequence.BURST_TICKS` (net/godwand/skill/SupernovaSequence.java:8)
- [x] `SupernovaSequence.SEQUENCE_END` (net/godwand/skill/SupernovaSequence.java:9)
- [x] `SupernovaSequence.PEEL_END` (net/godwand/skill/SupernovaSequence.java:10)
- [x] `SupernovaSequence.SWALLOW_END` (net/godwand/skill/SupernovaSequence.java:11)
- [x] `SupernovaSequence.QUIET_START` (net/godwand/skill/SupernovaSequence.java:12)
- [x] `SupernovaSequence.CUT_START` (net/godwand/skill/SupernovaSequence.java:13)
- [x] `SupernovaSequence.CUT_END` (net/godwand/skill/SupernovaSequence.java:14)
- [x] `SupernovaSequence.FLARE_START` (net/godwand/skill/SupernovaSequence.java:15)
- [x] `SupernovaSequence.IMPACT` (net/godwand/skill/SupernovaSequence.java:16)
- [x] `SupernovaSequence.phase(3)` (net/godwand/skill/SupernovaSequence.java:18)
- [x] `SupernovaSequence.chargeRiftScale(1)` (net/godwand/skill/SupernovaSequence.java:23)
- [x] `SupernovaSequence.skyCollapse(1)` (net/godwand/skill/SupernovaSequence.java:27)
- [x] `SupernovaSequence.skyBlackout(1)` (net/godwand/skill/SupernovaSequence.java:31)
- [x] `SupernovaSequence.skyScale(1)` (net/godwand/skill/SupernovaSequence.java:35)
- [x] `SupernovaSequence.activationPulse(1)` (net/godwand/skill/SupernovaSequence.java:39)
- [x] `SupernovaSequence.skyPulseRadius(1)` (net/godwand/skill/SupernovaSequence.java:43)
- [x] `SupernovaSequence.skyPulseStrength(1)` (net/godwand/skill/SupernovaSequence.java:47)
- [x] `SupernovaSequence.swallow(2)` (net/godwand/skill/SupernovaSequence.java:51)
- [x] `SupernovaSequence.pressure(1)` (net/godwand/skill/SupernovaSequence.java:55)
- [x] `SupernovaSequence.ribbonVisibility(1)` (net/godwand/skill/SupernovaSequence.java:59)
- [x] `SupernovaSequence.coreRadius(1)` (net/godwand/skill/SupernovaSequence.java:63)
- [x] `SupernovaSequence.cameraFov(1)` (net/godwand/skill/SupernovaSequence.java:68)
- [x] `SupernovaSequence.environmentGain(1)` (net/godwand/skill/SupernovaSequence.java:73)

### `DisplacedBlockRecord`: it uses Minecraft's `BlockPos`

- [ ] `DisplacedBlockRecord.writeBridgeData(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/DisplacedBlockRecord.java:46)
- [ ] `DisplacedBlockRecord.readBridgeData(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/DisplacedBlockRecord.java:55)
- [ ] `DisplacedBlockRecord.DisplacedBlockRecord(11)`: its class isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/DisplacedBlockRecord.java:63)
- [ ] `DisplacedBlockRecord.DisplacedBlockRecord(9)`: its class isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/DisplacedBlockRecord.java:105)
- [ ] `DisplacedBlockRecord.DisplacedBlockRecord(8)`: its class isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/DisplacedBlockRecord.java:131)
- [ ] `DisplacedBlockRecord.DisplacedBlockRecord(7)`: its class isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/DisplacedBlockRecord.java:156)
- [ ] `DisplacedBlockRecord.pos(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/DisplacedBlockRecord.java:174)
- [ ] `DisplacedBlockRecord.originalState(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/DisplacedBlockRecord.java:178)
- [ ] `DisplacedBlockRecord.state(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/DisplacedBlockRecord.java:182)
- [ ] `DisplacedBlockRecord.blockEntityNbt(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/DisplacedBlockRecord.java:186)
- [ ] `DisplacedBlockRecord.nbt(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/DisplacedBlockRecord.java:190)
- [ ] `DisplacedBlockRecord.getCurrentOffset(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/DisplacedBlockRecord.java:194)
- [ ] `DisplacedBlockRecord.beginRestoration(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/DisplacedBlockRecord.java:198)
- [ ] `DisplacedBlockRecord.interruptRestoration(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/DisplacedBlockRecord.java:205)
- [ ] `DisplacedBlockRecord.resetRestoreTick(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/DisplacedBlockRecord.java:212)
- [ ] `DisplacedBlockRecord.isRestoring(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/DisplacedBlockRecord.java:216)

### `DisplacedBlockRecord.DisplacementMode`


### `DisplacementPersistentState`: it extends Minecraft's `SavedData`

- [x] `DisplacementPersistentState.IDENTIFIER` (net/godwand/spatial/DisplacementPersistentState.java:26)
- [ ] `DisplacementPersistentState.TYPE`: it uses Minecraft's `SavedDataType` (net/godwand/spatial/DisplacementPersistentState.java:27)
- [ ] `DisplacementPersistentState.setWorldKey(1)`: its class isn't translated (it extends Minecraft's `SavedData`) (net/godwand/spatial/DisplacementPersistentState.java:36)
- [ ] `DisplacementPersistentState.createFromNbt(2)`: its body couldn't be read: a class declared inside a method (net/godwand/spatial/DisplacementPersistentState.java:40)
- [ ] `DisplacementPersistentState.save(2)`: its body couldn't be read: a class declared inside a method (net/godwand/spatial/DisplacementPersistentState.java:69)
- [ ] `DisplacementPersistentState.getServerState(1)`: it uses DisplacementPersistentState, which isn't translated (it extends Minecraft's `SavedData`) (net/godwand/spatial/DisplacementPersistentState.java:111)
- [ ] `DisplacementPersistentState.restoreAndCleanup(1)`: its class isn't translated (it extends Minecraft's `SavedData`) (net/godwand/spatial/DisplacementPersistentState.java:117)
- [ ] `DisplacementPersistentState.getUnrestoredRecords(0)`: its class isn't translated (it extends Minecraft's `SavedData`) (net/godwand/spatial/DisplacementPersistentState.java:158)

### `SpatialDisplacementManager`

- [ ] `SpatialDisplacementManager.ACTIVE_RECORDS`: it uses `DimensionPos` (net/godwand/spatial/SpatialDisplacementManager.java:49)
- [ ] `SpatialDisplacementManager.BRIDGE_DESTINATIONS`: it uses `DimensionPos` (net/godwand/spatial/SpatialDisplacementManager.java:50)
- [ ] `SpatialDisplacementManager.ACTIVE_ENTITIES`: it uses `UUID` (java.util) (net/godwand/spatial/SpatialDisplacementManager.java:51)
- [ ] `SpatialDisplacementManager.LAST_PATH_OPEN_SOUND`: it uses `UUID` (java.util) (net/godwand/spatial/SpatialDisplacementManager.java:52)
- [ ] `SpatialDisplacementManager.LAST_PATH_RETURN_SOUND`: it uses `UUID` (java.util) (net/godwand/spatial/SpatialDisplacementManager.java:53)
- [ ] `SpatialDisplacementManager.shouldPlayPathSound(3)`: it uses `UUID` (java.util) (net/godwand/spatial/SpatialDisplacementManager.java:55)
- [ ] `SpatialDisplacementManager.isPositionDisplaced(2)`: it uses Minecraft's `ResourceKey` (net/godwand/spatial/SpatialDisplacementManager.java:70)
- [ ] `SpatialDisplacementManager.isPositionDisplaced(1)`: it uses Minecraft's `BlockPos` (net/godwand/spatial/SpatialDisplacementManager.java:74)
- [ ] `SpatialDisplacementManager.canDisplace(3)`: it uses Minecraft's `ServerLevel` (net/godwand/spatial/SpatialDisplacementManager.java:78)
- [ ] `SpatialDisplacementManager.clearRuntimeState(0)`: it uses SpatialDisplacementManager.ACTIVE_RECORDS, which isn't translated (it uses `DimensionPos`) (net/godwand/spatial/SpatialDisplacementManager.java:95)
- [ ] `SpatialDisplacementManager.recoverBridge(1)`: it uses DisplacedBlockRecord, which isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/SpatialDisplacementManager.java:103)
- [ ] `SpatialDisplacementManager.moveBridge(2)`: its body couldn't be read: a class declared inside a method (net/godwand/spatial/SpatialDisplacementManager.java:110)
- [ ] `SpatialDisplacementManager.getActiveRecordCount(0)`: it uses SpatialDisplacementManager.ACTIVE_RECORDS, which isn't translated (it uses `DimensionPos`) (net/godwand/spatial/SpatialDisplacementManager.java:121)
- [ ] `SpatialDisplacementManager.getActiveEntityCount(0)`: it uses SpatialDisplacementManager.ACTIVE_ENTITIES, which isn't translated (it uses `UUID` (java.util)) (net/godwand/spatial/SpatialDisplacementManager.java:125)
- [ ] `SpatialDisplacementManager.getActiveRecord(2)`: it uses DisplacedBlockRecord, which isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/SpatialDisplacementManager.java:129)
- [ ] `SpatialDisplacementManager.getActiveEntity(1)`: it uses Minecraft's `BlockDisplay` (net/godwand/spatial/SpatialDisplacementManager.java:133)
- [ ] `SpatialDisplacementManager.tickManagedDisplay(1)`: it uses Minecraft's `BlockDisplay` (net/godwand/spatial/SpatialDisplacementManager.java:143)
- [ ] `SpatialDisplacementManager.trackEntity(2)`: it uses `UUID` (java.util) (net/godwand/spatial/SpatialDisplacementManager.java:154)
- [ ] `SpatialDisplacementManager.untrackEntity(1)`: it uses `UUID` (java.util) (net/godwand/spatial/SpatialDisplacementManager.java:160)
- [ ] `SpatialDisplacementManager.removeActiveRecord(2)`: it uses DisplacedBlockRecord, which isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/SpatialDisplacementManager.java:166)
- [ ] `SpatialDisplacementManager.getActiveRecordsForWorld(1)`: it uses DisplacedBlockRecord, which isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/SpatialDisplacementManager.java:183)
- [ ] `SpatialDisplacementManager.refreshDisplacement(3)`: its body couldn't be read: a class declared inside a method (net/godwand/spatial/SpatialDisplacementManager.java:195)
- [ ] `SpatialDisplacementManager.getMatchingDoubleBlockHalf(3)`: it uses Minecraft's `BlockPos` (net/godwand/spatial/SpatialDisplacementManager.java:203)
- [ ] `SpatialDisplacementManager.displaceBlock(6)`: it uses Minecraft's `ServerLevel` (net/godwand/spatial/SpatialDisplacementManager.java:235)
- [ ] `SpatialDisplacementManager.displaceBlock(7)`: it uses Minecraft's `ServerLevel` (net/godwand/spatial/SpatialDisplacementManager.java:239)
- [ ] `SpatialDisplacementManager.displaceBlock(8)`: it uses Minecraft's `ServerLevel` (net/godwand/spatial/SpatialDisplacementManager.java:245)
- [ ] `SpatialDisplacementManager.displaceBlock(12)`: it uses Minecraft's `ServerLevel` (net/godwand/spatial/SpatialDisplacementManager.java:264)
- [ ] `SpatialDisplacementManager.displaceBlock(13)`: it uses Minecraft's `ServerLevel` (net/godwand/spatial/SpatialDisplacementManager.java:283)
- [ ] `SpatialDisplacementManager.restorePositions(2)`: it uses Minecraft's `ServerLevel` (net/godwand/spatial/SpatialDisplacementManager.java:303)
- [ ] `SpatialDisplacementManager.displaceBlockPermanently(12)`: it uses Minecraft's `ServerLevel` (net/godwand/spatial/SpatialDisplacementManager.java:337)
- [ ] `SpatialDisplacementManager.displaceBlockInternal(14)`: its body couldn't be read: a class declared inside a method (net/godwand/spatial/SpatialDisplacementManager.java:356)
- [ ] `SpatialDisplacementManager.tick(1)`: its body couldn't be read: a class declared inside a method (net/godwand/spatial/SpatialDisplacementManager.java:471)
- [ ] `SpatialDisplacementManager.restoreRecordDirectly(3)`: it uses Minecraft's `ServerLevel` (net/godwand/spatial/SpatialDisplacementManager.java:691)
- [ ] `SpatialDisplacementManager.collapsePlayerAura(1)`: its body couldn't be read: a class declared inside a method (net/godwand/spatial/SpatialDisplacementManager.java:766)
- [ ] `SpatialDisplacementManager.isPathRecord(1)`: it uses DisplacedBlockRecord, which isn't translated (it uses Minecraft's `BlockPos`) (net/godwand/spatial/SpatialDisplacementManager.java:774)
- [ ] `SpatialDisplacementManager.restorePathForPlayer(2)`: it uses `UUID` (java.util) (net/godwand/spatial/SpatialDisplacementManager.java:780)
- [ ] `SpatialDisplacementManager.restoreAllForPlayer(2)`: it uses `UUID` (java.util) (net/godwand/spatial/SpatialDisplacementManager.java:784)
- [ ] `SpatialDisplacementManager.restoreForPlayer(3)`: it uses `UUID` (java.util) (net/godwand/spatial/SpatialDisplacementManager.java:788)
- [ ] `SpatialDisplacementManager.emergencyRestoreAll(1)`: it uses Minecraft's `ServerLevel` (net/godwand/spatial/SpatialDisplacementManager.java:825)
- [ ] `SpatialDisplacementManager.executeTwoPassRestoration(2)`: it uses Minecraft's `ServerLevel` (net/godwand/spatial/SpatialDisplacementManager.java:847)
- [ ] `SpatialDisplacementManager.collapseNearbyDisplacements(3)`: its body couldn't be read: a class declared inside a method (net/godwand/spatial/SpatialDisplacementManager.java:877)

### `SpatialMath`

- [x] `SpatialMath.PIVOT` (net/godwand/spatial/SpatialMath.java:12)
- [ ] `SpatialMath.pathLooseBlockVelocity(3)`: it creates Minecraft's `Vec3` (net/godwand/spatial/SpatialMath.java:14)
- [x] `SpatialMath.SpatialMath(0)` (net/godwand/spatial/SpatialMath.java:20)
- [ ] `SpatialMath.createPivotTransform(3)`: it uses Minecraft's `Transformation` (net/godwand/spatial/SpatialMath.java:23)
- [ ] `SpatialMath.createPivotTransform(3)`: it uses Minecraft's `Transformation` (net/godwand/spatial/SpatialMath.java:34)
- [ ] `SpatialMath.compensatePivot(2)`: JOML's Quaternionf.transform (net/godwand/spatial/SpatialMath.java:38)
- [ ] `SpatialMath.createThinBeamTransform(3)`: it uses Minecraft's `Transformation` (net/godwand/spatial/SpatialMath.java:47)
- [ ] `SpatialMath.computeDynamicAuraOffset(5)`: it uses Minecraft's `BlockPos` (net/godwand/spatial/SpatialMath.java:61)
- [ ] `SpatialMath.computeArchwayRotation(4)`: it uses Minecraft's `BlockPos` (net/godwand/spatial/SpatialMath.java:98)
- [ ] `SpatialMath.computeAuraDisplacement(4)`: it uses Minecraft's `BlockPos` (net/godwand/spatial/SpatialMath.java:125)
- [x] `SpatialMath.canOpenPathBlock(3)` (net/godwand/spatial/SpatialMath.java:129)
- [ ] `SpatialMath.supportsPathFeet(2)`: it uses Minecraft's `AABB` (net/godwand/spatial/SpatialMath.java:133)
- [ ] `SpatialMath.distanceToPathAura(4)`: it calls SpatialMath.distanceToAuraSpine, which isn't translated (net/godwand/spatial/SpatialMath.java:143)
- [ ] `SpatialMath.auraArrowIntersection(3)`: it calls Minecraft's Vec3.subtract (no Horizon helper for it yet) (net/godwand/spatial/SpatialMath.java:148)
- [ ] `SpatialMath.deflectAuraArrow(2)`: it calls Minecraft's Vec3.cross (no Horizon helper for it yet) (net/godwand/spatial/SpatialMath.java:172)
- [ ] `SpatialMath.auraMobVelocity(3)`: it calls Minecraft's Vec3.dot (no Horizon helper for it yet) (net/godwand/spatial/SpatialMath.java:186)
- [x] `SpatialMath.auraVisibility(1)` (net/godwand/spatial/SpatialMath.java:193)
- [ ] `SpatialMath.distanceToAuraSpine(4)`: it calls Minecraft's Vec3.subtract (no Horizon helper for it yet) (net/godwand/spatial/SpatialMath.java:198)
- [ ] `SpatialMath.computeInteractiveEvasion(3)`: it calls Minecraft's Vec3.subtract (no Horizon helper for it yet) (net/godwand/spatial/SpatialMath.java:206)
- [ ] `SpatialMath.computeCanopyVortexOffset(4)`: it uses Minecraft's `BlockPos` (net/godwand/spatial/SpatialMath.java:219)
- [ ] `SpatialMath.computeCanopyVortexOffset(5)`: it uses Minecraft's `BlockPos` (net/godwand/spatial/SpatialMath.java:230)
- [ ] `SpatialMath.computeCanopyVortexInternal(7)`: it uses Minecraft's `BlockPos` (net/godwand/spatial/SpatialMath.java:241)
- [ ] `SpatialMath.computeDynamicBlackHoleOrbit(6)`: it uses Minecraft's `BlockPos` (net/godwand/spatial/SpatialMath.java:278)
- [ ] `SpatialMath.computeSpiralReturnOffset(2)`: it uses Minecraft's `BlockPos` (net/godwand/spatial/SpatialMath.java:319)
- [ ] `SpatialMath.hash01(1)`: it uses Minecraft's `BlockPos` (net/godwand/spatial/SpatialMath.java:329)
- [x] `SpatialMath.hashCoordinateToPhase(1)` (net/godwand/spatial/SpatialMath.java:337)
