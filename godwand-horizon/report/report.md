# Horizon port report: God Wand

`godwand` 1.4.2 · godwand-fabric-26.3-1.4.2.jar · 170 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 12 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 2 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 31 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 20 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 9 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: net.godwand.fabric.GodWandFabricMod
- `client`: net.godwand.fabric.client.GodWandFabricClientMod
- `modmenu`: net.godwand.fabric.client.GodWandModMenuApi

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| CommandRegistrationCallback.EVENT | `GodWandFabricMod` | hook `command` | needs-translation | Server commands. Horizon's command hook is client-side. |
| PayloadTypeRegistry.clientboundPlay | `GodWandFabricMod` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| PayloadTypeRegistry.register | `GodWandFabricMod` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| PayloadTypeRegistry.serverboundPlay | `GodWandFabricMod` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| ServerPlayNetworking.registerGlobalReceiver | `GodWandFabricMod` | – | unsupported | Custom packets aren't supported. |
| ServerTickEvents.START_SERVER_TICK | `GodWandFabricMod` | hook `tick` | auto | The tick hook runs in every Game, the server's too: run server logic only when `!self.native_window()`. |
| AttackBlockCallback.EVENT | `GodWandFabricMod` | – | unsupported | No interaction hook yet: needs a raw target in Horizon's game code. |
| ServerTickEvents.END_LEVEL_TICK | `GodWandFabricMod` | hook `tick` | auto | The tick hook runs in every Game, the server's too: run server logic only when `!self.native_window()`. |
| KeyMappingHelper.registerKeyMapping | `GodWandFabricClientMod` | hook `key` | auto | Mojang-named variant of KeyBindingHelper. |
| EntityRendererRegistry.register | `GodWandFabricClientMod` | – | unsupported | Custom entity renderers aren't supported. |
| ClientPlayNetworking.registerGlobalReceiver | `GodWandFabricClientMod` | – | unsupported | Custom packets aren't supported. |
| HudElementRegistry.addLast | `GodWandFabricClientMod` | hook `hud` | auto |  |
| ClientTickEvents.END_CLIENT_TICK | `GodWandFabricClientMod` | hook `tick` | auto | The Fabric API layer (jfabricapi.rs) runs START_CLIENT_TICK, then the level ticks (START/END_LEVEL_TICK, while in a world), then END_CLIENT_TICK, from Horizon's tick hook, in the window's Game only. |
| ClientPlayConnectionEvents.DISCONNECT | `GodWandFabricClientMod` | hook `tick` | auto | The Fabric API layer runs JOIN on the window's first tick with a player after none, DISCONNECT on the first tick without one after. |
| ClientLifecycleEvents.CLIENT_STOPPING | `GodWandFabricClientMod` | hook `frame` | auto | CLIENT_STARTED: the Fabric API layer runs it on the window's first frame. CLIENT_STOPPING has no hook in Horizon a46ce68 (c9d474d has native_world_lifecycle). |
| ServerPlayNetworking.send | `FabricNetworkHelper` | – | unsupported | Custom packets aren't supported. |
| ClientPlayNetworking.send | `FabricNetworkHelper` | – | unsupported | Custom packets aren't supported. |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Inject(INVOKE) on Camera.update at Lnet/minecraft/client/Camera;alignWithEntity(F)V | `AtomicCameraMixin.godwand$atomicCamera` | AFTER_CALL | needs-mapping | No Horizon function known for Camera.update: run `match`. |
| @Inject(RETURN) on Camera.calculateFov | `AtomicCameraMixin.godwand$dynamicFov` | RETURN on horizon_client_render::camera::Camera::calculate_fov | auto | Horizon's RETURN runs at every normal exit and passes the return value on. |
| @Inject(TAIL) on LivingEntityRenderer.setupRotations | `AtomicFacingMixin.godwand$face` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for LivingEntityRenderer.setupRotations: run `match`. |
| @Inject(HEAD) on Hud.extractCrosshair | `AtomicGuiMixin.godwand$atomicHideCrosshair` | HEAD | needs-mapping | No Horizon function known for Hud.extractCrosshair: run `match`. |
| @Inject(HEAD) on Hud.extractHotbarAndDecorations | `AtomicGuiMixin.godwand$atomicHideCrosshair` | HEAD | needs-mapping | No Horizon function known for Hud.extractHotbarAndDecorations: run `match`. |
| @Inject(HEAD) on Hud.extractChat | `AtomicGuiMixin.godwand$atomicHideCrosshair` | HEAD | needs-mapping | No Horizon function known for Hud.extractChat: run `match`. |
| @Inject(HEAD) on Hud.extractScoreboardSidebar | `AtomicGuiMixin.godwand$atomicHideCrosshair` | HEAD | needs-mapping | No Horizon function known for Hud.extractScoreboardSidebar: run `match`. |
| @Inject(HEAD) on Hud.extractTabList | `AtomicGuiMixin.godwand$atomicHideCrosshair` | HEAD | needs-mapping | No Horizon function known for Hud.extractTabList: run `match`. |
| @Inject(HEAD) on Hud.extractEffects | `AtomicGuiMixin.godwand$atomicHideCrosshair` | HEAD | needs-mapping | No Horizon function known for Hud.extractEffects: run `match`. |
| @Inject(HEAD) on Hud.extractOverlayMessage | `AtomicGuiMixin.godwand$atomicHideCrosshair` | HEAD | needs-mapping | No Horizon function known for Hud.extractOverlayMessage: run `match`. |
| @Inject(HEAD) on Hud.extractTitle | `AtomicGuiMixin.godwand$atomicHideCrosshair` | HEAD | needs-mapping | No Horizon function known for Hud.extractTitle: run `match`. |
| @Inject(TAIL) on KeyboardInput.tick | `AtomicInputMixin.godwand$atomicInput` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for KeyboardInput.tick: run `match`. |
| @Inject(HEAD) on KeyboardHandler.keyPress | `AtomicKeyboardMixin.godwand$skipAtomic` | HEAD on hook `key` | auto |  |
| @Inject(HEAD) on Entity.move | `AtomicLocalMovementMixin.godwand$atomicStill` | HEAD | needs-mapping | No Horizon function known for Entity.move: run `match`. |
| @Inject(HEAD) on Entity.turn | `AtomicLocalMovementMixin.godwand$fugaFacing` | HEAD on horizon_client_input::mouse::Rotation::turn | auto |  |
| @Inject(INVOKE) on ServerGamePacketListenerImpl.handleMovePlayer at Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/server/level/ServerLevel;)V | `AtomicMovementMixin.godwand$atomicAnchor` | AFTER_CALL | needs-mapping | No Horizon function known for ServerGamePacketListenerImpl.handleMovePlayer: run `match`. The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet. |
| @Inject(TAIL) on HumanoidMobRenderer.extractHumanoidRenderState | `AtomicPoseCaptureMixin.godwand$extractAtomicPose` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for HumanoidMobRenderer.extractHumanoidRenderState: run `match`. |
| @Inject(TAIL) on HumanoidModel.setupAnim | `AtomicPoseMixin.godwand$raiseWand` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for HumanoidModel.setupAnim: run `match`. |
| @Inject(HEAD) on ToastManager.extractRenderState | `AtomicToastMixin.godwand$atomicDeferToasts` | HEAD | needs-mapping | No Horizon function known for ToastManager.extractRenderState: run `match`. |
| @Inject(HEAD) on Display.tick | `DisplayLifecycleMixin.godwand$reattachDisplacedDisplay` | HEAD | needs-mapping | No Horizon function known for Display.tick: run `match`. |
| @Inject(HEAD) on Entity.isInvulnerableToBase | `EntityMixin.godwand$atomicProtection` | HEAD | needs-mapping | No Horizon function known for Entity.isInvulnerableToBase: run `match`. |
| @Inject(HEAD) on Entity.isCurrentlyGlowing | `EntityMixin.godwand$preventDisplacedIsGlowing` | HEAD | needs-mapping | No Horizon function known for Entity.isCurrentlyGlowing: run `match`. |
| @Inject(HEAD) on Entity.hasGlowingTag | `EntityMixin.godwand$preventDisplacedHasGlowingTag` | HEAD | needs-mapping | No Horizon function known for Entity.hasGlowingTag: run `match`. |
| @Accessor on ExperienceOrb.count | `ExperienceOrbAccessor.godwand$getCount` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for ExperienceOrb.count: run `match`. |
| @Inject(HEAD) on FlowingFluid.canPassThrough | `FlowableFluidMixin.godwand$flow` | HEAD | needs-mapping | No Horizon function known for FlowingFluid.canPassThrough: run `match`. |
| @Inject(HEAD) on FlowingFluid.canMaybePassThrough | `FlowableFluidMixin.godwand$down` | HEAD | needs-mapping | No Horizon function known for FlowingFluid.canMaybePassThrough: run `match`. |
| @Inject(HEAD) on FlowingFluid.spreadTo | `FlowableFluidMixin.godwand$destination` | HEAD on horizon_world::level::fluids::Sim<'_, L>::spread_to | auto |  |
| @ModifyArg(INVOKE) on GameRenderer.renderLevel at Lnet/minecraft/client/renderer/ProjectionMatrixBuffer;getBuffer(Lorg/joml/Matrix4f;)Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice; | `GameRendererMixin.godwand$captureProjection` | MODIFY_ARG | needs-mapping | No Horizon function known for GameRenderer.renderLevel: run `match`. The call it hooks (ProjectionMatrixBuffer.getBuffer) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on GameRenderer.renderLevel at Lnet/minecraft/client/renderer/GameRenderer;render3dHud(Lnet/minecraft/client/renderer/state/level/CameraRenderState;Lnet/minecraft/client/renderer/state/level/PlayerRenderState;Lnet/minecraft/client/renderer/state/OptionsRenderState;Z)V | `GameRendererMixin.godwand$afterWorldComposite` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for GameRenderer.renderLevel: run `match`. The call it hooks (GameRenderer.render3dHud) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on GameRenderer.bobHurt | `GameRendererMixin.godwand$applyScreenShake` | HEAD on horizon_client_render::camera::bob_hurt | auto |  |
| @Inject(HEAD) on GameRenderer.renderItemInHand | `GameRendererMixin.godwand$atomicHideHand` | HEAD | needs-mapping | No Horizon function known for GameRenderer.renderItemInHand: run `match`. |
| @Inject(HEAD) on Minecraft.shouldEntityAppearGlowing | `MinecraftClientMixin.godwand$suppressBlockDisplayOutline` | HEAD | needs-mapping | No Horizon function known for Minecraft.shouldEntityAppearGlowing: run `match`. |
| @Accessor on PostChain.passes | `PostEffectProcessorAccessor.getPasses` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for PostChain.passes: run `match`. |
| @Accessor on PostPass.customUniforms | `PostPassUniformsAccessor.godwand$uniforms` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for PostPass.customUniforms: run `match`. |
| @Inject(HEAD) on MultiPlayerGameMode.useItem | `SkillCooldownUseMixin.godwand$skillCooldown` | HEAD | needs-mapping | No Horizon function known for MultiPlayerGameMode.useItem: run `match`. |
| @Inject(RETURN) on SoundEngine.calculateVolume | `SoundSystemMixin.godwand$applyVacuumSilence` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for SoundEngine.calculateVolume: run `match`. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends EntityRenderState | `ArcaneOrbEntityRenderer$State` | needs-translation | No mapping for this superclass yet. |
| extends EntityRenderer | `ArcaneOrbEntityRenderer` | needs-translation | No mapping for this superclass yet. |
| extends DynamicTexture | `AtomicRenderer$EnergyTexture` | needs-translation | No mapping for this superclass yet. |
| extends DynamicTexture | `FugaRenderer$FlameTexture` | needs-translation | No mapping for this superclass yet. |
| extends AbstractTickableSoundInstance | `FugaSoundscape$Hold` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `SkillWheelScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends EntityRenderState | `StoneCannonEntityRenderer$State` | needs-translation | No mapping for this superclass yet. |
| extends EntityRenderer | `StoneCannonEntityRenderer` | needs-translation | No mapping for this superclass yet. |
| extends AbstractTickableSoundInstance | `StoneCannonWindSound` | needs-translation | No mapping for this superclass yet. |
| extends AbstractTickableSoundInstance | `SupernovaSoundscape$Charge` | needs-translation | No mapping for this superclass yet. |
| extends AbstractSliderButton | `GodWandConfigScreen$1` | needs-translation | No mapping for this superclass yet. |
| extends AbstractSliderButton | `GodWandConfigScreen$2` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `GodWandConfigScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends Projectile | `ArcaneOrbEntity` | needs-translation | No mapping for this superclass yet. |
| extends Projectile | `StoneCannonProjectileEntity` | needs-translation | No mapping for this superclass yet. |
| extends Item | `GodWandItem` | needs-translation | Overrides chargeMeter, createMeleeAttributes, getSkillMode, setSkillMode, selectSkill, use, getUseDuration, getUseAnimation, onUseTick, releaseUsing, appendHoverText: the content ports, that behaviour needs native code. |
| extends ClipContext | `StoneCannonManager$1` | needs-translation | No mapping for this superclass yet. |
| extends SavedData | `DisplacementPersistentState` | needs-translation | No mapping for this superclass yet. |

