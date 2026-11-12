// God Wand, ported to Horizon by horizon-port from godwand-fabric-26.3-1.4.2.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_godwand). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "godwand";
pub const NAME: &str = "God Wand";
pub const VERSION: &str = "1.4.2";
pub const DESCRIPTION: &str = "Six spells in one wand. Shape terrain, control gravity and hold I am Atomic until you choose to release.";

/// The mod's keys for Options > Mods: name, label, default key.
pub const KEYS: &[[&str; 3]] = &[];

/// The mod's own Java that doesn't need the game, translated to Rust by horizon-port.
/// Each Java method is here whole or not at all; the ones left out say why.
#[allow(dead_code, unused_variables, unused_mut, unused_parens, unused_imports, unreachable_code,
    non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
pub mod java {
    use std::sync::{LazyLock, Mutex};

    /// A Java static that changes: one value behind a lock.
    pub struct JCell<T>(Mutex<T>);
    impl<T: Clone> JCell<T> {
        pub const fn new(v: T) -> Self {
            Self(Mutex::new(v))
        }
        pub fn get(&self) -> T {
            self.0.lock().unwrap_or_else(|e| e.into_inner()).clone()
        }
        pub fn set(&self, v: T) {
            *self.0.lock().unwrap_or_else(|e| e.into_inner()) = v;
        }
        pub fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            f(&mut self.0.lock().unwrap_or_else(|e| e.into_inner()))
        }
    }

    /// Java's HashMap, LinkedHashMap and TreeMap. Keeps insertion order (LinkedHashMap's; HashMap's
    /// order is unspecified), or key order when made with `new_sorted` (TreeMap's).
    #[derive(Clone, Debug)]
    pub struct JMap<K, V> {
        index: std::collections::HashMap<K, usize>,
        entries: Vec<(K, V)>,
        sorted: bool,
    }
    /// Java's Map.equals: the same keys with equal values, in any order.
    impl<K: std::hash::Hash + Eq, V: PartialEq> PartialEq for JMap<K, V> {
        fn eq(&self, o: &Self) -> bool {
            self.entries.len() == o.entries.len()
                && self.entries.iter().all(|(k, v)| o.index.get(k).is_some_and(|&i| o.entries[i].1 == *v))
        }
    }
    impl<K, V> Default for JMap<K, V> {
        fn default() -> Self {
            Self { index: std::collections::HashMap::new(), entries: Vec::new(), sorted: false }
        }
    }
    impl<K: std::hash::Hash + Ord + Clone, V> JMap<K, V> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self { sorted: true, ..Self::default() }
        }
        pub fn from_iter(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn from_iter_sorted(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new_sorted();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn len(&self) -> usize {
            self.entries.len()
        }
        pub fn is_empty(&self) -> bool {
            self.entries.is_empty()
        }
        pub fn clear(&mut self) {
            self.index.clear();
            self.entries.clear();
        }
        pub fn contains_key<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> bool
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.contains_key(k)
        }
        pub fn get<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> Option<&V>
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.get(k).map(|&i| &self.entries[i].1)
        }
        fn reindex(&mut self) {
            self.index.clear();
            for (i, (k, _)) in self.entries.iter().enumerate() {
                self.index.insert(k.clone(), i);
            }
        }
        fn slot(&mut self, k: K, v: V) -> usize {
            if self.sorted {
                let at = self.entries.partition_point(|e| e.0 < k);
                self.entries.insert(at, (k, v));
                self.reindex();
                at
            } else {
                self.index.insert(k.clone(), self.entries.len());
                self.entries.push((k, v));
                self.entries.len() - 1
            }
        }
        pub fn insert(&mut self, k: K, v: V) -> Option<V> {
            if let Some(&i) = self.index.get(&k) {
                return Some(std::mem::replace(&mut self.entries[i].1, v));
            }
            self.slot(k, v);
            None
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, k: &Q) -> Option<V>
        where
            K: std::borrow::Borrow<Q>,
        {
            let i = *self.index.get(k)?;
            let (_, v) = self.entries.remove(i);
            self.reindex();
            Some(v)
        }
        /// computeIfAbsent and putIfAbsent: the value, made first when the key is missing.
        pub fn or_insert_with(&mut self, k: K, f: impl FnOnce() -> V) -> &mut V {
            let i = match self.index.get(&k) {
                Some(&i) => i,
                None => self.slot(k, f()),
            };
            &mut self.entries[i].1
        }
        pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
            self.entries.iter().map(|(k, v)| (k, v))
        }
        pub fn keys(&self) -> impl Iterator<Item = &K> {
            self.entries.iter().map(|e| &e.0)
        }
        pub fn values(&self) -> impl Iterator<Item = &V> {
            self.entries.iter().map(|e| &e.1)
        }
        pub fn clone_pairs(&self) -> Vec<(K, V)>
        where
            V: Clone,
        {
            self.entries.clone()
        }
    }

    /// Java's HashSet, LinkedHashSet and TreeSet, kept like JMap's keys.
    #[derive(Clone, Debug)]
    pub struct JSet<T>(JMap<T, ()>);
    impl<T: std::hash::Hash + Eq> PartialEq for JSet<T> {
        fn eq(&self, o: &Self) -> bool {
            self.0 == o.0
        }
    }
    impl<T> Default for JSet<T> {
        fn default() -> Self {
            Self(JMap::default())
        }
    }
    impl<T: std::hash::Hash + Ord + Clone> JSet<T> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self(JMap::new_sorted())
        }
        pub fn from_iter(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn from_iter_sorted(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new_sorted();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn len(&self) -> usize {
            self.0.len()
        }
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
        pub fn clear(&mut self) {
            self.0.clear();
        }
        /// True when it wasn't there yet, as Java's add.
        pub fn insert(&mut self, v: T) -> bool {
            self.0.insert(v, ()).is_none()
        }
        pub fn contains<Q: std::hash::Hash + Eq + ?Sized>(&self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.contains_key(v)
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.remove(v).is_some()
        }
        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.0.keys()
        }
        pub fn clone_pairs(&self) -> Vec<T> {
            self.0.keys().cloned().collect()
        }
    }

    /// The mod's data folder in Horizon, set by the hooks (game_side.rs `ensure_init`) before the mod's code runs.
    pub static J_DATA_DIR: JCell<Option<std::path::PathBuf>> = JCell::new(None);
    /// FabricLoader's config folder: the mod's data folder in Horizon (./config until the game sets it).
    pub fn j_config_dir() -> std::path::PathBuf {
        J_DATA_DIR.get().unwrap_or_else(|| std::path::PathBuf::from("config"))
    }

    /// Minecraft's RandomSource / java.util.Random: other numbers than Java's, the same ranges.
    #[derive(Clone, Debug)]
    pub struct JRandom(u64);
    impl JRandom {
        pub fn new() -> Self {
            let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(0x9E37_79B9);
            Self(t | 1)
        }
        fn next(&mut self) -> u64 {
            let mut v = self.0;
            v ^= v << 13;
            v ^= v >> 7;
            v ^= v << 17;
            self.0 = v;
            v
        }
        /// 0 inclusive to n exclusive; Java throws on n <= 0.
        pub fn next_int(&mut self, n: i32) -> i32 {
            assert!(n > 0, "bound must be positive");
            (self.next() % n as u64) as i32
        }
        pub fn next_int_between(&mut self, lo: i32, hi: i32) -> i32 {
            lo + self.next_int(hi - lo)
        }
        pub fn next_float(&mut self) -> f32 {
            (self.next() >> 40) as f32 / (1u64 << 24) as f32
        }
        pub fn next_double(&mut self) -> f64 {
            (self.next() >> 11) as f64 / (1u64 << 53) as f64
        }
        pub fn next_boolean(&mut self) -> bool {
            self.next() & 1 == 1
        }
        pub fn next_gaussian(&mut self) -> f64 {
            let (u, v) = (self.next_double().max(1e-12), self.next_double());
            (-2.0 * u.ln()).sqrt() * (std::f64::consts::TAU * v).cos()
        }
    }

    /// Minecraft's Identifier (`namespace:path`), which is only text.
    #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct JIdentifier {
        pub namespace: String,
        pub path: String,
    }
    impl JIdentifier {
        pub fn new(namespace: &str, path: &str) -> Self {
            Self { namespace: namespace.to_string(), path: path.to_string() }
        }
        /// `a:b`, or `minecraft:b` without a namespace.
        pub fn parse(s: &str) -> Self {
            match s.split_once(':') {
                Some((n, p)) => Self::new(n, p),
                None => Self::new("minecraft", s),
            }
        }
        /// Identifier.tryParse: None (null) when it has characters an identifier can't.
        pub fn try_parse(s: &str) -> Option<Self> {
            let id = Self::parse(s);
            Self::try_build(&id.namespace, &id.path)
        }
        pub fn try_build(namespace: &str, path: &str) -> Option<Self> {
            let ns_ok = namespace.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-'));
            let path_ok = path.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-' | '/'));
            if ns_ok && path_ok { Some(Self::new(namespace, path)) } else { None }
        }
    }
    impl std::fmt::Display for JIdentifier {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}:{}", self.namespace, self.path)
        }
    }

    /// Java's String.equals.
    pub fn j_eq(a: &str, b: &str) -> bool {
        a == b
    }
    pub fn j_eq_ignore_case(a: &str, b: &str) -> bool {
        a.to_lowercase() == b.to_lowercase()
    }
    /// Java's String.length (in chars).
    pub fn j_len(s: &str) -> i32 {
        s.chars().count() as i32
    }
    pub fn j_char_at(s: &str, i: i32) -> char {
        s.chars().nth(i as usize).expect("charAt out of range")
    }
    /// Java's substring, by char.
    pub fn j_substring(s: &str, from: i32, to: Option<i32>) -> String {
        let n = s.chars().count() as i32;
        let to = to.unwrap_or(n);
        assert!(0 <= from && from <= to && to <= n, "substring out of range");
        s.chars().skip(from as usize).take((to - from) as usize).collect()
    }
    pub fn j_index_of(s: &str, sub: &str) -> i32 {
        s.find(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    pub fn j_index_of_char(s: &str, c: char) -> i32 {
        s.chars().position(|x| x == c).map_or(-1, |i| i as i32)
    }
    pub fn j_last_index_of(s: &str, sub: &str) -> i32 {
        s.rfind(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    /// Java's String.split on plain text: trailing empty parts dropped.
    pub fn j_split(s: &str, sep: &str) -> Vec<String> {
        let mut v: Vec<String> = s.split(sep).map(str::to_string).collect();
        while v.len() > 1 && v.last().is_some_and(|p| p.is_empty()) {
            v.pop();
        }
        v
    }
    /// Java's String.split(sep, limit) on plain text.
    pub fn j_split_limit(s: &str, sep: &str, limit: i32) -> Vec<String> {
        if limit == 0 {
            return j_split(s, sep);
        }
        if limit > 0 {
            return s.splitn(limit as usize, sep).map(str::to_string).collect();
        }
        s.split(sep).map(str::to_string).collect()
    }
    /// Java's String.compareTo: by UTF-16 unit, then by length.
    pub fn j_compare(a: &str, b: &str) -> i32 {
        let (x, y): (Vec<u16>, Vec<u16>) = (a.encode_utf16().collect(), b.encode_utf16().collect());
        for (p, q) in x.iter().zip(y.iter()) {
            if p != q {
                return *p as i32 - *q as i32;
            }
        }
        x.len() as i32 - y.len() as i32
    }
    /// Java's String.regionMatches, by char.
    pub fn j_region_matches(s: &str, ignore_case: bool, from: i32, other: &str, other_from: i32, len: i32) -> bool {
        let (a, b): (Vec<char>, Vec<char>) = (s.chars().collect(), other.chars().collect());
        if from < 0 || other_from < 0 || from as i64 + len as i64 > a.len() as i64 || other_from as i64 + len as i64 > b.len() as i64 {
            return false;
        }
        (0..len.max(0) as usize).all(|i| {
            let (p, q) = (a[from as usize + i], b[other_from as usize + i]);
            p == q || ignore_case && (j_upper_char(p) == j_upper_char(q) || j_lower_char(p) == j_lower_char(q))
        })
    }
    /// Java's Character.toLowerCase / toUpperCase (one char).
    pub fn j_lower_char(c: char) -> char {
        let mut l = c.to_lowercase();
        if l.len() == 1 { l.next().unwrap_or(c) } else { c }
    }
    pub fn j_upper_char(c: char) -> char {
        let mut u = c.to_uppercase();
        if u.len() == 1 { u.next().unwrap_or(c) } else { c }
    }
    /// Java's Arrays.copyOfRange: past the end is filled with defaults.
    pub fn j_copy_of_range<T: Clone + Default>(a: &[T], from: usize, to: usize) -> Vec<T> {
        assert!(from <= to && from <= a.len(), "copyOfRange out of range");
        let mut v: Vec<T> = a[from..to.min(a.len())].to_vec();
        v.resize(to - from, T::default());
        v
    }
    pub fn j_hash(s: &str) -> i32 {
        s.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
    }
    /// Java's Boolean.parseBoolean.
    pub fn j_parse_bool(s: &str) -> bool {
        s.eq_ignore_ascii_case("true")
    }
    /// A float as Java prints it in "..." + x (1.0, not 1).
    pub fn j_float(x: f64) -> String {
        if x.is_finite() && x == x.trunc() && x.abs() < 1e7 { format!("{x:.1}") } else { format!("{x}") }
    }
    /// Java's Math.round.
    pub fn j_round_f(x: f32) -> i32 {
        (x + 0.5).floor() as i32
    }
    pub fn j_round_d(x: f64) -> i64 {
        (x + 0.5).floor() as i64
    }
    /// Java's Math.clamp / Mth.clamp (no panic when min > max).
    pub fn j_clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
        if v < min { min } else if v > max { max } else { v }
    }
    pub fn j_lerp<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>>(d: T, a: T, b: T) -> T {
        a + d * (b - a)
    }
    pub fn j_signum_f(x: f32) -> f32 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_signum_d(x: f64) -> f64 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_wrap_degrees(x: f64) -> f64 {
        let mut v = x % 360.0;
        if v >= 180.0 {
            v -= 360.0;
        }
        if v < -180.0 {
            v += 360.0;
        }
        v
    }
    pub fn j_frac(x: f64) -> f64 {
        x - x.floor()
    }
    pub fn j_compare_f(a: f64, b: f64) -> i32 {
        a.partial_cmp(&b).map_or(0, |o| o as i32)
    }
    pub fn j_copy_of<T: Clone + Default>(a: &[T], n: usize) -> Vec<T> {
        let mut v: Vec<T> = a.iter().take(n).cloned().collect();
        v.resize(n, T::default());
        v
    }
    /// Java's System.currentTimeMillis.
    pub fn j_millis() -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
    }
    /// Java's System.nanoTime: only differences mean anything.
    pub fn j_nanos() -> i64 {
        static START: LazyLock<std::time::Instant> = LazyLock::new(std::time::Instant::now);
        START.elapsed().as_nanos() as i64
    }

    /// `ArcaneOrbChargeRenderer` (net/godwand/client/ArcaneOrbChargeRenderer.java)
    pub mod arcane_orb_charge_renderer {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ArcaneOrbChargeRenderer {
        }
        impl ArcaneOrbChargeRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ArcaneOrbChargeRenderer.render(3): not translated, it uses Minecraft's `Camera`.
    }

    /// `ArcaneOrbEntityRenderer` (net/godwand/client/ArcaneOrbEntityRenderer.java)
    pub mod arcane_orb_entity_renderer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EntityRenderer`. Its static parts are below.
        // ArcaneOrbEntityRenderer.ArcaneOrbEntityRenderer(1): not translated, its class isn't translated (it extends Minecraft's `EntityRenderer`).
        // ArcaneOrbEntityRenderer.createRenderState(0): not translated, its class isn't translated (it extends Minecraft's `EntityRenderer`).
        // ArcaneOrbEntityRenderer.extractRenderState(3): not translated, its class isn't translated (it extends Minecraft's `EntityRenderer`).
        // ArcaneOrbEntityRenderer.submit(4): not translated, its class isn't translated (it extends Minecraft's `EntityRenderer`).
    }

    /// `ArcaneOrbEntityRenderer.State` (net/godwand/client/ArcaneOrbEntityRenderer.java)
    pub mod arcane_orb_entity_renderer_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EntityRenderState`. Its static parts are below.
    }

    /// `ArcaneOrbModel` (net/godwand/client/ArcaneOrbModel.java)
    pub mod arcane_orb_model {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ArcaneOrbModel {
        }
        impl ArcaneOrbModel {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn charge_glow_alpha(radius: f32) -> f32 {
            let edge: f32 = 0.0f32.max(1.0f32 - (radius * radius));
            return edge * edge;
        }
        // ArcaneOrbModel.renderCharge(4): not translated, a try/finally.
        // ArcaneOrbModel.render(4): not translated, a try/finally.
        // ArcaneOrbModel.renderSphere(9): not translated, it uses Minecraft's `BufferBuilder`.
        // ArcaneOrbModel.renderEnergyRing(10): not translated, it uses Minecraft's `BufferBuilder`.
        // ArcaneOrbModel.renderInwardSpirals(4): not translated, it uses Minecraft's `BufferBuilder`.
    }

    /// `AtomicCinematic` (net/godwand/client/AtomicCinematic.java)
    pub mod atomic_cinematic {
        use super::*;
        pub static TITLE_FONT: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("godwand", "atomic_title"));
        pub static SKIPPED: JCell<bool> = JCell::new(false);
        pub static MOVEMENT_RELEASED: JCell<bool> = JCell::new(false);
        pub static MOVEMENT_RELEASE_SENT: JCell<bool> = JCell::new(false);
        pub static WIDE_STARTED: JCell<i64> = JCell::new(i64::MIN);
        pub static WIDE_ANGLE: JCell<f32> = JCell::new(0.0f32);
        pub static WIDE_HEIGHT: JCell<f64> = JCell::new(0.0f64);
        pub static ENVIRONMENT_GAIN: JCell<f32> = JCell::new(1.0f32);
        #[derive(Clone, Debug)]
        pub struct AtomicCinematic {
        }
        impl AtomicCinematic {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // AtomicCinematic.movementLocked(0): not translated, it uses Minecraft's `Minecraft`.
        // AtomicCinematic.active(0): not translated, it uses Minecraft's `Minecraft`.
        // AtomicCinematic.tick(1): not translated, it uses Minecraft's `Minecraft`.
        pub fn reset() {
            atomic_cinematic::SKIPPED.set(false);
            atomic_cinematic::MOVEMENT_RELEASED.set(false);
            atomic_cinematic::MOVEMENT_RELEASE_SENT.set(false);
            atomic_cinematic::WIDE_STARTED.set(i64::MIN);
            atomic_cinematic::ENVIRONMENT_GAIN.set(1.0f32);
        }
        pub fn on_cast_start() {
            atomic_cinematic::SKIPPED.set(false);
            atomic_cinematic::MOVEMENT_RELEASED.set(false);
            atomic_cinematic::MOVEMENT_RELEASE_SENT.set(false);
            atomic_cinematic::WIDE_STARTED.set(i64::MIN);
        }
        // AtomicCinematic.skip(0): not translated, it uses Frame, which isn't translated (it uses Minecraft's `Vec3`).
        // AtomicCinematic.camera(4): not translated, it uses Minecraft's `Vec3`.
        // AtomicCinematic.wideClearance(4): not translated, it uses Minecraft's `Minecraft`.
        // AtomicCinematic.loadedNeighborhood(2): not translated, it uses Minecraft's `Minecraft`.
        // AtomicCinematic.fov(2): not translated, it calls AtomicCinematic.active, which isn't translated.
        pub fn environment_gain() -> f32 {
            return atomic_cinematic::ENVIRONMENT_GAIN.get();
        }
        // AtomicCinematic.applyShake(2): not translated, it uses Minecraft's `Minecraft`.
        // AtomicCinematic.render(2): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // AtomicCinematic.releaseHint(2): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // AtomicCinematic.title(7): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // AtomicCinematic.lettering(1): not translated, it uses Minecraft's `Component`.
    }

    /// `AtomicCinematic.Pose` (net/godwand/client/AtomicCinematic.java)
    pub mod atomic_cinematic_pose {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // AtomicCinematic.Pose.Pose(3): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AtomicCinematic.Pose.position(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AtomicCinematic.Pose.yaw(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AtomicCinematic.Pose.pitch(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `AtomicPoseFrames` (net/godwand/client/AtomicPoseFrames.java)
    pub mod atomic_pose_frames {
        use super::*;
        // POSES: not translated, it uses Minecraft's `HumanoidRenderState`.
        // IDS: not translated, it uses Minecraft's `HumanoidRenderState`.
        // PARTIALS: not translated, it uses Minecraft's `HumanoidRenderState`.
        #[derive(Clone, Debug)]
        pub struct AtomicPoseFrames {
        }
        impl AtomicPoseFrames {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // AtomicPoseFrames.entityId(1): not translated, it uses Minecraft's `HumanoidRenderState`.
        // AtomicPoseFrames.partial(1): not translated, it uses Minecraft's `HumanoidRenderState`.
        // AtomicPoseFrames.capture(3): not translated, it uses Minecraft's `LivingEntity`.
        // AtomicPoseFrames.get(1): not translated, it uses Pose, which isn't translated (it keeps an atomic value in an object's field).
    }

    /// `AtomicPoseFrames.Pose` (net/godwand/client/AtomicPoseFrames.java)
    pub mod atomic_pose_frames_pose {
        use super::*;
        // The class itself isn't translated: it keeps an atomic value in an object's field. Its static parts are below.
        // AtomicPoseFrames.Pose.Pose(5): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // AtomicPoseFrames.Pose.entityId(0): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // AtomicPoseFrames.Pose.charging(0): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // AtomicPoseFrames.Pose.rightHand(0): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // AtomicPoseFrames.Pose.age(0): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // AtomicPoseFrames.Pose.frame(0): not translated, its class isn't translated (it keeps an atomic value in an object's field).
    }

    /// `AtomicPostFx` (net/godwand/client/AtomicPostFx.java)
    pub mod atomic_post_fx {
        use super::*;
        pub static PENDING: LazyLock<JCell<Option<atomic_post_fx_frame::Frame>>> = LazyLock::new(|| JCell::new(None));
        // chain: not translated, it uses GodWandPostChain, which isn't translated (it implements `AutoCloseable`).
        pub static WIDTH: JCell<i32> = JCell::new(0i32);
        pub static HEIGHT: JCell<i32> = JCell::new(0i32);
        pub static DOWNSAMPLE: JCell<i32> = JCell::new(0i32);
        pub static FAILED: JCell<bool> = JCell::new(false);
        // irisInUse: not translated, it uses `Method` (java.lang.reflect).
        // iris: not translated, it uses `Object`.
        pub static IRIS_CHECKED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct AtomicPostFx {
        }
        impl AtomicPostFx {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // AtomicPostFx.capture(3): not translated, a nullable object kept in a changing static.
        // AtomicPostFx.available(0): not translated, a nullable object kept in a changing static.
        // AtomicPostFx.reset(0): not translated, it calls AtomicPostFx.reload, which isn't translated.
        // AtomicPostFx.reload(0): not translated, it uses AtomicPostFx.chain, which isn't translated (it uses GodWandPostChain, which isn't translated (it implements `AutoCloseable`)).
        // AtomicPostFx.enabled(0): not translated, a nullable object kept in a changing static.
        // AtomicPostFx.shaderPackActive(0): not translated, a try/catch (ReflectiveOperationException, LinkageError).
        // AtomicPostFx.create(1): not translated, it uses Minecraft's `Minecraft`.
        // AtomicPostFx.prewarm(0): not translated, it uses Minecraft's `Minecraft`.
        pub fn warm() {
        }
        // AtomicPostFx.renderAfterWorld(0): not translated, it uses Minecraft's `Minecraft`.
    }

    /// `AtomicPostFx.Frame` (net/godwand/client/AtomicPostFx.java)
    pub mod atomic_post_fx_frame {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Frame {
            pub view: JMat4,
            pub projection: JMat4,
            pub delta: f32,
        }
        impl Frame {
            pub fn new(view: JMat4, projection: JMat4, delta: f32) -> Self {
                let mut s = Self { view: view, projection: projection, delta: delta };
                s.view = view;
                s.projection = projection;
                s.delta = delta;
                s
            }
            pub fn view(&self) -> JMat4 {
                return self.view;
            }
            pub fn projection(&self) -> JMat4 {
                return self.projection;
            }
            pub fn delta(&self) -> f32 {
                return self.delta;
            }
        }
    }

    /// `AtomicRenderer` (net/godwand/client/AtomicRenderer.java)
    pub mod atomic_renderer {
        use super::*;
        // ACTIVE: not translated, it uses State, which isn't translated (it keeps an atomic value in an object's field).
        // AUDIO_TAILS: not translated, it uses State, which isn't translated (it keeps an atomic value in an object's field).
        // SCARS: not translated, it uses Scar, which isn't translated (it uses Minecraft's `Vec3`).
        pub static ENERGY_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("godwand", "atomic_energy"));
        pub static TEXTURE_READY: JCell<bool> = JCell::new(false);
        // world: not translated, it uses Minecraft's `ClientLevel`.
        pub static SCAR_HEIGHT_TICK: JCell<i64> = JCell::new(i64::MIN);
        pub static SCAR_HEIGHTS: LazyLock<JCell<JMap<i64, i32>>> = LazyLock::new(|| JCell::new(JMap::new()));
        #[derive(Clone, Debug)]
        pub struct AtomicRenderer {
        }
        impl AtomicRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // AtomicRenderer.receive(1): not translated, it uses `AtomicFxPayload` (net.godwand.network).
        // AtomicRenderer.sound(3): not translated, it uses Minecraft's `SimpleSoundInstance`.
        pub fn distance_gain(distance: f64) -> f32 {
            return ((j_clamp(1.0f64 - ((0.0f64.max(distance - 48.0f64)) / 208.0f64), 0.0f64, 1.0f64)) as f32);
        }
        // AtomicRenderer.stop(1): not translated, it uses State, which isn't translated (it keeps an atomic value in an object's field).
        // AtomicRenderer.tick(1): not translated, it uses Minecraft's `Minecraft`.
        // AtomicRenderer.frame(2): not translated, it uses State, which isn't translated (it keeps an atomic value in an object's field).
        // AtomicRenderer.localFrame(1): not translated, it uses Minecraft's `Minecraft`.
        // AtomicRenderer.frameFor(2): not translated, it uses AtomicRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`).
        // AtomicRenderer.heldFrame(2): not translated, it uses Minecraft's `ItemStack`.
        // AtomicRenderer.captureSwordTip(3): not translated, it uses State, which isn't translated (it keeps an atomic value in an object's field).
        // AtomicRenderer.sampleSwordTip(3): not translated, it uses Minecraft's `Vec3`.
        // AtomicRenderer.swordTrail(5): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.nearestFrame(1): not translated, it uses Minecraft's `Minecraft`.
        // AtomicRenderer.reset(0): not translated, it uses AtomicRenderer.ACTIVE, which isn't translated (it uses State, which isn't translated (it keeps an atomic value in an object's field)).
        // AtomicRenderer.render(3): not translated, it uses Minecraft's `Camera`.
        // AtomicRenderer.emit(4): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.charge(4): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.bladeConvergence(6): not translated, it uses Minecraft's `VertexConsumer`.
        pub fn charge_alpha(age: f64) -> f32 {
            return ((0.12f32 + (atomic_sequence::charge(age) * 0.65f32)) * (1.0f32 - atomic_sequence::blackout(age))) * atomic_sequence::phase(age, 0.0f64, 12.0f64);
        }
        // AtomicRenderer.motes(4): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.release(3): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.fortress(4): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.fortressBand(6): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.trace(6): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.blast(5): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.pressureAura(5): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.scarHeight(2): not translated, it uses AtomicRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`).
        // AtomicRenderer.scars(3): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.polar(3): not translated, it uses Minecraft's `Vec3`.
        pub fn fract(n: f64) -> f64 {
            return n - n.floor();
        }
        // AtomicRenderer.ring(6): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.ringProgress(7): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.stroke(6): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.cylinder(7): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.shard(5): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.mote(6): not translated, it uses Minecraft's `VertexConsumer`.
        pub fn energy_alpha(x: i32, y: i32) -> i32 {
            let r: f64 = ((((x as f64) + 0.5f64) - 16.0f64) / 16.0f64).hypot((((y as f64) + 0.5f64) - 16.0f64) / 16.0f64);
            return if r >= 1.0f64 { 0 } else { ((j_round_d((255.0f64 * ((((-r) * r) * 3.0f64).exp())) * (j_clamp((1.0f64 - r) * 8.0f64, 0.0f64, 1.0f64)))) as i32) };
        }
        pub fn circuit_alpha(x: i32, y: i32) -> i32 {
            if (((x >= 3) && (x <= 60)) && (y >= 2)) && (y <= 125) {
                let mut light: f64 = 0.0f64;
                for lane in 0..6 {
                    let bend: f64 = ((22 + (lane * 5)) as f64);
                    let direction: f64 = if (lane % 2) == 0 { 1.0f64 } else { -1.0f64 };
                    let path: f64 = ((6 + (lane * 10)) as f64) + ((direction * 3.0f64) * ((j_clamp(((y as f64) - bend) / 3.0f64, 0.0f64, 1.0f64)) - (j_clamp(((y - (84 - (lane * 3))) as f64) / 3.0f64, 0.0f64, 1.0f64))));
                    let d: f64 = (x as f64) - path;
                    light = light.max((220.0f64 * ((((-d) * d) / 0.4f64).exp())) + (35.0f64 * ((((-d) * d) / 2.5f64).exp())));
                    let node: f64 = d.hypot(((y as f64) - bend) - 3.0f64);
                    light = light.max(220.0f64 * (0.0f64.max(1.0f64 - (node / 2.4f64))));
                }
                return j_clamp((j_round_d(light) as i32), 0, 255);
            } else {
                return 0;
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        // AtomicRenderer.circuitPanel(7): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.ensureEnergyTexture(0): not translated, it uses Minecraft's `NativeImage`.
        // AtomicRenderer.disc(8): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.quad(8): not translated, it uses Minecraft's `VertexConsumer`.
        // AtomicRenderer.energyVertex(7): not translated, it uses Minecraft's `VertexConsumer`.
    }

    /// `AtomicRenderer.EnergyTexture` (net/godwand/client/AtomicRenderer.java)
    pub mod atomic_renderer_energy_texture {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `DynamicTexture`. Its static parts are below.
        // AtomicRenderer.EnergyTexture.EnergyTexture(1): not translated, its class isn't translated (it extends Minecraft's `DynamicTexture`).
    }

    /// `AtomicRenderer.Frame` (net/godwand/client/AtomicRenderer.java)
    pub mod atomic_renderer_frame {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // AtomicRenderer.Frame.Frame(7): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AtomicRenderer.Frame.casterId(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AtomicRenderer.Frame.startedAt(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AtomicRenderer.Frame.center(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AtomicRenderer.Frame.age(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AtomicRenderer.Frame.yaw(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AtomicRenderer.Frame.waitTicks(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AtomicRenderer.Frame.waiting(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `AtomicRenderer.Scar` (net/godwand/client/AtomicRenderer.java)
    pub mod atomic_renderer_scar {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // AtomicRenderer.Scar.Scar(2): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AtomicRenderer.Scar.center(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AtomicRenderer.Scar.started(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `AtomicRenderer.State` (net/godwand/client/AtomicRenderer.java)
    pub mod atomic_renderer_state {
        use super::*;
        // The class itself isn't translated: it keeps an atomic value in an object's field. Its static parts are below.
        // AtomicRenderer.State.State(2): not translated, its class isn't translated (it keeps an atomic value in an object's field).
    }

    /// `AtomicRenderer.TipSample` (net/godwand/client/AtomicRenderer.java)
    pub mod atomic_renderer_tip_sample {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // AtomicRenderer.TipSample.TipSample(2): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AtomicRenderer.TipSample.position(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AtomicRenderer.TipSample.time(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `CannonPressureRenderer` (net/godwand/client/CannonPressureRenderer.java)
    pub mod cannon_pressure_renderer {
        use super::*;
        // BURSTS: not translated, it uses Burst, which isn't translated (it uses `CannonPressurePayload` (net.godwand.network)).
        // world: not translated, it uses Minecraft's `ClientLevel`.
        #[derive(Clone, Debug)]
        pub struct CannonPressureRenderer {
        }
        impl CannonPressureRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CannonPressureRenderer.reset(0): not translated, it uses CannonPressureRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`).
        // CannonPressureRenderer.receive(1): not translated, it uses `CannonPressurePayload` (net.godwand.network).
        // CannonPressureRenderer.conePoint(4): not translated, it uses Minecraft's `Vec3`.
        // CannonPressureRenderer.render(3): not translated, it uses Minecraft's `Camera`.
    }

    /// `CannonPressureRenderer.Burst` (net/godwand/client/CannonPressureRenderer.java)
    pub mod cannon_pressure_renderer_burst {
        use super::*;
        // The class itself isn't translated: it uses `CannonPressurePayload` (net.godwand.network). Its static parts are below.
        // CannonPressureRenderer.Burst.Burst(2): not translated, its class isn't translated (it uses `CannonPressurePayload` (net.godwand.network)).
        // CannonPressureRenderer.Burst.payload(0): not translated, its class isn't translated (it uses `CannonPressurePayload` (net.godwand.network)).
        // CannonPressureRenderer.Burst.tick(0): not translated, its class isn't translated (it uses `CannonPressurePayload` (net.godwand.network)).
    }

    /// `ExtractedModelVertices` (net/godwand/client/compat/ExtractedModelVertices.java)
    pub mod extracted_model_vertices {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `VertexConsumer`. Its static parts are below.
        // ExtractedModelVertices.addVertex(3): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // ExtractedModelVertices.setColor(4): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // ExtractedModelVertices.setColor(1): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // ExtractedModelVertices.setUv(2): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // ExtractedModelVertices.setUv1(2): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // ExtractedModelVertices.setUv2(2): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // ExtractedModelVertices.setUv3(2): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // ExtractedModelVertices.setNormal(3): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // ExtractedModelVertices.setLineWidth(1): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // ExtractedModelVertices.submit(2): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
    }

    /// `ExtractedModelVertices.Vertex` (net/godwand/client/compat/ExtractedModelVertices.java)
    pub mod extracted_model_vertices_vertex {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Vertex {
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub u: f32,
            pub v: f32,
            pub nx: f32,
            pub ny: f32,
            pub nz: f32,
            pub uv3x: f32,
            pub uv3y: f32,
            pub line_width: f32,
            pub color: i32,
            pub overlay: i32,
            pub light: i32,
        }
        impl Vertex {
            pub fn new() -> Self {
                let mut s = Self { x: 0.0f32, y: 0.0f32, z: 0.0f32, u: 0.0f32, v: 0.0f32, nx: 0.0f32, ny: 0.0f32, nz: 0.0f32, uv3x: 0.0f32, uv3y: 0.0f32, line_width: 0.0f32, color: -1, overlay: 0i32, light: 0i32 };
                s
            }
        }
    }

    /// `GodWandGeometry` (net/godwand/client/compat/GodWandGeometry.java)
    pub mod god_wand_geometry {
        use super::*;
        // ARENA: not translated, it uses Minecraft's `ByteBufferBuilder`.
        // PENDING: not translated, it uses Draw, which isn't translated (it uses Minecraft's `MeshData`).
        // PIPELINES: not translated, it uses Key, which isn't translated (it uses Minecraft's `PrimitiveTopology`).
        pub static BLEND: JCell<bool> = JCell::new(false);
        pub static ADDITIVE: JCell<bool> = JCell::new(false);
        pub static DEPTH: JCell<bool> = JCell::new(true);
        pub static WRITE_DEPTH: JCell<bool> = JCell::new(true);
        pub static CULL: JCell<bool> = JCell::new(true);
        pub static TEXTURE: JCell<bool> = JCell::new(false);
        pub static TEXTURE_ID: LazyLock<JCell<Option<JIdentifier>>> = LazyLock::new(|| JCell::new(None));
        // color: not translated, it uses `Vector4f` (org.joml).
        pub static DEFERRED_VIEW: LazyLock<JCell<Option<JMat4>>> = LazyLock::new(|| JCell::new(None));
        #[derive(Clone, Debug)]
        pub struct GodWandGeometry {
        }
        impl GodWandGeometry {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // GodWandGeometry.begin(2): not translated, it uses Minecraft's `BufferBuilder`.
        pub fn enable_blend() {
            god_wand_geometry::BLEND.set(true);
        }
        pub fn disable_blend() {
            god_wand_geometry::BLEND.set(false);
        }
        pub fn default_blend_func() {
            god_wand_geometry::ADDITIVE.set(false);
        }
        pub fn additive_blend() {
            god_wand_geometry::ADDITIVE.set(true);
        }
        pub fn enable_depth_test() {
            god_wand_geometry::DEPTH.set(true);
        }
        pub fn disable_depth_test() {
            god_wand_geometry::DEPTH.set(false);
        }
        pub fn depth_mask(value: bool) {
            god_wand_geometry::WRITE_DEPTH.set(value);
        }
        pub fn enable_cull() {
            god_wand_geometry::CULL.set(true);
        }
        pub fn disable_cull() {
            god_wand_geometry::CULL.set(false);
        }
        pub fn colored() {
            god_wand_geometry::TEXTURE.set(false);
        }
        pub fn textured() {
            god_wand_geometry::TEXTURE.set(true);
        }
        pub fn set_shader_texture(unit: i32, id: &JIdentifier) {
            if unit != 0 {
                panic!("{}", "God Wand geometry has one texture sampler");
            }
            let value_0 = Some(id.clone());
            god_wand_geometry::TEXTURE_ID.set(value_0);
        }
        // GodWandGeometry.setShaderColor(4): not translated, it uses GodWandGeometry.color, which isn't translated (it uses `Vector4f` (org.joml)).
        // GodWandGeometry.pipeline(1): not translated, it uses Minecraft's `RenderPipeline`.
        // GodWandGeometry.initialize(0): not translated, it uses Minecraft's `RenderPipeline`.
        // GodWandGeometry.initialize(1): not translated, it uses Minecraft's `RenderPipeline`.
        // GodWandGeometry.draw(1): not translated, it uses Minecraft's `MeshData`.
        // GodWandGeometry.reset(0): not translated, it uses Minecraft's `MeshData`.
        // GodWandGeometry.deferFlush(0): not translated, it calls Minecraft's `RenderSystem`.getModelViewMatrixCopy.
        // GodWandGeometry.flushDeferred(0): not translated, it calls GodWandGeometry.flush, which isn't translated.
        // GodWandGeometry.flush(0): not translated, it calls Minecraft's `RenderSystem`.getModelViewMatrixCopy.
        // GodWandGeometry.flush(1): not translated, it uses Minecraft's `Minecraft`.
    }

    /// `GodWandGeometry.Draw` (net/godwand/client/compat/GodWandGeometry.java)
    pub mod god_wand_geometry_draw {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `MeshData`. Its static parts are below.
        // GodWandGeometry.Draw.Draw(4): not translated, its class isn't translated (it uses Minecraft's `MeshData`).
        // GodWandGeometry.Draw.mesh(0): not translated, its class isn't translated (it uses Minecraft's `MeshData`).
        // GodWandGeometry.Draw.key(0): not translated, its class isn't translated (it uses Minecraft's `MeshData`).
        // GodWandGeometry.Draw.texture(0): not translated, its class isn't translated (it uses Minecraft's `MeshData`).
        // GodWandGeometry.Draw.color(0): not translated, its class isn't translated (it uses Minecraft's `MeshData`).
    }

    /// `GodWandGeometry.Key` (net/godwand/client/compat/GodWandGeometry.java)
    pub mod god_wand_geometry_key {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `PrimitiveTopology`. Its static parts are below.
        // GodWandGeometry.Key.Key(7): not translated, its class isn't translated (it uses Minecraft's `PrimitiveTopology`).
        // GodWandGeometry.Key.topology(0): not translated, its class isn't translated (it uses Minecraft's `PrimitiveTopology`).
        // GodWandGeometry.Key.texture(0): not translated, its class isn't translated (it uses Minecraft's `PrimitiveTopology`).
        // GodWandGeometry.Key.blend(0): not translated, its class isn't translated (it uses Minecraft's `PrimitiveTopology`).
        // GodWandGeometry.Key.additive(0): not translated, its class isn't translated (it uses Minecraft's `PrimitiveTopology`).
        // GodWandGeometry.Key.depth(0): not translated, its class isn't translated (it uses Minecraft's `PrimitiveTopology`).
        // GodWandGeometry.Key.writeDepth(0): not translated, its class isn't translated (it uses Minecraft's `PrimitiveTopology`).
        // GodWandGeometry.Key.cull(0): not translated, its class isn't translated (it uses Minecraft's `PrimitiveTopology`).
    }

    /// `CustomImpactFrameOverlay` (net/godwand/client/CustomImpactFrameOverlay.java)
    pub mod custom_impact_frame_overlay {
        use super::*;
        pub static FRAMES: LazyLock<Vec<JIdentifier>> = LazyLock::new(|| vec![JIdentifier::new("godwand", "textures/vfx/impact_frame_0.png"), JIdentifier::new("godwand", "textures/vfx/impact_frame_1.png"), JIdentifier::new("godwand", "textures/vfx/impact_frame_2.png"), JIdentifier::new("godwand", "textures/vfx/impact_frame_3.png")]);
        pub const IMPACT_FRAME_TICKS: i32 = 4;
        pub const FADEOUT_TICKS: i32 = 3;
        pub const TOTAL_DURATION: i32 = 7;
        pub static CURRENT_TICK: JCell<i32> = JCell::new(0);
        pub static ACTIVE: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct CustomImpactFrameOverlay {
        }
        impl CustomImpactFrameOverlay {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn trigger_1(duration_ticks: i32) {
            custom_impact_frame_overlay::CURRENT_TICK.set(0);
            custom_impact_frame_overlay::ACTIVE.set(true);
        }
        pub fn trigger_0() {
            custom_impact_frame_overlay::trigger_1(4);
        }
        pub fn reset() {
            custom_impact_frame_overlay::ACTIVE.set(false);
            custom_impact_frame_overlay::CURRENT_TICK.set(0);
        }
        pub fn on_client_tick() {
            if custom_impact_frame_overlay::ACTIVE.get() {
                custom_impact_frame_overlay::CURRENT_TICK.set(custom_impact_frame_overlay::CURRENT_TICK.get() + 1);
                if custom_impact_frame_overlay::CURRENT_TICK.get() >= 7 {
                    custom_impact_frame_overlay::ACTIVE.set(false);
                    custom_impact_frame_overlay::CURRENT_TICK.set(0);
                }
            }
        }
        // CustomImpactFrameOverlay.render(2): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
    }

    /// `DrillWindRenderer` (net/godwand/client/DrillWindRenderer.java)
    pub mod drill_wind_renderer {
        use super::*;
        // WINDS: not translated, it uses Wind, which isn't translated (it uses `DrillWindPayload` (net.godwand.network)).
        // world: not translated, it uses Minecraft's `ClientLevel`.
        #[derive(Clone, Debug)]
        pub struct DrillWindRenderer {
        }
        impl DrillWindRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DrillWindRenderer.receive(1): not translated, it uses `DrillWindPayload` (net.godwand.network).
        // DrillWindRenderer.reset(0): not translated, it uses DrillWindRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`).
        // DrillWindRenderer.render(3): not translated, it uses Minecraft's `Camera`.
        // DrillWindRenderer.drawCloud(5): not translated, a try/finally.
        // DrillWindRenderer.draw(5): not translated, it calls DrillWindRenderer.draw, which isn't translated.
        // DrillWindRenderer.draw(6): not translated, a try/finally.
    }

    /// `DrillWindRenderer.Wind` (net/godwand/client/DrillWindRenderer.java)
    pub mod drill_wind_renderer_wind {
        use super::*;
        // The class itself isn't translated: it uses `DrillWindPayload` (net.godwand.network). Its static parts are below.
        // DrillWindRenderer.Wind.Wind(2): not translated, its class isn't translated (it uses `DrillWindPayload` (net.godwand.network)).
        // DrillWindRenderer.Wind.payload(0): not translated, its class isn't translated (it uses `DrillWindPayload` (net.godwand.network)).
        // DrillWindRenderer.Wind.tick(0): not translated, its class isn't translated (it uses `DrillWindPayload` (net.godwand.network)).
    }

    /// `EarthDrillSoundscape` (net/godwand/client/EarthDrillSoundscape.java)
    pub mod earth_drill_soundscape {
        use super::*;
        pub static AUDIO: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("godwand", "earth_drill_sequence"));
        pub const PLACEHOLDER_SHA: &str = "225b65ec21cc27259fd742d6e1488a69c28f4a0365301d533eeaf433e3c60443";
        // voices: not translated, it uses Minecraft's `SoundInstance`.
        // world: not translated, it uses Minecraft's `ClientLevel`.
        pub static START: JCell<i64> = JCell::new(0i64);
        pub static LAST_AGE: JCell<i32> = JCell::new(0i32);
        pub static GAIN: JCell<f32> = JCell::new(0.0f32);
        pub static CUSTOM: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EarthDrillSoundscape {
        }
        impl EarthDrillSoundscape {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EarthDrillSoundscape.isCustomAudio(1): not translated, a try/catch (NoSuchAlgorithmException).
        pub fn distance_gain(distance: f64) -> f32 {
            return ((j_clamp((160.0f64 - distance) / 112.0f64, 0.0f64, 1.0f64)) as f32);
        }
        // EarthDrillSoundscape.start(1): not translated, it uses Minecraft's `Vec3`.
        // EarthDrillSoundscape.tick(1): not translated, it uses Minecraft's `Minecraft`.
        // EarthDrillSoundscape.cue(1): not translated, it's overloaded with the same number of parameters.
        // EarthDrillSoundscape.play(3): not translated, it's overloaded with the same number of parameters.
        // EarthDrillSoundscape.play(3): not translated, it's overloaded with the same number of parameters.
        // EarthDrillSoundscape.playAt(4): not translated, it uses Minecraft's `Vec3`.
        // EarthDrillSoundscape.reset(0): not translated, it uses Minecraft's `Minecraft`.
    }

    /// `FugaAimRig` (net/godwand/client/FugaAimRig.java)
    pub mod fuga_aim_rig {
        use super::*;
        // availableFor: not translated, a function kept in a variable or field (only parameters can be).
        #[derive(Clone, Debug)]
        pub struct FugaAimRig {
        }
        impl FugaAimRig {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn weight(age: f64) -> f32 {
            return if age < 144.0f64 { 0.0f32 } else { fuga_cinematic::smooth(age, 144.0f64, 184.0f64) };
        }
        // FugaAimRig.joints(2): not translated, it uses Minecraft's `Vec3`.
        // FugaAimRig.hand(2): not translated, it uses Minecraft's `Vec3`.
    }

    /// `FugaAimRig.Joints` (net/godwand/client/FugaAimRig.java)
    pub mod fuga_aim_rig_joints {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // FugaAimRig.Joints.Joints(3): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaAimRig.Joints.shoulder(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaAimRig.Joints.elbow(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaAimRig.Joints.tip(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `FugaCinematic` (net/godwand/client/FugaCinematic.java)
    pub mod fuga_cinematic {
        use super::*;
        pub static CAST: JCell<i64> = JCell::new(i64::MIN);
        pub static SKIPPED: JCell<bool> = JCell::new(false);
        // world: not translated, it uses Minecraft's `ClientLevel`.
        pub static RENDERED_FOV: JCell<f64> = JCell::new(70.0f64);
        pub static CAMERA_PARTIAL: JCell<f32> = JCell::new(0.0f32);
        pub const END_CUT: f64 = 120.0f64;
        pub const PILLAR_CUT: f64 = 90.0f64;
        pub static IMPACT_KEYS: LazyLock<Vec<f64>> = LazyLock::new(|| vec![0.0f64, 36.0f64, 39.0f64, 41.0f64, 52.0f64, 100.0f64, 122.0f64, 140.0f64, 160.0f64]);
        pub static IMPACT_OUT: LazyLock<Vec<f64>> = LazyLock::new(|| vec![30.0f64, 44.0f64, 52.0f64, 60.0f64, 48.0f64, 48.0f64, 48.0f64, 48.0f64, 48.0f64]);
        pub static IMPACT_SIDE: LazyLock<Vec<f64>> = LazyLock::new(|| vec![4.0f64, 4.0f64, 4.0f64, 4.0f64, 4.0f64, 4.0f64, 4.0f64, 4.0f64, 4.0f64]);
        pub static CHARGE_KEYS: LazyLock<Vec<f64>> = LazyLock::new(|| vec![0.0f64, 24.0f64, 48.0f64, 90.0f64, 96.0f64, 104.0f64, 128.0f64, 144.0f64, 172.0f64, 200.0f64]);
        pub static ORBIT: LazyLock<Vec<f64>> = LazyLock::new(|| vec![-0.32f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.82f64, 2.25f64, 2.94f64]);
        pub static DISTANCE: LazyLock<Vec<f64>> = LazyLock::new(|| vec![0.95f64, 1.05f64, 1.1f64, 2.4f64, 1.2f64, 3.6f64, 4.0f64, 4.4f64, 3.9f64, 3.26f64]);
        pub static HEIGHT: LazyLock<Vec<f64>> = LazyLock::new(|| vec![1.74f64, 1.68f64, 1.38f64, 1.5f64, 1.45f64, 1.6f64, 1.8f64, 1.95f64, 2.1f64, 1.87f64]);
        #[derive(Clone, Debug)]
        pub struct FugaCinematic {
        }
        impl FugaCinematic {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FugaCinematic.present(2): not translated, it uses Minecraft's `Minecraft`.
        // FugaCinematic.openingFaceVisible(1): not translated, it uses Frame, which isn't translated (it uses Minecraft's `Vec3`).
        // FugaCinematic.showCaster(1): not translated, it calls FugaCinematic.openingFaceVisible, which isn't translated.
        // FugaCinematic.active(0): not translated, it uses Minecraft's `Minecraft`.
        // FugaCinematic.movementLocked(0): not translated, it uses Minecraft's `Minecraft`.
        // FugaCinematic.rotationLocked(0): not translated, it calls FugaCinematic.movementLocked, which isn't translated.
        // FugaCinematic.tick(1): not translated, it uses Minecraft's `Minecraft`.
        pub fn skip() {
            fuga_cinematic::SKIPPED.set(true);
        }
        // FugaCinematic.reset(0): not translated, it uses FugaCinematic.world, which isn't translated (it uses Minecraft's `ClientLevel`).
        // FugaCinematic.camera(4): not translated, it uses Pose, which isn't translated (it uses Minecraft's `Vec3`).
        // FugaCinematic.cameraFor(8): not translated, it uses Pose, which isn't translated (it uses Minecraft's `Vec3`).
        // FugaCinematic.blend(3): not translated, it uses Pose, which isn't translated (it uses Minecraft's `Vec3`).
        pub fn spline(age: f64, keys: &[f64], values: &[f64]) -> f64 {
            let mut i: i32 = 0;
            let last: i32 = (keys.len() as i32) - 1;
            while (i < (last - 1)) && (age > keys[(i + 1) as usize]) {
                i += 1;
            }
            let span: f64 = keys[(i + 1) as usize] - keys[i as usize];
            let t: f64 = 0.0f64.max(1.0f64.min((age - keys[i as usize]) / span));
            let m0: f64 = if i == 0 { 0.0f64 } else { (values[(i + 1) as usize] - values[(i - 1) as usize]) / (keys[(i + 1) as usize] - keys[(i - 1) as usize]) };
            let m1: f64 = if (i + 1) == last { 0.0f64 } else { (values[(i + 2) as usize] - values[i as usize]) / (keys[(i + 2) as usize] - keys[i as usize]) };
            return ((((((((2.0f64 * t) * t) * t) - ((3.0f64 * t) * t)) + 1.0f64) * values[i as usize]) + ((((((t * t) * t) - ((2.0f64 * t) * t)) + t) * span) * m0)) + ((((((-2.0f64) * t) * t) * t) + ((3.0f64 * t) * t)) * values[(i + 1) as usize])) + (((((t * t) * t) - (t * t)) * span) * m1);
        }
        // FugaCinematic.chargeCamera(7): not translated, it uses Pose, which isn't translated (it uses Minecraft's `Vec3`).
        pub fn sample(age: f64, values: &[f64], endpoint: f64) -> f64 {
            let mut i: i32 = 0;
            let last: i32 = ((*fuga_cinematic::CHARGE_KEYS).len() as i32) - 1;
            while (i < (last - 1)) && (age > ((*fuga_cinematic::CHARGE_KEYS)[(i + 1) as usize])) {
                i += 1;
            }
            let span: f64 = ((*fuga_cinematic::CHARGE_KEYS)[(i + 1) as usize]) - ((*fuga_cinematic::CHARGE_KEYS)[i as usize]);
            let t: f64 = 0.0f64.max(1.0f64.min((age - ((*fuga_cinematic::CHARGE_KEYS)[i as usize])) / span));
            let a: f64 = values[i as usize];
            let b: f64 = if (i + 1) == last { endpoint } else { values[(i + 1) as usize] };
            let before: f64 = if i == 0 { a } else { values[(i - 1) as usize] };
            let after: f64 = if (i + 2) >= last { endpoint } else { values[(i + 2) as usize] };
            let m0: f64 = if i == 0 { (b - a) / span } else { (b - before) / (((*fuga_cinematic::CHARGE_KEYS)[(i + 1) as usize]) - ((*fuga_cinematic::CHARGE_KEYS)[(i - 1) as usize])) };
            let m1: f64 = if (i + 1) == last { 0.0f64 } else { (after - a) / (((*fuga_cinematic::CHARGE_KEYS)[(i + 2) as usize]) - ((*fuga_cinematic::CHARGE_KEYS)[i as usize])) };
            return ((((((((2.0f64 * t) * t) * t) - ((3.0f64 * t) * t)) + 1.0f64) * a) + ((((((t * t) * t) - ((2.0f64 * t) * t)) + t) * span) * m0)) + ((((((-2.0f64) * t) * t) * t) + ((3.0f64 * t) * t)) * b)) + (((((t * t) * t) - (t * t)) * span) * m1);
        }
        // FugaCinematic.shoulder(3): not translated, it uses Pose, which isn't translated (it uses Minecraft's `Vec3`).
        // FugaCinematic.look(2): not translated, it uses Pose, which isn't translated (it uses Minecraft's `Vec3`).
        pub fn smooth(age: f64, start: f64, end: f64) -> f32 {
            let t: f32 = ((0.0f64.max(1.0f64.min((age - start) / (end - start)))) as f32);
            return (t * t) * (3.0f32 - (2.0f32 * t));
        }
        // FugaCinematic.environmentGain(0): not translated, it calls FugaCinematic.present, which isn't translated.
        pub fn darkness(age: f64) -> f32 {
            return 1.0f32 - fuga_cinematic::smooth(age, 110.0f64, 140.0f64);
        }
        pub fn silence(age: f64) -> f32 {
            return fuga_cinematic::smooth(age, 96.0f64, 97.5f64) * (1.0f32 - fuga_cinematic::smooth(age, 104.0f64, 112.0f64));
        }
        // FugaCinematic.fov(2): not translated, it calls FugaCinematic.active, which isn't translated.
        // FugaCinematic.shake(1): not translated, a nullable object kept in a changing static.
        pub fn ring_kick(age: f64) -> f32 {
            let mut best: f64 = 0.0f64;
            for i in 0..3 {
                let mut cycle: f64 = (((age - 40.0f64) / 10.0f64) + ((i as f64) / 3.0f64)) % 1.0f64;
                if cycle < 0.0f64 {
                    cycle += 1.0f64;
                }
                best = best.max(((-cycle) * 14.0f64).exp());
            }
            return (best as f32);
        }
        pub fn letterbox(phase: i32, age: f64) -> f32 {
            if phase == 0 {
                let kick: f32 = if age >= 95.0f64 { 0.06f32 * ((((-(age - 95.0f64)) / 4.0f64).exp()) as f32) } else { 0.0f32 };
                return (((0.12f32 - (0.04f32 * fuga_cinematic::smooth(age, 20.0f64, 40.0f64))) + (0.03f32 * fuga_cinematic::smooth(age, 95.0f64, 99.0f64))) + kick) * (1.0f32 - fuga_cinematic::smooth(age, 165.0f64, 195.0f64));
            }
            if phase == 1 {
                return 0.08f32 + (0.05f32 * ((((-age) / 5.0f64).exp()) as f32));
            }
            if age < 40.0f64 {
                return 0.07f32 + (0.05f32 * fuga_cinematic::smooth(age, 8.0f64, 37.0f64));
            }
            let kick: f32 = 0.09f32 * ((((-(age - 40.0f64)) / 5.0f64).exp()) as f32);
            return ((if age < 90.0f64 { 0.1f32 } else { if age < 120.0f64 { 0.07f32 } else { 0.05f32 } }) + (kick * (1.0f32 - fuga_cinematic::smooth(age, 148.0f64, 160.0f64)))) - (0.05f32 * fuga_cinematic::smooth(age, 148.0f64, 160.0f64));
        }
        pub fn record_fov(fov: f64) {
            if (fov.is_finite() && (fov > 1.0f64)) && (fov < 179.0f64) {
                fuga_cinematic::RENDERED_FOV.set(fov);
            }
        }
        // FugaCinematic.project(7): not translated, it uses Minecraft's `Vec3`.
        // FugaCinematic.aimTarget(2): not translated, it uses Minecraft's `Vec3`.
        // FugaCinematic.reticle(3): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        pub fn overlay_flash(phase: i32, age: f64) -> f32 {
            return if phase == 1 { (fuga_cinematic::smooth(age, 3.0f64, 3.6f64) * (1.0f32 - fuga_cinematic::smooth(age, 3.6f64, 7.0f64))) * 0.32f32 } else { if phase == 2 { fuga_cinematic::flash(age) * 0.92f32 } else { 0.0f32 } };
        }
        pub fn flash(impact_age: f64) -> f32 {
            return fuga_cinematic::smooth(impact_age, 40.4f64, 40.8f64) * (1.0f32 - fuga_cinematic::smooth(impact_age, 41.5f64, 46.0f64));
        }
        // FugaCinematic.tint(7): needs the game, so it's in game_side.rs.
        // FugaCinematic.grade(5): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // FugaCinematic.render(2): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
    }

    /// `FugaPose` (net/godwand/client/FugaPose.java)
    pub mod fuga_pose {
        use super::*;
        pub const TWIST: f32 = 0.85f32;
        pub const CLAP: f64 = 95.0f64;
        #[derive(Clone, Debug)]
        pub struct FugaPose {
        }
        impl FugaPose {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FugaPose.apply(6): not translated, it uses Minecraft's `ModelPart`.
        pub fn arms(phase: i32, age: f64, pitch: f32, yaw: f32) -> fuga_pose_arms::Arms {
            let charge: f64 = if phase == 0 { age.min(200.0f64) } else { 200.0f64 };
            let pull: f32 = fuga_cinematic::smooth(charge, 48.0f64, 64.0f64) * (1.0f32 - fuga_cinematic::smooth(charge, 90.0f64, 95.0f64));
            let draw: f32 = fuga_cinematic::smooth(charge, 144.0f64, 190.0f64);
            let settle: f32 = fuga_cinematic::smooth(charge, 172.0f64, 200.0f64);
            let idle: f32 = if (phase == 0) && (age > 200.0f64) { (((((age - 200.0f64) * 0.065f64).sin()) as f32) * 0.009f32) * fuga_cinematic::smooth(age, 200.0f64, 212.0f64) } else { 0.0f32 };
            let mut right_pitch: f32 = j_lerp(draw, (-1.35f32) - (0.32f32 * pull), if phase == 0 { (((-std::f64::consts::PI) / 2.0f64) as f32) } else { -1.5f32 });
            let mut left_pitch: f32 = j_lerp(draw, (-1.35f32) + (0.26f32 * pull), (((-std::f64::consts::PI) / 2.0f64) as f32));
            let mut right_yaw: f32 = j_lerp(draw, j_lerp(pull, -0.55f32, 0.22f32), -0.5f32);
            let mut left_yaw: f32 = j_lerp(draw, j_lerp(pull, 0.55f32, -0.22f32), if phase == 0 { 0.1f32 } else { 0.42f32 });
            right_pitch += (pitch * draw) + (if phase == 0 { 0.0f32 } else { idle });
            left_pitch += pitch * draw;
            right_yaw += yaw * draw;
            left_yaw += yaw * draw;
            let release: f32 = if phase == 0 { 0.0f32 } else { if phase == 1 { fuga_cinematic::smooth(age, 0.0f64, 7.0f64) } else { 1.0f32 } };
            let relax: f32 = if phase == 1 { fuga_cinematic::smooth(age, 7.0f64, 18.0f64) } else { if phase == 2 { 1.0f32 } else { 0.0f32 } };
            right_pitch += release * 0.2f32;
            right_yaw -= release * 0.22f32;
            left_pitch -= (release * (1.0f32 - relax)) * 0.32f32;
            let rest: f32 = relax * 0.8f32;
            right_pitch = j_lerp(rest, right_pitch, -0.2f32);
            right_yaw = j_lerp(rest, right_yaw, 0.0f32);
            left_pitch = j_lerp(rest, left_pitch, -0.2f32);
            left_yaw = j_lerp(rest, left_yaw, 0.0f32);
            let weight: f32 = if phase == 2 { 1.0f32 - fuga_cinematic::smooth(age, 0.0f64, 14.0f64) } else { 1.0f32 };
            let tension: f32 = 0.04f32 * (((std::f64::consts::PI * (settle as f64)).sin()) as f32);
            return fuga_pose_arms::Arms::new(right_pitch - (if phase == 0 { 0.0f32 } else { tension }), right_yaw, (((-0.12f32) * (1.0f32 - draw)) - ((if phase == 0 { 0.0f32 } else { 0.08f32 }) * draw)) - (release * 0.2f32), left_pitch, left_yaw, 0.12f32 * (1.0f32 - draw), if phase == 0 { 0.85f32 * draw } else { (((0.85f32 + (0.23f32 * release)) * draw) + tension) * (1.0f32 - rest) }, weight);
        }
        // FugaPose.stance(4): not translated, it uses Minecraft's `ModelPart`.
        // FugaPose.handFocus(2): not translated, it uses Minecraft's `Vec3`.
        // FugaPose.handFocus(3): not translated, it uses Minecraft's `Vec3`.
        // FugaPose.hand(3): not translated, it uses Minecraft's `Vec3`.
        // FugaPose.hand(4): not translated, it uses Minecraft's `Vec3`.
        // FugaPose.wrist(4): not translated, it uses Minecraft's `Vec3`.
    }

    /// `FugaPose.Arms` (net/godwand/client/FugaPose.java)
    pub mod fuga_pose_arms {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Arms {
            pub right_pitch: f32,
            pub right_yaw: f32,
            pub right_roll: f32,
            pub left_pitch: f32,
            pub left_yaw: f32,
            pub left_roll: f32,
            pub body_yaw: f32,
            pub weight: f32,
        }
        impl Arms {
            pub fn new(right_pitch: f32, right_yaw: f32, right_roll: f32, left_pitch: f32, left_yaw: f32, left_roll: f32, body_yaw: f32, weight: f32) -> Self {
                let mut s = Self { right_pitch: right_pitch, right_yaw: right_yaw, right_roll: right_roll, left_pitch: left_pitch, left_yaw: left_yaw, left_roll: left_roll, body_yaw: body_yaw, weight: weight };
                s.right_pitch = right_pitch;
                s.right_yaw = right_yaw;
                s.right_roll = right_roll;
                s.left_pitch = left_pitch;
                s.left_yaw = left_yaw;
                s.left_roll = left_roll;
                s.body_yaw = body_yaw;
                s.weight = weight;
                s
            }
            pub fn right_pitch(&self) -> f32 {
                return self.right_pitch;
            }
            pub fn right_yaw(&self) -> f32 {
                return self.right_yaw;
            }
            pub fn right_roll(&self) -> f32 {
                return self.right_roll;
            }
            pub fn left_pitch(&self) -> f32 {
                return self.left_pitch;
            }
            pub fn left_yaw(&self) -> f32 {
                return self.left_yaw;
            }
            pub fn left_roll(&self) -> f32 {
                return self.left_roll;
            }
            pub fn body_yaw(&self) -> f32 {
                return self.body_yaw;
            }
            pub fn weight(&self) -> f32 {
                return self.weight;
            }
        }
    }

    /// `FugaRenderer` (net/godwand/client/FugaRenderer.java)
    pub mod fuga_renderer {
        use super::*;
        // ACTIVE: not translated, it uses `FugaFxPayload` (net.godwand.network).
        pub static RECEIVED: LazyLock<JMap<i32, i64>> = LazyLock::new(|| JMap::new());
        pub static PHASE_SEEN: LazyLock<JMap<i32, i64>> = LazyLock::new(|| JMap::new());
        // LATEST: not translated, it uses `FugaFxPayload` (net.godwand.network).
        // world: not translated, it uses Minecraft's `ClientLevel`.
        // AFTERMATH: not translated, it uses Aftermath, which isn't translated (it uses Minecraft's `Vec3`).
        pub const AFTERMATH_TICKS: i32 = 900;
        pub static TAU: LazyLock<f64> = LazyLock::new(|| std::f64::consts::PI * 2.0f64);
        // glow: not translated, it uses Glow, which isn't translated (it uses Minecraft's `Vec3`).
        pub const GLOW_TICKS: i32 = 340;
        pub static CLIENT_TICKS: JCell<i64> = JCell::new(0i64);
        pub const FUTURE_TOLERANCE: i64 = 200i64;
        pub static CLOCKS: LazyLock<JCell<JMap<i32, fuga_renderer_clock::Clock>>> = LazyLock::new(|| JCell::new(JMap::new()));
        pub const IMPACT_BEATS: f64 = 4.0f64;
        pub const HIT_STOP: f64 = 10.0f64;
        pub const FREEZE_AGE: f64 = 30.0f64;
        pub static RENDER_PARTIAL: JCell<f32> = JCell::new(0.0f32);
        // LAUNCH: a game value, so it's in game_side.rs.
        pub static SHADER_CASTER: JCell<i32> = JCell::new(i32::MIN);
        pub static PASS: JCell<i32> = JCell::new(-1);
        pub static FLAME_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("godwand", "fuga_flame"));
        pub static TEXTURE_READY: JCell<bool> = JCell::new(false);
        pub const SOLID_U: f32 = 0.5f32;
        pub const SOLID_V: f32 = 0.9375f32;
        pub const EDGE_V: f32 = 0.0078125f32;
        pub const SPINE_V: f32 = 0.4375f32;
        pub const FAR_V: f32 = 0.8671875f32;
        pub static FLOW: JCell<f64> = JCell::new(0.0f64);
        // deferred: not translated, it uses Deferred, which isn't translated (it uses Minecraft's `Camera`).
        pub const PULL_BALL: bool = true;
        pub const STREAMS: i32 = 10;
        pub static PUFF_COS: LazyLock<Vec<f64>> = LazyLock::new(|| vec![0.0f64; 14]);
        pub static PUFF_SIN: LazyLock<Vec<f64>> = LazyLock::new(|| vec![0.0f64; 14]);
        #[derive(Clone, Debug)]
        pub struct FugaRenderer {
        }
        impl FugaRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FugaRenderer.newer(2): not translated, it uses `FugaFxPayload` (net.godwand.network).
        // FugaRenderer.receive(1): not translated, it uses `FugaFxPayload` (net.godwand.network).
        // FugaRenderer.reset(0): not translated, it uses FugaRenderer.LAUNCH, which isn't translated (it uses Minecraft's `Vec3`).
        // FugaRenderer.aftermath(1): not translated, it uses Minecraft's `Vec3`.
        // FugaRenderer.phaseSeen(2): not translated, it uses FugaRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`).
        // FugaRenderer.tick(1): not translated, it uses Minecraft's `Minecraft`.
        // FugaRenderer.particles(1): not translated, it uses Minecraft's `Minecraft`.
        // FugaRenderer.frameFor(2): not translated, it uses FugaRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`).
        // FugaRenderer.steadyAge(4): not translated, it uses `FugaFxPayload` (net.godwand.network).
        // FugaRenderer.hitStop(1): not translated, it calls FugaRenderer.hitStop, which isn't translated.
        // FugaRenderer.hitStop(2): not translated, it uses `FugaFxPayload` (net.godwand.network).
        // FugaRenderer.impactBeat(2): not translated, it uses `FugaFxPayload` (net.godwand.network).
        pub fn flight_time(distance: f64) -> f64 {
            let mut lo: f64 = 0.0f64;
            let mut hi: f64 = 400.0f64;
            for i in 0..40 {
                let mid: f64 = (lo + hi) / 2.0f64;
                if fuga_sequence::flight_distance(mid) < distance {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            return hi;
        }
        // FugaRenderer.localFrame(1): not translated, it uses Minecraft's `LocalPlayer`.
        // FugaRenderer.shaderFrame(1): not translated, it calls FugaRenderer.localFrame, which isn't translated.
        // FugaRenderer.afterglow(2): not translated, it uses FugaRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`).
        // FugaRenderer.ensureFlameTexture(0): not translated, it uses Minecraft's `NativeImage`.
        pub fn flame_alpha(x: i32, y: i32) -> i32 {
            if y >= 56 {
                return 255;
            }
            let u: f64 = (x as f64) / 256.0f64;
            let w: f64 = (((y as f64) - 27.5f64).abs()) / 27.5f64;
            let edge: i32 = if y < 28 { 0 } else { 1 };
            let rim: f64 = (0.38f64 + (0.5f64 * (fuga_renderer::fibre(u * 6.0f64, ((edge * 5) as f64) + 1.3f64, 6)))) + (0.2f64 * (fuga_renderer::fibre(u * 19.0f64, ((edge * 7) as f64) + 4.1f64, 19)));
            let slit: f64 = (fuga_renderer::fibre(u * 6.0f64, (w * 4.5f64) + 8.0f64, 6)) * (1.0f64 - (w * 0.5f64));
            let f: f64 = 0.0f64.max(1.0f64.min(((rim - w) * 6.0f64).min((0.64f64 - slit) * 5.0f64)));
            return (j_round_d(255.0f64 * f) as i32);
        }
        pub fn fibre(x: f64, y: f64, period: i32) -> f64 {
            let ix: i32 = (x.floor() as i32);
            let iy: i32 = (y.floor() as i32);
            let mut fx: f64 = x - (ix as f64);
            let mut fy: f64 = y - (iy as f64);
            fx = (fx * fx) * (3.0f64 - (2.0f64 * fx));
            fy = (fy * fy) * (3.0f64 - (2.0f64 * fy));
            let x0: i32 = ix.rem_euclid(period);
            let x1: i32 = (ix + 1).rem_euclid(period);
            let a: f64 = fuga_renderer::hash(x0, iy);
            let b: f64 = fuga_renderer::hash(x1, iy);
            let c: f64 = fuga_renderer::hash(x0, iy + 1);
            let d: f64 = fuga_renderer::hash(x1, iy + 1);
            return ((a + ((b - a) * fx)) + ((c - a) * fy)) + (((((a - b) - c) + d) * fx) * fy);
        }
        // FugaRenderer.render(3): not translated, it uses Minecraft's `Camera`.
        // FugaRenderer.deferForShaderPack(3): not translated, it uses Minecraft's `Camera`.
        pub fn flush_deferred() {
        }
        // FugaRenderer.valid(2): not translated, it uses Minecraft's `Vec3`.
        // FugaRenderer.accents(4): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.groundRings(5): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.groundRing(8): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.emitBackdrop(5): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.emit(4): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.emit(5): not translated, it uses Minecraft's `VertexConsumer`.
        pub fn ribbon_volume(age: f64) -> f64 {
            return ((fuga_cinematic::smooth(age, 24.0f64, 40.0f64) * (1.0f32 - fuga_cinematic::smooth(age, 93.0f64, 96.0f64))) as f64);
        }
        // FugaRenderer.shaderAxis(1): not translated, it uses Minecraft's `Vec3`.
        // FugaRenderer.chargeHand(3): not translated, it uses Minecraft's `Vec3`.
        // FugaRenderer.ritualRibbon(4): not translated, it uses Minecraft's `VertexConsumer`.
        pub fn crackle(age: f64) -> f64 {
            if (!(age < 40.0f64)) && (!(age >= 95.0f64)) {
                let tick: i32 = (age.floor() as i32);
                if (tick != 93) && (tick != 91) {
                    let chance: f64 = 0.07f64 + (0.4f64 * (fuga_renderer::smooth((age - 73.0f64) / 20.0f64)));
                    return if fuga_renderer::hash(tick, 211) < chance { 0.55f64 + (0.45f64 * fuga_renderer::hash(tick, 212)) } else { 0.0f64 };
                } else {
                    return 1.0f64;
                }
            } else {
                return 0.0f64;
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        // FugaRenderer.clap(4): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.stream(6): not translated, it uses Minecraft's `Vec3`.
        // FugaRenderer.vortex(7): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.flame(7): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.flame(8): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.ground(6): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.cinders(6): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.storms(3): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.storm(4): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.puff(7): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.light(9): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.loop(8): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.glowDisc(6): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.streaks(8): not translated, it uses Minecraft's `VertexConsumer`.
        pub fn mix_rgb(a: i32, b: i32, t: f64) -> i32 {
            let r: i32 = ((j_round_d(((((a >> 16) & 0xFF) as f64) * (1.0f64 - t)) + ((((b >> 16) & 0xFF) as f64) * t))) as i32);
            let g: i32 = ((j_round_d(((((a >> 8) & 0xFF) as f64) * (1.0f64 - t)) + ((((b >> 8) & 0xFF) as f64) * t))) as i32);
            let bl: i32 = ((j_round_d((((a & 0xFF) as f64) * (1.0f64 - t)) + (((b & 0xFF) as f64) * t))) as i32);
            return ((r << 16) | (g << 8)) | bl;
        }
        // FugaRenderer.ember(7): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.dot(7): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.glint(6): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.flowArrow(8): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.charging(6): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.flight(4): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.streamers(4): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.airRush(4): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.impactPoint(4): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.skyBeam(5): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.lingeringTrail(4): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.launch(4): not translated, it uses Minecraft's `VertexConsumer`.
        pub fn orb_radius(age: f64) -> f64 {
            return 44.0f64;
        }
        pub fn orb_scale(age: f64) -> f64 {
            return (1.0f64 + (0.1f64 * (fuga_renderer::smooth((age - 33.0f64) / 6.0f64)))) + (0.12f64 * (fuga_renderer::smooth((age - 38.5f64) / 1.1f64)));
        }
        pub fn orb_gap(age: f64) -> f64 {
            return 10.0f64 * ((1.0f64 - ((0.0f64.max(1.0f64.min(age / 39.0f64))).powf(1.25f64))).powf(1.2f64));
        }
        pub fn orb_center(age: f64) -> f64 {
            return fuga_renderer::orb_gap(age) + (fuga_renderer::orb_radius(age) * fuga_renderer::orb_scale(age));
        }
        // FugaRenderer.impactVolume(5): not translated, it uses Minecraft's `VertexConsumer`.
        pub fn taper(h: f64, floor: f64, height: f64) -> f64 {
            return 1.0f64.min(((h - floor).min(height - h)) / 30.0f64);
        }
        // FugaRenderer.impactAccents(4): not translated, it uses Minecraft's `VertexConsumer`.
        pub fn layer(glow: bool) -> bool {
            return (fuga_renderer::PASS.get() < 0) || ((fuga_renderer::PASS.get() == 1) == glow);
        }
        // FugaRenderer.strand(8): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.strand(9): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.line(9): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.spark(5): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.fireBody(9): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.licks(10): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.licks(13): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.ellipsoid(9): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.ring(8): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.ring(9): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.ring(10): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.quad2(10): not translated, it uses Minecraft's `VertexConsumer`.
        pub fn hot(c: i32) -> i32 {
            return ((((((c >> 16) & 0xFF) + 255) / 2) << 16) | (((((c >> 8) & 0xFF) + 255) / 2) << 8)) | (((c & 0xFF) + 255) / 2);
        }
        // FugaRenderer.quad(8): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.vertex(5): not translated, it uses Minecraft's `VertexConsumer`.
        // FugaRenderer.vertex(7): not translated, it uses Minecraft's `VertexConsumer`.
        pub fn hash(x: i32, y: i32) -> f64 {
            let mut h: i32 = (x * 374761393) + (y * 668265263);
            h = (h ^ (((h as u32) >> 13) as i32)) * 1274126177;
            return (((h ^ (((h as u32) >> 16) as i32)) & 65535) as f64) / 65535.0f64;
        }
        pub fn noise(x: f64, y: f64) -> f64 {
            let ix: i32 = (x.floor() as i32);
            let iy: i32 = (y.floor() as i32);
            let mut fx: f64 = x - (ix as f64);
            let mut fy: f64 = y - (iy as f64);
            fx = (fx * fx) * (3.0f64 - (2.0f64 * fx));
            fy = (fy * fy) * (3.0f64 - (2.0f64 * fy));
            let a: f64 = fuga_renderer::hash(ix, iy);
            let b: f64 = fuga_renderer::hash(ix + 1, iy);
            let c: f64 = fuga_renderer::hash(ix, iy + 1);
            let d: f64 = fuga_renderer::hash(ix + 1, iy + 1);
            return ((a + ((b - a) * fx)) + ((c - a) * fy)) + (((((a - b) - c) + d) * fx) * fy);
        }
        pub fn ease_out(mut t: f64) -> f64 {
            t = 0.0f64.max(1.0f64.min(t));
            return 1.0f64 - (((1.0f64 - t) * (1.0f64 - t)) * (1.0f64 - t));
        }
        // FugaRenderer.direction(1): not translated, it uses Minecraft's `Vec3`.
        // FugaRenderer.side(1): not translated, it uses Minecraft's `Vec3`.
        pub fn smooth(mut t: f64) -> f64 {
            t = 0.0f64.max(1.0f64.min(t));
            return (t * t) * (3.0f64 - (2.0f64 * t));
        }
        pub fn finite(values: &[f64]) -> bool {
            for &v in values.iter() {
                if !v.is_finite() {
                    return false;
                }
            }
            return true;
        }
    }

    /// `FugaRenderer.Aftermath` (net/godwand/client/FugaRenderer.java)
    pub mod fuga_renderer_aftermath {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // FugaRenderer.Aftermath.Aftermath(3): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaRenderer.Aftermath.center(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaRenderer.Aftermath.at(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaRenderer.Aftermath.cast(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `FugaRenderer.Clock` (net/godwand/client/FugaRenderer.java)
    pub mod fuga_renderer_clock {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Clock {
            pub token: i64,
            pub phase: i32,
            pub age: f64,
            pub at: f64,
        }
        impl Clock {
            pub fn new(token: i64, phase: i32, age: f64, at: f64) -> Self {
                let mut s = Self { token: token, phase: phase, age: age, at: at };
                s.token = token;
                s.phase = phase;
                s.age = age;
                s.at = at;
                s
            }
            pub fn token(&self) -> i64 {
                return self.token;
            }
            pub fn phase(&self) -> i32 {
                return self.phase;
            }
            pub fn age(&self) -> f64 {
                return self.age;
            }
            pub fn at(&self) -> f64 {
                return self.at;
            }
        }
    }

    /// `FugaRenderer.Deferred` (net/godwand/client/FugaRenderer.java)
    pub mod fuga_renderer_deferred {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Camera`. Its static parts are below.
        // FugaRenderer.Deferred.Deferred(5): not translated, its class isn't translated (it uses Minecraft's `Camera`).
        // FugaRenderer.Deferred.pose(0): not translated, its class isn't translated (it uses Minecraft's `Camera`).
        // FugaRenderer.Deferred.modelView(0): not translated, its class isn't translated (it uses Minecraft's `Camera`).
        // FugaRenderer.Deferred.projection(0): not translated, its class isn't translated (it uses Minecraft's `Camera`).
        // FugaRenderer.Deferred.camera(0): not translated, its class isn't translated (it uses Minecraft's `Camera`).
        // FugaRenderer.Deferred.partial(0): not translated, its class isn't translated (it uses Minecraft's `Camera`).
    }

    /// `FugaRenderer.FlameTexture` (net/godwand/client/FugaRenderer.java)
    pub mod fuga_renderer_flame_texture {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `DynamicTexture`. Its static parts are below.
        // FugaRenderer.FlameTexture.FlameTexture(1): not translated, its class isn't translated (it extends Minecraft's `DynamicTexture`).
    }

    /// `FugaRenderer.Frame` (net/godwand/client/FugaRenderer.java)
    pub mod fuga_renderer_frame {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // FugaRenderer.Frame.Frame(6): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaRenderer.Frame.center(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaRenderer.Frame.arrowPosition(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaRenderer.Frame.casterId(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaRenderer.Frame.startedAt(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaRenderer.Frame.phase(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaRenderer.Frame.age(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaRenderer.Frame.origin(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaRenderer.Frame.target(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `FugaRenderer.Glow` (net/godwand/client/FugaRenderer.java)
    pub mod fuga_renderer_glow {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // FugaRenderer.Glow.Glow(4): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaRenderer.Glow.caster(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaRenderer.Glow.token(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaRenderer.Glow.phaseAt(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FugaRenderer.Glow.center(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `FugaSoundscape` (net/godwand/client/FugaSoundscape.java)
    pub mod fuga_soundscape {
        use super::*;
        pub const CHARGE: &str = "fuga_charge";
        pub const HOLD: &str = "fuga_hold";
        pub const RELEASE: &str = "fuga_release";
        pub const END: &str = "fuga_end";
        pub const PLACEHOLDER_SHA: &str = "9dfd4844812c24144949e8743103d18ff27928717aab16c69fabbc7225934eeb";
        // CASTS: not translated, it uses Cast, which isn't translated (it uses Minecraft's `SimpleSoundInstance`).
        pub static CUSTOM: LazyLock<JCell<JMap<String, bool>>> = LazyLock::new(|| JCell::new(JMap::new()));
        // world: not translated, it uses Minecraft's `ClientLevel`.
        #[derive(Clone, Debug)]
        pub struct FugaSoundscape {
        }
        impl FugaSoundscape {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn id(namespace: &str, path: &str) -> JIdentifier {
            return JIdentifier::new(&namespace, &path);
        }
        // FugaSoundscape.isCustomAudio(1): not translated, a try/catch (NoSuchAlgorithmException).
        // FugaSoundscape.custom(2): not translated, it uses Minecraft's `Minecraft`.
        // FugaSoundscape.tick(3): not translated, it uses Minecraft's `Minecraft`.
        // FugaSoundscape.gain(3): not translated, it uses Minecraft's `Minecraft`.
        // FugaSoundscape.play(3): not translated, it uses Minecraft's `SimpleSoundInstance`.
        // FugaSoundscape.cue(5): not translated, it uses Minecraft's `Minecraft`.
        // FugaSoundscape.fadeHold(1): not translated, it uses Hold, which isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`).
        // FugaSoundscape.stop(2): not translated, it uses Minecraft's `Minecraft`.
        // FugaSoundscape.reset(0): not translated, it calls FugaSoundscape.stop, which isn't translated.
    }

    /// `FugaSoundscape.Cast` (net/godwand/client/FugaSoundscape.java)
    pub mod fuga_soundscape_cast {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `SimpleSoundInstance`. Its static parts are below.
        // FugaSoundscape.Cast.Cast(1): not translated, its class isn't translated (it uses Minecraft's `SimpleSoundInstance`).
    }

    /// `FugaSoundscape.Hold` (net/godwand/client/FugaSoundscape.java)
    pub mod fuga_soundscape_hold {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractTickableSoundInstance`. Its static parts are below.
        // FugaSoundscape.Hold.Hold(2): not translated, its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`).
        // FugaSoundscape.Hold.tick(0): not translated, its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`).
    }

    /// `GodWandCinematicEngine` (net/godwand/client/GodWandCinematicEngine.java)
    pub mod god_wand_cinematic_engine {
        use super::*;
        pub static TRAUMA: JCell<f32> = JCell::new(0.0f32);
        pub static TRAUMA_DECAY: JCell<f32> = JCell::new(0.045f32);
        pub static SHAKE_TIME: JCell<f32> = JCell::new(0.0f32);
        pub static TARGET_FOV_MULTIPLIER: JCell<f32> = JCell::new(1.0f32);
        pub static CURRENT_FOV_MULTIPLIER: JCell<f32> = JCell::new(1.0f32);
        pub static PREV_FOV_MULTIPLIER: JCell<f32> = JCell::new(1.0f32);
        pub static FOV_HOLD_TICKS: JCell<i32> = JCell::new(0);
        pub static FOV_RECOVERY_SPEED: JCell<f32> = JCell::new(0.08f32);
        pub static VACUUM_SILENCE_TICKS: JCell<i32> = JCell::new(0);
        pub static ENVIRONMENT_GAIN: JCell<f32> = JCell::new(1.0f32);
        #[derive(Clone, Debug)]
        pub struct GodWandCinematicEngine {
        }
        impl GodWandCinematicEngine {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // GodWandCinematicEngine.onCinematicEvent(1): not translated, it uses CinematicFxPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        pub fn distance_factor(event_type: i32, distance: f64) -> f32 {
            let supernova: bool = (event_type == 5) || (event_type == 6);
            let near: f64 = if supernova { 64.0f64 } else { 8.0f64 };
            let far: f64 = if supernova { 250.0f64 } else { 64.0f64 };
            return ((j_clamp(1.0f64 - ((distance - near) / (far - near)), 0.0f64, 1.0f64)) as f32);
        }
        pub fn set_fov_focus(target_multiplier: f32, hold_ticks: i32, recovery_speed: f32) {
            god_wand_cinematic_engine::TARGET_FOV_MULTIPLIER.set(target_multiplier);
            god_wand_cinematic_engine::FOV_HOLD_TICKS.set(hold_ticks);
            god_wand_cinematic_engine::FOV_RECOVERY_SPEED.set(recovery_speed);
        }
        pub fn add_trauma(amount: f32, duration_ticks: i32) {
            god_wand_cinematic_engine::TRAUMA.set(j_clamp(god_wand_cinematic_engine::TRAUMA.get() + amount, 0.0f32, 1.0f32));
            if duration_ticks > 0 {
                god_wand_cinematic_engine::TRAUMA_DECAY.set(0.015f32.max(1.0f32 / (duration_ticks as f32)));
            }
        }
        pub fn set_fov_punch(multiplier: f32, hold_ticks: i32) {
            god_wand_cinematic_engine::TARGET_FOV_MULTIPLIER.set(multiplier);
            god_wand_cinematic_engine::CURRENT_FOV_MULTIPLIER.set(multiplier);
            god_wand_cinematic_engine::FOV_HOLD_TICKS.set(hold_ticks);
            god_wand_cinematic_engine::FOV_RECOVERY_SPEED.set(0.065f32);
        }
        pub fn set_vacuum_silence(ticks: i32) {
            god_wand_cinematic_engine::VACUUM_SILENCE_TICKS.set((0i32).max(ticks));
        }
        // GodWandCinematicEngine.onClientTick(0): not translated, it uses Minecraft's `Minecraft`.
        // GodWandCinematicEngine.applyScreenShake(2): not translated, a nullable object kept in a changing static.
        // GodWandCinematicEngine.getFovMultiplier(1): not translated, a nullable object kept in a changing static.
        // GodWandCinematicEngine.getEnvironmentGain(0): not translated, it calls FugaCinematic.environmentGain, which isn't translated.
        pub fn is_vacuum_silence_active() -> bool {
            return god_wand_cinematic_engine::VACUUM_SILENCE_TICKS.get() > 0;
        }
        // GodWandCinematicEngine.reset(0): not translated, it calls SupernovaSoundscape.reset, which isn't translated.
    }

    /// `GodWandPostFxManager` (net/godwand/client/GodWandPostFxManager.java)
    pub mod god_wand_post_fx_manager {
        use super::*;
        pub static PENDING_FRAME: LazyLock<JCell<Option<god_wand_post_fx_manager_frame::Frame>>> = LazyLock::new(|| JCell::new(None));
        // postProcessor: not translated, it uses GodWandPostChain, which isn't translated (it implements `AutoCloseable`).
        // novaProcessor: not translated, it uses GodWandPostChain, which isn't translated (it implements `AutoCloseable`).
        pub static ACTIVE: JCell<bool> = JCell::new(false);
        pub static CURRENT_TICK: JCell<i32> = JCell::new(0);
        pub static WAITING_FOR_BURST_TICKS: JCell<i32> = JCell::new(0i32);
        pub static MAX_TICKS: JCell<i32> = JCell::new(0);
        pub static FX_TYPE: JCell<i32> = JCell::new(0);
        pub static BASE_INTENSITY: JCell<f32> = JCell::new(1.0f32);
        // blastCenter: not translated, it uses Minecraft's `Vec3`.
        pub static RECALL_CORE_START: JCell<f32> = JCell::new(0.0f32);
        #[derive(Clone, Debug)]
        pub struct GodWandPostFxManager {
        }
        impl GodWandPostFxManager {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn singularity_radius(progress: f32) -> f32 {
            let grow: f32 = j_clamp(progress / 0.1f32, 0.0f32, 1.0f32);
            let collapse: f32 = j_clamp((progress - 0.8f32) / 0.2f32, 0.0f32, 1.0f32);
            return (j_lerp(grow * (2.0f32 - grow), 0.18f32, 0.68f32)) * (1.0f32 - (collapse * collapse));
        }
        pub fn singularity_recall_radius(start: f32, progress: f32) -> f32 {
            let collapse: f32 = j_clamp(progress / 0.6f32, 0.0f32, 1.0f32);
            return start * (1.0f32 - (collapse * collapse));
        }
        pub fn supernova_radius(progress: f32) -> f32 {
            let burst: f32 = j_clamp(progress / 0.1f32, 0.0f32, 1.0f32);
            let radius: f32 = j_lerp(1.0f32 - ((1.0f32 - burst) * (1.0f32 - burst)), 14.0f32, 68.0f32);
            let swell: f32 = j_clamp((progress - 0.1f32) / 0.3f32, 0.0f32, 1.0f32);
            return j_lerp(swell * (2.0f32 - swell), radius, 98.0f32);
        }
        pub fn black_hole_radius(progress: f32) -> f32 {
            return supernova_sequence::core_radius(((progress * 340.0f32) as f64));
        }
        pub fn supernova_aftermath(progress: f32) -> f32 {
            let t: f32 = j_clamp((progress - 0.22f32) / 0.18f32, 0.0f32, 1.0f32);
            return (t * t) * (3.0f32 - (2.0f32 * t));
        }
        // GodWandPostFxManager.triggerImpact(6): not translated, it uses GodWandPostFxManager.blastCenter, which isn't translated (it uses Minecraft's `Vec3`).
        // GodWandPostFxManager.onClientTick(0): not translated, `++` inside an expression.
        // GodWandPostFxManager.reset(0): not translated, it calls GodWandPostFxManager.reloadShaders, which isn't translated.
        // GodWandPostFxManager.reloadShaders(0): not translated, it uses GodWandPostFxManager.postProcessor, which isn't translated (it uses GodWandPostChain, which isn't translated (it implements `AutoCloseable`)).
        // GodWandPostFxManager.bloomDimension(1): not translated, a nullable object kept in a changing static.
        // GodWandPostFxManager.onWorldRender(3): not translated, a nullable object kept in a changing static.
        // GodWandPostFxManager.renderAfterWorld(0): not translated, it calls GodWandPostFxManager.renderFrame, which isn't translated.
        // GodWandPostFxManager.renderFrame(1): not translated, it uses Minecraft's `Vec3`.
    }

    /// `GodWandPostFxManager.Frame` (net/godwand/client/GodWandPostFxManager.java)
    pub mod god_wand_post_fx_manager_frame {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Frame {
            pub position_matrix: JMat4,
            pub projection_matrix: JMat4,
            pub tick_delta: f32,
        }
        impl Frame {
            pub fn new(position_matrix: JMat4, projection_matrix: JMat4, tick_delta: f32) -> Self {
                let mut s = Self { position_matrix: position_matrix, projection_matrix: projection_matrix, tick_delta: tick_delta };
                s.position_matrix = position_matrix;
                s.projection_matrix = projection_matrix;
                s.tick_delta = tick_delta;
                s
            }
            pub fn position_matrix(&self) -> JMat4 {
                return self.position_matrix;
            }
            pub fn projection_matrix(&self) -> JMat4 {
                return self.projection_matrix;
            }
            pub fn tick_delta(&self) -> f32 {
                return self.tick_delta;
            }
        }
    }

    /// `GodWandConfigScreen` (net/godwand/client/gui/GodWandConfigScreen.java)
    pub mod god_wand_config_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        // GodWandConfigScreen.GodWandConfigScreen(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GodWandConfigScreen.init(0): not translated, its body couldn't be read: an anonymous class.
        // GodWandConfigScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GodWandConfigScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `ImpactDebrisRenderer` (net/godwand/client/ImpactDebrisRenderer.java)
    pub mod impact_debris_renderer {
        use super::*;
        pub const SEGMENTS: i32 = 64;
        pub const RINGS: i32 = 12;
        // BURSTS: not translated, it uses Burst, which isn't translated (it uses Minecraft's `Vec3`).
        // world: not translated, it uses Minecraft's `ClientLevel`.
        #[derive(Clone, Debug)]
        pub struct ImpactDebrisRenderer {
        }
        impl ImpactDebrisRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ImpactDebrisRenderer.reset(0): not translated, it uses ImpactDebrisRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`).
        // ImpactDebrisRenderer.add(4): not translated, it uses Minecraft's `Vec3`.
        // ImpactDebrisRenderer.receive(1): not translated, it's overloaded with the same number of parameters.
        // ImpactDebrisRenderer.receive(1): not translated, it's overloaded with the same number of parameters.
        pub fn wave_radius(radius: f64, age: f64, nova: bool) -> f64 {
            let progress: f64 = j_clamp((age - ((if nova { 6 } else { 0 }) as f64)) / ((if nova { 42 } else { 18 }) as f64), 0.0f64, 1.0f64);
            return radius * progress.sqrt();
        }
        // ImpactDebrisRenderer.fragmentOffset(4): not translated, it uses Minecraft's `Vec3`.
        // ImpactDebrisRenderer.render(3): not translated, it uses Minecraft's `Camera`.
        // ImpactDebrisRenderer.emit(4): not translated, it uses Minecraft's `VertexConsumer`.
        // ImpactDebrisRenderer.emitNovaPressure(4): not translated, it uses Minecraft's `VertexConsumer`.
        // ImpactDebrisRenderer.emitNovaEmbers(4): not translated, it uses Minecraft's `VertexConsumer`.
        // ImpactDebrisRenderer.vertex(7): not translated, it uses Minecraft's `VertexConsumer`.
        // ImpactDebrisRenderer.quad(10): not translated, it uses Minecraft's `VertexConsumer`.
    }

    /// `ImpactDebrisRenderer.Burst` (net/godwand/client/ImpactDebrisRenderer.java)
    pub mod impact_debris_renderer_burst {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // ImpactDebrisRenderer.Burst.Burst(5): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ImpactDebrisRenderer.Burst.sampleSurface(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ImpactDebrisRenderer.Burst.point(3): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ImpactDebrisRenderer.Burst.height(2): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `AquaHeartiaModel` (net/godwand/client/model/AquaHeartiaModel.java)
    pub mod aqua_heartia_model {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ModelPart`. Its static parts are below.
        // CHASSIS_QUADS: not translated, it calls AquaHeartiaModel.createChassisQuads, which isn't translated.
        // SILVER_QUADS: not translated, it calls AquaHeartiaModel.createSilverQuads, which isn't translated.
        // VENT_QUADS: not translated, it calls AquaHeartiaModel.createVentQuads, which isn't translated.
        // SPHERE_QUADS: not translated, it calls AquaHeartiaModel.createSphereQuads, which isn't translated.
        // CORE_SPHERE_QUADS: not translated, it calls AquaHeartiaModel.createSphereQuads, which isn't translated.
        // AURA_SPHERE_QUADS: not translated, it calls AquaHeartiaModel.createSphereQuads, which isn't translated.
        // AquaHeartiaModel.AquaHeartiaModel(1): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // AquaHeartiaModel.getTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // AquaHeartiaModel.setupProceduralAnimations(3): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // AquaHeartiaModel.renderToBuffer(5): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // AquaHeartiaModel.renderWand(6): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // AquaHeartiaModel.renderQuads(6): not translated, it uses Minecraft's `VertexConsumer`.
        // AquaHeartiaModel.quad(9): not translated, it uses Minecraft's `VertexConsumer`.
        // AquaHeartiaModel.enlargeHead(1): not translated, new HashMap<>() without its types.
        pub fn add_mitered_tube(list: &mut Vec<Vec<JVec3>>, points: &[Vec<f32>], radii: &[f32]) {
            let n: i32 = (points.len() as i32);
            if n >= 2 {
                let mut pts: Vec<JVec3> = vec![JVec3::new(0.0, 0.0, 0.0); n as usize];
                for i in 0..n {
                    pts[i as usize] = JVec3::new(points[i as usize][0] / 16.0f32, points[i as usize][1] / 16.0f32, points[i as usize][2] / 16.0f32);
                }
                let mut tangents: Vec<JVec3> = vec![JVec3::new(0.0, 0.0, 0.0); n as usize];
                for i in 0..n {
                    if i == 0 {
                        let mut t: JVec3 = pts[1].sub(pts[0]);
                        { let joml_0 = t.normalize(); t = joml_0; joml_0 };
                        tangents[i as usize] = t;
                    } else if i == (n - 1) {
                        let mut t: JVec3 = pts[(n - 1) as usize].sub(pts[(n - 2) as usize]);
                        { let joml_1 = t.normalize(); t = joml_1; joml_1 };
                        tangents[i as usize] = t;
                    } else {
                        let t1: JVec3 = (pts[i as usize].sub(pts[(i - 1) as usize])).normalize();
                        let t2: JVec3 = (pts[(i + 1) as usize].sub(pts[i as usize])).normalize();
                        let mut t: JVec3 = (t1.add(t2)).normalize();
                        tangents[i as usize] = t;
                    }
                }
                let mut rings: Vec<Vec<JVec3>> = Vec::new();
                for i in 0..n {
                    let mut t: JVec3 = tangents[i as usize];
                    let up: JVec3 = if t.y.abs() < 0.9f32 { JVec3::new(0.0f32, 1.0f32, 0.0f32) } else { JVec3::new(1.0f32, 0.0f32, 0.0f32) };
                    let s1: JVec3 = (t.cross(up)).normalize();
                    let s2: JVec3 = (t.cross(s1)).normalize();
                    let r: f32 = radii[i as usize] / 16.0f32;
                    let mut ring: Vec<JVec3> = vec![JVec3::new(0.0, 0.0, 0.0); 8];
                    for j in 0..8 {
                        let ang: f64 = (((j as f64) + 0.5f64) * std::f64::consts::PI) / 4.0f64;
                        let c: f32 = (ang.cos() as f32);
                        let s: f32 = (ang.sin() as f32);
                        let off: JVec3 = (s1.mul_s(c * r)).add(s2.mul_s(s * r));
                        ring[j as usize] = pts[i as usize].add(off);
                    }
                    rings.push(ring.to_vec());
                }
                for i in 0..(n - 1) {
                    let r1 = &rings[i as usize];
                    let r2 = &rings[(i + 1) as usize];
                    for j in 0..8 {
                        let nxt: i32 = (j + 1) % 8;
                        list.push(vec![r1[j as usize], r2[j as usize], r2[nxt as usize], r1[nxt as usize]]);
                    }
                }
            }
        }
        pub fn add_beveled_box(list: &mut Vec<Vec<JVec3>>, min_x: f32, max_x: f32, min_y: f32, max_y: f32, min_z: f32, max_z: f32) {
            let x0: f32 = min_x / 16.0f32;
            let x1: f32 = max_x / 16.0f32;
            let y0: f32 = min_y / 16.0f32;
            let y1: f32 = max_y / 16.0f32;
            let z0: f32 = min_z / 16.0f32;
            let z1: f32 = max_z / 16.0f32;
            let p0: JVec3 = JVec3::new(x0, y0, z0);
            let p1: JVec3 = JVec3::new(x1, y0, z0);
            let p2: JVec3 = JVec3::new(x1, y1, z0);
            let p3: JVec3 = JVec3::new(x0, y1, z0);
            let p4: JVec3 = JVec3::new(x0, y0, z1);
            let p5: JVec3 = JVec3::new(x1, y0, z1);
            let p6: JVec3 = JVec3::new(x1, y1, z1);
            let p7: JVec3 = JVec3::new(x0, y1, z1);
            list.push(vec![p0, p1, p2, p3]);
            list.push(vec![p5, p4, p7, p6]);
            list.push(vec![p4, p0, p3, p7]);
            list.push(vec![p1, p5, p6, p2]);
            list.push(vec![p3, p2, p6, p7]);
            list.push(vec![p4, p5, p1, p0]);
        }
        // AquaHeartiaModel.createSphereQuads(3): not translated, List.toArray.
        // AquaHeartiaModel.createChassisQuads(0): not translated, it calls AquaHeartiaModel.enlargeHead, which isn't translated.
        // AquaHeartiaModel.createSilverQuads(0): not translated, it calls AquaHeartiaModel.enlargeHead, which isn't translated.
        pub fn get_band_corners(pt: JVec3, nx: f32, ny: f32, nz: f32, bx: f32, by: f32, bz: f32, w: f32, h: f32) -> Vec<JVec3> {
            let wn: f32 = (w * 0.5f32) / 16.0f32;
            let hb: f32 = (h * 0.5f32) / 16.0f32;
            let px: f32 = pt.x / 16.0f32;
            let py: f32 = pt.y / 16.0f32;
            let pz: f32 = pt.z / 16.0f32;
            return vec![JVec3::new((px - (nx * wn)) - (bx * hb), (py - (ny * wn)) - (by * hb), (pz - (nz * wn)) - (bz * hb)), JVec3::new((px + (nx * wn)) - (bx * hb), (py + (ny * wn)) - (by * hb), (pz + (nz * wn)) - (bz * hb)), JVec3::new((px + (nx * wn)) + (bx * hb), (py + (ny * wn)) + (by * hb), (pz + (nz * wn)) + (bz * hb)), JVec3::new((px - (nx * wn)) + (bx * hb), (py - (ny * wn)) + (by * hb), (pz - (nz * wn)) + (bz * hb))];
        }
        // AquaHeartiaModel.createVentQuads(0): not translated, it calls AquaHeartiaModel.enlargeHead, which isn't translated.
    }

    /// `GodWandModel` (net/godwand/client/model/GodWandModel.java)
    pub mod god_wand_model {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ModelPart`. Its static parts are below.
        pub const CX: f32 = -0.55f32;
        pub const CY: f32 = 21.2f32;
        pub const C_RADIUS: f32 = 5.9f32;
        pub const NUM_SEGMENTS: i32 = 9;
        // SHAFT_QUADS: not translated, it calls GodWandModel.createShaftQuads, which isn't translated.
        // GOLD_COLLARS: not translated, it calls GodWandModel.createGoldCollars, which isn't translated.
        // LEATHER_QUADS: not translated, it calls GodWandModel.createLeatherQuads, which isn't translated.
        // WRAPPING_QUADS: not translated, it calls GodWandModel.createWrappingQuads, which isn't translated.
        // REACTOR_QUADS: not translated, it calls GodWandModel.createReactorQuads, which isn't translated.
        // CRESCENT_QUADS: not translated, it calls GodWandModel.createCrescentQuads, which isn't translated.
        // GOLD_RAIL: not translated, it calls GodWandModel.createGoldRail, which isn't translated.
        // RED_VENTS: not translated, it calls GodWandModel.createRedVents, which isn't translated.
        // GOLD_BRACKETS: not translated, it calls GodWandModel.createGoldBrackets, which isn't translated.
        // TOP_TALON: not translated, it calls GodWandModel.createTopTalon, which isn't translated.
        // POMMEL_BLADE: not translated, it calls GodWandModel.createPommelBlade, which isn't translated.
        // GodWandModel.GodWandModel(1): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // GodWandModel.getTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // GodWandModel.setupProceduralAnimations(3): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // GodWandModel.renderWand(6): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // GodWandModel.renderQuads(6): not translated, it uses Minecraft's `VertexConsumer`.
        pub fn add_octagonal_prism(list: &mut Vec<Vec<JVec3>>, r_bot: f32, r_top: f32, y_bot: f32, y_top: f32, cx: f32, cz: f32) {
            let mut b_pts: Vec<JVec3> = vec![JVec3::new(0.0, 0.0, 0.0); 8];
            let mut t_pts: Vec<JVec3> = vec![JVec3::new(0.0, 0.0, 0.0); 8];
            for i in 0..8 {
                let ang: f64 = (((i as f64) + 0.5f64) * std::f64::consts::PI) / 4.0f64;
                let c: f32 = (ang.cos() as f32);
                let s: f32 = (ang.sin() as f32);
                b_pts[i as usize] = JVec3::new(cx + (r_bot * c), y_bot, cz + (r_bot * s));
                t_pts[i as usize] = JVec3::new(cx + (r_top * c), y_top, cz + (r_top * s));
            }
            for i in 0..8 {
                let nxt: i32 = (i + 1) % 8;
                list.push(vec![b_pts[i as usize], t_pts[i as usize], t_pts[nxt as usize], b_pts[nxt as usize]]);
            }
            let cb: JVec3 = JVec3::new(cx, y_bot, cz);
            let ct: JVec3 = JVec3::new(cx, y_top, cz);
            for i in 0..8 {
                let nxt: i32 = (i + 1) % 8;
                list.push(vec![cb, b_pts[nxt as usize], b_pts[i as usize], b_pts[i as usize]]);
                list.push(vec![ct, t_pts[i as usize], t_pts[nxt as usize], t_pts[nxt as usize]]);
            }
        }
        pub fn add_beveled_ring(list: &mut Vec<Vec<JVec3>>, r_in: f32, r_out: f32, y_mid: f32, height: f32, cx: f32, cz: f32) {
            let chamfer: f32 = 0.18f32.min(height * 0.3f32);
            let y0: f32 = y_mid - (height * 0.5f32);
            let y1: f32 = y0 + chamfer;
            let y2: f32 = (y_mid + (height * 0.5f32)) - chamfer;
            let y3: f32 = y_mid + (height * 0.5f32);
            let y_levels_b: Vec<f32> = vec![y0, y1, y2];
            let y_levels_t: Vec<f32> = vec![y1, y2, y3];
            let r_levels_b: Vec<f32> = vec![r_out - chamfer, r_out, r_out];
            let r_levels_t: Vec<f32> = vec![r_out, r_out, r_out - chamfer];
            for layer in 0..3 {
                let yb: f32 = y_levels_b[layer as usize];
                let yt: f32 = y_levels_t[layer as usize];
                let rb: f32 = r_levels_b[layer as usize];
                let rt: f32 = r_levels_t[layer as usize];
                for i in 0..8 {
                    let ang0: f64 = (((i as f64) + 0.5f64) * std::f64::consts::PI) / 4.0f64;
                    let ang1: f64 = (((i as f64) + 1.5f64) * std::f64::consts::PI) / 4.0f64;
                    let c0: f32 = (ang0.cos() as f32);
                    let s0: f32 = (ang0.sin() as f32);
                    let c1: f32 = (ang1.cos() as f32);
                    let s1: f32 = (ang1.sin() as f32);
                    list.push(vec![JVec3::new(cx + (rb * c0), yb, cz + (rb * s0)), JVec3::new(cx + (rt * c0), yt, cz + (rt * s0)), JVec3::new(cx + (rt * c1), yt, cz + (rt * s1)), JVec3::new(cx + (rb * c1), yb, cz + (rb * s1))]);
                }
            }
        }
        // GodWandModel.createShaftQuads(0): not translated, List.toArray.
        // GodWandModel.createGoldCollars(0): not translated, List.toArray.
        // GodWandModel.createLeatherQuads(0): not translated, List.toArray.
        // GodWandModel.createWrappingQuads(0): not translated, List.toArray.
        // GodWandModel.createReactorQuads(0): not translated, List.toArray.
        // GodWandModel.createCrescentQuads(0): not translated, List.toArray.
        // GodWandModel.renderCrescent(4): not translated, it uses Minecraft's `VertexConsumer`.
        // GodWandModel.createGoldRail(0): not translated, List.toArray.
        // GodWandModel.createRedVents(0): not translated, List.toArray.
        // GodWandModel.createGoldBrackets(0): not translated, List.toArray.
        // GodWandModel.createTopTalon(0): not translated, List.toArray.
        // GodWandModel.createPommelBlade(0): not translated, List.toArray.
        // GodWandModel.renderPommelBlade(4): not translated, it uses Minecraft's `VertexConsumer`.
        // GodWandModel.renderDiamondBezel(4): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // GodWandModel.renderFacetedGem(8): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // GodWandModel.renderEmberMotes(4): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // GodWandModel.quad(9): not translated, it uses Minecraft's `VertexConsumer`.
        // GodWandModel.vertex(9): not translated, it uses Minecraft's `VertexConsumer`.
        // GodWandModel.renderToBuffer(5): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
    }

    /// `ShadowBladeModel` (net/godwand/client/model/ShadowBladeModel.java)
    pub mod shadow_blade_model {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ModelPart`. Its static parts are below.
        // POMMEL_QUADS: not translated, it calls ShadowBladeModel.createPommelQuads, which isn't translated.
        // HILT_QUADS: not translated, it calls ShadowBladeModel.createHiltQuads, which isn't translated.
        // TSUBA_QUADS: not translated, it calls ShadowBladeModel.createTsubaQuads, which isn't translated.
        // BLADE_QUADS: not translated, it calls ShadowBladeModel.createBladeQuads, which isn't translated.
        // ShadowBladeModel.ShadowBladeModel(1): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // ShadowBladeModel.getTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // ShadowBladeModel.setupProceduralAnimations(3): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // ShadowBladeModel.renderToBuffer(5): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // ShadowBladeModel.renderBlade(6): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // ShadowBladeModel.renderQuads(6): not translated, it uses Minecraft's `VertexConsumer`.
        // ShadowBladeModel.quad(9): not translated, it uses Minecraft's `VertexConsumer`.
        // ShadowBladeModel.createHiltQuads(0): not translated, List.toArray.
        // ShadowBladeModel.createTsubaQuads(0): not translated, List.toArray.
        // ShadowBladeModel.createPommelQuads(0): not translated, List.toArray.
        pub fn profile(faces: &mut Vec<Vec<JVec3>>, outline: &[Vec<f32>], thickness: f32) {
            let mut cy: f32 = 0.0f32;
            let mut cz: f32 = 0.0f32;
            let mut area: f32 = 0.0f32;
            for i in 0..(outline.len() as i32) {
                let p = &outline[i as usize];
                let next = &outline[((i + 1) % (outline.len() as i32)) as usize];
                cy += p[0] / ((outline.len() as i32) as f32);
                cz += p[1] / ((outline.len() as i32) as f32);
                area += (p[0] * next[1]) - (next[0] * p[1]);
            }
            for i in 0..(outline.len() as i32) {
                let a = &outline[(if area >= 0.0f32 { i } else { (i + 1) % (outline.len() as i32) }) as usize];
                let b = &outline[(if area >= 0.0f32 { (i + 1) % (outline.len() as i32) } else { i }) as usize];
                let edge_a: JVec3 = shadow_blade_model::point(0.0f32, a[0], a[1]);
                let edge_b: JVec3 = shadow_blade_model::point(0.0f32, b[0], b[1]);
                for &side in (vec![-1, 1]).iter() {
                    let inset_a: JVec3 = shadow_blade_model::point((side as f32) * thickness, cy + ((a[0] - cy) * 0.76f32), cz + ((a[1] - cz) * 0.76f32));
                    let inset_b: JVec3 = shadow_blade_model::point((side as f32) * thickness, cy + ((b[0] - cy) * 0.76f32), cz + ((b[1] - cz) * 0.76f32));
                    if side > 0 {
                        faces.push(vec![edge_a, edge_b, inset_b, inset_a]);
                        faces.push(vec![shadow_blade_model::point(thickness, cy, cz), inset_a, inset_b, shadow_blade_model::point(thickness, cy, cz)]);
                    } else {
                        faces.push(vec![edge_b, edge_a, inset_a, inset_b]);
                        faces.push(vec![shadow_blade_model::point(-thickness, cy, cz), inset_b, inset_a, shadow_blade_model::point(-thickness, cy, cz)]);
                    }
                }
            }
        }
        // ShadowBladeModel.createBladeQuads(0): not translated, List.toArray.
        pub fn point(x: f32, y: f32, z: f32) -> JVec3 {
            return JVec3::new(x / 16.0f32, y / 16.0f32, z / 16.0f32);
        }
    }

    /// `GodWandPostChain` (net/godwand/client/postfx/GodWandPostChain.java)
    pub mod god_wand_post_chain {
        use super::*;
        // The class itself isn't translated: it implements `AutoCloseable`. Its static parts are below.
        // GodWandPostChain.GodWandPostChain(1): not translated, its class isn't translated (it implements `AutoCloseable`).
        // GodWandPostChain.setUniform(2): not translated, its class isn't translated (it implements `AutoCloseable`).
        // GodWandPostChain.setUniform(2): not translated, its class isn't translated (it implements `AutoCloseable`).
        // GodWandPostChain.resize(2): not translated, its class isn't translated (it implements `AutoCloseable`).
        // GodWandPostChain.process(0): not translated, its class isn't translated (it implements `AutoCloseable`).
        // GodWandPostChain.close(0): not translated, its class isn't translated (it implements `AutoCloseable`).
    }

    /// `GodWandItemModel` (net/godwand/client/render/GodWandItemModel.java)
    pub mod god_wand_item_model {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ItemModel`. Its static parts are below.
        // GodWandItemModel.GodWandItemModel(1): not translated, its class isn't translated (it implements Minecraft's `ItemModel`).
        // GodWandItemModel.update(7): not translated, its class isn't translated (it implements Minecraft's `ItemModel`).
    }

    /// `GodWandItemModel.Renderer` (net/godwand/client/render/GodWandItemModel.java)
    pub mod god_wand_item_model_renderer {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `SpecialModelRenderer`. Its static parts are below.
        // GodWandItemModel.Renderer.Renderer(1): not translated, its class isn't translated (it implements Minecraft's `SpecialModelRenderer`).
        // GodWandItemModel.Renderer.submit(7): not translated, its class isn't translated (it implements Minecraft's `SpecialModelRenderer`).
        // GodWandItemModel.Renderer.getExtents(1): not translated, its class isn't translated (it implements Minecraft's `SpecialModelRenderer`).
        // GodWandItemModel.Renderer.extractArgument(1): not translated, its class isn't translated (it implements Minecraft's `SpecialModelRenderer`).
    }

    /// `GodWandItemModel.Unbaked` (net/godwand/client/render/GodWandItemModel.java)
    pub mod god_wand_item_model_unbaked {
        use super::*;
        // The class itself isn't translated: it implements `Unbaked`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // GodWandItemModel.Unbaked.Unbaked(0): not translated, its class isn't translated (it implements `Unbaked`).
        // GodWandItemModel.Unbaked.type(0): not translated, its class isn't translated (it implements `Unbaked`).
        // GodWandItemModel.Unbaked.bake(2): not translated, its class isn't translated (it implements `Unbaked`).
        // GodWandItemModel.Unbaked.resolveDependencies(1): not translated, its class isn't translated (it implements `Unbaked`).
    }

    /// `GodWandItemRenderer` (net/godwand/client/render/GodWandItemRenderer.java)
    pub mod god_wand_item_renderer {
        use super::*;
        // The class itself isn't translated: it uses GodWandModel, which isn't translated (it uses Minecraft's `ModelPart`). Its static parts are below.
        pub static TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("godwand", "textures/item/eclipse_materials.png"));
        // chargingStack: not translated, it uses Minecraft's `ItemStack`.
        // releasedStack: not translated, it uses Minecraft's `ItemStack`.
        pub static PREVIOUS_CHARGE: JCell<f32> = JCell::new(0.0f32);
        pub static RELEASE_TICK: JCell<i64> = JCell::new(-100i64);
        // trackedWorld: not translated, it uses Minecraft's `ClientLevel`.
        // GodWandItemRenderer.tick(1): not translated, it uses Minecraft's `Minecraft`.
        pub fn release_recoil(ticks: f32) -> f32 {
            return if (!(ticks < 0.0f32)) && (!(ticks >= 12.0f32)) { ((((ticks as f64) * ((((-ticks) as f64) * 0.65f64).exp())) * 1.7f64) as f32) } else { 0.0f32 };
        }
        pub fn core_color(mode: i32, charge: f32) -> i32 {
            let target: i32 = match mode {
                0 => 16723285,
                1 => 12486911,
                2 => 16739633,
                3 => 1445158,
                4 => 16773568,
                5 => 14242047,
                _ => 16723285,
            };
            let t: f32 = 0.28f32 + (j_clamp(charge, 0.0f32, 1.0f32) * 0.72f32);
            let mut color: i32 = -16777216;
            for &shift in (vec![16, 8, 0]).iter() {
                let idle: i32 = (16719944 >> shift) & 0xFF;
                color |= (j_round_f((idle as f32) + (((((target >> shift) & 0xFF) - idle) as f32) * t))) << shift;
            }
            return color;
        }
        // GodWandItemRenderer.submit(6): not translated, its class isn't translated (it uses GodWandModel, which isn't translated (it uses Minecraft's `ModelPart`)).
        // GodWandItemRenderer.render(6): not translated, its class isn't translated (it uses GodWandModel, which isn't translated (it uses Minecraft's `ModelPart`)).
        // GodWandItemRenderer.applyUseTransform(3): not translated, PoseStack.rotate.
        // GodWandItemRenderer.applyAtomicHoldTransform(3): not translated, PoseStack.rotate.
        // GodWandItemRenderer.applySwordDisplayTransform(2): not translated, it uses Minecraft's `ItemDisplayContext`.
        // GodWandItemRenderer.applyAtomicSwordHoldTransform(2): not translated, PoseStack.rotate.
        // GodWandItemRenderer.applyDisplayTransform(2): not translated, its class isn't translated (it uses GodWandModel, which isn't translated (it uses Minecraft's `ModelPart`)).
        pub fn charge_ratio(mode: i32, ticks: i32) -> f32 {
            let maximum: i32 = match mode {
                1 => 45,
                2 => 68,
                3 => 140,
                4 => 80,
                5 => 380,
                _ => 0,
            };
            return if maximum == 0 { 0.0f32 } else { j_clamp((ticks as f32) / (maximum as f32), 0.0f32, 1.0f32) };
        }
        // GodWandItemRenderer.getChargeRatio(2): not translated, its class isn't translated (it uses GodWandModel, which isn't translated (it uses Minecraft's `ModelPart`)).
    }

    /// `GodWandItemRenderer.Buffers` (net/godwand/client/render/GodWandItemRenderer.java)
    pub mod god_wand_item_renderer_buffers {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GodWandItemRenderer.Buffers.getBuffer(1): not translated, it has no body.
    }

    /// `SingularityThreadRenderer` (net/godwand/client/SingularityThreadRenderer.java)
    pub mod singularity_thread_renderer {
        use super::*;
        pub static THREADS: LazyLock<JCell<JMap<i32, singularity_thread_renderer_thread::Thread>>> = LazyLock::new(|| JCell::new(JMap::new()));
        // world: not translated, it uses Minecraft's `ClientLevel`.
        #[derive(Clone, Debug)]
        pub struct SingularityThreadRenderer {
        }
        impl SingularityThreadRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SingularityThreadRenderer.reset(0): not translated, it uses SingularityThreadRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`).
        // SingularityThreadRenderer.receive(1): not translated, it uses `SingularityThreadsPayload` (net.godwand.network).
        pub fn pulse_fraction(time: f32, seed: i32, recall_progress: f32) -> f32 {
            let phase: f32 = (((seed * 31).rem_euclid(101)) as f32) / 101.0f32;
            if recall_progress >= 0.0f32 {
                return 1.0f32 - (j_clamp((recall_progress + (phase * 0.12f32)) / 0.85f32, 0.0f32, 1.0f32));
            }
            let cycle: f32 = (time * 0.032f32) + phase;
            return 1.0f32 - (cycle - (((cycle as f64).floor()) as f32));
        }
        // SingularityThreadRenderer.render(3): not translated, it uses Minecraft's `Camera`.
    }

    /// `SingularityThreadRenderer.Thread` (net/godwand/client/SingularityThreadRenderer.java)
    pub mod singularity_thread_renderer_thread {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Thread {
            pub received: i64,
            pub recall_ticks: i32,
        }
        impl Thread {
            pub fn new(received: i64, recall_ticks: i32) -> Self {
                let mut s = Self { received: received, recall_ticks: recall_ticks };
                s.received = received;
                s.recall_ticks = recall_ticks;
                s
            }
            pub fn received(&self) -> i64 {
                return self.received;
            }
            pub fn recall_ticks(&self) -> i32 {
                return self.recall_ticks;
            }
        }
    }

    /// `SkillCooldownClient` (net/godwand/client/SkillCooldownClient.java)
    pub mod skill_cooldown_client {
        use super::*;
        // observer: not translated, it uses Minecraft's `Player`.
        // session: not translated, it uses `Object`.
        pub static CLOCK: JCell<i64> = JCell::new(0i64);
        pub static TIMERS: LazyLock<JCell<skill_cooldown_state::SkillCooldownState>> = LazyLock::new(|| JCell::new(skill_cooldown_state::SkillCooldownState::new()));
        pub static RECALL: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct SkillCooldownClient {
        }
        impl SkillCooldownClient {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SkillCooldownClient.observe(1): not translated, it uses Minecraft's `Minecraft`.
        // SkillCooldownClient.tick(1): not translated, it uses Minecraft's `Minecraft`.
        // SkillCooldownClient.receive(1): not translated, it uses `SkillCooldownPayload` (net.godwand.network).
        // SkillCooldownClient.remaining(1): not translated, it calls SkillCooldownClient.observe, which isn't translated.
        // SkillCooldownClient.fraction(1): not translated, it calls SkillCooldownClient.remaining, which isn't translated.
        // SkillCooldownClient.canUse(1): not translated, it calls SkillCooldownClient.remaining, which isn't translated.
        // SkillCooldownClient.render(1): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
    }

    /// `SkillWheelScreen` (net/godwand/client/SkillWheelScreen.java)
    pub mod skill_wheel_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        // CATEGORY: not translated, it uses Minecraft's `Category`.
        // OPEN: not translated, it uses Minecraft's `KeyMapping`.
        // OPEN_CONFIG: not translated, it uses Minecraft's `KeyMapping`.
        pub static ART: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("godwand", "textures/gui/skill_wheel.png"));
        pub static SECTORS: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("godwand", "textures/gui/skill_sectors.png"));
        pub static SECTOR: LazyLock<f64> = LazyLock::new(|| (std::f64::consts::PI * 2.0f64) / 7.0f64);
        // SkillWheelScreen.SkillWheelScreen(3): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SkillWheelScreen.handleKeyInput(1): not translated, it uses Minecraft's `Minecraft`.
        // SkillWheelScreen.wheelScale(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SkillWheelScreen.canSelect(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SkillWheelScreen.updateSelection(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SkillWheelScreen.confirm(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SkillWheelScreen.tick(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SkillWheelScreen.keyReleased(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SkillWheelScreen.mouseReleased(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SkillWheelScreen.isPauseScreen(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SkillWheelScreen.extractBackground(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SkillWheelScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SkillWheelScreen.drawWheel(4): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        pub fn skill_color(mode: i32) -> i32 {
            return match mode {
                0 => -7612161,
                1 => -12019457,
                2 => -39882,
                3 => -947285,
                4 => -5208231,
                _ => -4097294,
                6 => -25544,
            };
        }
    }

    /// `SkillWheelSelection` (net/godwand/client/SkillWheelSelection.java)
    pub mod skill_wheel_selection {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SkillWheelSelection {
        }
        impl SkillWheelSelection {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn mode_at(x: f64, y: f64) -> i32 {
            if ((x * x) + (y * y)) < 2304.0f64 {
                return -1;
            }
            let angle: f64 = (y.atan2(x)) + (std::f64::consts::PI / 2.0f64);
            return ((((angle / ((std::f64::consts::PI * 2.0f64) / 7.0f64)) + 0.5f64).floor()) as i32).rem_euclid(7);
        }
    }

    /// `SpatialAuraRenderer` (net/godwand/client/SpatialAuraRenderer.java)
    pub mod spatial_aura_renderer {
        use super::*;
        pub static ACTIVE: LazyLock<JCell<JMap<i32, i64>>> = LazyLock::new(|| JCell::new(JMap::new()));
        pub static ACTIVATIONS: LazyLock<JCell<JMap<i32, i64>>> = LazyLock::new(|| JCell::new(JMap::new()));
        // HITS: not translated, it uses Hit, which isn't translated (it uses Minecraft's `Vec3`).
        // world: not translated, it uses Minecraft's `ClientLevel`.
        pub static EDGES: LazyLock<Vec<f32>> = LazyLock::new(|| vec![-1.0f32, -0.28f32, 0.28f32, 1.0f32]);
        #[derive(Clone, Debug)]
        pub struct SpatialAuraRenderer {
        }
        impl SpatialAuraRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SpatialAuraRenderer.reset(0): not translated, it uses SpatialAuraRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`).
        // SpatialAuraRenderer.receive(1): not translated, it uses `SpatialAuraPayload` (net.godwand.network).
        // SpatialAuraRenderer.render(3): not translated, it uses Minecraft's `Camera`.
        // SpatialAuraRenderer.emitField(5): not translated, it uses Minecraft's `VertexConsumer`.
        // SpatialAuraRenderer.emitRibbon(7): not translated, it uses Minecraft's `VertexConsumer`.
        // SpatialAuraRenderer.emitHit(6): not translated, it uses Minecraft's `VertexConsumer`.
        pub fn smooth(start: f64, end: f64, value: f64) -> f64 {
            let t: f64 = j_clamp((value - start) / (end - start), 0.0f64, 1.0f64);
            return (t * t) * (3.0f64 - (2.0f64 * t));
        }
        // SpatialAuraRenderer.emitActivationShell(6): not translated, it uses Minecraft's `VertexConsumer`.
        // SpatialAuraRenderer.emitEquatorRing(5): not translated, it uses Minecraft's `VertexConsumer`.
    }

    /// `SpatialAuraRenderer.Hit` (net/godwand/client/SpatialAuraRenderer.java)
    pub mod spatial_aura_renderer_hit {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // SpatialAuraRenderer.Hit.Hit(3): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // SpatialAuraRenderer.Hit.playerId(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // SpatialAuraRenderer.Hit.normal(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // SpatialAuraRenderer.Hit.tick(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `StoneCannonChargeRenderer` (net/godwand/client/StoneCannonChargeRenderer.java)
    pub mod stone_cannon_charge_renderer {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct StoneCannonChargeRenderer {
        }
        impl StoneCannonChargeRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // StoneCannonChargeRenderer.render(3): not translated, it uses Minecraft's `Camera`.
    }

    /// `StoneCannonEntityRenderer` (net/godwand/client/StoneCannonEntityRenderer.java)
    pub mod stone_cannon_entity_renderer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EntityRenderer`. Its static parts are below.
        // StoneCannonEntityRenderer.StoneCannonEntityRenderer(1): not translated, its class isn't translated (it extends Minecraft's `EntityRenderer`).
        // StoneCannonEntityRenderer.createRenderState(0): not translated, its class isn't translated (it extends Minecraft's `EntityRenderer`).
        // StoneCannonEntityRenderer.extractRenderState(3): not translated, its class isn't translated (it extends Minecraft's `EntityRenderer`).
        // StoneCannonEntityRenderer.submit(4): not translated, its class isn't translated (it extends Minecraft's `EntityRenderer`).
    }

    /// `StoneCannonEntityRenderer.State` (net/godwand/client/StoneCannonEntityRenderer.java)
    pub mod stone_cannon_entity_renderer_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EntityRenderState`. Its static parts are below.
    }

    /// `StoneCannonModel` (net/godwand/client/StoneCannonModel.java)
    pub mod stone_cannon_model {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct StoneCannonModel {
        }
        impl StoneCannonModel {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn charge_wind_point(t: f32, phase: f32, scale: f32, charge: f32) -> JVec3 {
            let angle: f32 = phase - (t * (7.0f32 + (charge * 3.0f32)));
            let radius: f32 = 0.08f32 + ((((((t as f64) * std::f64::consts::PI) * 0.5f64).sin()) as f32) * (1.65f32 - (charge * 0.35f32)));
            let nose: f32 = 2.45f32 * scale;
            return JVec3::new((((angle as f64).cos()) as f32) * radius, (((angle as f64).sin()) as f32) * radius, nose + (t * (((-4.0f32) - (charge * 2.5f32)) - nose)));
        }
        pub fn charge_wind_visibility(time: f32) -> f32 {
            let fade: f32 = 0.0f32.max(1.0f32.min((time - 20.0f32) / 24.0f32));
            return (fade * fade) * (3.0f32 - (2.0f32 * fade));
        }
        pub fn charge_drill_rotation(time: f32) -> f32 {
            let ramp: f32 = time.min(80.0f32);
            return ((time * 62.0f32) + ((ramp * ramp) * 0.45f32)) + ((0.0f32.max(time - 80.0f32)) * 72.0f32);
        }
        pub fn charge_wind_rotation(time: f32) -> f32 {
            let elapsed: f32 = 0.0f32.max(time - 20.0f32);
            let ramp: f32 = elapsed.min(60.0f32);
            return ((elapsed * 0.84f32) + ((ramp * ramp) * 0.0056f32)) + ((0.0f32.max(elapsed - 60.0f32)) * 0.672f32);
        }
        // StoneCannonModel.renderChargeWind(4): not translated, a try/finally.
        pub fn surface(t: f32, angle: f32) -> JVec3 {
            let radius: f32 = ((((1.0f32 - t) as f64).powf(0.8f64)) as f32) * (0.4f32 + (0.065f32 * (((((angle * 3.0f32) - (t * 18.0f32)) as f64).cos()) as f32)));
            return JVec3::new((((angle as f64).cos()) as f32) * radius, (((angle as f64).sin()) as f32) * radius, (-0.75f32) + (t * 3.2f32));
        }
        // StoneCannonModel.render(4): not translated, a try/finally.
    }

    /// `StoneCannonWindSound` (net/godwand/client/StoneCannonWindSound.java)
    pub mod stone_cannon_wind_sound {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractTickableSoundInstance`. Its static parts are below.
        // instance: not translated, it uses StoneCannonWindSound, which isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`).
        // StoneCannonWindSound.StoneCannonWindSound(1): not translated, its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`).
        pub fn wind_volume(ticks: i32) -> f32 {
            let ramp: f32 = j_clamp(((ticks - 50) as f32) / 30.0f32, 0.0f32, 1.0f32);
            return (0.18f32 * ramp) * ramp;
        }
        // StoneCannonWindSound.update(1): not translated, it uses Minecraft's `Minecraft`.
        // StoneCannonWindSound.reset(0): not translated, it uses StoneCannonWindSound.instance, which isn't translated (it uses StoneCannonWindSound, which isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`)).
        // StoneCannonWindSound.tick(0): not translated, its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`).
    }

    /// `SupernovaAccretionRenderer` (net/godwand/client/SupernovaAccretionRenderer.java)
    pub mod supernova_accretion_renderer {
        use super::*;
        // world: not translated, it uses Minecraft's `ClientLevel`.
        // center: not translated, it uses Minecraft's `Vec3`.
        pub static START: JCell<i64> = JCell::new(0i64);
        pub static ACTIVE: JCell<bool> = JCell::new(false);
        // chargeWorld: not translated, it uses Minecraft's `ClientLevel`.
        // chargeCenter: not translated, it uses Minecraft's `Vec3`.
        pub static CHARGE_TICKS: JCell<f32> = JCell::new(0.0f32);
        pub static PREVIOUS_CHARGE_TICKS: JCell<f32> = JCell::new(0.0f32);
        pub static CHARGING: JCell<bool> = JCell::new(false);
        pub static EDGES: LazyLock<Vec<f32>> = LazyLock::new(|| vec![-1.0f32, -0.22f32, 0.22f32, 1.0f32]);
        // currentCameraRel: not translated, it uses Minecraft's `Vec3`.
        #[derive(Clone, Debug)]
        pub struct SupernovaAccretionRenderer {
        }
        impl SupernovaAccretionRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SupernovaAccretionRenderer.reset(0): not translated, it uses SupernovaAccretionRenderer.world, which isn't translated (it uses Minecraft's `ClientLevel`).
        // SupernovaAccretionRenderer.receive(1): not translated, it uses ImpactFxPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SupernovaAccretionRenderer.tickCharge(1): not translated, it uses Minecraft's `Minecraft`.
        // SupernovaAccretionRenderer.skyFrame(1): not translated, it uses Minecraft's `Vec3`.
        // SupernovaAccretionRenderer.age(1): not translated, it calls SupernovaAccretionRenderer.skyAge, which isn't translated.
        // SupernovaAccretionRenderer.skyAge(1): not translated, it uses Minecraft's `Minecraft`.
        // SupernovaAccretionRenderer.center(0): not translated, it uses Minecraft's `Vec3`.
        // SupernovaAccretionRenderer.render(3): not translated, it uses Minecraft's `Camera`.
        // SupernovaAccretionRenderer.emitChargeMarker(3): not translated, it uses Minecraft's `VertexConsumer`.
        pub fn stream_phase(age: f64, strand: i32) -> f64 {
            let travel: f64 = (age * (0.006f64 + ((supernova_sequence::pressure(age) as f64) * 0.009f64))) + ((strand as f64) * 0.618034f64);
            return travel - travel.floor();
        }
        // SupernovaAccretionRenderer.streamPoint(3): not translated, it uses Minecraft's `Vec3`.
        // SupernovaAccretionRenderer.emit(3): not translated, it uses Minecraft's `VertexConsumer`.
        // SupernovaAccretionRenderer.emitInfallSparks(5): not translated, it uses Minecraft's `VertexConsumer`.
        // SupernovaAccretionRenderer.arc(11): not translated, it uses Minecraft's `VertexConsumer`.
        // SupernovaAccretionRenderer.vertex(7): not translated, it uses Minecraft's `VertexConsumer`.
        // SupernovaAccretionRenderer.emitPreImpactShockwave(3): not translated, it uses Minecraft's `VertexConsumer`.
        // SupernovaAccretionRenderer.emitShockwaveRing(7): not translated, it uses Minecraft's `VertexConsumer`.
    }

    /// `SupernovaAccretionRenderer.SkyFrame` (net/godwand/client/SupernovaAccretionRenderer.java)
    pub mod supernova_accretion_renderer_sky_frame {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // SupernovaAccretionRenderer.SkyFrame.SkyFrame(6): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // SupernovaAccretionRenderer.SkyFrame.center(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // SupernovaAccretionRenderer.SkyFrame.scale(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // SupernovaAccretionRenderer.SkyFrame.collapse(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // SupernovaAccretionRenderer.SkyFrame.pulseRadius(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // SupernovaAccretionRenderer.SkyFrame.pulseStrength(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // SupernovaAccretionRenderer.SkyFrame.time(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `SupernovaSoundscape` (net/godwand/client/SupernovaSoundscape.java)
    pub mod supernova_soundscape {
        use super::*;
        pub static AUDIO: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("godwand", "supernova_sequence"));
        pub const PLACEHOLDER_SHA: &str = "225b65ec21cc27259fd742d6e1488a69c28f4a0365301d533eeaf433e3c60443";
        // chargeSound: not translated, it uses Charge, which isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`).
        // sequence: not translated, it uses Minecraft's `SimpleSoundInstance`.
        // world: not translated, it uses Minecraft's `ClientLevel`.
        pub static CUSTOM: JCell<bool> = JCell::new(false);
        pub static CUSTOM_CHECKED: JCell<bool> = JCell::new(false);
        pub static CHARGING: JCell<bool> = JCell::new(false);
        pub static ACTIVATED: JCell<bool> = JCell::new(false);
        pub static CANCEL_GRACE_TICKS: JCell<i32> = JCell::new(0i32);
        pub static ACTIVATION_TIME: JCell<i64> = JCell::new(0i64);
        #[derive(Clone, Debug)]
        pub struct SupernovaSoundscape {
        }
        impl SupernovaSoundscape {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SupernovaSoundscape.isCustomAudio(1): not translated, a try/catch (NoSuchAlgorithmException).
        // SupernovaSoundscape.checkCustom(1): not translated, it uses Minecraft's `Minecraft`.
        pub fn volume(age: f64, charge: f32) -> f32 {
            return if (age < 0.0f64) && (charge > 0.0f32) { 0.08f32 + (j_clamp(charge, 0.0f32, 1.0f32) * 0.18f32) } else { 0.0f32 };
        }
        // SupernovaSoundscape.start(1): not translated, it uses Minecraft's `Vec3`.
        // SupernovaSoundscape.tick(3): not translated, it uses Minecraft's `Minecraft`.
        // SupernovaSoundscape.reset(0): not translated, it uses Minecraft's `Minecraft`.
    }

    /// `SupernovaSoundscape.Charge` (net/godwand/client/SupernovaSoundscape.java)
    pub mod supernova_soundscape_charge {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractTickableSoundInstance`. Its static parts are below.
        // SupernovaSoundscape.Charge.Charge(1): not translated, its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`).
        // SupernovaSoundscape.Charge.tick(0): not translated, its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`).
    }

    /// `GodWandCommands` (net/godwand/command/GodWandCommands.java)
    pub mod god_wand_commands {
        use super::*;
        pub static SKILLS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["spatial_path".to_string(), "singularity".to_string(), "earth_drill".to_string(), "supernova".to_string(), "stone_cannon".to_string(), "atomic".to_string(), "fuga".to_string()]);
        #[derive(Clone, Debug)]
        pub struct GodWandCommands {
        }
        impl GodWandCommands {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // GodWandCommands.register(1): not translated, it uses Minecraft's `CommandDispatcher`.
        // GodWandCommands.resetCooldowns(2): not translated, it uses Minecraft's `CommandSourceStack`.
        // GodWandCommands.change(2): not translated, it uses Minecraft's `CommandSourceStack`.
    }

    /// `GodWandConfig` (net/godwand/config/GodWandConfig.java)
    pub mod god_wand_config {
        use super::*;
        pub static JSON: LazyLock<JGson> = LazyLock::new(|| JGson { pretty: true, ..JGson::default() });
        // CLIENT: not translated, it uses Client, which isn't translated (it uses WandSkin, which isn't translated (its constants carry values)).
        pub static SERVER: LazyLock<JCell<Option<god_wand_config_server::Server>>> = LazyLock::new(|| JCell::new(Some(god_wand_config_server::Server::new())));
        #[derive(Clone, Debug)]
        pub struct GodWandConfig {
        }
        impl GodWandConfig {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // GodWandConfig.loadClient(1): not translated, it uses `Class`.
        // GodWandConfig.loadServer(1): not translated, it uses `Class`.
        // GodWandConfig.saveClient(0): not translated, it uses IPlatformHelper, which isn't translated (it's an interface).
        // GodWandConfig.saveClient(1): not translated, it uses `T`.
        // GodWandConfig.saveServer(0): not translated, it uses IPlatformHelper, which isn't translated (it's an interface).
        // GodWandConfig.saveServer(1): not translated, it uses `T`.
        // GodWandConfig.upgradeCooldownDefaults(1): not translated, it uses `Object`.
        // GodWandConfig.read(3): not translated, it's generic.
        // GodWandConfig.write(2): not translated, it's generic.
        pub fn validated_cooldowns(values: Option<Vec<i32>>, defaults: &[i32]) -> Vec<i32> {
            let mut result: Vec<i32> = defaults.to_vec();
            if values.is_some() {
                {
                    let mut i: i32 = 0;
                    while i < (((values.as_ref().expect("null")).len() as i32).min((result.len() as i32))) {
                        result[i as usize] = j_clamp((values.as_ref().expect("null"))[i as usize], 0, 72000);
                        i += 1;
                    }
                }
            }
            return result.to_vec();
        }
        pub fn validated_damage(values: Option<Vec<f32>>, count: i32) -> Vec<f32> {
            let mut result: Vec<f32> = vec![0.0f32; count as usize];
            result.fill(1.0f32);
            if values.is_some() {
                {
                    let mut i: i32 = 0;
                    while i < (((values.as_ref().expect("null")).len() as i32).min(count)) {
                        result[i as usize] = god_wand_config::finite_clamp((values.as_ref().expect("null"))[i as usize], 0.0f32, 1000.0f32, 1.0f32);
                        i += 1;
                    }
                }
            }
            return result.to_vec();
        }
        pub fn tier_index(level: i32, length: i32) -> i32 {
            return j_clamp(level - 1, 0, length - 1);
        }
        // GodWandConfig.skillCooldownTicks(2): not translated, a nullable object kept in a changing static.
        // GodWandConfig.skillDamageMultiplier(2): not translated, a nullable object kept in a changing static.
        // GodWandConfig.cooldownTicks(1): not translated, a nullable object kept in a changing static.
        pub fn finite_clamp(value: f32, min: f32, max: f32, fallback: f32) -> f32 {
            return if value.is_finite() { j_clamp(value, min, max) } else { fallback };
        }
    }

    /// `GodWandConfig.Client` (net/godwand/config/GodWandConfig.java)
    pub mod god_wand_config_client {
        use super::*;
        // The class itself isn't translated: it uses WandSkin, which isn't translated (its constants carry values). Its static parts are below.
        // GodWandConfig.Client.getActiveSkin(1): not translated, its class isn't translated (it uses WandSkin, which isn't translated (its constants carry values)).
        // GodWandConfig.Client.validate(0): not translated, its class isn't translated (it uses WandSkin, which isn't translated (its constants carry values)).
    }

    /// `GodWandConfig.Server` (net/godwand/config/GodWandConfig.java)
    pub mod god_wand_config_server {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Server {
            pub cooldown_defaults_version: i32,
            pub cooldown_multiplier: f32,
            pub damage_multiplier: f32,
            pub spatial_path_cooldown_ticks: i32,
            pub singularity_cooldown_ticks: Vec<i32>,
            pub earth_drill_cooldown_ticks: Vec<i32>,
            pub stone_cannon_cooldown_ticks: Vec<i32>,
            pub supernova_cooldown_ticks: i32,
            pub atomic_cooldown_ticks: i32,
            pub fuga_cooldown_ticks: i32,
            pub singularity_damage_multipliers: Vec<f32>,
            pub earth_drill_damage_multipliers: Vec<f32>,
            pub stone_cannon_damage_multipliers: Vec<f32>,
            pub supernova_damage_multiplier: f32,
            pub supernova_instant_kill_health_threshold: f32,
            pub supernova_terrain_damage: bool,
            pub supernova_block_lift: bool,
            pub earth_drill_terrain_damage: bool,
            pub earth_drill_block_lift: bool,
            pub stone_cannon_terrain_damage: bool,
            pub supernova_terrain_radius: i32,
            pub supernova_max_displays: i32,
            pub atomic_terrain_radius: i32,
            pub atomic_damage: f32,
            pub atomic_shockwave_damage: f32,
            pub atomic_terrain_damage: bool,
            pub fuga_terrain_damage: bool,
            pub fuga_terrain_radius: i32,
            pub fuga_damage: f32,
            pub dimension_blacklist: Option<Vec<String>>,
        }
        impl Server {
            pub fn new() -> Self {
                let mut s = Self { cooldown_defaults_version: 1, cooldown_multiplier: 1.0f32, damage_multiplier: 1.0f32, spatial_path_cooldown_ticks: 20, singularity_cooldown_ticks: vec![60, 120, 200], earth_drill_cooldown_ticks: vec![80, 140, 220, 320], stone_cannon_cooldown_ticks: vec![40, 80, 140, 220], supernova_cooldown_ticks: 900, atomic_cooldown_ticks: 1200, fuga_cooldown_ticks: 1200, singularity_damage_multipliers: vec![1.0f32, 1.0f32, 1.0f32], earth_drill_damage_multipliers: vec![1.0f32, 1.0f32, 1.0f32, 1.0f32], stone_cannon_damage_multipliers: vec![1.0f32, 1.0f32, 1.0f32, 1.0f32], supernova_damage_multiplier: 1.0f32, supernova_instant_kill_health_threshold: 50.0f32, supernova_terrain_damage: true, supernova_block_lift: true, earth_drill_terrain_damage: true, earth_drill_block_lift: true, stone_cannon_terrain_damage: true, supernova_terrain_radius: 120, supernova_max_displays: 1800, atomic_terrain_radius: 98, atomic_damage: 9999.0f32, atomic_shockwave_damage: 15.0f32, atomic_terrain_damage: true, fuga_terrain_damage: true, fuga_terrain_radius: 48, fuga_damage: 9999.0f32, dimension_blacklist: Some(Vec::new()) };
                s
            }
            pub fn is_dimension_blacklisted(&self, dimension_id: &str) -> bool {
                return !(self.is_dimension_allowed(&dimension_id));
            }
            pub fn is_dimension_allowed(&self, dimension_id: &str) -> bool {
                return self.dimension_blacklist.is_none() || (!((self.dimension_blacklist.as_ref().expect("null")).iter().any(|e| e == &dimension_id)));
            }
            pub fn validate(&mut self) {
                self.spatial_path_cooldown_ticks = j_clamp(self.spatial_path_cooldown_ticks, 0, 72000);
                self.supernova_cooldown_ticks = j_clamp(self.supernova_cooldown_ticks, 0, 72000);
                self.atomic_cooldown_ticks = j_clamp(self.atomic_cooldown_ticks, 0, 72000);
                self.fuga_cooldown_ticks = j_clamp(self.fuga_cooldown_ticks, 0, 72000);
                self.fuga_terrain_radius = j_clamp(self.fuga_terrain_radius, 0, 64);
                self.fuga_damage = god_wand_config::finite_clamp(self.fuga_damage, 0.0f32, 1000000.0f32, 9999.0f32);
                self.singularity_cooldown_ticks = god_wand_config::validated_cooldowns(Some(self.singularity_cooldown_ticks.to_vec()), &(vec![60, 120, 200]));
                self.earth_drill_cooldown_ticks = god_wand_config::validated_cooldowns(Some(self.earth_drill_cooldown_ticks.to_vec()), &(vec![80, 140, 220, 320]));
                self.stone_cannon_cooldown_ticks = god_wand_config::validated_cooldowns(Some(self.stone_cannon_cooldown_ticks.to_vec()), &(vec![40, 80, 140, 220]));
                self.singularity_damage_multipliers = god_wand_config::validated_damage(Some(self.singularity_damage_multipliers.to_vec()), 3);
                self.earth_drill_damage_multipliers = god_wand_config::validated_damage(Some(self.earth_drill_damage_multipliers.to_vec()), 4);
                self.stone_cannon_damage_multipliers = god_wand_config::validated_damage(Some(self.stone_cannon_damage_multipliers.to_vec()), 4);
                self.supernova_damage_multiplier = god_wand_config::finite_clamp(self.supernova_damage_multiplier, 0.0f32, 1000.0f32, 1.0f32);
                self.supernova_instant_kill_health_threshold = god_wand_config::finite_clamp(self.supernova_instant_kill_health_threshold, 0.0f32, 1000000.0f32, 50.0f32);
                self.cooldown_multiplier = god_wand_config::finite_clamp(self.cooldown_multiplier, 0.1f32, 10.0f32, 1.0f32);
                self.damage_multiplier = god_wand_config::finite_clamp(self.damage_multiplier, 0.0f32, 100.0f32, 1.0f32);
                self.supernova_terrain_radius = j_clamp(self.supernova_terrain_radius, 16, 180);
                self.supernova_max_displays = j_clamp(self.supernova_max_displays, 0, 1800);
                self.atomic_terrain_radius = j_clamp(self.atomic_terrain_radius, 0, 98);
                self.atomic_damage = god_wand_config::finite_clamp(self.atomic_damage, 0.0f32, 1000000.0f32, 9999.0f32);
                self.atomic_shockwave_damage = god_wand_config::finite_clamp(self.atomic_shockwave_damage, 0.0f32, 1000000.0f32, 15.0f32);
                if self.dimension_blacklist.is_none() {
                    self.dimension_blacklist = Some(Vec::new());
                }
            }
        }
    }

    /// `WandSkin` (net/godwand/config/WandSkin.java)
    pub mod wand_skin {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // WandSkin.WandSkin(2): not translated, its class isn't translated (its constants carry values).
        // WandSkin.getId(0): not translated, its class isn't translated (its constants carry values).
        // WandSkin.getDisplayName(0): not translated, its class isn't translated (its constants carry values).
        // WandSkin.fromId(1): not translated, it uses WandSkin, which isn't translated (its constants carry values).
    }

    /// `ArcaneOrbEntity` (net/godwand/entity/ArcaneOrbEntity.java)
    pub mod arcane_orb_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Projectile`. Its static parts are below.
        // CHARGE_LEVEL: not translated, it uses Minecraft's `EntityDataAccessor`.
        // ArcaneOrbEntity.ArcaneOrbEntity(2): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
        // ArcaneOrbEntity.ArcaneOrbEntity(3): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
        // ArcaneOrbEntity.setChargeLevel(1): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
        // ArcaneOrbEntity.getChargeLevel(0): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
        // ArcaneOrbEntity.defineSynchedData(1): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
        // ArcaneOrbEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
        // ArcaneOrbEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
        // ArcaneOrbEntity.tick(0): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
        // ArcaneOrbEntity.canHitEntity(1): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
        // ArcaneOrbEntity.onHit(1): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
    }

    /// `StoneCannonProjectileEntity` (net/godwand/entity/StoneCannonProjectileEntity.java)
    pub mod stone_cannon_projectile_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Projectile`. Its static parts are below.
        // CHARGE_LEVEL: not translated, it uses Minecraft's `EntityDataAccessor`.
        // StoneCannonProjectileEntity.markHit(1): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
        // StoneCannonProjectileEntity.StoneCannonProjectileEntity(2): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
        // StoneCannonProjectileEntity.StoneCannonProjectileEntity(3): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
        // StoneCannonProjectileEntity.setChargeLevel(1): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
        // StoneCannonProjectileEntity.getChargeLevel(0): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
        // StoneCannonProjectileEntity.defineSynchedData(1): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
        // StoneCannonProjectileEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
        // StoneCannonProjectileEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
        // StoneCannonProjectileEntity.tick(0): not translated, its class isn't translated (it extends Minecraft's `Projectile`).
    }

    /// `GodWandFabricClientMod` (net/godwand/fabric/client/GodWandFabricClientMod.java)
    pub mod god_wand_fabric_client_mod {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        // GodWandFabricClientMod.onInitializeClient(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `GodWandModMenuApi` (net/godwand/fabric/client/GodWandModMenuApi.java)
    pub mod god_wand_mod_menu_api {
        use super::*;
        // The class itself isn't translated: it implements `ModMenuApi` (com.terraformersmc.modmenu.api). Its static parts are below.
        // GodWandModMenuApi.getModConfigScreenFactory(0): not translated, its class isn't translated (it implements `ModMenuApi` (com.terraformersmc.modmenu.api)).
    }

    /// `GodWandFabricMod` (net/godwand/fabric/GodWandFabricMod.java)
    pub mod god_wand_fabric_mod {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ModInitializer`. Its static parts are below.
        // GodWandFabricMod.onInitialize(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
    }

    /// `FabricNetworkHelper` (net/godwand/fabric/platform/FabricNetworkHelper.java)
    pub mod fabric_network_helper {
        use super::*;
        // The class itself isn't translated: it implements `INetworkHelper` (net.godwand.platform). Its static parts are below.
        // FabricNetworkHelper.sendToPlayer(2): not translated, its class isn't translated (it implements `INetworkHelper` (net.godwand.platform)).
        // FabricNetworkHelper.sendToNearby(4): not translated, its class isn't translated (it implements `INetworkHelper` (net.godwand.platform)).
        // FabricNetworkHelper.sendToServer(1): not translated, its class isn't translated (it implements `INetworkHelper` (net.godwand.platform)).
    }

    /// `FabricPlatformHelper` (net/godwand/fabric/platform/FabricPlatformHelper.java)
    pub mod fabric_platform_helper {
        use super::*;
        // The class itself isn't translated: it implements `IPlatformHelper` (net.godwand.platform). Its static parts are below.
        // FabricPlatformHelper.getPlatformName(0): not translated, its class isn't translated (it implements `IPlatformHelper` (net.godwand.platform)).
        // FabricPlatformHelper.isModLoaded(1): not translated, its class isn't translated (it implements `IPlatformHelper` (net.godwand.platform)).
        // FabricPlatformHelper.isDevelopmentEnvironment(0): not translated, its class isn't translated (it implements `IPlatformHelper` (net.godwand.platform)).
        // FabricPlatformHelper.getConfigDirectory(0): not translated, its class isn't translated (it implements `IPlatformHelper` (net.godwand.platform)).
    }

    /// `GodWandMod` (net/godwand/GodWandMod.java)
    pub mod god_wand_mod {
        use super::*;
        pub const MOD_ID: &str = "godwand";
        // LOGGER: the logger. Log calls become comments.
        // ARCANE_ORB_ENTITY_TYPE: not translated, it uses Minecraft's `EntityType`.
        // STONE_CANNON_ENTITY_TYPE: not translated, it uses Minecraft's `EntityType`.
        // GOD_WAND_ITEM: not translated, it uses Minecraft's `Item`.
        // GODWAND_ITEM_GROUP_KEY: not translated, it uses Minecraft's `ResourceKey`.
        // GODWAND_ITEM_GROUP: not translated, it uses Minecraft's `CreativeModeTab`.
        #[derive(Clone, Debug)]
        pub struct GodWandMod {
        }
        impl GodWandMod {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `GodWandItem` (net/godwand/item/GodWandItem.java)
    pub mod god_wand_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        pub const MODE_SPATIAL_PATH: i32 = 0;
        pub const MODE_ARCANE_ORB: i32 = 1;
        pub const MODE_RATIO_COLLAPSE: i32 = 2;
        pub const MODE_SUPERNOVA: i32 = 3;
        pub const MODE_STONE_CANNON: i32 = 4;
        pub const MODE_ATOMIC: i32 = 5;
        pub const MODE_FUGA: i32 = 6;
        pub const TOTAL_MODES: i32 = 7;
        pub fn charge_meter(level: i32, maximum: i32) -> String {
            let filled: i32 = j_clamp(level, 0, maximum);
            return format!("\u{a7}f[\u{a7}b{}\u{a7}8{}\u{a7}f]", "\u{25a0}".repeat(filled as usize), "\u{25a1}".repeat((maximum - filled) as usize));
        }
        // GodWandItem.GodWandItem(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // GodWandItem.createMeleeAttributes(0): not translated, it uses Minecraft's `ItemAttributeModifiers`.
        // GodWandItem.getSkillMode(1): needs the game, so it's in game_side.rs.
        // GodWandItem.setSkillMode(2): not translated, it uses Minecraft's `ItemStack`.
        // GodWandItem.selectSkill(3): not translated, it uses Minecraft's `Player`.
        // GodWandItem.use(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // GodWandItem.getUseDuration(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // GodWandItem.getUseAnimation(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // GodWandItem.onUseTick(4): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // GodWandItem.releaseUsing(4): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // GodWandItem.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `WandSkillLock` (net/godwand/item/WandSkillLock.java)
    pub mod wand_skill_lock {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct WandSkillLock {
        }
        impl WandSkillLock {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // WandSkillLock.isLocked(1): needs the game, so it's in game_side.rs.
        // WandSkillLock.lock(2): not translated, it uses Minecraft's `ItemStack`.
        // WandSkillLock.unlock(1): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `CinematicFxPayload` (net/godwand/network/CinematicFxPayload.java)
    pub mod cinematic_fx_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // ID: not translated, it uses Minecraft's `Type`.
        pub const EVENT_TRAUMA_SHAKE: i32 = 0;
        pub const EVENT_FOV_PUNCH: i32 = 1;
        pub const EVENT_VACUUM_SILENCE: i32 = 2;
        pub const EVENT_METEOR_FOCUS: i32 = 3;
        pub const EVENT_METEOR_SLAM: i32 = 4;
        pub const EVENT_SUPERNOVA_COMPRESS: i32 = 5;
        pub const EVENT_SUPERNOVA_BURST: i32 = 6;
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // CinematicFxPayload.CinematicFxPayload(6): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // CinematicFxPayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // CinematicFxPayload.eventType(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // CinematicFxPayload.x(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // CinematicFxPayload.y(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // CinematicFxPayload.z(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // CinematicFxPayload.intensity(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // CinematicFxPayload.durationTicks(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `ImpactFxPayload` (net/godwand/network/ImpactFxPayload.java)
    pub mod impact_fx_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // ID: not translated, it uses Minecraft's `Type`.
        pub const TYPE_DETONATION: i32 = 0;
        pub const TYPE_IMPLOSION: i32 = 1;
        pub const TYPE_COLLAPSE: i32 = 2;
        pub const TYPE_SUPERNOVA: i32 = 3;
        pub const TYPE_BLACK_HOLE: i32 = 4;
        pub const TYPE_SEISMIC_SLAM: i32 = 5;
        pub const TYPE_GRAVITY_CORE: i32 = 6;
        pub const TYPE_REASSEMBLY: i32 = 7;
        pub const TYPE_SUPERNOVA_IMPACT: i32 = 8;
        pub const SUPERNOVA_IMPACT_TICKS: i32 = 4;
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ImpactFxPayload.ImpactFxPayload(6): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ImpactFxPayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ImpactFxPayload.x(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ImpactFxPayload.y(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ImpactFxPayload.z(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ImpactFxPayload.fxType(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ImpactFxPayload.durationTicks(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ImpactFxPayload.intensity(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `INetworkHelper` (net/godwand/platform/INetworkHelper.java)
    pub mod i_network_helper {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // INetworkHelper.sendToPlayer(2): not translated, it has no body.
        // INetworkHelper.sendToNearby(4): not translated, it has no body.
        // INetworkHelper.sendToServer(1): not translated, it has no body.
    }

    /// `IPlatformHelper` (net/godwand/platform/IPlatformHelper.java)
    pub mod i_platform_helper {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IPlatformHelper.getPlatformName(0): not translated, it has no body.
        // IPlatformHelper.isModLoaded(1): not translated, it has no body.
        // IPlatformHelper.isDevelopmentEnvironment(0): not translated, it has no body.
        // IPlatformHelper.getConfigDirectory(0): not translated, it has no body.
    }

    /// `Services` (net/godwand/platform/Services.java)
    pub mod services {
        use super::*;
        // PLATFORM: not translated, it uses IPlatformHelper, which isn't translated (it's an interface).
        // NETWORK: not translated, it uses INetworkHelper, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct Services {
        }
        impl Services {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Services.load(1): not translated, it's generic.
    }

    /// `AtomicCrater` (net/godwand/skill/AtomicCrater.java)
    pub mod atomic_crater {
        use super::*;
        // The class itself isn't translated: it keeps an atomic value in an object's field. Its static parts are below.
        // AtomicCrater.AtomicCrater(2): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        pub fn terrain_radius(configured_radius: i32) -> i32 {
            return (0i32).max(configured_radius.min(98));
        }
        // AtomicCrater.finished(0): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // AtomicCrater.carve(2): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // AtomicCrater.carve(3): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // AtomicCrater.carve(4): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // AtomicCrater.loadedChunk(4): not translated, it uses Minecraft's `LevelChunk`.
    }

    /// `AtomicManager` (net/godwand/skill/AtomicManager.java)
    pub mod atomic_manager {
        use super::*;
        // TASKS: not translated, it uses `UUID` (java.util).
        #[derive(Clone, Debug)]
        pub struct AtomicManager {
        }
        impl AtomicManager {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // AtomicManager.invulnerable(1): not translated, it uses Minecraft's `ServerPlayer`.
        // AtomicManager.movementLocked(1): not translated, it uses Minecraft's `ServerPlayer`.
        // AtomicManager.releaseMovement(2): not translated, it uses Minecraft's `ServerPlayer`.
        // AtomicManager.beginCharge(2): not translated, it uses Minecraft's `ServerPlayer`.
        // AtomicManager.arm(1): not translated, it uses Minecraft's `ServerPlayer`.
        // AtomicManager.cancelCharge(1): not translated, it uses Minecraft's `ServerPlayer`.
        // AtomicManager.releaseCharge(1): not translated, it uses Minecraft's `ServerPlayer`.
        // AtomicManager.tick(1): not translated, it uses Minecraft's `ServerLevel`.
        // AtomicManager.damage(1): not translated, a nullable object kept in a changing static.
        // AtomicManager.broadcast(2): not translated, it uses Task, which isn't translated (it keeps an atomic value in an object's field).
        // AtomicManager.clearAllTasks(0): not translated, it uses AtomicManager.TASKS, which isn't translated (it uses `UUID` (java.util)).
    }

    /// `AtomicManager.Task` (net/godwand/skill/AtomicManager.java)
    pub mod atomic_manager_task {
        use super::*;
        // The class itself isn't translated: it keeps an atomic value in an object's field. Its static parts are below.
        // AtomicManager.Task.Task(2): not translated, its class isn't translated (it keeps an atomic value in an object's field).
    }

    /// `AtomicSequence` (net/godwand/skill/AtomicSequence.java)
    pub mod atomic_sequence {
        use super::*;
        pub const BEAM_RETRACTION: i32 = 574;
        pub const CHARGE_TICKS: i32 = 380;
        pub const WAIT_TICK: i32 = 406;
        pub const DETONATION: i32 = 414;
        pub const END: i32 = 834;
        pub const MAX_RADIUS: i32 = 98;
        pub const CAGE_RADIUS: i32 = 26;
        pub const ATOMIC_WORD: i32 = 396;
        pub const BEAM_HEIGHT: i32 = 1024;
        pub const BEAM_DEPTH: i32 = 30;
        pub const CRATER_DEPTH: i32 = 16;
        pub const SCAR_FADE_START: i32 = 1434;
        pub const SCAR_END: i32 = 1494;
        #[derive(Clone, Debug)]
        pub struct AtomicSequence {
        }
        impl AtomicSequence {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn phase(age: f64, start: f64, end: f64) -> f32 {
            let t: f32 = ((j_clamp((age - start) / (end - start), 0.0f64, 1.0f64)) as f32);
            return (t * t) * (3.0f32 - (2.0f32 * t));
        }
        pub fn cast_age(now: f64, started: i64, released_at: i64) -> f32 {
            return ((if (released_at >= 0i64) && (now >= (released_at as f64)) { (406.0f64 + now) - (released_at as f64) } else { 406.0f64.min(0.0f64.max(now - (started as f64))) }) as f32);
        }
        pub fn release_time(now: i64, started: i64) -> i64 {
            return now.max((started + 406i64) + 3i64);
        }
        pub fn heartbeat(ticks: f64) -> f32 {
            if ticks < 0.0f64 {
                return 0.0f32;
            }
            let p: f64 = ticks % 28.0f64;
            let first: f64 = p.min(28.0f64 - p);
            let second: f64 = p - 5.0f64;
            return ((((((-first) * first) / 2.0f64).exp()).max(0.6f64 * ((((-second) * second) / 2.0f64).exp()))) as f32);
        }
        pub fn hold_idle(ticks: f64) -> f32 {
            return (((((ticks * 0.075f64).sin()) * 0.018f64) + (((ticks * 0.023f64).sin()) * 0.006f64)) as f32);
        }
        pub fn release_flash(age: f64) -> f32 {
            return atomic_sequence::phase(age, 406.0f64, 408.0f64) * (1.0f32 - atomic_sequence::phase(age, 408.0f64, 410.0f64));
        }
        pub fn charge(age: f64) -> f32 {
            return atomic_sequence::phase(age, 0.0f64, 380.0f64);
        }
        pub fn visibility(age: f64) -> f32 {
            return if age < 0.0f64 { 0.0f32 } else { 1.0f32 - atomic_sequence::phase(age, 814.0f64, 834.0f64) };
        }
        pub fn compression(age: f64) -> f32 {
            let remaining: f64 = 1.0f64 - (j_clamp((age - 396.0f64) / 10.0f64, 0.0f64, 1.0f64));
            return ((1.0f64 - ((remaining * remaining) * remaining)) as f32);
        }
        pub fn blackout(age: f64) -> f32 {
            return (0.64f32 * atomic_sequence::phase(age, 400.0f64, 406.0f64)) * (1.0f32 - atomic_sequence::phase(age, 406.0f64, 408.0f64));
        }
        pub fn circuit_progress(age: f64, seed: f64) -> f32 {
            return atomic_sequence::phase(age, 115.0f64 + (seed * 12.0f64), 180.0f64);
        }
        pub fn junction_wave_radius(age: f64) -> f32 {
            return if (!(age <= 180.0f64)) && (!(age >= 203.0f64)) { ((age - 180.0f64) as f32) * 1.55f32 } else { 0.0f32 };
        }
        pub fn arm_raise(age: f64) -> f32 {
            return atomic_sequence::phase(age, 294.0f64, 344.0f64);
        }
        pub fn arm_spread(age: f64) -> f32 {
            return atomic_sequence::phase(age, 396.0f64, 402.0f64);
        }
        pub fn arm_windup(age: f64) -> f32 {
            return atomic_sequence::phase(age, 382.0f64, 394.0f64) * (1.0f32 - atomic_sequence::arm_spread(age));
        }
        pub fn facing(age: f64) -> f32 {
            return atomic_sequence::phase(age, 374.0f64, 394.0f64) * (1.0f32 - atomic_sequence::phase(age, 774.0f64, 802.0f64));
        }
        // AtomicSequence.blendHeadYaw(3): not translated, it calls Minecraft's `Mth`.rotLerp.
        // AtomicSequence.headLook(2): not translated, it uses Minecraft's `Vec3`.
        // AtomicSequence.wideFocus(2): not translated, it uses Minecraft's `Vec3`.
        pub fn transition_opacity(age: f64) -> f32 {
            let mut opacity: f32 = 0.0f32;
            for &cut in (vec![115, 205, 285]).iter() {
                opacity = opacity.max(0.48f32 * (1.0f32 - (atomic_sequence::phase((age - (cut as f64)).abs(), 0.0f64, 6.0f64))));
            }
            return opacity;
        }
        pub fn arm_pitch(age: f64) -> f32 {
            let start: f32 = (-0.12f32) - (1.3f32 * atomic_sequence::activation(age));
            let mut up: f32 = start + (((-2.2f32) - start) * atomic_sequence::arm_raise(age));
            up += ((-1.15f32) - up) * atomic_sequence::arm_windup(age);
            return up + (((-0.12f32) - up) * atomic_sequence::arm_spread(age));
        }
        pub fn holo_radius(age: f64) -> f32 {
            return 0.25f32 + (2.1f32 * atomic_sequence::phase(age, 290.0f64, 370.0f64));
        }
        pub fn shock_radius(age: f64) -> f32 {
            return if age < 414.0f64 { 0.0f32 } else { 98.0f32 * atomic_sequence::phase(age, 414.0f64, 534.0f64) };
        }
        pub fn beam_radius(age: f64) -> f32 {
            return atomic_sequence::shock_radius(age) * (1.0f32 - atomic_sequence::phase(age, 574.0f64, 734.0f64));
        }
        pub fn beam_height(age: f64) -> f32 {
            return if age < 414.0f64 { 0.0f32 } else { 1024.0f32 * atomic_sequence::phase(age, 414.0f64, 430.0f64) };
        }
        pub fn blast_pulse(age: f64) -> f32 {
            return 1.0f32;
        }
        pub fn scar_opacity(age: f64) -> f32 {
            return (atomic_sequence::phase(age, 644.0f64, 734.0f64) * (0.42f32 + (0.33f32 * (1.0f32 - atomic_sequence::phase(age, 814.0f64, 834.0f64))))) * (1.0f32 - atomic_sequence::phase(age, 1434.0f64, 1494.0f64));
        }
        pub fn protected_during_cast(age: f64) -> bool {
            return (age >= 0.0f64) && (age < 834.0f64);
        }
        // AtomicSequence.columnOccludes(3): not translated, it uses Minecraft's `Vec3`.
        pub fn letterbox(age: f64) -> f32 {
            return if (age >= 396.0f64) && (age < 414.0f64) { 1.0f32 } else { 0.0f32 };
        }
        pub fn return_blend(age: f64) -> f32 {
            return atomic_sequence::phase(age, 802.0f64, 826.0f64);
        }
        pub fn movement_locked(age: f64) -> bool {
            return (age >= 0.0f64) && (age < 826.0f64);
        }
        pub fn activation(age: f64) -> f32 {
            return atomic_sequence::phase(age, 0.0f64, 8.0f64) * (1.0f32 - atomic_sequence::phase(age, 22.0f64, 40.0f64));
        }
        pub fn impact(age: f64) -> bool {
            return (((age >= 10.0f64) && (age < 14.0f64)) || ((age >= 396.0f64) && (age < 400.0f64))) || ((age >= 410.0f64) && (age < 412.0f64));
        }
        // AtomicSequence.groundTracePoint(2): not translated, it uses Minecraft's `Vec3`.
        // AtomicSequence.polar(3): not translated, it uses Minecraft's `Vec3`.
        // AtomicSequence.convergencePoint(4): not translated, it uses Minecraft's `Vec3`.
        pub fn mote_noise(seed: f64, salt: f64) -> f64 {
            let value: f64 = (((seed * 1437.7f64) + (salt * 311.7f64)).sin()) * 43758.5453f64;
            return value - value.floor();
        }
        // AtomicSequence.risingMote(2): not translated, it uses Minecraft's `Vec3`.
        pub fn mote_clock(age: f64) -> f64 {
            return age.min(390.0f64);
        }
        // AtomicSequence.releaseMote(2): not translated, it uses Minecraft's `Vec3`.
        pub fn fortress_wave_height(age: f64) -> f32 {
            if (!(age < 170.0f64)) && (!(age >= 350.0f64)) {
                let cycle: f64 = (age - 170.0f64) % 60.0f64;
                return if (!(cycle < 8.0f64)) && (!(cycle >= 50.0f64)) { 54.0f32 * atomic_sequence::phase(cycle, 8.0f64, 50.0f64) } else { -1.0f32 };
            } else {
                return -1.0f32;
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        pub fn release_radius(age: f64) -> f32 {
            return if (!(age < 396.0f64)) && (!(age >= 414.0f64)) { 1.0f32 + (34.0f32 * atomic_sequence::phase(age, 396.0f64, 404.0f64)) } else { 0.0f32 };
        }
        pub fn camera_budget_1(chunks: i32) -> f32 {
            return ((j_clamp((((2i32).max(chunks)) - 1) * 16, 8, 112)) as f32);
        }
        pub fn camera_budget_2(chunks: i32, age: f64) -> f32 {
            return atomic_sequence::camera_budget_1(chunks) + ((if (age >= 466.0f64) && (age < 614.0f64) { 16 } else { 0 }) as f32);
        }
        pub fn pressure(age: f64) -> f32 {
            return atomic_sequence::phase(age, 422.0f64, 442.0f64) * (1.0f32 - atomic_sequence::phase(age, 614.0f64, 644.0f64));
        }
        pub fn pressure_wave_radius(age: f64, wave: i32) -> f32 {
            if atomic_sequence::pressure(age) <= 0.0f32 {
                return 0.0f32;
            }
            let cycle: f64 = ((age - 414.0f64) / 30.0f64) + ((wave as f64) * 0.25f64);
            return atomic_sequence::beam_radius(age) + (((cycle - cycle.floor()) as f32) * 32.0f32);
        }
        pub fn pressure_crosses(age: f64, wave: i32, distance: f64) -> bool {
            if (((!(distance <= 98.0f64)) && (!(distance > 130.0f64))) && (!(atomic_sequence::pressure(age) <= 0.15f32))) && (!(atomic_sequence::pressure(age - 1.0f64) <= 0.15f32)) {
                let before: f32 = atomic_sequence::pressure_wave_radius(age - 1.0f64, wave);
                let now: f32 = atomic_sequence::pressure_wave_radius(age, wave);
                return ((now >= before) && (distance >= ((before as f64) - 1.25f64))) && (distance <= ((now as f64) + 1.25f64));
            } else {
                return false;
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        pub fn pressure_shake(age: f64, camera_radius: f64) -> f32 {
            let mut impulse: f32 = 0.0f32;
            for wave in 0..4 {
                let radius: f32 = atomic_sequence::pressure_wave_radius(age + 2.0f64, wave);
                if radius > 0.0f32 {
                    impulse += (((-((((radius as f64) - camera_radius) / 2.2f64).powf(2.0f64))).exp()) as f32) * 0.32f32;
                }
            }
            return impulse * atomic_sequence::pressure(age);
        }
        pub fn shake(age: f64) -> f32 {
            if !(age < 414.0f64) {
                let t: f64 = age - 414.0f64;
                return (((((1.3f64 * (((-t) / 18.0f64).exp())) + (0.34f64 * ((-(((t - 26.0f64) / 7.0f64).powf(2.0f64))).exp()))) + (0.2f64 * ((-(((t - 120.0f64) / 12.0f64).powf(2.0f64))).exp()))) + (0.12f64 * (atomic_sequence::pressure(age) as f64))) as f32) * (1.0f32 - atomic_sequence::return_blend(age));
            } else {
                let activation: f32 = if (!(age < 10.0f64)) && (!(age >= 28.0f64)) { ((0.32f64 * (((-(age - 10.0f64)) / 4.0f64).exp())) as f32) } else { 0.0f32 };
                let release: f32 = if (!(age < 396.0f64)) && (!(age >= 408.0f64)) { ((0.22f64 * (((-(age - 396.0f64)) / 3.0f64).exp())) as f32) } else { 0.0f32 };
                let gathering: f32 = (0.1f32 * atomic_sequence::phase(age, 294.0f64, 310.0f64)) * (1.0f32 - atomic_sequence::phase(age, 378.0f64, 384.0f64));
                return ((activation + release) + gathering) + (0.55f32 * atomic_sequence::phase(age, 412.0f64, 414.0f64));
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        // AtomicSequence.shot(2): not translated, it uses Minecraft's `Vec3`.
    }

    /// `AtomicSequence.Shot` (net/godwand/skill/AtomicSequence.java)
    pub mod atomic_sequence_shot {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // AtomicSequence.Shot.Shot(3): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AtomicSequence.Shot.offset(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AtomicSequence.Shot.focus(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AtomicSequence.Shot.fov(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `AtomicTerrainPlanner` (net/godwand/skill/AtomicTerrainPlanner.java)
    pub mod atomic_terrain_planner {
        use super::*;
        // WORKER: not translated, it uses `ExecutorService` (java.util.concurrent).
        #[derive(Clone, Debug)]
        pub struct AtomicTerrainPlanner {
        }
        impl AtomicTerrainPlanner {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // AtomicTerrainPlanner.request(1): not translated, it calls AtomicTerrainPlanner.request, which isn't translated.
        // AtomicTerrainPlanner.request(2): not translated, a try/catch (RejectedExecutionException).
        // AtomicTerrainPlanner.plan(1): not translated, it calls AtomicTerrainPlanner.plan, which isn't translated.
        // AtomicTerrainPlanner.plan(2): not translated, it uses `IntArrayList` (it.unimi.dsi.fastutil.ints).
    }

    /// `AtomicTerrainPlanner.Columns` (net/godwand/skill/AtomicTerrainPlanner.java)
    pub mod atomic_terrain_planner_columns {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Columns {
            pub offsets: Vec<i32>,
            pub distances: Vec<i32>,
        }
        impl Columns {
            pub fn new(offsets: &[i32], distances: &[i32]) -> Self {
                let mut s = Self { offsets: Vec::new(), distances: Vec::new() };
                s.offsets = offsets.to_vec();
                s.distances = distances.to_vec();
                s
            }
            pub fn offsets(&self) -> Vec<i32> {
                return self.offsets.to_vec();
            }
            pub fn distances(&self) -> Vec<i32> {
                return self.distances.to_vec();
            }
        }
    }

    /// `FugaCrater` (net/godwand/skill/FugaCrater.java)
    pub mod fuga_crater {
        use super::*;
        // The class itself isn't translated: it keeps an atomic value in an object's field. Its static parts are below.
        pub const SPIRES: i32 = 16;
        pub const PRESS_SINK: i32 = 4;
        pub const PRESS_LIP: i32 = 3;
        // FugaCrater.FugaCrater(2): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // FugaCrater.finished(0): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        pub fn depth(radius: i32, dx: i32, dz: i32) -> i32 {
            let angle: f64 = (dz as f64).atan2((dx as f64));
            let rim: f64 = (radius as f64) * (((0.9f64 + (0.05f64 * (((angle * 5.0f64) + 1.3f64).sin()))) + (0.035f64 * (((angle * 11.0f64) + 0.4f64).sin()))) + (0.015f64 * ((angle * 23.0f64).sin())));
            let r: f64 = ((((dx * dx) + (dz * dz)) as f64).sqrt()) / rim;
            if r > 1.0f64 {
                return 0;
            }
            let grain: f64 = ((((dx * 73856093) ^ (dz * 19349663)) & 0xFF) as f64) / 255.0f64;
            return 1 + (((((radius as f64) * 0.6f64) * ((1.0f64 - (r * r)).sqrt())) * (0.86f64 + (0.14f64 * grain))) as i32);
        }
        pub fn reach(radius: i32) -> i32 {
            return (98i32).min((radius * 3) / 2);
        }
        pub fn blast(radius: i32, dx: i32, dz: i32) -> bool {
            return (fuga_crater::depth(radius, dx, dz) == 0) && (((dx * dx) + (dz * dz)) <= (fuga_crater::reach(radius) * fuga_crater::reach(radius)));
        }
        pub fn scour(radius: i32, dx: i32, dz: i32) -> i32 {
            let out: f64 = (((((dx * dx) + (dz * dz)) as f64).sqrt()) - ((radius as f64) * 0.9f64)) / (1.0f64.max((fuga_crater::reach(radius) as f64) - ((radius as f64) * 0.9f64)));
            let grain: f64 = ((((dx * 19349663) ^ (dz * 83492791)) & 0xFF) as f64) / 255.0f64;
            return if grain < 0.2f64 { 0 } else { ((j_round_d(((0.0f64.max(1.0f64 - out)) * 4.0f64) * grain)) as i32) };
        }
        // FugaCrater.carve(4): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        pub fn spire(radius: i32, i: i32) -> Vec<i32> {
            let mut h: i32 = (i * 374761393) + (radius * 668265263);
            h = (h ^ (((h as u32) >> 13) as i32)) * 1274126177;
            h ^= ((h as u32) >> 16) as i32;
            let a: f64 = ((((h & 1023) as f64) / 1023.0f64) * std::f64::consts::PI) * 2.0f64;
            let r: f64 = (radius as f64) * (0.25f64 + ((0.5f64 * (((((h as u32) >> 10) as i32) & 0xFF) as f64)) / 255.0f64));
            return vec![((j_round_d(a.cos() * r)) as i32), ((j_round_d(a.sin() * r)) as i32), 6 + ((((((h as u32) >> 18) as i32) & 15) * 2) / 3), 1 + ((((h as u32) >> 22) as i32) & 1), 12 + ((((h as u32) >> 23) as i32) & 31)];
        }
        // FugaCrater.upheave(3): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        pub fn press_shape(progress: f64, contact_radius: f64, r: f64) -> i32 {
            if (progress <= 0.0f64) || (contact_radius <= 0.0f64) {
                return 0;
            }
            if r < contact_radius {
                return ((j_round_d((4.0f64 * progress) * (1.0f64 - ((r * r) / (contact_radius * contact_radius))))) as i32);
            }
            let lip: f64 = (r - contact_radius) / 3.0f64;
            return if lip < 1.0f64 { -((j_round_d((2.0f64 * progress) * (1.0f64 - lip))) as i32) } else { 0 };
        }
        // FugaCrater.press(3): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // FugaCrater.scorch(5): not translated, its class isn't translated (it keeps an atomic value in an object's field).
    }

    /// `FugaManager` (net/godwand/skill/FugaManager.java)
    pub mod fuga_manager {
        use super::*;
        // TASKS: not translated, it uses `UUID` (java.util).
        // SETTLING: not translated, it uses Settling, which isn't translated (it uses Minecraft's `ServerLevel`).
        pub const FORCE_PER_TICK: i32 = 2;
        #[derive(Clone, Debug)]
        pub struct FugaManager {
        }
        impl FugaManager {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FugaManager.beginCharge(2): not translated, it uses Minecraft's `ServerPlayer`.
        // FugaManager.movementLocked(1): not translated, it uses Minecraft's `ServerPlayer`.
        // FugaManager.invulnerable(1): not translated, it uses Minecraft's `ServerPlayer`.
        // FugaManager.cancelCharge(1): not translated, it uses Minecraft's `ServerPlayer`.
        // FugaManager.releaseCharge(1): not translated, it uses Minecraft's `ServerPlayer`.
        // FugaManager.invalid(1): not translated, it uses Minecraft's `ServerPlayer`.
        // FugaManager.tick(1): not translated, it uses Minecraft's `ServerLevel`.
        // FugaManager.settle(3): not translated, it uses Minecraft's `ServerLevel`.
        // FugaManager.advance(1): not translated, it uses Minecraft's `Vec3`.
        // FugaManager.loadedEndpoint(3): not translated, it uses Minecraft's `Vec3`.
        // FugaManager.loaded(2): not translated, it uses Minecraft's `ServerLevel`.
        // FugaManager.damage(2): not translated, it uses Minecraft's `Vec3`.
        // FugaManager.slow(1): not translated, it uses Minecraft's `Vec3`.
        // FugaManager.hit(3): not translated, it uses Minecraft's `Entity`.
        // FugaManager.broadcast(2): not translated, it uses Minecraft's `Vec3`.
        // FugaManager.clearAllTasks(0): not translated, it uses FugaManager.TASKS, which isn't translated (it uses `UUID` (java.util)).
        // FugaManager.forceOrder(2): not translated, it uses Minecraft's `BlockPos`.
        // FugaManager.forceSome(4): not translated, it uses Minecraft's `ServerLevel`.
        // FugaManager.release(2): not translated, it uses Minecraft's `ServerLevel`.
        // FugaManager.outflow(2): not translated, it uses Minecraft's `Vec3`.
    }

    /// `FugaManager.Settling` (net/godwand/skill/FugaManager.java)
    pub mod fuga_manager_settling {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ServerLevel`. Its static parts are below.
        // FugaManager.Settling.Settling(4): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // FugaManager.Settling.world(0): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // FugaManager.Settling.crater(0): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // FugaManager.Settling.impactAt(0): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // FugaManager.Settling.forced(0): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
    }

    /// `FugaManager.Task` (net/godwand/skill/FugaManager.java)
    pub mod fuga_manager_task {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ServerPlayer`. Its static parts are below.
        // FugaManager.Task.Task(2): not translated, its class isn't translated (it uses Minecraft's `ServerPlayer`).
    }

    /// `FugaSequence` (net/godwand/skill/FugaSequence.java)
    pub mod fuga_sequence {
        use super::*;
        pub const CHARGE_TICKS: i32 = 200;
        pub const ORB_TICKS: i32 = 80;
        pub const RETRACT_TICK: i32 = 225;
        pub const END_TICK: i32 = 305;
        pub const COLUMN_HEIGHT: i32 = 384;
        pub const SETTLE_TICKS: i32 = 240;
        pub const SPEED: f64 = 4.0f64;
        pub const MAX_RANGE: f64 = 160.0f64;
        pub const RADIUS: f64 = 48.0f64;
        pub const TRANSITION_TICKS: i32 = 14;
        pub const FLIGHT_EASE: f64 = 6.0f64;
        #[derive(Clone, Debug)]
        pub struct FugaSequence {
        }
        impl FugaSequence {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn charge_age(age: f64) -> f64 {
            return (age * 120.0f64) / 200.0f64;
        }
        pub fn visual_age(age: f64) -> f64 {
            return if age < 80.0f64 { age * 0.5f64 } else { if age < 105.0f64 { 40.0f64 + ((age - 80.0f64) * 0.8f64) } else { 60.0f64 + (((age - 80.0f64) - 25.0f64) * 0.5f64) } };
        }
        pub fn movement_locked(phase: i32) -> bool {
            return phase == 0;
        }
        pub fn ready(now: i64, started: i64) -> bool {
            return (now >= started) && ((now - started) >= 200i64);
        }
        pub fn impact_age(now: f64, phase_at: i64) -> f64 {
            return 0.0f64.max(now - (phase_at as f64));
        }
        pub fn terrain_active(age: f64) -> bool {
            return (age >= 72.0f64) && (age < 545.0f64);
        }
        pub fn column_radius(age: f64) -> f32 {
            return if (!(age < 80.0f64)) && (!(age >= 305.0f64)) { ((48.0f64 * (if age <= 225.0f64 { 1.0f64 } else { (305.0f64 - age) / 80.0f64 })) as f32) } else { 0.0f32 };
        }
        // FugaSequence.launchTarget(2): not translated, it uses Minecraft's `Vec3`.
        pub fn flight_distance(age: f64) -> f64 {
            return if !(age > 0.0f64) { 0.0f64 } else { (4.0f64 * age) - (18.0f64 * (1.0f64 - (((-age) / 6.0f64).exp()))) };
        }
        // FugaSequence.arrowPosition(3): not translated, it uses Minecraft's `Vec3`.
    }

    /// `RatioCollapseManager` (net/godwand/skill/RatioCollapseManager.java)
    pub mod ratio_collapse_manager {
        use super::*;
        pub const ASSEMBLY_END: i32 = 88;
        pub const SPIN_END: i32 = 158;
        pub const IMPACT_TICK: i32 = 167;
        pub const SEQUENCE_END: i32 = 176;
        // ACTIVE_TASKS: not translated, it uses `Queue` (java.util).
        #[derive(Clone, Debug)]
        pub struct RatioCollapseManager {
        }
        impl RatioCollapseManager {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RatioCollapseManager.cast(1): not translated, it uses Minecraft's `ServerPlayer`.
        // RatioCollapseManager.cast(2): not translated, it uses Minecraft's `ServerPlayer`.
        // RatioCollapseManager.launch(4): not translated, it uses Minecraft's `ServerLevel`.
        // RatioCollapseManager.launch(9): not translated, it uses Minecraft's `ServerLevel`.
        // RatioCollapseManager.launch(10): not translated, it uses Minecraft's `ServerLevel`.
        // RatioCollapseManager.computeBoulderClusterOffset(3): not translated, it uses Minecraft's `Vec3`.
        // RatioCollapseManager.computeSpiralDrillOffset(4): not translated, it uses Minecraft's `Vec3`.
        // RatioCollapseManager.tick(1): not translated, it uses Minecraft's `ServerLevel`.
        // RatioCollapseManager.carveCrater(1): not translated, it uses Minecraft's `ServerLevel`.
        pub fn drill_crater_depth(x: i32, z: i32, radius: i32, max_depth: i32) -> i32 {
            let distance: f64 = (x as f64).hypot((z as f64));
            let edge: f64 = 0.0f64.max(1.0f64 - (distance / (radius as f64)));
            return ((j_round_d(((edge.powf(1.55f64)) * (max_depth as f64)) + ((edge * (((((z as f64).atan2((x as f64))) * 3.0f64) + (distance * 0.85f64)).sin())) * 1.5f64))) as i32);
        }
        pub fn drill_hover_height(radius: f64) -> f64 {
            return (18.0f64 + (radius * 2.2f64)) + ((0.0f64.max(radius - 5.0f64)) * 2.0f64);
        }
        pub fn drill_wind_visibility(speed: f32) -> f32 {
            let t: f32 = j_clamp((speed - 0.5f32) / 0.1f32, 0.0f32, 1.0f32);
            return (t * t) * (3.0f32 - (2.0f32 * t));
        }
        pub fn drill_heat(speed: f32) -> f32 {
            return j_clamp((speed - 0.5f32) / 0.35f32, 0.0f32, 1.0f32);
        }
        // RatioCollapseManager.sendDrillWind(3): not translated, it uses Minecraft's `Vec3`.
        // RatioCollapseManager.getActiveTaskCount(0): not translated, it uses RatioCollapseManager.ACTIVE_TASKS, which isn't translated (it uses `Queue` (java.util)).
        // RatioCollapseManager.clearAllTasks(0): not translated, it uses RatioCollapseManager.ACTIVE_TASKS, which isn't translated (it uses `Queue` (java.util)).
    }

    /// `RatioCollapseManager.RatioCollapseTask` (net/godwand/skill/RatioCollapseManager.java)
    pub mod ratio_collapse_manager_ratio_collapse_task {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ServerLevel`. Its static parts are below.
        // RatioCollapseManager.RatioCollapseTask.RatioCollapseTask(9): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
    }

    /// `ShockwaveSlicer` (net/godwand/skill/ShockwaveSlicer.java)
    pub mod shockwave_slicer {
        use super::*;
        // ACTIVE_TASKS: not translated, it uses `Queue` (java.util).
        #[derive(Clone, Debug)]
        pub struct ShockwaveSlicer {
        }
        impl ShockwaveSlicer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ShockwaveSlicer.isExposedSurfaceBlock(2): not translated, it uses Minecraft's `ServerLevel`.
        // ShockwaveSlicer.queueCalamityDetonation(3): not translated, it uses Minecraft's `ServerLevel`.
        // ShockwaveSlicer.queueCalamityDetonation(4): not translated, it uses Minecraft's `ServerLevel`.
        // ShockwaveSlicer.spawnThreeOrbitalParticleRings(4): not translated, it uses Minecraft's `ServerLevel`.
        // ShockwaveSlicer.hasActiveTask(1): not translated, it uses Minecraft's `ServerPlayer`.
        // ShockwaveSlicer.triggerEarlySnap(1): not translated, it uses Minecraft's `ServerPlayer`.
        // ShockwaveSlicer.snapTask(1): not translated, it uses Minecraft's `ServerPlayer`.
        // ShockwaveSlicer.tick(1): not translated, it uses Minecraft's `ServerLevel`.
        pub fn return_progress(progress: f32) -> f32 {
            let t: f32 = j_clamp(progress, 0.0f32, 1.0f32);
            return (t * t) * (3.0f32 - (2.0f32 * t));
        }
        // ShockwaveSlicer.animateReturn(2): not translated, it uses Minecraft's `BlockPos`.
        // ShockwaveSlicer.executeRestoration(1): not translated, it uses Minecraft's `ServerPlayer`.
        // ShockwaveSlicer.updateBlockPositions(3): not translated, it uses Minecraft's `BlockPos`.
        // ShockwaveSlicer.sendThreadFx(2): not translated, it uses Minecraft's `BlockPos`.
        // ShockwaveSlicer.getActiveTaskCount(0): not translated, it uses ShockwaveSlicer.ACTIVE_TASKS, which isn't translated (it uses `Queue` (java.util)).
        // ShockwaveSlicer.updateBeam(4): not translated, it uses Minecraft's `BlockPos`.
        // ShockwaveSlicer.clearBeams(1): not translated, it uses Minecraft's `BlockPos`.
        // ShockwaveSlicer.clearAllTasks(0): not translated, it uses ShockwaveSlicer.ACTIVE_TASKS, which isn't translated (it uses `Queue` (java.util)).
    }

    /// `ShockwaveSlicer.SingularityTask` (net/godwand/skill/ShockwaveSlicer.java)
    pub mod shockwave_slicer_singularity_task {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ServerLevel`. Its static parts are below.
        // ShockwaveSlicer.SingularityTask.SingularityTask(4): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // ShockwaveSlicer.SingularityTask.SingularityTask(5): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
    }

    /// `SkillCooldowns` (net/godwand/skill/SkillCooldowns.java)
    pub mod skill_cooldowns {
        use super::*;
        // STATES: not translated, it uses Minecraft's `ServerPlayer`.
        #[derive(Clone, Debug)]
        pub struct SkillCooldowns {
        }
        impl SkillCooldowns {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SkillCooldowns.state(1): not translated, it uses Minecraft's `ServerPlayer`.
        // SkillCooldowns.reset(1): not translated, it uses Minecraft's `ServerPlayer`.
        // SkillCooldowns.ready(2): not translated, it uses Minecraft's `Player`.
        // SkillCooldowns.start(3): not translated, it uses Minecraft's `Player`.
        // SkillCooldowns.sync(2): not translated, it uses Minecraft's `ServerPlayer`.
    }

    /// `SkillCooldowns.Entry` (net/godwand/skill/SkillCooldowns.java)
    pub mod skill_cooldowns_entry {
        use super::*;
        // The class itself isn't translated: it uses `WeakReference` (java.lang.ref). Its static parts are below.
        // SkillCooldowns.Entry.Entry(2): not translated, its class isn't translated (it uses `WeakReference` (java.lang.ref)).
        // SkillCooldowns.Entry.owner(0): not translated, its class isn't translated (it uses `WeakReference` (java.lang.ref)).
        // SkillCooldowns.Entry.timers(0): not translated, its class isn't translated (it uses `WeakReference` (java.lang.ref)).
    }

    /// `SkillCooldownState` (net/godwand/skill/SkillCooldownState.java)
    pub mod skill_cooldown_state {
        use super::*;
        pub const MAX_TICKS: i32 = 720000;
        #[derive(Clone, Debug)]
        pub struct SkillCooldownState {
            pub ends: Vec<i64>,
            pub durations: Vec<i32>,
        }
        impl SkillCooldownState {
            pub fn new() -> Self {
                let mut s = Self { ends: vec![0i64; 7], durations: vec![0i32; 7] };
                s
            }
            pub fn set(&mut self, mode: i32, duration: i32, remaining: i32, now: i64) {
                if (((((mode >= 0) && (mode < (self.ends.len() as i32))) && (duration >= 0)) && (duration <= 720000)) && (remaining >= 0)) && (remaining <= duration) {
                    self.durations[mode as usize] = duration;
                    self.ends[mode as usize] = now + (remaining as i64);
                } else {
                    panic!("{}", "Invalid skill cooldown");
                }
            }
            pub fn remaining(&self, mode: i32, now: i64) -> i32 {
                return if (mode >= 0) && (mode < (self.ends.len() as i32)) { ((0i64.max(self.ends[mode as usize] - now)) as i32) } else { 0 };
            }
            pub fn duration(&self, mode: i32) -> i32 {
                return if (mode >= 0) && (mode < (self.ends.len() as i32)) { self.durations[mode as usize] } else { 0 };
            }
            pub fn fraction(&self, mode: i32, now: i64) -> f32 {
                return if (self.duration(mode)) == 0 { 0.0f32 } else { ((self.remaining(mode, now)) as f32) / ((self.duration(mode)) as f32) };
            }
        }
    }

    /// `SpatialBridge` (net/godwand/skill/SpatialBridge.java)
    pub mod spatial_bridge {
        use super::*;
        // ENABLED: not translated, it uses `UUID` (java.util).
        // PRESSED: not translated, it uses `UUID` (java.util).
        // ELIGIBLE: not translated, it uses `UUID` (java.util).
        // FLIGHTS: not translated, it uses Flight, which isn't translated (it uses Minecraft's `ServerLevel`).
        // LEVEL: not translated, it uses `UUID` (java.util).
        #[derive(Clone, Debug)]
        pub struct SpatialBridge {
        }
        impl SpatialBridge {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SpatialBridge.resetRuntime(0): not translated, it uses SpatialBridge.ENABLED, which isn't translated (it uses `UUID` (java.util)).
        // SpatialBridge.disable(1): not translated, it uses `UUID` (java.util).
        pub fn raised_level(current: i32, feet_y: f64) -> i32 {
            return if feet_y >= ((current as f64) + 1.9f64) { current + 1 } else { current };
        }
        // SpatialBridge.plan(3): not translated, it uses Minecraft's `BlockPos`.
        // SpatialBridge.selected(1): not translated, it uses Minecraft's `ServerPlayer`.
        // SpatialBridge.toggle(1): not translated, it uses Minecraft's `ServerPlayer`.
        // SpatialBridge.tickPlayer(1): not translated, it uses Minecraft's `ServerPlayer`.
        // SpatialBridge.usable(3): not translated, it uses Minecraft's `ServerLevel`.
        // SpatialBridge.occupied(2): not translated, it uses Minecraft's `ServerLevel`.
        // SpatialBridge.clearVisuals(1): not translated, it uses Minecraft's `ServerLevel`.
        // SpatialBridge.tickWorld(1): not translated, it uses Minecraft's `ServerLevel`.
    }

    /// `SpatialBridge.Flight` (net/godwand/skill/SpatialBridge.java)
    pub mod spatial_bridge_flight {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ServerLevel`. Its static parts are below.
        // SpatialBridge.Flight.Flight(4): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // SpatialBridge.Flight.world(0): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // SpatialBridge.Flight.display(0): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // SpatialBridge.Flight.target(0): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // SpatialBridge.Flight.born(0): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
    }

    /// `SpatialPathWalker` (net/godwand/skill/SpatialPathWalker.java)
    pub mod spatial_path_walker {
        use super::*;
        // LAST_SHIELD_SOUND: not translated, it uses `UUID` (java.util).
        // ACTIVE_PLAYERS: not translated, it uses `UUID` (java.util).
        // LAST_GROUND_Y: not translated, it uses `UUID` (java.util).
        // LAST_PLAYER_POS: not translated, it uses `UUID` (java.util).
        // CURRENT_FORWARD_LEAD: not translated, it uses `UUID` (java.util).
        // CURRENT_PLAYER_SPEED: not translated, it uses `UUID` (java.util).
        #[derive(Clone, Debug)]
        pub struct SpatialPathWalker {
        }
        impl SpatialPathWalker {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SpatialPathWalker.isActive(1): not translated, it uses Minecraft's `Player`.
        // SpatialPathWalker.toggle(1): not translated, it uses Minecraft's `ServerPlayer`.
        // SpatialPathWalker.hasGodWand(1): not translated, it uses Minecraft's `Player`.
        // SpatialPathWalker.resetRuntime(0): not translated, it uses SpatialPathWalker.ACTIVE_PLAYERS, which isn't translated (it uses `UUID` (java.util)).
        // SpatialPathWalker.deactivate(1): not translated, it uses Minecraft's `Player`.
        // SpatialPathWalker.syncAura(1): not translated, it uses Minecraft's `ServerPlayer`.
        // SpatialPathWalker.shouldPlayShieldSound(2): not translated, it uses `UUID` (java.util).
        // SpatialPathWalker.sendAuraHit(2): not translated, it uses Minecraft's `ServerPlayer`.
        // SpatialPathWalker.deflectProjectiles(1): not translated, it uses Minecraft's `ServerPlayer`.
        // SpatialPathWalker.repelMobs(1): not translated, it uses Minecraft's `ServerPlayer`.
        // SpatialPathWalker.updatePlayerForwardLead(1): not translated, it uses Minecraft's `ServerPlayer`.
        // SpatialPathWalker.getPlayerSpeed(1): not translated, it uses Minecraft's `ServerPlayer`.
        // SpatialPathWalker.getPlayerForwardLead(1): not translated, it uses Minecraft's `ServerPlayer`.
        // SpatialPathWalker.tickPlayer(1): not translated, it uses Minecraft's `ServerPlayer`.
    }

    /// `SpellDamage` (net/godwand/skill/SpellDamage.java)
    pub mod spell_damage {
        use super::*;
        // TYPE: not translated, it uses Minecraft's `ResourceKey`.
        pub const WORM: &str = "net.jelly.sandworm_mod.entity.IK.worm.";
        // WARNED: not translated, it uses `Class`.
        // KILLS: not translated, it uses Minecraft's `ServerLevel`.
        #[derive(Clone, Debug)]
        pub struct SpellDamage {
        }
        impl SpellDamage {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SpellDamage.isCaster(2): not translated, it uses Minecraft's `Entity`.
        // SpellDamage.hurt(3): not translated, it uses Minecraft's `Entity`.
        // SpellDamage.noteKill(2): not translated, it uses Minecraft's `ServerLevel`.
        // SpellDamage.poolExperience(1): not translated, it uses Minecraft's `ServerLevel`.
        // SpellDamage.wormPart(1): not translated, it uses Minecraft's `Entity`.
        // SpellDamage.wormOwner(1): not translated, it uses Minecraft's `Entity`.
        // SpellDamage.identity(1): not translated, it uses `UUID` (java.util).
        // SpellDamage.extraTargets(2): not translated, it uses Minecraft's `Entity`.
        // SpellDamage.hurtExtras(4): not translated, it uses Minecraft's `ServerLevel`.
        // SpellDamage.warn(2): not translated, it uses Minecraft's `Entity`.
    }

    /// `StoneCannonManager` (net/godwand/skill/StoneCannonManager.java)
    pub mod stone_cannon_manager {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct StoneCannonManager {
        }
        impl StoneCannonManager {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn calculate_charge_level(charge_ticks: i32) -> i32 {
            if charge_ticks >= 80 {
                return 4;
            } else if charge_ticks >= 50 {
                return 3;
            } else {
                return if charge_ticks >= 25 { 2 } else { 1 };
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        pub fn calculate_direct_damage(level: i32) -> f32 {
            return match level {
                1 => 65.0f32,
                2 => 120.0f32,
                3 => 185.0f32,
                _ => 260.0f32,
            };
        }
        pub fn calculate_crater_radius(level: i32) -> f64 {
            return match level {
                1 => 3.0f64,
                2 => 4.5f64,
                3 => 6.0f64,
                _ => 8.0f64,
            };
        }
        pub fn calculate_speed(level: i32) -> f64 {
            return match level {
                1 => 2.8f64,
                2 => 3.6f64,
                3 => 4.5f64,
                _ => 5.4f64,
            };
        }
        pub fn calculate_recoil(level: i32) -> f64 {
            return match level {
                1 => 0.4f64,
                2 => 0.75f64,
                3 => 1.15f64,
                _ => 1.65f64,
            };
        }
        // StoneCannonManager.insideDrillSweep(5): not translated, it uses Minecraft's `Vec3`.
        // StoneCannonManager.excavateImpact(3): not translated, its body couldn't be read: an anonymous class.
        pub fn charge_stone_volume(ticks: i32) -> f32 {
            return 0.28f32 + (0.14f32 * (j_clamp((ticks as f32) / 80.0f32, 0.0f32, 1.0f32)));
        }
        // StoneCannonManager.tickCharge(3): not translated, it uses Minecraft's `ServerPlayer`.
        // StoneCannonManager.fire(2): not translated, it uses Minecraft's `ServerPlayer`.
        // StoneCannonManager.onProjectileImpact(2): not translated, it uses StoneCannonProjectileEntity, which isn't translated (it extends Minecraft's `Projectile`).
    }

    /// `SupernovaCraterCursor` (net/godwand/skill/SupernovaCraterCursor.java)
    pub mod supernova_crater_cursor {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SupernovaCraterCursor {
            pub columns: i32,
            pub visited: i32,
            pub x: i32,
            pub z: i32,
            pub dx: i32,
            pub dz: i32,
        }
        impl SupernovaCraterCursor {
            pub fn new(radius: i32) -> Self {
                let mut s = Self { columns: 0i32, visited: 0i32, x: 0i32, z: 0i32, dx: 0i32, dz: -1 };
                s.columns = ((radius * 2) + 1) * ((radius * 2) + 1);
                s
            }
            pub fn x(&self) -> i32 {
                return self.x;
            }
            pub fn z(&self) -> i32 {
                return self.z;
            }
            pub fn finished(&self) -> bool {
                return self.visited >= self.columns;
            }
            pub fn advance(&mut self) {
                if !self.finished() {
                    if ((self.x == self.z) || ((self.x < 0) && (self.x == (-self.z)))) || ((self.x > 0) && (self.x == (1 - self.z))) {
                        let previous_dx: i32 = self.dx;
                        self.dx = -self.dz;
                        self.dz = previous_dx;
                    }
                    self.x = self.x + self.dx;
                    self.z = self.z + self.dz;
                    self.visited += 1;
                }
            }
        }
    }

    /// `SupernovaManager` (net/godwand/skill/SupernovaManager.java)
    pub mod supernova_manager {
        use super::*;
        pub const DETONATION_TICK: i32 = 348;
        pub const IMPACT_TICK: i32 = 344;
        pub const TERRAIN_START_TICK: i32 = 352;
        pub const MAX_LIFETIME_TICKS: i32 = 628;
        pub const MAX_DISPLAYS_CAP: i32 = 1800;
        // ACTIVE_TASKS: not translated, it uses `Queue` (java.util).
        // CHARGE_TARGETS: not translated, it uses Minecraft's `ServerPlayer`.
        #[derive(Clone, Debug)]
        pub struct SupernovaManager {
        }
        impl SupernovaManager {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SupernovaManager.isFoliageOrNonSolid(3): not translated, it uses Minecraft's `ServerLevel`.
        // SupernovaManager.mobPullVelocity(3): not translated, it uses Minecraft's `Vec3`.
        // SupernovaManager.findTarget(1): not translated, it uses Minecraft's `BlockPos`.
        // SupernovaManager.beginCharge(1): not translated, it uses Minecraft's `ServerPlayer`.
        // SupernovaManager.cancelCharge(1): not translated, it uses Minecraft's `ServerPlayer`.
        // SupernovaManager.cast(1): not translated, it uses Minecraft's `ServerPlayer`.
        // SupernovaManager.prepareTerrain(1): not translated, it uses Minecraft's `ServerLevel`.
        // SupernovaManager.tick(1): not translated, it uses Minecraft's `ServerLevel`.
        // SupernovaManager.broadcastDetonation(1): not translated, it uses ImpactFxPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SupernovaManager.carveSphericalCraterShell(3): not translated, it uses Minecraft's `ServerLevel`.
        // SupernovaManager.getActiveTaskCount(0): not translated, it uses SupernovaManager.ACTIVE_TASKS, which isn't translated (it uses `Queue` (java.util)).
        // SupernovaManager.clearAllTasks(0): not translated, it uses SupernovaManager.ACTIVE_TASKS, which isn't translated (it uses `Queue` (java.util)).
    }

    /// `SupernovaManager.SupernovaTask` (net/godwand/skill/SupernovaManager.java)
    pub mod supernova_manager_supernova_task {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ServerLevel`. Its static parts are below.
        // SupernovaManager.SupernovaTask.SupernovaTask(4): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
    }

    /// `SupernovaSequence` (net/godwand/skill/SupernovaSequence.java)
    pub mod supernova_sequence {
        use super::*;
        pub const CHARGE_TICKS: i32 = 140;
        pub const SKY_RELEASE: i32 = 348;
        pub const SKY_CLOSE_START: i32 = 442;
        pub const SKY_CLOSE_END: i32 = 534;
        pub const BURST_TICKS: i32 = 280;
        pub const SEQUENCE_END: i32 = 628;
        pub const PEEL_END: i32 = 250;
        pub const SWALLOW_END: i32 = 326;
        pub const QUIET_START: i32 = 324;
        pub const CUT_START: i32 = 328;
        pub const CUT_END: i32 = 330;
        pub const FLARE_START: i32 = 332;
        pub const IMPACT: i32 = 340;
        #[derive(Clone, Debug)]
        pub struct SupernovaSequence {
        }
        impl SupernovaSequence {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn phase(age: f64, start: f64, end: f64) -> f32 {
            let t: f32 = ((j_clamp((age - start) / (end - start), 0.0f64, 1.0f64)) as f32);
            return (t * t) * (3.0f32 - (2.0f32 * t));
        }
        pub fn charge_rift_scale(ticks: f64) -> f32 {
            return supernova_sequence::phase(ticks, 0.0f64, 140.0f64);
        }
        pub fn sky_collapse(age: f64) -> f32 {
            return supernova_sequence::phase(age, 442.0f64, 534.0f64);
        }
        pub fn sky_blackout(age: f64) -> f32 {
            return supernova_sequence::phase(age, 348.0f64, 368.0f64) * (1.0f32 - supernova_sequence::sky_collapse(age));
        }
        pub fn sky_scale(age: f64) -> f32 {
            return (1.0f32 + (0.65f32 * supernova_sequence::phase(age, 348.0f64, 368.0f64))) * (1.0f32 - supernova_sequence::sky_collapse(age));
        }
        pub fn activation_pulse(age: f64) -> f32 {
            return supernova_sequence::phase(age, 0.0f64, 2.0f64) * (1.0f32 - supernova_sequence::phase(age, 5.0f64, 22.0f64));
        }
        pub fn sky_pulse_radius(age: f64) -> f32 {
            return if age < 442.0f64 { 6.0f32 + (90.0f32 * supernova_sequence::phase(age, 0.0f64, 22.0f64)) } else { 225.0f32 * (1.0f32 - supernova_sequence::sky_collapse(age)) };
        }
        pub fn sky_pulse_strength(age: f64) -> f32 {
            return if age < 442.0f64 { supernova_sequence::activation_pulse(age) * 0.65f32 } else { (0.65f32 * supernova_sequence::phase(age, 442.0f64, 448.0f64)) * (1.0f32 - supernova_sequence::phase(age, 528.0f64, 534.0f64)) };
        }
        pub fn swallow(age: f64, seed: f64) -> f32 {
            return supernova_sequence::phase(age, 250.0f64 + (j_clamp(seed, 0.0f64, 1.0f64) * 14.0f64), 326.0f64);
        }
        pub fn pressure(age: f64) -> f32 {
            return supernova_sequence::phase(age, 12.0f64, 220.0f64) * (0.35f32 + (0.65f32 * supernova_sequence::phase(age, 175.0f64, 320.0f64)));
        }
        pub fn ribbon_visibility(age: f64) -> f32 {
            return supernova_sequence::phase(age, 0.0f64, 25.0f64) * (1.0f32 - supernova_sequence::phase(age, 322.0f64, 328.0f64));
        }
        pub fn core_radius(age: f64) -> f32 {
            let grow: f32 = ((j_clamp(age / 340.0f64, 0.0f64, 0.9647058823529412f64).powf(0.95f64)) as f32);
            return (0.15f32 + (grow * 8.35f32)) * (1.0f32 - supernova_sequence::phase(age, 328.0f64, 330.0f64));
        }
        pub fn camera_fov(age: f64) -> f32 {
            let wide: f32 = (0.96f32 + (0.09f32 * supernova_sequence::phase(age, 0.0f64, 80.0f64))) + (0.03f32 * supernova_sequence::phase(age, 80.0f64, 220.0f64));
            return (wide + ((0.88f32 - wide) * supernova_sequence::phase(age, 250.0f64, 324.0f64))) + (0.055f32 * supernova_sequence::activation_pulse(age));
        }
        pub fn environment_gain(age: f64) -> f32 {
            return (1.0f32 - (0.2f32 * supernova_sequence::pressure(age))) * (1.0f32 - (0.96f32 * supernova_sequence::phase(age, 316.0f64, 324.0f64)));
        }
    }

    /// `DisplacedBlockRecord` (net/godwand/spatial/DisplacedBlockRecord.java)
    pub mod displaced_block_record {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // DisplacedBlockRecord.writeBridgeData(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // DisplacedBlockRecord.readBridgeData(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // DisplacedBlockRecord.DisplacedBlockRecord(11): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // DisplacedBlockRecord.DisplacedBlockRecord(9): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // DisplacedBlockRecord.DisplacedBlockRecord(8): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // DisplacedBlockRecord.DisplacedBlockRecord(7): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // DisplacedBlockRecord.pos(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // DisplacedBlockRecord.originalState(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // DisplacedBlockRecord.state(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // DisplacedBlockRecord.blockEntityNbt(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // DisplacedBlockRecord.nbt(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // DisplacedBlockRecord.getCurrentOffset(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // DisplacedBlockRecord.beginRestoration(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // DisplacedBlockRecord.interruptRestoration(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // DisplacedBlockRecord.resetRestoreTick(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // DisplacedBlockRecord.isRestoring(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `DisplacedBlockRecord.DisplacementMode` (net/godwand/spatial/DisplacedBlockRecord.java)
    pub mod displaced_block_record_displacement_mode {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum DisplacementMode {
            PathWalker,
            HysteresisAura,
            TimedLifecycle,
            CanopyVortex,
            CustomScripted,
        }
        impl DisplacementMode {
            pub const VALUES: [DisplacementMode; 5] = [DisplacementMode::PathWalker, DisplacementMode::HysteresisAura, DisplacementMode::TimedLifecycle, DisplacementMode::CanopyVortex, DisplacementMode::CustomScripted, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    DisplacementMode::PathWalker => "PATH_WALKER",
                    DisplacementMode::HysteresisAura => "HYSTERESIS_AURA",
                    DisplacementMode::TimedLifecycle => "TIMED_LIFECYCLE",
                    DisplacementMode::CanopyVortex => "CANOPY_VORTEX",
                    DisplacementMode::CustomScripted => "CUSTOM_SCRIPTED",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "PATH_WALKER" => DisplacementMode::PathWalker,
                    "HYSTERESIS_AURA" => DisplacementMode::HysteresisAura,
                    "TIMED_LIFECYCLE" => DisplacementMode::TimedLifecycle,
                    "CANOPY_VORTEX" => DisplacementMode::CanopyVortex,
                    "CUSTOM_SCRIPTED" => DisplacementMode::CustomScripted,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `DisplacementPersistentState` (net/godwand/spatial/DisplacementPersistentState.java)
    pub mod displacement_persistent_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SavedData`. Its static parts are below.
        pub const IDENTIFIER: &str = "godwand_displacements";
        // TYPE: not translated, it uses Minecraft's `SavedDataType`.
        // DisplacementPersistentState.setWorldKey(1): not translated, its class isn't translated (it extends Minecraft's `SavedData`).
        // DisplacementPersistentState.createFromNbt(2): not translated, its body couldn't be read: a class declared inside a method.
        // DisplacementPersistentState.save(2): not translated, its body couldn't be read: a class declared inside a method.
        // DisplacementPersistentState.getServerState(1): not translated, it uses DisplacementPersistentState, which isn't translated (it extends Minecraft's `SavedData`).
        // DisplacementPersistentState.restoreAndCleanup(1): not translated, its class isn't translated (it extends Minecraft's `SavedData`).
        // DisplacementPersistentState.getUnrestoredRecords(0): not translated, its class isn't translated (it extends Minecraft's `SavedData`).
    }

    /// `SpatialDisplacementManager` (net/godwand/spatial/SpatialDisplacementManager.java)
    pub mod spatial_displacement_manager {
        use super::*;
        // ACTIVE_RECORDS: not translated, it uses `DimensionPos`.
        // BRIDGE_DESTINATIONS: not translated, it uses `DimensionPos`.
        // ACTIVE_ENTITIES: not translated, it uses `UUID` (java.util).
        // LAST_PATH_OPEN_SOUND: not translated, it uses `UUID` (java.util).
        // LAST_PATH_RETURN_SOUND: not translated, it uses `UUID` (java.util).
        #[derive(Clone, Debug)]
        pub struct SpatialDisplacementManager {
        }
        impl SpatialDisplacementManager {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SpatialDisplacementManager.shouldPlayPathSound(3): not translated, it uses `UUID` (java.util).
        // SpatialDisplacementManager.isPositionDisplaced(2): not translated, it uses Minecraft's `ResourceKey`.
        // SpatialDisplacementManager.isPositionDisplaced(1): not translated, it uses Minecraft's `BlockPos`.
        // SpatialDisplacementManager.canDisplace(3): not translated, it uses Minecraft's `ServerLevel`.
        // SpatialDisplacementManager.clearRuntimeState(0): not translated, it uses SpatialDisplacementManager.ACTIVE_RECORDS, which isn't translated (it uses `DimensionPos`).
        // SpatialDisplacementManager.recoverBridge(1): not translated, it uses DisplacedBlockRecord, which isn't translated (it uses Minecraft's `BlockPos`).
        // SpatialDisplacementManager.moveBridge(2): not translated, its body couldn't be read: a class declared inside a method.
        // SpatialDisplacementManager.getActiveRecordCount(0): not translated, it uses SpatialDisplacementManager.ACTIVE_RECORDS, which isn't translated (it uses `DimensionPos`).
        // SpatialDisplacementManager.getActiveEntityCount(0): not translated, it uses SpatialDisplacementManager.ACTIVE_ENTITIES, which isn't translated (it uses `UUID` (java.util)).
        // SpatialDisplacementManager.getActiveRecord(2): not translated, it uses DisplacedBlockRecord, which isn't translated (it uses Minecraft's `BlockPos`).
        // SpatialDisplacementManager.getActiveEntity(1): not translated, it uses Minecraft's `BlockDisplay`.
        // SpatialDisplacementManager.tickManagedDisplay(1): not translated, it uses Minecraft's `BlockDisplay`.
        // SpatialDisplacementManager.trackEntity(2): not translated, it uses `UUID` (java.util).
        // SpatialDisplacementManager.untrackEntity(1): not translated, it uses `UUID` (java.util).
        // SpatialDisplacementManager.removeActiveRecord(2): not translated, it uses DisplacedBlockRecord, which isn't translated (it uses Minecraft's `BlockPos`).
        // SpatialDisplacementManager.getActiveRecordsForWorld(1): not translated, it uses DisplacedBlockRecord, which isn't translated (it uses Minecraft's `BlockPos`).
        // SpatialDisplacementManager.refreshDisplacement(3): not translated, its body couldn't be read: a class declared inside a method.
        // SpatialDisplacementManager.getMatchingDoubleBlockHalf(3): not translated, it uses Minecraft's `BlockPos`.
        // SpatialDisplacementManager.displaceBlock(6): not translated, it uses Minecraft's `ServerLevel`.
        // SpatialDisplacementManager.displaceBlock(7): not translated, it uses Minecraft's `ServerLevel`.
        // SpatialDisplacementManager.displaceBlock(8): not translated, it uses Minecraft's `ServerLevel`.
        // SpatialDisplacementManager.displaceBlock(12): not translated, it uses Minecraft's `ServerLevel`.
        // SpatialDisplacementManager.displaceBlock(13): not translated, it uses Minecraft's `ServerLevel`.
        // SpatialDisplacementManager.restorePositions(2): not translated, it uses Minecraft's `ServerLevel`.
        // SpatialDisplacementManager.displaceBlockPermanently(12): not translated, it uses Minecraft's `ServerLevel`.
        // SpatialDisplacementManager.displaceBlockInternal(14): not translated, its body couldn't be read: a class declared inside a method.
        // SpatialDisplacementManager.tick(1): not translated, its body couldn't be read: a class declared inside a method.
        // SpatialDisplacementManager.restoreRecordDirectly(3): not translated, it uses Minecraft's `ServerLevel`.
        // SpatialDisplacementManager.collapsePlayerAura(1): not translated, its body couldn't be read: a class declared inside a method.
        // SpatialDisplacementManager.isPathRecord(1): not translated, it uses DisplacedBlockRecord, which isn't translated (it uses Minecraft's `BlockPos`).
        // SpatialDisplacementManager.restorePathForPlayer(2): not translated, it uses `UUID` (java.util).
        // SpatialDisplacementManager.restoreAllForPlayer(2): not translated, it uses `UUID` (java.util).
        // SpatialDisplacementManager.restoreForPlayer(3): not translated, it uses `UUID` (java.util).
        // SpatialDisplacementManager.emergencyRestoreAll(1): not translated, it uses Minecraft's `ServerLevel`.
        // SpatialDisplacementManager.executeTwoPassRestoration(2): not translated, it uses Minecraft's `ServerLevel`.
        // SpatialDisplacementManager.collapseNearbyDisplacements(3): not translated, its body couldn't be read: a class declared inside a method.
    }

    /// `SpatialMath` (net/godwand/spatial/SpatialMath.java)
    pub mod spatial_math {
        use super::*;
        pub static PIVOT: LazyLock<JVec3> = LazyLock::new(|| JVec3::new(0.5f32, 0.5f32, 0.5f32));
        #[derive(Clone, Debug)]
        pub struct SpatialMath {
        }
        impl SpatialMath {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SpatialMath.pathLooseBlockVelocity(3): not translated, it uses Minecraft's `Vec3`.
        // SpatialMath.createPivotTransform(3): not translated, it uses Minecraft's `Transformation`.
        // SpatialMath.createPivotTransform(3): not translated, it uses Minecraft's `Transformation`.
        // SpatialMath.compensatePivot(2): not translated, JOML's Quaternionf.transform.
        // SpatialMath.createThinBeamTransform(3): not translated, it uses Minecraft's `Transformation`.
        // SpatialMath.computeDynamicAuraOffset(5): not translated, it uses Minecraft's `Vec3`.
        // SpatialMath.computeArchwayRotation(4): not translated, it uses Minecraft's `BlockPos`.
        // SpatialMath.computeAuraDisplacement(4): not translated, it uses Minecraft's `Vec3`.
        pub fn can_open_path_block(block_y: i32, feet_y: f64, allow_down: bool) -> bool {
            return allow_down || (((block_y as f64) + 1.0f64) > (feet_y + 0.125f64));
        }
        // SpatialMath.supportsPathFeet(2): not translated, it uses Minecraft's `AABB`.
        // SpatialMath.distanceToPathAura(4): not translated, it uses Minecraft's `Vec3`.
        // SpatialMath.auraArrowIntersection(3): not translated, it uses Minecraft's `Vec3`.
        // SpatialMath.deflectAuraArrow(2): not translated, it uses Minecraft's `Vec3`.
        // SpatialMath.auraMobVelocity(3): not translated, it uses Minecraft's `Vec3`.
        pub fn aura_visibility(facing: f64) -> f32 {
            let t: f64 = j_clamp((facing - 0.25f64) / 0.65f64, 0.0f64, 1.0f64);
            return ((1.0f64 - (((0.94f64 * t) * t) * (3.0f64 - (2.0f64 * t)))) as f32);
        }
        // SpatialMath.distanceToAuraSpine(4): not translated, it uses Minecraft's `Vec3`.
        // SpatialMath.computeInteractiveEvasion(3): not translated, it uses Minecraft's `Vec3`.
        // SpatialMath.computeCanopyVortexOffset(4): not translated, it uses Minecraft's `Vec3`.
        // SpatialMath.computeCanopyVortexOffset(5): not translated, it uses Minecraft's `Vec3`.
        // SpatialMath.computeCanopyVortexInternal(7): not translated, it uses Minecraft's `Vec3`.
        // SpatialMath.computeDynamicBlackHoleOrbit(6): not translated, it uses Minecraft's `Vec3`.
        // SpatialMath.computeSpiralReturnOffset(2): not translated, it uses Minecraft's `Vec3`.
        // SpatialMath.hash01(1): not translated, it uses Minecraft's `BlockPos`.
        pub fn hash_coordinate_to_phase(pos_as_long: i64) -> f64 {
            let h: i64 = (pos_as_long * 25214903917i64) + 11i64;
            return (((((h >> 16) & 65535i64) as f64) / 65535.0f64) * 2.0f64) * std::f64::consts::PI;
        }
    }

    /// JOML's Vector3f as a value (Java's methods change the vector; the translation writes the result back).
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct JVec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    impl JVec3 {
        pub const fn new(x: f32, y: f32, z: f32) -> Self {
            Self { x, y, z }
        }
        pub fn add(self, o: JVec3) -> Self {
            Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
        }
        pub fn add3(self, x: f32, y: f32, z: f32) -> Self {
            Self::new(self.x + x, self.y + y, self.z + z)
        }
        pub fn sub(self, o: JVec3) -> Self {
            Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
        }
        pub fn mul_s(self, s: f32) -> Self {
            Self::new(self.x * s, self.y * s, self.z * s)
        }
        pub fn mul_v(self, o: JVec3) -> Self {
            Self::new(self.x * o.x, self.y * o.y, self.z * o.z)
        }
        pub fn negate(self) -> Self {
            Self::new(-self.x, -self.y, -self.z)
        }
        pub fn dot(self, o: JVec3) -> f32 {
            self.x * o.x + self.y * o.y + self.z * o.z
        }
        pub fn length(self) -> f32 {
            self.dot(self).sqrt()
        }
        pub fn length_squared(self) -> f32 {
            self.dot(self)
        }
        /// JOML's normalize: divides by the length (a zero vector becomes NaNs, as in Java).
        pub fn normalize(self) -> Self {
            let inv = 1.0 / self.length();
            self.mul_s(inv)
        }
        pub fn cross(self, o: JVec3) -> Self {
            Self::new(self.y * o.z - self.z * o.y, self.z * o.x - self.x * o.z, self.x * o.y - self.y * o.x)
        }
        /// JOML's lerp: this + (other - this) * t.
        pub fn lerp(self, o: JVec3, t: f32) -> Self {
            Self::new(self.x + (o.x - self.x) * t, self.y + (o.y - self.y) * t, self.z + (o.z - self.z) * t)
        }
        pub fn distance(self, o: JVec3) -> f32 {
            self.sub(o).length()
        }
    }
    
    /// JOML's Quaternionf (x, y, z, w).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JQuat {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
    impl Default for JQuat {
        fn default() -> Self {
            Self::IDENTITY
        }
    }
    impl JQuat {
        pub const IDENTITY: JQuat = JQuat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
            Self { x, y, z, w }
        }
        /// `rotationX(angle)`: a turn about +X by `angle` radians (Axis.XP.rotation).
        pub fn rotation_x(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(s, 0.0, 0.0, c)
        }
        pub fn rotation_y(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(0.0, s, 0.0, c)
        }
        pub fn rotation_z(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(0.0, 0.0, s, c)
        }
        pub fn normalize(self) -> Self {
            let inv = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
            Self::new(self.x * inv, self.y * inv, self.z * inv, self.w * inv)
        }
        /// this * q.
        pub fn mul(self, q: JQuat) -> Self {
            Self::new(
                self.w * q.x + self.x * q.w + self.y * q.z - self.z * q.y,
                self.w * q.y - self.x * q.z + self.y * q.w + self.z * q.x,
                self.w * q.z + self.x * q.y - self.y * q.x + self.z * q.w,
                self.w * q.w - self.x * q.x - self.y * q.y - self.z * q.z,
            )
        }
        pub fn rotate_x(self, a: f32) -> Self {
            self.mul(Self::rotation_x(a))
        }
        pub fn rotate_y(self, a: f32) -> Self {
            self.mul(Self::rotation_y(a))
        }
        pub fn rotate_z(self, a: f32) -> Self {
            self.mul(Self::rotation_z(a))
        }
    }
    
    /// JOML's Matrix3f, column-major, from three column vectors.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JMat3 {
        pub m: [f32; 9],
    }
    impl JMat3 {
        pub fn from_cols(c0: JVec3, c1: JVec3, c2: JVec3) -> Self {
            Self { m: [c0.x, c0.y, c0.z, c1.x, c1.y, c1.z, c2.x, c2.y, c2.z] }
        }
    }
    
    /// JOML's Matrix4f: column-major, `m[col * 4 + row]`; `a.mul(b)` is a * b, and translate/rotate/scale
    /// multiply on the right, as JOML does.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JMat4 {
        pub m: [f32; 16],
    }
    impl Default for JMat4 {
        fn default() -> Self {
            Self::IDENTITY
        }
    }
    impl JMat4 {
        pub const IDENTITY: JMat4 = JMat4 { m: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0] };
        pub fn identity() -> Self {
            Self::IDENTITY
        }
        fn at(&self, row: usize, col: usize) -> f32 {
            self.m[col * 4 + row]
        }
        pub fn mul(self, r: JMat4) -> Self {
            let mut out = [0.0f32; 16];
            for col in 0..4 {
                for row in 0..4 {
                    let mut v = 0.0;
                    for k in 0..4 {
                        v += self.at(row, k) * r.at(k, col);
                    }
                    out[col * 4 + row] = v;
                }
            }
            Self { m: out }
        }
        pub fn translation(x: f32, y: f32, z: f32) -> Self {
            let mut t = Self::IDENTITY;
            t.m[12] = x;
            t.m[13] = y;
            t.m[14] = z;
            t
        }
        pub fn translate(self, x: f32, y: f32, z: f32) -> Self {
            self.mul(Self::translation(x, y, z))
        }
        pub fn translate_v(self, v: JVec3) -> Self {
            self.translate(v.x, v.y, v.z)
        }
        pub fn scaling(x: f32, y: f32, z: f32) -> Self {
            let mut t = Self::IDENTITY;
            t.m[0] = x;
            t.m[5] = y;
            t.m[10] = z;
            t
        }
        pub fn scale(self, x: f32, y: f32, z: f32) -> Self {
            self.mul(Self::scaling(x, y, z))
        }
        pub fn scale_s(self, s: f32) -> Self {
            self.scale(s, s, s)
        }
        /// The rotation a unit quaternion stands for.
        pub fn rotation(q: JQuat) -> Self {
            let (x, y, z, w) = (q.x, q.y, q.z, q.w);
            let (xx, yy, zz) = (x * x, y * y, z * z);
            let (xy, xz, yz, wx, wy, wz) = (x * y, x * z, y * z, w * x, w * y, w * z);
            Self {
                m: [
                    1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0,
                    2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0,
                    2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0,
                    0.0, 0.0, 0.0, 1.0,
                ],
            }
        }
        pub fn rotate(self, q: JQuat) -> Self {
            self.mul(Self::rotation(q))
        }
        pub fn rotate_x(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_x(a))
        }
        pub fn rotate_y(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_y(a))
        }
        pub fn rotate_z(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_z(a))
        }
        /// JOML's rotateXYZ: about X, then Y, then Z.
        pub fn rotate_xyz(self, x: f32, y: f32, z: f32) -> Self {
            self.rotate_x(x).rotate_y(y).rotate_z(z)
        }
        /// JOML's rotateAround(q, ox, oy, oz): about the point (ox, oy, oz).
        pub fn rotate_around(self, q: JQuat, ox: f32, oy: f32, oz: f32) -> Self {
            self.translate(ox, oy, oz).rotate(q).translate(-ox, -oy, -oz)
        }
        /// The general inverse (JOML's invert); a singular matrix gives infinities, as in Java.
        pub fn invert(self) -> Self {
            let a = &self.m;
            let mut inv = [0.0f32; 16];
            inv[0] = a[5] * a[10] * a[15] - a[5] * a[11] * a[14] - a[9] * a[6] * a[15] + a[9] * a[7] * a[14] + a[13] * a[6] * a[11] - a[13] * a[7] * a[10];
            inv[4] = -a[4] * a[10] * a[15] + a[4] * a[11] * a[14] + a[8] * a[6] * a[15] - a[8] * a[7] * a[14] - a[12] * a[6] * a[11] + a[12] * a[7] * a[10];
            inv[8] = a[4] * a[9] * a[15] - a[4] * a[11] * a[13] - a[8] * a[5] * a[15] + a[8] * a[7] * a[13] + a[12] * a[5] * a[11] - a[12] * a[7] * a[9];
            inv[12] = -a[4] * a[9] * a[14] + a[4] * a[10] * a[13] + a[8] * a[5] * a[14] - a[8] * a[6] * a[13] - a[12] * a[5] * a[10] + a[12] * a[6] * a[9];
            inv[1] = -a[1] * a[10] * a[15] + a[1] * a[11] * a[14] + a[9] * a[2] * a[15] - a[9] * a[3] * a[14] - a[13] * a[2] * a[11] + a[13] * a[3] * a[10];
            inv[5] = a[0] * a[10] * a[15] - a[0] * a[11] * a[14] - a[8] * a[2] * a[15] + a[8] * a[3] * a[14] + a[12] * a[2] * a[11] - a[12] * a[3] * a[10];
            inv[9] = -a[0] * a[9] * a[15] + a[0] * a[11] * a[13] + a[8] * a[1] * a[15] - a[8] * a[3] * a[13] - a[12] * a[1] * a[11] + a[12] * a[3] * a[9];
            inv[13] = a[0] * a[9] * a[14] - a[0] * a[10] * a[13] - a[8] * a[1] * a[14] + a[8] * a[2] * a[13] + a[12] * a[1] * a[10] - a[12] * a[2] * a[9];
            inv[2] = a[1] * a[6] * a[15] - a[1] * a[7] * a[14] - a[5] * a[2] * a[15] + a[5] * a[3] * a[14] + a[13] * a[2] * a[7] - a[13] * a[3] * a[6];
            inv[6] = -a[0] * a[6] * a[15] + a[0] * a[7] * a[14] + a[4] * a[2] * a[15] - a[4] * a[3] * a[14] - a[12] * a[2] * a[7] + a[12] * a[3] * a[6];
            inv[10] = a[0] * a[5] * a[15] - a[0] * a[7] * a[13] - a[4] * a[1] * a[15] + a[4] * a[3] * a[13] + a[12] * a[1] * a[7] - a[12] * a[3] * a[5];
            inv[14] = -a[0] * a[5] * a[14] + a[0] * a[6] * a[13] + a[4] * a[1] * a[14] - a[4] * a[2] * a[13] - a[12] * a[1] * a[6] + a[12] * a[2] * a[5];
            inv[3] = -a[1] * a[6] * a[11] + a[1] * a[7] * a[10] + a[5] * a[2] * a[11] - a[5] * a[3] * a[10] - a[9] * a[2] * a[7] + a[9] * a[3] * a[6];
            inv[7] = a[0] * a[6] * a[11] - a[0] * a[7] * a[10] - a[4] * a[2] * a[11] + a[4] * a[3] * a[10] + a[8] * a[2] * a[7] - a[8] * a[3] * a[6];
            inv[11] = -a[0] * a[5] * a[11] + a[0] * a[7] * a[9] + a[4] * a[1] * a[11] - a[4] * a[3] * a[9] - a[8] * a[1] * a[7] + a[8] * a[3] * a[5];
            inv[15] = a[0] * a[5] * a[10] - a[0] * a[6] * a[9] - a[4] * a[1] * a[10] + a[4] * a[2] * a[9] + a[8] * a[1] * a[6] - a[8] * a[2] * a[5];
            let det = a[0] * inv[0] + a[1] * inv[4] + a[2] * inv[8] + a[3] * inv[12];
            let d = 1.0 / det;
            for v in inv.iter_mut() {
                *v *= d;
            }
            Self { m: inv }
        }
        /// The point v moved by this matrix (w = 1).
        pub fn transform_position(&self, v: JVec3) -> JVec3 {
            JVec3::new(
                self.at(0, 0) * v.x + self.at(0, 1) * v.y + self.at(0, 2) * v.z + self.at(0, 3),
                self.at(1, 0) * v.x + self.at(1, 1) * v.y + self.at(1, 2) * v.z + self.at(1, 3),
                self.at(2, 0) * v.x + self.at(2, 1) * v.y + self.at(2, 2) * v.z + self.at(2, 3),
            )
        }
        /// The direction v turned by this matrix (w = 0).
        pub fn transform_direction(&self, v: JVec3) -> JVec3 {
            JVec3::new(
                self.at(0, 0) * v.x + self.at(0, 1) * v.y + self.at(0, 2) * v.z,
                self.at(1, 0) * v.x + self.at(1, 1) * v.y + self.at(1, 2) * v.z,
                self.at(2, 0) * v.x + self.at(2, 1) * v.y + self.at(2, 2) * v.z,
            )
        }
        /// JOML's set3x3: the upper-left 3x3 from `m`, the rest kept.
        pub fn set3x3(self, m: JMat3) -> Self {
            let mut out = self;
            for col in 0..3 {
                for row in 0..3 {
                    out.m[col * 4 + row] = m.m[col * 3 + row];
                }
            }
            out
        }
        /// JOML's transpose3x3: the upper-left 3x3 transposed, the rest kept.
        pub fn transpose3x3(self) -> Self {
            let mut out = self;
            for col in 0..3 {
                for row in 0..3 {
                    out.m[col * 4 + row] = self.m[row * 4 + col];
                }
            }
            out
        }
        pub fn get_translation(&self) -> JVec3 {
            JVec3::new(self.m[12], self.m[13], self.m[14])
        }
    }
    
    /// Minecraft's PoseStack: a stack of matrices (normals aren't kept: Horizon's hand renderer takes one matrix).
    #[derive(Clone, Debug)]
    pub struct JPoseStack {
        pub stack: Vec<JMat4>,
    }
    impl Default for JPoseStack {
        fn default() -> Self {
            Self { stack: vec![JMat4::IDENTITY] }
        }
    }
    impl JPoseStack {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn of(m: JMat4) -> Self {
            Self { stack: vec![m] }
        }
        fn top(&mut self) -> &mut JMat4 {
            self.stack.last_mut().expect("PoseStack is empty")
        }
        pub fn push_pose(&mut self) {
            let m = *self.top();
            self.stack.push(m);
        }
        pub fn pop_pose(&mut self) {
            self.stack.pop();
        }
        /// Writes the top matrix (Java changed `pose.last().pose()` in place).
        pub fn set_last(&mut self, m: JMat4) {
            *self.top() = m;
        }
        pub fn last_pose(&self) -> JMat4 {
            *self.stack.last().expect("PoseStack is empty")
        }
        pub fn set_identity(&mut self) {
            *self.top() = JMat4::IDENTITY;
        }
        pub fn translate(&mut self, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.translate(x, y, z);
        }
        pub fn scale(&mut self, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.scale(x, y, z);
        }
        pub fn mul_pose(&mut self, q: JQuat) {
            let m = self.top();
            *m = m.rotate(q);
        }
        pub fn mul_pose_matrix(&mut self, r: JMat4) {
            let m = self.top();
            *m = m.mul(r);
        }
        pub fn rotate_around(&mut self, q: JQuat, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.rotate_around(q, x, y, z);
        }
    }
    
    #[cfg(test)]
    mod joml_tests {
        use super::*;
    
        fn close(a: JVec3, b: JVec3) -> bool {
            (a.x - b.x).abs() < 1e-5 && (a.y - b.y).abs() < 1e-5 && (a.z - b.z).abs() < 1e-5
        }
    
        #[test]
        fn rotations_turn_axes_as_joml_does() {
            let q = JQuat::rotation_x(std::f32::consts::FRAC_PI_2);
            // A positive turn about X takes +Y to +Z (right-handed), as JOML's rotationX.
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(0.0, 1.0, 0.0)), JVec3::new(0.0, 0.0, 1.0)));
            let q = JQuat::rotation_y(std::f32::consts::FRAC_PI_2);
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(0.0, 0.0, 1.0)), JVec3::new(1.0, 0.0, 0.0)));
            let q = JQuat::rotation_z(std::f32::consts::FRAC_PI_2);
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(0.0, 1.0, 0.0)));
        }
    
        #[test]
        fn translate_then_rotate_multiplies_on_the_right() {
            // JOML: new Matrix4f().translate(1, 0, 0).rotateZ(90) moves (1, 0, 0) to (1, 1, 0).
            let m = JMat4::identity().translate(1.0, 0.0, 0.0).rotate_z(std::f32::consts::FRAC_PI_2);
            assert!(close(m.transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(1.0, 1.0, 0.0)));
        }
    
        #[test]
        fn invert_undoes() {
            let m = JMat4::identity().translate(1.0, 2.0, 3.0).rotate_x(0.7).rotate_y(-0.3).scale(2.0, 0.5, 1.5);
            let p = JVec3::new(0.3, -1.2, 4.0);
            assert!(close(m.invert().transform_position(m.transform_position(p)), p));
            assert!(close(m.mul(m.invert()).transform_position(p), p));
        }
    
        #[test]
        fn rotate_around_keeps_its_point() {
            let m = JMat4::identity().rotate_around(JQuat::rotation_y(1.1), 0.5, 0.25, 0.0);
            assert!(close(m.transform_position(JVec3::new(0.5, 0.25, 0.0)), JVec3::new(0.5, 0.25, 0.0)));
        }
    
        #[test]
        fn pose_stack_push_pop() {
            let mut p = JPoseStack::new();
            p.translate(1.0, 0.0, 0.0);
            p.push_pose();
            p.scale(2.0, 2.0, 2.0);
            assert!(close(p.last_pose().transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(3.0, 0.0, 0.0)));
            p.pop_pose();
            assert!(close(p.last_pose().transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(2.0, 0.0, 0.0)));
        }
    
        #[test]
        fn quaternion_matrix_agrees_with_axis_turns() {
            let q = JQuat::rotation_x(0.4).mul(JQuat::rotation_y(-0.9));
            let a = JMat4::rotation(q);
            let b = JMat4::identity().rotate_x(0.4).rotate_y(-0.9);
            let p = JVec3::new(1.0, 2.0, 3.0);
            assert!(close(a.transform_position(p), b.transform_position(p)));
        }
    
        #[test]
        fn set3x3_and_transpose() {
            let m3 = JMat3::from_cols(JVec3::new(0.0, 1.0, 0.0), JVec3::new(-1.0, 0.0, 0.0), JVec3::new(0.0, 0.0, 1.0));
            let m = JMat4::identity().set3x3(m3);
            assert!(close(m.transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(0.0, 1.0, 0.0)));
            assert!(close(m.transpose3x3().transform_position(JVec3::new(0.0, 1.0, 0.0)), JVec3::new(1.0, 0.0, 0.0)));
        }
    }

    
    // ---- Gson (com.google.gson): JSON values, a reader and Gson's writer ----
    // JsonElement, JsonObject, JsonArray and JsonPrimitive are all one Rust type. Numbers keep their text, as
    // Gson's LazilyParsedNumber does. Objects keep their insertion order, as Gson's LinkedTreeMap does.
    
    #[derive(Clone, Debug, PartialEq)]
    pub enum JJson {
        Null,
        Bool(bool),
        Num(String),
        Str(String),
        Arr(Vec<JJson>),
        Obj(Vec<(String, JJson)>),
    }
    
    /// A float or double as Java prints it (Float.toString / Double.toString).
    pub fn j_java_num(x: f64, float: bool) -> String {
        if x.is_nan() {
            return "NaN".into();
        }
        if x.is_infinite() {
            return if x > 0.0 { "Infinity".into() } else { "-Infinity".into() };
        }
        let a = x.abs();
        if x == 0.0 {
            return if x.is_sign_negative() { "-0.0".into() } else { "0.0".into() };
        }
        let digits = if float { format!("{:e}", x as f32) } else { format!("{:e}", x) };
        let (mant, exp) = digits.split_once('e').unwrap();
        let exp: i32 = exp.parse().unwrap();
        if (1e-3..1e7).contains(&a) {
            let plain = if float { format!("{}", x as f32) } else { format!("{}", x) };
            if plain.contains('.') { plain } else { plain + ".0" }
        } else {
            let mant = if mant.contains('.') { mant.to_string() } else { format!("{mant}.0") };
            format!("{mant}E{exp}")
        }
    }
    
    struct JJsonReader<'a> {
        s: &'a [char],
        i: usize,
    }
    
    impl JJsonReader<'_> {
        fn ws(&mut self) {
            while self.i < self.s.len() && self.s[self.i].is_whitespace() {
                self.i += 1;
            }
        }
        fn fail(&self, what: &str) -> ! {
            panic!("JsonSyntaxException: {what} at {}", self.i)
        }
        fn value(&mut self) -> JJson {
            self.ws();
            match self.s.get(self.i) {
                Some('{') => {
                    self.i += 1;
                    let mut members = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&'}') {
                        self.i += 1;
                        return JJson::Obj(members);
                    }
                    loop {
                        self.ws();
                        let k = match self.s.get(self.i) {
                            Some('"') | Some('\'') => self.string(),
                            _ => self.bare(),
                        };
                        self.ws();
                        if !matches!(self.s.get(self.i), Some(':') | Some('=')) {
                            self.fail("expected ':'");
                        }
                        self.i += 1;
                        let v = self.value();
                        // A repeated name: Gson keeps the last one.
                        members.retain(|(n, _): &(String, JJson)| *n != k);
                        members.push((k, v));
                        self.ws();
                        match self.s.get(self.i) {
                            Some(',') | Some(';') => self.i += 1,
                            Some('}') => {
                                self.i += 1;
                                return JJson::Obj(members);
                            }
                            _ => self.fail("expected ',' or '}'"),
                        }
                    }
                }
                Some('[') => {
                    self.i += 1;
                    let mut items = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&']') {
                        self.i += 1;
                        return JJson::Arr(items);
                    }
                    loop {
                        items.push(self.value());
                        self.ws();
                        match self.s.get(self.i) {
                            Some(',') | Some(';') => self.i += 1,
                            Some(']') => {
                                self.i += 1;
                                return JJson::Arr(items);
                            }
                            _ => self.fail("expected ',' or ']'"),
                        }
                    }
                }
                Some('"') | Some('\'') => JJson::Str(self.string()),
                Some(_) => {
                    let word = self.bare();
                    match word.as_str() {
                        "true" => JJson::Bool(true),
                        "false" => JJson::Bool(false),
                        "null" => JJson::Null,
                        w if w.parse::<f64>().is_ok() => JJson::Num(w.to_string()),
                        // Gson is lenient: an unquoted word is a string.
                        w => JJson::Str(w.to_string()),
                    }
                }
                None => self.fail("end of input"),
            }
        }
        fn bare(&mut self) -> String {
            let start = self.i;
            while self.i < self.s.len() && !matches!(self.s[self.i], ',' | ':' | '=' | ';' | '}' | ']' | '{' | '[' | '"' | '\'') && !self.s[self.i].is_whitespace() {
                self.i += 1;
            }
            if start == self.i {
                self.fail("expected a value");
            }
            self.s[start..self.i].iter().collect()
        }
        fn string(&mut self) -> String {
            let q = self.s[self.i];
            self.i += 1;
            let mut out = String::new();
            loop {
                let Some(&c) = self.s.get(self.i) else { self.fail("unterminated string") };
                self.i += 1;
                match c {
                    c if c == q => return out,
                    '\\' => {
                        let Some(&e) = self.s.get(self.i) else { self.fail("unterminated string") };
                        self.i += 1;
                        match e {
                            'n' => out.push('\n'),
                            't' => out.push('\t'),
                            'r' => out.push('\r'),
                            'b' => out.push('\u{8}'),
                            'f' => out.push('\u{c}'),
                            'u' => {
                                let hex: String = self.s[self.i..(self.i + 4).min(self.s.len())].iter().collect();
                                self.i += 4;
                                let unit = u16::from_str_radix(&hex, 16).unwrap_or_else(|_| self.fail("bad \\u escape"));
                                // A surrogate pair: two escapes make one char.
                                if (0xD800..0xDC00).contains(&unit) && self.s.get(self.i) == Some(&'\\') && self.s.get(self.i + 1) == Some(&'u') {
                                    let hex2: String = self.s[self.i + 2..(self.i + 6).min(self.s.len())].iter().collect();
                                    if let Ok(low) = u16::from_str_radix(&hex2, 16) {
                                        self.i += 6;
                                        out.extend(char::decode_utf16([unit, low]).map(|r| r.unwrap_or('\u{FFFD}')));
                                        continue;
                                    }
                                }
                                out.push(char::from_u32(unit as u32).unwrap_or('\u{FFFD}'));
                            }
                            other => out.push(other),
                        }
                    }
                    c => out.push(c),
                }
            }
        }
    }
    
    impl JJson {
        /// JsonParser.parseString: panics (Java throws JsonSyntaxException) on bad JSON.
        pub fn parse(s: &str) -> JJson {
            let chars: Vec<char> = s.chars().collect();
            let mut r = JJsonReader { s: &chars, i: 0 };
            let v = r.value();
            r.ws();
            if r.i != chars.len() {
                r.fail("more after the value");
            }
            v
        }
        pub fn object() -> JJson {
            JJson::Obj(Vec::new())
        }
        pub fn array() -> JJson {
            JJson::Arr(Vec::new())
        }
        pub fn int(v: i64) -> JJson {
            JJson::Num(v.to_string())
        }
        pub fn float(v: f32) -> JJson {
            JJson::Num(j_java_num(v as f64, true))
        }
        pub fn double(v: f64) -> JJson {
            JJson::Num(j_java_num(v, false))
        }
        pub fn str(v: &str) -> JJson {
            JJson::Str(v.to_string())
        }
        pub fn is_json_object(&self) -> bool {
            matches!(self, JJson::Obj(_))
        }
        pub fn is_json_array(&self) -> bool {
            matches!(self, JJson::Arr(_))
        }
        pub fn is_json_null(&self) -> bool {
            matches!(self, JJson::Null)
        }
        pub fn is_json_primitive(&self) -> bool {
            matches!(self, JJson::Bool(_) | JJson::Num(_) | JJson::Str(_))
        }
        pub fn is_string(&self) -> bool {
            matches!(self, JJson::Str(_))
        }
        pub fn is_number(&self) -> bool {
            matches!(self, JJson::Num(_))
        }
        pub fn is_boolean(&self) -> bool {
            matches!(self, JJson::Bool(_))
        }
        /// getAsJsonObject() / getAsJsonArray(): Java throws IllegalStateException on the wrong kind.
        pub fn as_object(&self) -> JJson {
            assert!(self.is_json_object(), "Not a JSON Object: {self}");
            self.clone()
        }
        pub fn as_array(&self) -> JJson {
            assert!(self.is_json_array(), "Not a JSON Array: {self}");
            self.clone()
        }
        fn members(&self) -> &Vec<(String, JJson)> {
            match self {
                JJson::Obj(m) => m,
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        /// The (name, value) pairs of an object, as entrySet() walks them.
        pub fn entries(&self) -> impl Iterator<Item = (&String, &JJson)> {
            self.members().iter().map(|(k, v)| (k, v))
        }
        pub fn keys(&self) -> impl Iterator<Item = &String> {
            self.members().iter().map(|(k, _)| k)
        }
        pub fn elements(&self) -> impl Iterator<Item = &JJson> {
            match self {
                JJson::Arr(a) => a.iter(),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        pub fn has(&self, k: &str) -> bool {
            self.members().iter().any(|(n, _)| n == k)
        }
        /// JsonObject.get: None (null) when absent.
        pub fn get(&self, k: &str) -> Option<JJson> {
            self.members().iter().find(|(n, _)| n == k).map(|(_, v)| v.clone())
        }
        /// JsonObject.getAsJsonObject(name) / getAsJsonArray(name): null when absent, a cast that throws on the wrong kind.
        pub fn member_object(&self, k: &str) -> Option<JJson> {
            self.get(k).map(|v| v.as_object())
        }
        pub fn member_array(&self, k: &str) -> Option<JJson> {
            self.get(k).map(|v| v.as_array())
        }
        /// JsonArray.get(i).
        pub fn at(&self, i: i32) -> JJson {
            match self {
                JJson::Arr(a) => a.get(i as usize).cloned().unwrap_or_else(|| panic!("Index {i} out of bounds for length {}", a.len())),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        pub fn size(&self) -> i32 {
            match self {
                JJson::Arr(a) => a.len() as i32,
                JJson::Obj(m) => m.len() as i32,
                _ => panic!("size() of a JSON primitive"),
            }
        }
        pub fn is_empty(&self) -> bool {
            self.size() == 0
        }
        /// The single value of a one-element array, as Gson's JsonArray.getAsX does.
        fn single(&self) -> &JJson {
            match self {
                JJson::Arr(a) if a.len() == 1 => &a[0],
                JJson::Arr(_) => panic!("Array must have size 1"),
                other => other,
            }
        }
        pub fn as_string(&self) -> String {
            match self.single() {
                JJson::Str(s) => s.clone(),
                JJson::Num(n) => n.clone(),
                JJson::Bool(b) => b.to_string(),
                other => panic!("UnsupportedOperationException: {other} as a string"),
            }
        }
        fn number_text(&self) -> String {
            match self.single() {
                JJson::Num(n) | JJson::Str(n) => n.trim().to_string(),
                other => panic!("UnsupportedOperationException: {other} as a number"),
            }
        }
        pub fn as_f64(&self) -> f64 {
            let t = self.number_text();
            t.parse().unwrap_or_else(|_| panic!("NumberFormatException: {t}"))
        }
        pub fn as_float(&self) -> f32 {
            self.as_f64() as f32
        }
        pub fn as_double(&self) -> f64 {
            self.as_f64()
        }
        /// getAsInt / getAsLong: a whole number, else the number cut to one.
        pub fn as_long(&self) -> i64 {
            let t = self.number_text();
            t.parse::<i64>().unwrap_or_else(|_| t.parse::<f64>().map(|f| f as i64).unwrap_or_else(|_| panic!("NumberFormatException: {t}")))
        }
        pub fn as_int(&self) -> i32 {
            self.as_long() as i32
        }
        pub fn as_bool(&self) -> bool {
            match self.single() {
                JJson::Bool(b) => *b,
                // Gson: Boolean.parseBoolean of the text
                other => other.as_string().eq_ignore_ascii_case("true"),
            }
        }
        pub fn add(&mut self, k: &str, v: JJson) {
            match self {
                JJson::Obj(m) => {
                    m.retain(|(n, _)| n != k);
                    m.push((k.to_string(), v));
                }
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        pub fn remove(&mut self, k: &str) -> Option<JJson> {
            match self {
                JJson::Obj(m) => {
                    let i = m.iter().position(|(n, _)| n == k)?;
                    Some(m.remove(i).1)
                }
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        pub fn push(&mut self, v: JJson) {
            match self {
                JJson::Arr(a) => a.push(v),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        fn write(&self, out: &mut String, pretty: bool, html: bool, depth: usize) {
            let nl = |out: &mut String, d: usize| {
                if pretty {
                    out.push('\n');
                    out.push_str(&"  ".repeat(d));
                }
            };
            match self {
                JJson::Null => out.push_str("null"),
                JJson::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
                JJson::Num(n) => out.push_str(n),
                JJson::Str(s) => j_json_quote(s, html, out),
                JJson::Arr(a) => {
                    out.push('[');
                    for (i, v) in a.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        nl(out, depth + 1);
                        v.write(out, pretty, html, depth + 1);
                    }
                    if !a.is_empty() {
                        nl(out, depth);
                    }
                    out.push(']');
                }
                JJson::Obj(m) => {
                    out.push('{');
                    for (i, (k, v)) in m.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        nl(out, depth + 1);
                        j_json_quote(k, html, out);
                        out.push(':');
                        if pretty {
                            out.push(' ');
                        }
                        v.write(out, pretty, html, depth + 1);
                    }
                    if !m.is_empty() {
                        nl(out, depth);
                    }
                    out.push('}');
                }
            }
        }
    }
    
    fn j_json_quote(s: &str, html: bool, out: &mut String) {
        out.push('"');
        for c in s.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                '\u{8}' => out.push_str("\\b"),
                '\u{c}' => out.push_str("\\f"),
                '<' | '>' | '&' | '=' | '\'' if html => out.push_str(&format!("\\u{:04x}", c as u32)),
                '\u{2028}' | '\u{2029}' => out.push_str(&format!("\\u{:04x}", c as u32)),
                c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('"');
    }
    
    /// JsonElement.toString: compact, without Gson's HTML escaping.
    impl std::fmt::Display for JJson {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut out = String::new();
            self.write(&mut out, false, false, 0);
            f.write_str(&out)
        }
    }
    
    /// A Gson (from `new Gson()` or a GsonBuilder): how it writes.
    #[derive(Clone, Copy, Debug)]
    pub struct JGson {
        pub pretty: bool,
        pub html: bool,
        pub nulls: bool,
    }
    
    impl Default for JGson {
        fn default() -> Self {
            JGson { pretty: false, html: true, nulls: false }
        }
    }
    
    impl JGson {
        /// Gson.toJson.
        pub fn to_json(&self, v: &JJson) -> String {
            let mut out = String::new();
            self.strip(v).write(&mut out, self.pretty, self.html, 0);
            out
        }
        /// Without serializeNulls, Gson leaves out null fields of objects.
        fn strip(&self, v: &JJson) -> JJson {
            match v {
                JJson::Obj(m) => JJson::Obj(m.iter().filter(|(_, x)| self.nulls || !x.is_json_null()).map(|(k, x)| (k.clone(), self.strip(x))).collect()),
                JJson::Arr(a) => JJson::Arr(a.iter().map(|x| self.strip(x)).collect()),
                other => other.clone(),
            }
        }
    }
    
    #[cfg(test)]
    mod jjson_tests {
        use super::*;
    
        #[test]
        fn read_and_write() {
            let j = JJson::parse("{\"a\": 1, \"b\": [1.5, \"x\", true, null], \"c\": {\"d\": -2e3}}");
            assert!(j.has("a") && !j.has("z"));
            assert_eq!(j.get("a").unwrap().as_int(), 1);
            assert_eq!(j.member_array("b").unwrap().at(0).as_float(), 1.5);
            assert_eq!(j.member_array("b").unwrap().size(), 4);
            assert_eq!(j.member_object("c").unwrap().get("d").unwrap().as_double(), -2000.0);
            assert_eq!(j.get("z"), None);
            assert_eq!(j.to_string(), "{\"a\":1,\"b\":[1.5,\"x\",true,null],\"c\":{\"d\":-2e3}}");
            let g = JGson { pretty: true, ..JGson::default() };
            assert_eq!(g.to_json(&JJson::parse("{\"a\":[1,2],\"b\":{},\"n\":null,\"s\":\"<x>\"}")),
                       "{\n  \"a\": [\n    1,\n    2\n  ],\n  \"b\": {},\n  \"s\": \"\\u003cx\\u003e\"\n}");
        }
    
        #[test]
        fn java_numbers() {
            assert_eq!(j_java_num(1.0, true), "1.0");
            assert_eq!(j_java_num(0.1, true), "0.1");
            assert_eq!(j_java_num(1.0e-4, false), "1.0E-4");
            assert_eq!(j_java_num(12345678.0, false), "1.2345678E7");
            assert_eq!(j_java_num(-0.75, false), "-0.75");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "godwand");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
