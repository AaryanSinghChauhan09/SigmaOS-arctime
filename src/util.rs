//! arctime model — 3D animation/keyframe editor.

#[derive(Debug, Clone)]
pub struct Keyframe {
    pub object: String,
    pub property: String,
    pub time: f64,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub name: String,
    pub keyframes: Vec<Keyframe>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub scenes: Vec<Scene>,
}

impl Default for Model {
    fn default() -> Self { Self { scenes: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_scene(&mut self, scene: Scene) { self.scenes.push(scene); }

    pub fn add_keyframe(&mut self, scene: &str, kf: Keyframe) -> bool {
        if let Some(s) = self.scenes.iter_mut().find(|s| s.name == scene) {
            s.keyframes.push(kf); true
        } else { false }
    }

    pub fn list_scenes(&self) -> &[Scene] { &self.scenes }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scene_keyframe() {
        let mut a = Model::new();
        a.add_scene(Scene { name: "scene1".into(), keyframes: vec![] });
        assert!(a.add_keyframe("scene1", Keyframe { object: "cube".into(), property: "location".into(), time: 0.0, value: "0,0,0".into() }));
        assert_eq!(a.list_scenes()[0].keyframes.len(), 1);
    }
}