## Access wideners

| File | In | Status | Note |
|---|---|---|---|
| access widener (opens private game fields) | `godwand.accesswidener` | needs-translation | Native helpers already see the game's fields, once each field is mapped. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| registers blocks or items in code | `GodWandFabricMod` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| 37 asset files, 11 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

63 of 878 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/Camera#alignWithEntity` | horizon_client_render::camera::Camera::align_with_entity (suggested) |
| `net/minecraft/client/Camera#calculateFov` | horizon_client_render::camera::Camera::calculate_fov (suggested) |
| `net/minecraft/client/KeyMapping#<init>` | `self.native_mod_key(MOD_ID, name, default)` |
| `net/minecraft/client/KeyMapping#consumeClick` | `key == self.native_mod_key(..) && pressed, in the key hook` |
| `net/minecraft/client/KeyboardHandler#keyPress` | hook `key` |
| `net/minecraft/client/Minecraft#font` | `(implicit)` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/gui/Font#width` | `self.native_text_width(text)` |
| `net/minecraft/client/multiplayer/ClientLevel#getHeight` | horizon_world::level::feature_world::<LevelFeatureWorld<'_, '_, L> as PlacementContext>::height (suggested) |
| `net/minecraft/client/multiplayer/ClientLevel#getMinY` | horizon_client::game::Dim::min_y (suggested) |
| `net/minecraft/client/player/LocalPlayer#getViewVector` | horizon_client::mobs::goals::Mob::view_vector (suggested) |
| `net/minecraft/client/player/LocalPlayer#getViewYRot` | horizon_client_render::camera::CameraEntity::view_y_rot (suggested) |
| `net/minecraft/client/renderer/GameRenderer#bobHurt` | horizon_client_render::camera::bob_hurt (suggested) |
| `net/minecraft/client/sounds/SoundManager#isActive` | horizon_client_render::sound_engine::is_active (suggested) |
| `net/minecraft/core/Direction#getOpposite` | horizon_client::mobs::shulker::opposite (suggested) |
| `net/minecraft/core/Registry#register` | content |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK` | content |
| `net/minecraft/core/registries/BuiltInRegistries#ITEM` | content |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |
| `net/minecraft/server/level/ServerLevel#destroyBlock` | horizon_world::level::update::Sim<'_, L>::destroy_block (suggested) |
| `net/minecraft/server/level/ServerLevel#getHeight` | horizon_world::level::feature_world::<LevelFeatureWorld<'_, '_, L> as PlacementContext>::height (suggested) |
| `net/minecraft/server/level/ServerLevel#getMaxY` | horizon_client::game::Dim::max_y (suggested) |
| `net/minecraft/server/level/ServerLevel#getMinY` | horizon_client::game::Dim::min_y (suggested) |
| `net/minecraft/server/level/ServerLevel#hasChunkAt` | horizon_client::raid::has_chunk_at (suggested) |
| `net/minecraft/server/level/ServerLevel#removeBlock` | horizon_world::level::update::Sim<'_, L>::remove_block (suggested) |
| `net/minecraft/server/level/ServerLevel#sendParticles` | horizon_client::game::fishing::Game<'_>::send_particles (suggested) |
| `net/minecraft/server/level/ServerLevel#setBlock` | horizon_world::level::Behaviours::set_block (suggested) |
| `net/minecraft/server/level/ServerPlayer#getViewVector` | horizon_client::mobs::goals::Mob::view_vector (suggested) |
| `net/minecraft/server/level/ServerPlayer#onGround` | horizon_client::game::minecarts::Minecart::on_ground (suggested) |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#teleport` | horizon_client::lan::hub::Hub::teleport (suggested) |
| `net/minecraft/util/ARGB#colorFromFloat` | horizon_client_render::hud_layout::color_from_float (suggested) |
| `net/minecraft/util/Mth#atan2` | horizon_core::geometry::trig::atan2 (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/util/Mth#floor` | horizon_client_render::gfx::occlusion::floor (suggested) |
| `net/minecraft/util/Mth#lerp` | horizon_world::stepped_interpolation::lerp (suggested) |
| `net/minecraft/util/Mth#rotLerp` | horizon_world::stepped_interpolation::rot_lerp (suggested) |
| `net/minecraft/util/Mth#wrapDegrees` | horizon_world::stepped_interpolation::wrap_degrees (suggested) |
| `net/minecraft/world/damagesource/DamageSources#indirectMagic` | horizon_client::mobs::Damage::indirect_magic (suggested) |
| `net/minecraft/world/entity/Entity#getViewVector` | horizon_client::mobs::goals::Mob::view_vector (suggested) |
| `net/minecraft/world/entity/Entity#getX` | `self.native_player()?.x` |
| `net/minecraft/world/entity/Entity#getY` | `self.native_player()?.y` |
| `net/minecraft/world/entity/Entity#getYRot` | `self.native_player()?.yaw` |
| `net/minecraft/world/entity/Entity#getZ` | `self.native_player()?.z` |
| `net/minecraft/world/entity/Entity#turn` | horizon_client_input::mouse::Rotation::turn (suggested) |
| `net/minecraft/world/entity/LivingEntity#die` | horizon_client::mobs::drops::die (suggested) |
| `net/minecraft/world/entity/LivingEntity#onGround` | horizon_client::game::minecarts::Minecart::on_ground (suggested) |
| `net/minecraft/world/entity/boss/enderdragon/EnderDragon#hurt` | horizon_client::game::dragon::Dragon::hurt (suggested) |
| `net/minecraft/world/entity/player/Player#getViewVector` | horizon_client::mobs::goals::Mob::view_vector (suggested) |
| `net/minecraft/world/entity/player/Player#swing` | horizon_client::mobs::ai::Mob::swing (suggested) |
| `net/minecraft/world/item/Item#<init>` | content |
| `net/minecraft/world/item/Item$Properties#stacksTo` | content |
| `net/minecraft/world/level/Level#getMaxY` | horizon_client::game::Dim::max_y (suggested) |
| `net/minecraft/world/level/Level#getMinY` | horizon_client::game::Dim::min_y (suggested) |
| `net/minecraft/world/level/Level#isClientSide` | horizon_world::level::is_client_side (suggested) |
| `net/minecraft/world/level/block/state/BlockState#canBeReplaced` | horizon_world::level::place::Sim<'_, L>::can_be_replaced (suggested) |
| `net/minecraft/world/level/block/state/BlockState#canOcclude` | horizon_world::worldgen::placement::context::can_occlude (suggested) |
| `net/minecraft/world/level/block/state/BlockState#getCollisionShape` | horizon_client_render::collisions::<SceneCollisions<'_, '_> as PistonCollisionWorld>::collision_shape (suggested) |
| `net/minecraft/world/level/material/FlowingFluid#spreadTo` | horizon_world::level::fluids::Sim<'_, L>::spread_to (suggested) |
| `net/minecraft/world/phys/AABB#intersects` | horizon_core::geometry::Aabb::intersects_corners (suggested) |
| `net/minecraft/world/phys/Vec3#directionFromRotation` | horizon_client::auto_jump::direction_from_rotation (suggested) |
| `net/minecraft/world/phys/Vec3#normalize` | horizon_client::mobs::creaking::normalize (suggested) |
| `net/minecraft/world/phys/Vec3#yRot` | horizon_client::mobs::raider::y_rot (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/buffers/Std140Builder#get` | method | 1 |
| `com/mojang/blaze3d/buffers/Std140Builder#onStack` | method | 1 |
| `com/mojang/blaze3d/buffers/Std140Builder#putFloat` | method | 1 |
| `com/mojang/blaze3d/buffers/Std140Builder#putMat4f` | method | 1 |
| `com/mojang/blaze3d/buffers/Std140Builder#putVec2` | method | 1 |
| `com/mojang/blaze3d/buffers/Std140Builder#putVec3` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderTarget#getColorTextureView` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderTarget#getDepthTextureView` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants#getKey` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants#isKeyDown` | method | 3 |
| `com/mojang/blaze3d/platform/InputConstants$Key#getType` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants$Key#getValue` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants$Type#KEYBOARD` | field | 1 |
| `com/mojang/blaze3d/platform/InputConstants$Type#MOUSE` | field | 1 |
| `com/mojang/blaze3d/platform/NativeImage#<init>` | method | 2 |
| `com/mojang/blaze3d/platform/NativeImage#setPixelABGR` | method | 2 |
| `com/mojang/blaze3d/platform/Window#getHeight` | method | 2 |
| `com/mojang/blaze3d/platform/Window#getScreenHeight` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getScreenWidth` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getWidth` | method | 2 |
| `com/mojang/blaze3d/platform/Window#handle` | method | 1 |
| `com/mojang/blaze3d/resource/CrossFrameResourcePool#<init>` | method | 1 |
| `com/mojang/blaze3d/resource/CrossFrameResourcePool#clear` | method | 1 |
| `com/mojang/blaze3d/resource/CrossFrameResourcePool#close` | method | 1 |
| `com/mojang/blaze3d/resource/CrossFrameResourcePool#endFrame` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getCompiledPipeline` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getDevice` | method | 2 |
| `com/mojang/blaze3d/systems/RenderSystem#getDynamicUniforms` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getModelViewMatrixCopy` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getProjectionMatrixBuffer` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getSamplerCache` | method | 2 |
| `com/mojang/blaze3d/systems/RenderSystem#getSequentialBuffer` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#queueFencedTask` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem$AutoStorageIndexBuffer#getBuffer` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem$AutoStorageIndexBuffer#type` | method | 1 |
| `com/mojang/blaze3d/systems/SamplerCache#getClampToEdge` | method | 1 |
| `com/mojang/blaze3d/systems/SamplerCache#getRepeat` | method | 1 |
| `com/mojang/blaze3d/vertex/BufferBuilder#<init>` | method | 1 |
| `com/mojang/blaze3d/vertex/BufferBuilder#addVertex` | method | 6 |
| `com/mojang/blaze3d/vertex/BufferBuilder#build` | method | 5 |
| `com/mojang/blaze3d/vertex/BufferBuilder#buildOrThrow` | method | 6 |
| `com/mojang/blaze3d/vertex/ByteBufferBuilder#<init>` | method | 1 |
| `com/mojang/blaze3d/vertex/ByteBufferBuilder#clear` | method | 1 |
| `com/mojang/blaze3d/vertex/DefaultVertexFormat#POSITION_COLOR` | field | 10 |
| `com/mojang/blaze3d/vertex/DefaultVertexFormat#POSITION_TEX_COLOR` | field | 3 |
| `com/mojang/blaze3d/vertex/MeshData#close` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData#drawState` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData#vertexBuffer` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData$DrawState#format` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData$DrawState#indexCount` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData$DrawState#primitiveTopology` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData$DrawState#vertexCount` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#<init>` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#last` | method | 16 |
| `com/mojang/blaze3d/vertex/PoseStack#popPose` | method | 13 |
| `com/mojang/blaze3d/vertex/PoseStack#pushPose` | method | 13 |
| `com/mojang/blaze3d/vertex/PoseStack#rotate` | method | 12 |
| `com/mojang/blaze3d/vertex/PoseStack#scale` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#translate` | method | 13 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#normal` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#pose` | method | 16 |
| `com/mojang/blaze3d/vertex/VertexConsumer#addVertex` | method | 9 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setColor` | method | 14 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setLight` | method | 2 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setNormal` | method | 3 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setOverlay` | method | 2 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setUv` | method | 5 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setUv1` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setUv2` | method | 1 |
| `net/minecraft/client/Camera#entity` | method | 1 |
| `net/minecraft/client/Camera#isDetached` | method | 1 |
| `net/minecraft/client/Camera#position` | method | 12 |
| `net/minecraft/client/Camera#rotation` | method | 1 |
| `net/minecraft/client/Camera#update` | method | 1 |
| `net/minecraft/client/Camera#xRot` | method | 1 |
| `net/minecraft/client/Camera#yRot` | method | 1 |
| `net/minecraft/client/CameraType#isFirstPerson` | method | 1 |
| `net/minecraft/client/DeltaTracker#getGameTimeDeltaPartialTick` | method | 4 |
| `net/minecraft/client/KeyMapping#getTranslatedKeyMessage` | method | 1 |
| `net/minecraft/client/KeyMapping#matches` | method | 1 |
| `net/minecraft/client/KeyMapping#matchesMouse` | method | 1 |
| `net/minecraft/client/KeyMapping#saveString` | method | 1 |
| `net/minecraft/client/KeyMapping$Category#register` | method | 1 |
| `net/minecraft/client/Minecraft#execute` | method | 1 |
| `net/minecraft/client/Minecraft#gameRenderer` | field | 7 |
| `net/minecraft/client/Minecraft#getConnection` | method | 1 |
| `net/minecraft/client/Minecraft#getDeltaTracker` | method | 1 |
| `net/minecraft/client/Minecraft#getResourceManager` | method | 4 |
| `net/minecraft/client/Minecraft#getSoundManager` | method | 5 |
| `net/minecraft/client/Minecraft#getTextureManager` | method | 4 |
| `net/minecraft/client/Minecraft#getWindow` | method | 4 |
| `net/minecraft/client/Minecraft#gui` | field | 5 |
| `net/minecraft/client/Minecraft#isPaused` | method | 1 |
| `net/minecraft/client/Minecraft#isWindowActive` | method | 3 |
| `net/minecraft/client/Minecraft#level` | field | 20 |
| `net/minecraft/client/Minecraft#mouseHandler` | field | 1 |
| `net/minecraft/client/Minecraft#options` | field | 3 |
| `net/minecraft/client/Minecraft#setScreenAndShow` | method | 2 |
| `net/minecraft/client/Minecraft#shouldEntityAppearGlowing` | method | 1 |
| `net/minecraft/client/MouseHandler#xpos` | method | 1 |
| `net/minecraft/client/MouseHandler#ypos` | method | 1 |
| `net/minecraft/client/OptionInstance#get` | method | 2 |
| `net/minecraft/client/Options#fov` | method | 1 |
| `net/minecraft/client/Options#getCameraType` | method | 1 |
| `net/minecraft/client/Options#getEffectiveRenderDistance` | method | 1 |
| `net/minecraft/client/Options#particles` | method | 1 |
| `net/minecraft/client/gui/Gui#hud` | field | 1 |
| `net/minecraft/client/gui/Gui#screen` | method | 5 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blit` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#centeredText` | method | 3 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fill` | method | 4 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#guiHeight` | method | 4 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#guiWidth` | method | 4 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#pose` | method | 3 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#text` | method | 2 |
| `net/minecraft/client/gui/Hud#extractChat` | method | 1 |
| `net/minecraft/client/gui/Hud#extractCrosshair` | method | 1 |
| `net/minecraft/client/gui/Hud#extractEffects` | method | 1 |
| `net/minecraft/client/gui/Hud#extractHotbarAndDecorations` | method | 1 |
| `net/minecraft/client/gui/Hud#extractOverlayMessage` | method | 1 |
| `net/minecraft/client/gui/Hud#extractScoreboardSidebar` | method | 1 |
| `net/minecraft/client/gui/Hud#extractTabList` | method | 1 |
| `net/minecraft/client/gui/Hud#extractTitle` | method | 1 |
| `net/minecraft/client/gui/Hud#isHidden` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSliderButton#<init>` | method | 2 |
| `net/minecraft/client/gui/components/Button#builder` | method | 1 |
| `net/minecraft/client/gui/components/Button$Builder#bounds` | method | 1 |
| `net/minecraft/client/gui/components/Button$Builder#build` | method | 1 |
| `net/minecraft/client/gui/components/CycleButton#builder` | method | 1 |
| `net/minecraft/client/gui/components/CycleButton#onOffBuilder` | method | 1 |
| `net/minecraft/client/gui/components/CycleButton$Builder#create` | method | 1 |
| `net/minecraft/client/gui/components/CycleButton$Builder#withValues` | method | 1 |
| `net/minecraft/client/gui/components/toasts/ToastManager#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#<init>` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#keyReleased` | method | 1 |
| `net/minecraft/client/input/KeyEvent#key` | method | 1 |
| `net/minecraft/client/model/HumanoidModel#setupAnim` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#getChild` | method | 2 |
| `net/minecraft/client/model/geom/ModelPart#loadPose` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#render` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#storePose` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#translateAndRotate` | method | 2 |
| `net/minecraft/client/model/geom/ModelPart#x` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#xRot` | field | 4 |
| `net/minecraft/client/model/geom/ModelPart#xScale` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#y` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#yRot` | field | 4 |
| `net/minecraft/client/model/geom/ModelPart#yScale` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#z` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#zRot` | field | 4 |
| `net/minecraft/client/model/geom/ModelPart#zScale` | field | 1 |
| `net/minecraft/client/model/geom/PartPose#ZERO` | field | 3 |
| `net/minecraft/client/model/geom/PartPose#offset` | method | 2 |
| `net/minecraft/client/model/geom/builders/CubeListBuilder#create` | method | 3 |
| `net/minecraft/client/model/geom/builders/LayerDefinition#bakeRoot` | method | 1 |
| `net/minecraft/client/model/geom/builders/LayerDefinition#create` | method | 3 |
| `net/minecraft/client/model/geom/builders/MeshDefinition#<init>` | method | 3 |
| `net/minecraft/client/model/geom/builders/MeshDefinition#getRoot` | method | 3 |
| `net/minecraft/client/model/geom/builders/PartDefinition#addOrReplaceChild` | method | 3 |
| `net/minecraft/client/multiplayer/ClientChunkCache#hasChunk` | method | 4 |
| `net/minecraft/client/multiplayer/ClientLevel#addAlwaysVisibleParticle` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#clip` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#getChunkSource` | method | 4 |
| `net/minecraft/client/multiplayer/ClientLevel#getEntities` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getEntity` | method | 4 |
| `net/minecraft/client/multiplayer/ClientLevel#getGameTime` | method | 13 |
| `net/minecraft/client/multiplayer/ClientLevel#getRandom` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#players` | method | 2 |
| `net/minecraft/client/multiplayer/MultiPlayerGameMode#useItem` | method | 1 |
| `net/minecraft/client/player/ClientInput#<init>` | method | 1 |
| `net/minecraft/client/player/KeyboardInput#tick` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#distanceToSqr` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#fallDistance` | field | 1 |
| `net/minecraft/client/player/LocalPlayer#getEyePosition` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#getId` | method | 4 |
| `net/minecraft/client/player/LocalPlayer#getInventory` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getItemInHand` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getMainHandItem` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#getOffhandItem` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getPosition` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getTicksUsingItem` | method | 4 |
| `net/minecraft/client/player/LocalPlayer#getUseItem` | method | 4 |
| `net/minecraft/client/player/LocalPlayer#getViewXRot` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isAlive` | method | 3 |
| `net/minecraft/client/player/LocalPlayer#isSpectator` | method | 3 |
| `net/minecraft/client/player/LocalPlayer#isUsingItem` | method | 5 |
| `net/minecraft/client/player/LocalPlayer#position` | method | 8 |
| `net/minecraft/client/player/LocalPlayer#sendOverlayMessage` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#setDeltaMovement` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#tickCount` | field | 1 |
| `net/minecraft/client/renderer/DynamicGpuData#writeTransform` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#gameRenderState` | method | 3 |
| `net/minecraft/client/renderer/GameRenderer#mainCamera` | method | 4 |
| `net/minecraft/client/renderer/GameRenderer#mainRenderTarget` | method | 3 |
| `net/minecraft/client/renderer/GameRenderer#render3dHud` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#renderItemInHand` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#renderLevel` | method | 2 |
| `net/minecraft/client/renderer/PostChain#MAIN_TARGET_ID` | field | 1 |
| `net/minecraft/client/renderer/PostChain#close` | method | 1 |
| `net/minecraft/client/renderer/PostChain#load` | method | 1 |
| `net/minecraft/client/renderer/PostChain#passes` | method | 1 |
| `net/minecraft/client/renderer/PostChain#process` | method | 1 |
| `net/minecraft/client/renderer/PostChainConfig#CODEC` | field | 1 |
| `net/minecraft/client/renderer/PostPass#customUniforms` | method | 1 |
| `net/minecraft/client/renderer/Projection#<init>` | method | 1 |
| `net/minecraft/client/renderer/Projection#setupOrtho` | method | 1 |
| `net/minecraft/client/renderer/ProjectionMatrixBuffer#<init>` | method | 1 |
| `net/minecraft/client/renderer/ProjectionMatrixBuffer#close` | method | 1 |
| `net/minecraft/client/renderer/ProjectionMatrixBuffer#getBuffer` | method | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_TEXTURED` | field | 2 |
| `net/minecraft/client/renderer/RenderPipelines#register` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitCustomGeometry` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderer#<init>` | method | 2 |
| `net/minecraft/client/renderer/entity/EntityRenderer#extractRenderState` | method | 2 |
| `net/minecraft/client/renderer/entity/EntityRenderer#submit` | method | 2 |
| `net/minecraft/client/renderer/entity/HumanoidMobRenderer#extractHumanoidRenderState` | method | 1 |
| `net/minecraft/client/renderer/entity/LivingEntityRenderer#setupRotations` | method | 1 |
| `net/minecraft/client/renderer/entity/state/EntityRenderState#<init>` | method | 2 |
| `net/minecraft/client/renderer/item/ItemModels#ID_MAPPER` | field | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#newLayer` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#setAnimated` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#setExtents` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#setLocalTransform` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#setUsesBlockLight` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#setupSpecialModel` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityCutout` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityTranslucentEmissive` | method | 1 |
| `net/minecraft/client/renderer/state/GameRenderState#levelRenderState` | field | 3 |
| `net/minecraft/client/renderer/state/level/CameraRenderState#cameraEntityPartialTicks` | field | 1 |
| `net/minecraft/client/renderer/state/level/CameraRenderState#pos` | field | 2 |
| `net/minecraft/client/renderer/state/level/CameraRenderState#viewRotationMatrix` | field | 1 |
| `net/minecraft/client/renderer/state/level/LevelRenderState#cameraRenderState` | field | 3 |
| `net/minecraft/client/renderer/state/level/LevelRenderState#worldPartialTicks` | field | 2 |
| `net/minecraft/client/renderer/texture/AbstractTexture#getSampler` | method | 1 |
| `net/minecraft/client/renderer/texture/AbstractTexture#getTextureView` | method | 1 |
| `net/minecraft/client/renderer/texture/DynamicTexture#<init>` | method | 2 |
| `net/minecraft/client/renderer/texture/TextureManager#getTexture` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureManager#register` | method | 2 |
| `net/minecraft/client/resources/sounds/AbstractTickableSoundInstance#<init>` | method | 3 |
| `net/minecraft/client/resources/sounds/SimpleSoundInstance#<init>` | method | 4 |
| `net/minecraft/client/resources/sounds/SoundInstance#getSource` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance$Attenuation#LINEAR` | field | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance$Attenuation#NONE` | field | 7 |
| `net/minecraft/client/sounds/SoundEngine#calculateVolume` | method | 1 |
| `net/minecraft/client/sounds/SoundManager#play` | method | 5 |
| `net/minecraft/client/sounds/SoundManager#stop` | method | 5 |
| `net/minecraft/commands/CommandSourceStack#getPlayerOrException` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#sendSuccess` | method | 1 |
| `net/minecraft/commands/Commands#LEVEL_GAMEMASTERS` | field | 1 |
| `net/minecraft/commands/Commands#LEVEL_OWNERS` | field | 1 |
| `net/minecraft/commands/Commands#argument` | method | 1 |
| `net/minecraft/commands/Commands#hasPermission` | method | 1 |
| `net/minecraft/commands/Commands#literal` | method | 1 |
| `net/minecraft/commands/arguments/EntityArgument#getPlayers` | method | 1 |
| `net/minecraft/commands/arguments/EntityArgument#players` | method | 1 |
| `net/minecraft/core/BlockPos#<init>` | method | 3 |
| `net/minecraft/core/BlockPos#above` | method | 3 |
| `net/minecraft/core/BlockPos#asLong` | method | 6 |
| `net/minecraft/core/BlockPos#atY` | method | 1 |
| `net/minecraft/core/BlockPos#below` | method | 5 |
| `net/minecraft/core/BlockPos#betweenClosed` | method | 2 |
| `net/minecraft/core/BlockPos#containing` | method | 10 |
| `net/minecraft/core/BlockPos#distSqr` | method | 1 |
| `net/minecraft/core/BlockPos#getX` | method | 13 |
| `net/minecraft/core/BlockPos#getY` | method | 9 |
| `net/minecraft/core/BlockPos#getZ` | method | 13 |
| `net/minecraft/core/BlockPos#hashCode` | method | 1 |
| `net/minecraft/core/BlockPos#immutable` | method | 9 |
| `net/minecraft/core/BlockPos#of` | method | 1 |
| `net/minecraft/core/BlockPos#offset` | method | 1 |
| `net/minecraft/core/BlockPos#relative` | method | 2 |
| `net/minecraft/core/BlockPos$MutableBlockPos#<init>` | method | 6 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getX` | method | 2 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getY` | method | 2 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getZ` | method | 2 |
| `net/minecraft/core/BlockPos$MutableBlockPos#immutable` | method | 4 |
| `net/minecraft/core/BlockPos$MutableBlockPos#move` | method | 2 |
| `net/minecraft/core/BlockPos$MutableBlockPos#set` | method | 6 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setY` | method | 1 |
| `net/minecraft/core/Direction#getUnitVec3` | method | 1 |
| `net/minecraft/core/Direction#values` | method | 1 |
| `net/minecraft/core/Direction$Axis#Y` | field | 1 |
| `net/minecraft/core/Holder#value` | method | 1 |
| `net/minecraft/core/Holder$Reference#value` | method | 5 |
| `net/minecraft/core/NonNullList#iterator` | method | 1 |
| `net/minecraft/core/Registry#getOrThrow` | method | 1 |
| `net/minecraft/core/RegistryAccess#lookupOrThrow` | method | 1 |
| `net/minecraft/core/UUIDUtil#CODEC` | field | 1 |
| `net/minecraft/core/component/DataComponents#CUSTOM_DATA` | field | 2 |
| `net/minecraft/core/particles/BlockParticleOption#<init>` | method | 3 |
| `net/minecraft/core/particles/ColorParticleOption#create` | method | 3 |
| `net/minecraft/core/particles/DustParticleOptions#<init>` | method | 5 |
| `net/minecraft/core/particles/ParticleTypes#BLOCK` | field | 3 |
| `net/minecraft/core/particles/ParticleTypes#CAMPFIRE_COSY_SMOKE` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#CAMPFIRE_SIGNAL_SMOKE` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#ELECTRIC_SPARK` | field | 6 |
| `net/minecraft/core/particles/ParticleTypes#ENCHANTED_HIT` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#END_ROD` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#EXPLOSION_EMITTER` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#FLAME` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#FLASH` | field | 3 |
| `net/minecraft/core/particles/ParticleTypes#LAVA` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#REVERSE_PORTAL` | field | 3 |
| `net/minecraft/core/particles/ParticleTypes#SMALL_FLAME` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#SONIC_BOOM` | field | 5 |
| `net/minecraft/core/particles/ParticleTypes#SWEEP_ATTACK` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#CREATIVE_MODE_TAB` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#ENTITY_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#CREATIVE_MODE_TAB` | field | 1 |
| `net/minecraft/core/registries/Registries#DAMAGE_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#DIMENSION` | field | 1 |
| `net/minecraft/core/registries/Registries#ENTITY_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#ITEM` | field | 1 |
| `net/minecraft/nbt/CompoundTag#<init>` | method | 1 |
| `net/minecraft/nbt/CompoundTag#CODEC` | field | 1 |
| `net/minecraft/nbt/CompoundTag#contains` | method | 2 |
| `net/minecraft/nbt/CompoundTag#get` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getBooleanOr` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getCompound` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getCompoundOrEmpty` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getInt` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getIntOr` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getListOrEmpty` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getLongOr` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getStringOr` | method | 1 |
| `net/minecraft/nbt/CompoundTag#put` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putBoolean` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putInt` | method | 3 |
| `net/minecraft/nbt/CompoundTag#putLong` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putString` | method | 1 |
| `net/minecraft/nbt/CompoundTag#read` | method | 1 |
| `net/minecraft/nbt/CompoundTag#remove` | method | 1 |
| `net/minecraft/nbt/CompoundTag#store` | method | 1 |
| `net/minecraft/nbt/ListTag#<init>` | method | 1 |
| `net/minecraft/nbt/ListTag#add` | method | 1 |
| `net/minecraft/nbt/ListTag#getCompoundOrEmpty` | method | 1 |
| `net/minecraft/nbt/ListTag#size` | method | 1 |
| `net/minecraft/nbt/NbtUtils#readBlockState` | method | 1 |
| `net/minecraft/nbt/NbtUtils#writeBlockState` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readBoolean` | method | 2 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readDouble` | method | 4 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readFloat` | method | 3 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readLong` | method | 4 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readVarInt` | method | 7 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeBoolean` | method | 2 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeDouble` | method | 4 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeFloat` | method | 3 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeLong` | method | 4 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeVarInt` | method | 7 |
| `net/minecraft/network/chat/CommonComponents#GUI_DONE` | field | 1 |
| `net/minecraft/network/chat/Component#literal` | method | 5 |
| `net/minecraft/network/chat/Component#translatable` | method | 19 |
| `net/minecraft/network/chat/FontDescription$Resource#<init>` | method | 1 |
| `net/minecraft/network/chat/Style#withFont` | method | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#DOUBLE` | field | 2 |
| `net/minecraft/network/codec/ByteBufCodecs#FLOAT` | field | 2 |
| `net/minecraft/network/codec/ByteBufCodecs#INT` | field | 2 |
| `net/minecraft/network/codec/StreamCodec#composite` | method | 2 |
| `net/minecraft/network/protocol/PacketUtils#ensureRunningOnSameThread` | method | 1 |
| `net/minecraft/network/protocol/common/custom/CustomPacketPayload$Type#<init>` | method | 11 |
| `net/minecraft/network/protocol/game/ServerboundMovePlayerPacket#getXRot` | method | 1 |
| `net/minecraft/network/protocol/game/ServerboundMovePlayerPacket#getYRot` | method | 1 |
| `net/minecraft/network/protocol/game/ServerboundMovePlayerPacket#hasRotation` | method | 1 |
| `net/minecraft/network/syncher/EntityDataSerializers#INT` | field | 2 |
| `net/minecraft/network/syncher/SynchedEntityData#defineId` | method | 2 |
| `net/minecraft/network/syncher/SynchedEntityData#get` | method | 2 |
| `net/minecraft/network/syncher/SynchedEntityData#set` | method | 2 |
| `net/minecraft/network/syncher/SynchedEntityData$Builder#define` | method | 2 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 27 |
| `net/minecraft/resources/Identifier#getPath` | method | 1 |
| `net/minecraft/resources/Identifier#parse` | method | 1 |
| `net/minecraft/resources/Identifier#toString` | method | 5 |
| `net/minecraft/resources/Identifier#withDefaultNamespace` | method | 1 |
| `net/minecraft/resources/Identifier#withPath` | method | 1 |
| `net/minecraft/resources/ResourceKey#create` | method | 4 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 5 |
| `net/minecraft/server/MinecraftServer#execute` | method | 1 |
| `net/minecraft/server/MinecraftServer#getAllLevels` | method | 1 |
| `net/minecraft/server/MinecraftServer#getLevel` | method | 1 |
| `net/minecraft/server/MinecraftServer#getPlayerList` | method | 3 |
| `net/minecraft/server/level/ParticleStatus#MINIMAL` | field | 1 |
| `net/minecraft/server/level/ServerChunkCache#getChunkNow` | method | 4 |
| `net/minecraft/server/level/ServerChunkCache#hasChunk` | method | 4 |
| `net/minecraft/server/level/ServerLevel#addFreshEntity` | method | 5 |
| `net/minecraft/server/level/ServerLevel#clip` | method | 3 |
| `net/minecraft/server/level/ServerLevel#damageSources` | method | 6 |
| `net/minecraft/server/level/ServerLevel#dimension` | method | 11 |
| `net/minecraft/server/level/ServerLevel#getAllEntities` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getBlockEntity` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getBlockState` | method | 7 |
| `net/minecraft/server/level/ServerLevel#getChunkSource` | method | 7 |
| `net/minecraft/server/level/ServerLevel#getDataStorage` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getEntities` | method | 2 |
| `net/minecraft/server/level/ServerLevel#getEntitiesOfClass` | method | 8 |
| `net/minecraft/server/level/ServerLevel#getEntity` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getGameTime` | method | 7 |
| `net/minecraft/server/level/ServerLevel#getNearestPlayer` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getRandom` | method | 4 |
| `net/minecraft/server/level/ServerLevel#getServer` | method | 2 |
| `net/minecraft/server/level/ServerLevel#getWorldBorder` | method | 2 |
| `net/minecraft/server/level/ServerLevel#isInWorldBounds` | method | 4 |
| `net/minecraft/server/level/ServerLevel#isOutsideBuildHeight` | method | 1 |
| `net/minecraft/server/level/ServerLevel#playSound` | method | 8 |
| `net/minecraft/server/level/ServerLevel#players` | method | 2 |
| `net/minecraft/server/level/ServerLevel#registryAccess` | method | 2 |
| `net/minecraft/server/level/ServerLevel#removeBlockEntity` | method | 1 |
| `net/minecraft/server/level/ServerLevel#setBlockEntity` | method | 1 |
| `net/minecraft/server/level/ServerLevel#setChunkForced` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#blockPosition` | method | 3 |
| `net/minecraft/server/level/ServerPlayer#closeContainer` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#connection` | field | 2 |
| `net/minecraft/server/level/ServerPlayer#containerMenu` | field | 2 |
| `net/minecraft/server/level/ServerPlayer#fallDistance` | field | 2 |
| `net/minecraft/server/level/ServerPlayer#getAbilities` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#getBoundingBox` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#getDeltaMovement` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getEyePosition` | method | 5 |
| `net/minecraft/server/level/ServerPlayer#getEyeY` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getId` | method | 3 |
| `net/minecraft/server/level/ServerPlayer#getInventory` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getItemInHand` | method | 5 |
| `net/minecraft/server/level/ServerPlayer#getLookAngle` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#getMainHandItem` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#getOffhandItem` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getUUID` | method | 7 |
| `net/minecraft/server/level/ServerPlayer#getUseItem` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getX` | method | 4 |
| `net/minecraft/server/level/ServerPlayer#getXRot` | method | 4 |
| `net/minecraft/server/level/ServerPlayer#getY` | method | 5 |
| `net/minecraft/server/level/ServerPlayer#getYRot` | method | 3 |
| `net/minecraft/server/level/ServerPlayer#getZ` | method | 4 |
| `net/minecraft/server/level/ServerPlayer#hasDisconnected` | method | 3 |
| `net/minecraft/server/level/ServerPlayer#isAlive` | method | 6 |
| `net/minecraft/server/level/ServerPlayer#isPassenger` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#isShiftKeyDown` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#isSpectator` | method | 5 |
| `net/minecraft/server/level/ServerPlayer#isSprinting` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#isUsingItem` | method | 4 |
| `net/minecraft/server/level/ServerPlayer#level` | method | 9 |
| `net/minecraft/server/level/ServerPlayer#pick` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#position` | method | 8 |
| `net/minecraft/server/level/ServerPlayer#push` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#sendOverlayMessage` | method | 7 |
| `net/minecraft/server/level/ServerPlayer#setDeltaMovement` | method | 3 |
| `net/minecraft/server/level/ServerPlayer#setXRot` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#setYRot` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#stopUsingItem` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#syncVelocity` | field | 2 |
| `net/minecraft/server/level/ServerPlayer#tickCount` | field | 2 |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#handleMovePlayer` | method | 1 |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#player` | field | 1 |
| `net/minecraft/server/packs/PackType#CLIENT_RESOURCES` | field | 1 |
| `net/minecraft/server/packs/resources/Resource#open` | method | 3 |
| `net/minecraft/server/packs/resources/ResourceManager#getResource` | method | 3 |
| `net/minecraft/server/packs/resources/ResourceManager#openAsReader` | method | 1 |
| `net/minecraft/server/players/PlayerList#getPlayer` | method | 2 |
| `net/minecraft/server/players/PlayerList#getPlayers` | method | 1 |
| `net/minecraft/sounds/SoundEvent#createVariableRangeEvent` | method | 1 |
| `net/minecraft/sounds/SoundEvent#location` | method | 1 |
| `net/minecraft/sounds/SoundEvents#AMETHYST_BLOCK_CHIME` | field | 2 |
| `net/minecraft/sounds/SoundEvents#ANVIL_LAND` | field | 2 |
| `net/minecraft/sounds/SoundEvents#ANVIL_PLACE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BASALT_BREAK` | field | 3 |
| `net/minecraft/sounds/SoundEvents#BASALT_HIT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BASALT_PLACE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BEACON_ACTIVATE` | field | 2 |
| `net/minecraft/sounds/SoundEvents#BEACON_AMBIENT` | field | 3 |
| `net/minecraft/sounds/SoundEvents#BEACON_DEACTIVATE` | field | 3 |
| `net/minecraft/sounds/SoundEvents#BREEZE_IDLE_AIR` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BREEZE_WIND_CHARGE_BURST` | field | 1 |
| `net/minecraft/sounds/SoundEvents#DEEPSLATE_BREAK` | field | 1 |
| `net/minecraft/sounds/SoundEvents#DEEPSLATE_PLACE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ELYTRA_FLYING` | field | 1 |
| `net/minecraft/sounds/SoundEvents#FIRE_EXTINGUISH` | field | 1 |
| `net/minecraft/sounds/SoundEvents#GENERIC_EXPLODE` | field | 4 |
| `net/minecraft/sounds/SoundEvents#GRINDSTONE_USE` | field | 2 |
| `net/minecraft/sounds/SoundEvents#HEAVY_CORE_BREAK` | field | 1 |
| `net/minecraft/sounds/SoundEvents#HEAVY_CORE_PLACE` | field | 3 |
| `net/minecraft/sounds/SoundEvents#HEAVY_CORE_STEP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#MACE_SMASH_GROUND_HEAVY` | field | 3 |
| `net/minecraft/sounds/SoundEvents#RESPAWN_ANCHOR_AMBIENT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#RESPAWN_ANCHOR_CHARGE` | field | 4 |
| `net/minecraft/sounds/SoundEvents#RESPAWN_ANCHOR_DEPLETE` | field | 3 |
| `net/minecraft/sounds/SoundEvents#STONE_PLACE` | field | 2 |
| `net/minecraft/sounds/SoundEvents#TRIDENT_THUNDER` | field | 1 |
| `net/minecraft/sounds/SoundEvents#VAULT_OPEN_SHUTTER` | field | 1 |
| `net/minecraft/sounds/SoundEvents#WARDEN_SONIC_BOOM` | field | 4 |
| `net/minecraft/sounds/SoundSource#BLOCKS` | field | 2 |
| `net/minecraft/sounds/SoundSource#MASTER` | field | 1 |
| `net/minecraft/sounds/SoundSource#PLAYERS` | field | 13 |
| `net/minecraft/tags/BlockTags#CROPS` | field | 1 |
| `net/minecraft/tags/BlockTags#FLOWERS` | field | 1 |
| `net/minecraft/tags/BlockTags#LEAVES` | field | 2 |
| `net/minecraft/tags/BlockTags#LOGS` | field | 1 |
| `net/minecraft/tags/BlockTags#SAPLINGS` | field | 1 |
| `net/minecraft/util/ARGB#colorFromVector3f` | method | 4 |
| `net/minecraft/util/Brightness#<init>` | method | 1 |
| `net/minecraft/util/ExtraCodecs$LateBoundIdMapper#put` | method | 1 |
| `net/minecraft/util/Mth#sin` | method | 1 |
| `net/minecraft/util/ProblemReporter#DISCARDING` | field | 1 |
| `net/minecraft/util/RandomSource#create` | method | 7 |
| `net/minecraft/util/RandomSource#nextDouble` | method | 5 |
| `net/minecraft/util/RandomSource#nextFloat` | method | 1 |
| `net/minecraft/util/RandomSource#nextGaussian` | method | 1 |
| `net/minecraft/util/RandomSource#nextInt` | method | 1 |
| `net/minecraft/world/Clearable#clearContent` | method | 1 |
| `net/minecraft/world/InteractionHand#MAIN_HAND` | field | 3 |
| `net/minecraft/world/InteractionHand#OFF_HAND` | field | 2 |
| `net/minecraft/world/InteractionResult#CONSUME` | field | 1 |
| `net/minecraft/world/InteractionResult#FAIL` | field | 2 |
| `net/minecraft/world/InteractionResult#PASS` | field | 2 |
| `net/minecraft/world/InteractionResult#SUCCESS` | field | 2 |
| `net/minecraft/world/damagesource/DamageSource#<init>` | method | 1 |
| `net/minecraft/world/damagesource/DamageSource#getDirectEntity` | method | 1 |
| `net/minecraft/world/damagesource/DamageSource#getEntity` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#explosion` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#fallingBlock` | method | 3 |
| `net/minecraft/world/damagesource/DamageSources#fellOutOfWorld` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#inWall` | method | 1 |
| `net/minecraft/world/effect/MobEffectInstance#<init>` | method | 1 |
| `net/minecraft/world/effect/MobEffects#SLOWNESS` | field | 1 |
| `net/minecraft/world/entity/Display#tick` | method | 1 |
| `net/minecraft/world/entity/Display$BlockDisplay#<init>` | method | 3 |
| `net/minecraft/world/entity/Display$BlockDisplay#addTag` | method | 3 |
| `net/minecraft/world/entity/Display$BlockDisplay#calculateInterpolationProgress` | method | 1 |
| `net/minecraft/world/entity/Display$BlockDisplay#discard` | method | 3 |
| `net/minecraft/world/entity/Display$BlockDisplay#distanceToSqr` | method | 1 |
| `net/minecraft/world/entity/Display$BlockDisplay#entityTags` | method | 1 |
| `net/minecraft/world/entity/Display$BlockDisplay#getId` | method | 1 |
| `net/minecraft/world/entity/Display$BlockDisplay#getPosition` | method | 1 |
| `net/minecraft/world/entity/Display$BlockDisplay#getUUID` | method | 2 |
| `net/minecraft/world/entity/Display$BlockDisplay#isAlive` | method | 4 |
| `net/minecraft/world/entity/Display$BlockDisplay#level` | method | 1 |
| `net/minecraft/world/entity/Display$BlockDisplay#position` | method | 1 |
| `net/minecraft/world/entity/Display$BlockDisplay#renderState` | method | 1 |
| `net/minecraft/world/entity/Display$BlockDisplay#setBlockState` | method | 4 |
| `net/minecraft/world/entity/Display$BlockDisplay#setBrightnessOverride` | method | 2 |
| `net/minecraft/world/entity/Display$BlockDisplay#setGlowColorOverride` | method | 3 |
| `net/minecraft/world/entity/Display$BlockDisplay#setGlowingTag` | method | 3 |
| `net/minecraft/world/entity/Display$BlockDisplay#setPos` | method | 3 |
| `net/minecraft/world/entity/Display$BlockDisplay#setTransformation` | method | 5 |
| `net/minecraft/world/entity/Display$BlockDisplay#setTransformationInterpolationDelay` | method | 5 |
| `net/minecraft/world/entity/Display$BlockDisplay#setTransformationInterpolationDuration` | method | 5 |
| `net/minecraft/world/entity/Display$GenericInterpolator#get` | method | 1 |
| `net/minecraft/world/entity/Display$RenderState#transformation` | method | 1 |
| `net/minecraft/world/entity/Entity#discard` | method | 1 |
| `net/minecraft/world/entity/Entity#distanceToSqr` | method | 1 |
| `net/minecraft/world/entity/Entity#entityTags` | method | 3 |
| `net/minecraft/world/entity/Entity#getBoundingBox` | method | 2 |
| `net/minecraft/world/entity/Entity#getEyePosition` | method | 1 |
| `net/minecraft/world/entity/Entity#getId` | method | 1 |
| `net/minecraft/world/entity/Entity#getPickRadius` | method | 1 |
| `net/minecraft/world/entity/Entity#getPosition` | method | 3 |
| `net/minecraft/world/entity/Entity#getUUID` | method | 1 |
| `net/minecraft/world/entity/Entity#hasGlowingTag` | method | 1 |
| `net/minecraft/world/entity/Entity#isAlive` | method | 4 |
| `net/minecraft/world/entity/Entity#isCurrentlyGlowing` | method | 1 |
| `net/minecraft/world/entity/Entity#isInvulnerableToBase` | method | 1 |
| `net/minecraft/world/entity/Entity#isPickable` | method | 2 |
| `net/minecraft/world/entity/Entity#isSpectator` | method | 1 |
| `net/minecraft/world/entity/Entity#move` | method | 1 |
| `net/minecraft/world/entity/Entity#position` | method | 4 |
| `net/minecraft/world/entity/EntityType$Builder#build` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#of` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#sized` | method | 1 |
| `net/minecraft/world/entity/EntityTypes#BLOCK_DISPLAY` | field | 3 |
| `net/minecraft/world/entity/EquipmentSlotGroup#MAINHAND` | field | 1 |
| `net/minecraft/world/entity/ExperienceOrb#<init>` | method | 1 |
| `net/minecraft/world/entity/ExperienceOrb#count` | method | 1 |
| `net/minecraft/world/entity/ExperienceOrb#discard` | method | 1 |
| `net/minecraft/world/entity/ExperienceOrb#getValue` | method | 1 |
| `net/minecraft/world/entity/ExperienceOrb#getX` | method | 1 |
| `net/minecraft/world/entity/ExperienceOrb#getY` | method | 1 |
| `net/minecraft/world/entity/ExperienceOrb#getZ` | method | 1 |
| `net/minecraft/world/entity/ExperienceOrb#setDeltaMovement` | method | 1 |
| `net/minecraft/world/entity/HumanoidArm#RIGHT` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#addEffect` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#distanceToSqr` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getBbWidth` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getBoundingBox` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getDeltaMovement` | method | 3 |
| `net/minecraft/world/entity/LivingEntity#getEyePosition` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getId` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getMainArm` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getMainHandItem` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getMaxHealth` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getOffhandItem` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getRemainingFireTicks` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getTicksUsingItem` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getUUID` | method | 3 |
| `net/minecraft/world/entity/LivingEntity#getUseItem` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getUsedItemHand` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getX` | method | 4 |
| `net/minecraft/world/entity/LivingEntity#getY` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getYHeadRot` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getZ` | method | 4 |
| `net/minecraft/world/entity/LivingEntity#hurtServer` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isAlive` | method | 6 |
| `net/minecraft/world/entity/LivingEntity#isDeadOrDying` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isPassengerOfSameVehicle` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isSpectator` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isUsingItem` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#level` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#position` | method | 6 |
| `net/minecraft/world/entity/LivingEntity#push` | method | 5 |
| `net/minecraft/world/entity/LivingEntity#setDeltaMovement` | method | 5 |
| `net/minecraft/world/entity/LivingEntity#setHealth` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#setRemainingFireTicks` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#syncVelocity` | field | 7 |
| `net/minecraft/world/entity/LivingEntity#yBodyRot` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#yBodyRotO` | field | 1 |
| `net/minecraft/world/entity/MobCategory#MISC` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier$Operation#ADD_VALUE` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#ATTACK_DAMAGE` | field | 1 |
| `net/minecraft/world/entity/boss/enderdragon/EnderDragon#head` | field | 1 |
| `net/minecraft/world/entity/item/FallingBlockEntity#fall` | method | 2 |
| `net/minecraft/world/entity/item/FallingBlockEntity#getY` | method | 1 |
| `net/minecraft/world/entity/item/FallingBlockEntity#isAlive` | method | 1 |
| `net/minecraft/world/entity/item/FallingBlockEntity#position` | method | 1 |
| `net/minecraft/world/entity/item/FallingBlockEntity#setDeltaMovement` | method | 2 |
| `net/minecraft/world/entity/item/FallingBlockEntity#syncVelocity` | field | 2 |
| `net/minecraft/world/entity/player/Abilities#flying` | field | 2 |
| `net/minecraft/world/entity/player/Input#EMPTY` | field | 1 |
| `net/minecraft/world/entity/player/Inventory#getNonEquipmentItems` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#getSelectedSlot` | method | 2 |
| `net/minecraft/world/entity/player/Player#getEyePosition` | method | 4 |
| `net/minecraft/world/entity/player/Player#getHealth` | method | 1 |
| `net/minecraft/world/entity/player/Player#getInventory` | method | 1 |
| `net/minecraft/world/entity/player/Player#getItemInHand` | method | 2 |
| `net/minecraft/world/entity/player/Player#getMainHandItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#getOffhandItem` | method | 2 |
| `net/minecraft/world/entity/player/Player#getTicksUsingItem` | method | 2 |
| `net/minecraft/world/entity/player/Player#getUUID` | method | 1 |
| `net/minecraft/world/entity/player/Player#getUseItem` | method | 3 |
| `net/minecraft/world/entity/player/Player#getViewXRot` | method | 1 |
| `net/minecraft/world/entity/player/Player#getViewYRot` | method | 1 |
| `net/minecraft/world/entity/player/Player#getX` | method | 1 |
| `net/minecraft/world/entity/player/Player#getY` | method | 1 |
| `net/minecraft/world/entity/player/Player#getZ` | method | 1 |
| `net/minecraft/world/entity/player/Player#hurtServer` | method | 1 |
| `net/minecraft/world/entity/player/Player#isCreative` | method | 2 |
| `net/minecraft/world/entity/player/Player#isShiftKeyDown` | method | 1 |
| `net/minecraft/world/entity/player/Player#isSpectator` | method | 2 |
| `net/minecraft/world/entity/player/Player#isUsingItem` | method | 3 |
| `net/minecraft/world/entity/player/Player#level` | method | 3 |
| `net/minecraft/world/entity/player/Player#pick` | method | 1 |
| `net/minecraft/world/entity/player/Player#position` | method | 1 |
| `net/minecraft/world/entity/player/Player#push` | method | 1 |
| `net/minecraft/world/entity/player/Player#sendOverlayMessage` | method | 1 |
| `net/minecraft/world/entity/player/Player#startUsingItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#stopUsingItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#syncVelocity` | field | 1 |
| `net/minecraft/world/entity/projectile/Projectile#<init>` | method | 2 |
| `net/minecraft/world/entity/projectile/Projectile#addAdditionalSaveData` | method | 2 |
| `net/minecraft/world/entity/projectile/Projectile#canHitEntity` | method | 1 |
| `net/minecraft/world/entity/projectile/Projectile#getDeltaMovement` | method | 1 |
| `net/minecraft/world/entity/projectile/Projectile#getOwner` | method | 1 |
| `net/minecraft/world/entity/projectile/Projectile#onHit` | method | 1 |
| `net/minecraft/world/entity/projectile/Projectile#position` | method | 1 |
| `net/minecraft/world/entity/projectile/Projectile#readAdditionalSaveData` | method | 2 |
| `net/minecraft/world/entity/projectile/Projectile#setDeltaMovement` | method | 1 |
| `net/minecraft/world/entity/projectile/Projectile#setPos` | method | 1 |
| `net/minecraft/world/entity/projectile/Projectile#syncVelocity` | field | 1 |
| `net/minecraft/world/entity/projectile/Projectile#tick` | method | 2 |
| `net/minecraft/world/entity/projectile/ProjectileUtil#getHitResultOnMoveVector` | method | 2 |
| `net/minecraft/world/entity/projectile/hurtingprojectile/AbstractHurtingProjectile#accelerationPower` | field | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#broadcastChanges` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#stillValid` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Builder#build` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Builder#displayItems` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Builder#icon` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Builder#title` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Output#accept` | method | 1 |
| `net/minecraft/world/item/CreativeModeTabs#COMBAT` | field | 1 |
| `net/minecraft/world/item/Item#appendHoverText` | method | 1 |
| `net/minecraft/world/item/Item$Properties#<init>` | method | 1 |
| `net/minecraft/world/item/Item$Properties#attributes` | method | 1 |
| `net/minecraft/world/item/Item$Properties#setId` | method | 1 |
| `net/minecraft/world/item/ItemDisplayContext#FIRST_PERSON_LEFT_HAND` | field | 2 |
| `net/minecraft/world/item/ItemDisplayContext#FIRST_PERSON_RIGHT_HAND` | field | 2 |
| `net/minecraft/world/item/ItemDisplayContext#FIXED` | field | 2 |
| `net/minecraft/world/item/ItemDisplayContext#GROUND` | field | 2 |
| `net/minecraft/world/item/ItemDisplayContext#GUI` | field | 2 |
| `net/minecraft/world/item/ItemDisplayContext#THIRD_PERSON_LEFT_HAND` | field | 2 |
| `net/minecraft/world/item/ItemDisplayContext#THIRD_PERSON_RIGHT_HAND` | field | 2 |
| `net/minecraft/world/item/ItemDisplayContext#ordinal` | method | 2 |
| `net/minecraft/world/item/ItemDisplayContext#values` | method | 1 |
| `net/minecraft/world/item/ItemStack#<init>` | method | 1 |
| `net/minecraft/world/item/ItemStack#get` | method | 2 |
| `net/minecraft/world/item/ItemStack#getItem` | method | 16 |
| `net/minecraft/world/item/ItemStack#is` | method | 4 |
| `net/minecraft/world/item/ItemUseAnimation#BOW` | field | 1 |
| `net/minecraft/world/item/ItemUseAnimation#NONE` | field | 1 |
| `net/minecraft/world/item/component/CustomData#copyTag` | method | 2 |
| `net/minecraft/world/item/component/CustomData#update` | method | 2 |
| `net/minecraft/world/item/component/ItemAttributeModifiers#builder` | method | 1 |
| `net/minecraft/world/item/component/ItemAttributeModifiers$Builder#add` | method | 1 |
| `net/minecraft/world/item/component/ItemAttributeModifiers$Builder#build` | method | 1 |
| `net/minecraft/world/item/component/SwingAnimation#DEFAULT` | field | 1 |
| `net/minecraft/world/level/ClipContext#<init>` | method | 5 |
| `net/minecraft/world/level/ClipContext$Block#COLLIDER` | field | 5 |
| `net/minecraft/world/level/ClipContext$Fluid#NONE` | field | 5 |
| `net/minecraft/world/level/Level#OVERWORLD` | field | 3 |
| `net/minecraft/world/level/Level#addFreshEntity` | method | 1 |
| `net/minecraft/world/level/Level#addParticle` | method | 1 |
| `net/minecraft/world/level/Level#dimension` | method | 1 |
| `net/minecraft/world/level/Level#getBlockState` | method | 1 |
| `net/minecraft/world/level/Level#getChunkSource` | method | 1 |
| `net/minecraft/world/level/Level#playSound` | method | 1 |
| `net/minecraft/world/level/block/BaseFireBlock#getState` | method | 1 |
| `net/minecraft/world/level/block/BedBlock#FACING` | field | 1 |
| `net/minecraft/world/level/block/BedBlock#PART` | field | 1 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 7 |
| `net/minecraft/world/level/block/Blocks#AIR` | field | 5 |
| `net/minecraft/world/level/block/Blocks#AMETHYST_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BARRIER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BASALT` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BEDROCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BLACKSTONE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#COARSE_DIRT` | field | 1 |
| `net/minecraft/world/level/block/Blocks#COBBLESTONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#END_PORTAL` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LAVA` | field | 1 |
| `net/minecraft/world/level/block/Blocks#MAGMA_BLOCK` | field | 2 |
| `net/minecraft/world/level/block/Blocks#OBSIDIAN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WATER` | field | 1 |
| `net/minecraft/world/level/block/DoorBlock#HALF` | field | 1 |
| `net/minecraft/world/level/block/DoublePlantBlock#HALF` | field | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#clearRemoved` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#loadWithComponents` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#saveWithFullMetadata` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#setChanged` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#equals` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#getDestroySpeed` | method | 7 |
| `net/minecraft/world/level/block/state/BlockState#getFluidState` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#getValue` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#hasBlockEntity` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#hasProperty` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#ignitedByLava` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#is` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#isAir` | method | 8 |
| `net/minecraft/world/level/block/state/BlockState#isCollisionShapeFullBlock` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#isSolidRender` | method | 1 |
| `net/minecraft/world/level/block/state/properties/BedPart#FOOT` | field | 1 |
| `net/minecraft/world/level/block/state/properties/DoubleBlockHalf#LOWER` | field | 1 |
| `net/minecraft/world/level/block/state/properties/DoubleBlockHalf#UPPER` | field | 1 |
| `net/minecraft/world/level/border/WorldBorder#isWithinBounds` | method | 2 |
| `net/minecraft/world/level/chunk/ChunkSource#hasChunk` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getBlockState` | method | 4 |
| `net/minecraft/world/level/chunk/LevelChunk#getHeight` | method | 4 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#MOTION_BLOCKING` | field | 3 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#MOTION_BLOCKING_NO_LEAVES` | field | 1 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#WORLD_SURFACE` | field | 4 |
| `net/minecraft/world/level/material/FlowingFluid#canMaybePassThrough` | method | 1 |
| `net/minecraft/world/level/material/FlowingFluid#canPassThrough` | method | 1 |
| `net/minecraft/world/level/material/FluidState#isEmpty` | method | 3 |
| `net/minecraft/world/level/saveddata/SavedData#<init>` | method | 1 |
| `net/minecraft/world/level/saveddata/SavedDataType#<init>` | method | 1 |
| `net/minecraft/world/level/storage/SavedDataStorage#computeIfAbsent` | method | 1 |
| `net/minecraft/world/level/storage/TagValueInput#create` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#getIntOr` | method | 2 |
| `net/minecraft/world/level/storage/ValueOutput#putInt` | method | 2 |
| `net/minecraft/world/phys/AABB#<init>` | method | 12 |
| `net/minecraft/world/phys/AABB#clip` | method | 2 |
| `net/minecraft/world/phys/AABB#contains` | method | 2 |
| `net/minecraft/world/phys/AABB#getCenter` | method | 1 |
| `net/minecraft/world/phys/AABB#inflate` | method | 8 |
| `net/minecraft/world/phys/AABB#maxX` | field | 2 |
| `net/minecraft/world/phys/AABB#maxY` | field | 2 |
| `net/minecraft/world/phys/AABB#maxZ` | field | 2 |
| `net/minecraft/world/phys/AABB#minX` | field | 2 |
| `net/minecraft/world/phys/AABB#minY` | field | 2 |
| `net/minecraft/world/phys/AABB#minZ` | field | 2 |
| `net/minecraft/world/phys/AABB#move` | method | 1 |
| `net/minecraft/world/phys/AABB#ofSize` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#getBlockPos` | method | 4 |
| `net/minecraft/world/phys/BlockHitResult#getDirection` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#getLocation` | method | 4 |
| `net/minecraft/world/phys/BlockHitResult#getType` | method | 5 |
| `net/minecraft/world/phys/EntityHitResult#getEntity` | method | 1 |
| `net/minecraft/world/phys/HitResult#getLocation` | method | 2 |
| `net/minecraft/world/phys/HitResult#getType` | method | 5 |
| `net/minecraft/world/phys/HitResult$Type#BLOCK` | field | 3 |
| `net/minecraft/world/phys/HitResult$Type#MISS` | field | 7 |
| `net/minecraft/world/phys/Vec2#ZERO` | field | 1 |
| `net/minecraft/world/phys/Vec3#<init>` | method | 30 |
| `net/minecraft/world/phys/Vec3#ZERO` | field | 13 |
| `net/minecraft/world/phys/Vec3#add` | method | 50 |
| `net/minecraft/world/phys/Vec3#atCenterOf` | method | 11 |
| `net/minecraft/world/phys/Vec3#atLowerCornerOf` | method | 1 |
| `net/minecraft/world/phys/Vec3#cross` | method | 10 |
| `net/minecraft/world/phys/Vec3#distanceTo` | method | 16 |
| `net/minecraft/world/phys/Vec3#distanceToSqr` | method | 11 |
| `net/minecraft/world/phys/Vec3#dot` | method | 8 |
| `net/minecraft/world/phys/Vec3#horizontalDistance` | method | 3 |
| `net/minecraft/world/phys/Vec3#length` | method | 12 |
| `net/minecraft/world/phys/Vec3#lengthSqr` | method | 16 |
| `net/minecraft/world/phys/Vec3#lerp` | method | 15 |
| `net/minecraft/world/phys/Vec3#multiply` | method | 2 |
| `net/minecraft/world/phys/Vec3#reverse` | method | 5 |
| `net/minecraft/world/phys/Vec3#scale` | method | 27 |
| `net/minecraft/world/phys/Vec3#subtract` | method | 27 |
| `net/minecraft/world/phys/Vec3#x` | field | 35 |
| `net/minecraft/world/phys/Vec3#y` | field | 36 |
| `net/minecraft/world/phys/Vec3#z` | field | 35 |
| `net/minecraft/world/phys/shapes/CollisionContext#of` | method | 1 |
| `net/minecraft/world/phys/shapes/Shapes#block` | method | 1 |
| `net/minecraft/world/phys/shapes/Shapes#empty` | method | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#isEmpty` | method | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#max` | method | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#toAabbs` | method | 1 |

</details>

