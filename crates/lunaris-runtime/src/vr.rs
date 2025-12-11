//! VR/AR Support
//!
//! Virtual and Augmented Reality support for all major headsets.

use glam::{Vec3, Quat, Mat4};

/// VR headset type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VRHeadset {
    /// Meta Quest 3
    MetaQuest3,
    /// Meta Quest Pro
    MetaQuestPro,
    /// Apple Vision Pro
    AppleVisionPro,
    /// Valve Index
    ValveIndex,
    /// HTC Vive Pro 2
    HTCVivePro2,
    /// PlayStation VR2
    PSVR2,
    /// HP Reverb G2
    HPReverbG2,
    /// Bigscreen Beyond
    BigscreenBeyond,
    /// Pimax Crystal
    PimaxCrystal,
    /// Generic OpenXR
    GenericOpenXR,
}

impl VRHeadset {
    /// Get resolution per eye
    #[must_use]
    pub fn resolution(&self) -> (u32, u32) {
        match self {
            Self::MetaQuest3 => (2064, 2208),
            Self::MetaQuestPro => (1800, 1920),
            Self::AppleVisionPro => (3660, 3200),
            Self::ValveIndex => (1440, 1600),
            Self::HTCVivePro2 => (2448, 2448),
            Self::PSVR2 => (2000, 2040),
            Self::HPReverbG2 => (2160, 2160),
            Self::BigscreenBeyond => (2560, 2560),
            Self::PimaxCrystal => (2880, 2880),
            Self::GenericOpenXR => (1920, 1920),
        }
    }

    /// Get refresh rate
    #[must_use]
    pub fn refresh_rate(&self) -> u32 {
        match self {
            Self::MetaQuest3 => 120,
            Self::MetaQuestPro => 90,
            Self::AppleVisionPro => 100,
            Self::ValveIndex => 144,
            Self::HTCVivePro2 => 120,
            Self::PSVR2 => 120,
            Self::HPReverbG2 => 90,
            Self::BigscreenBeyond => 90,
            Self::PimaxCrystal => 160,
            Self::GenericOpenXR => 90,
        }
    }

    /// Has hand tracking
    #[must_use]
    pub fn hand_tracking(&self) -> bool {
        matches!(self, 
            Self::MetaQuest3 | Self::MetaQuestPro | Self::AppleVisionPro | Self::PSVR2
        )
    }

    /// Has eye tracking
    #[must_use]
    pub fn eye_tracking(&self) -> bool {
        matches!(self,
            Self::MetaQuestPro | Self::AppleVisionPro | Self::PSVR2 | 
            Self::HTCVivePro2 | Self::PimaxCrystal
        )
    }

    /// Has passthrough
    #[must_use]
    pub fn passthrough(&self) -> bool {
        matches!(self,
            Self::MetaQuest3 | Self::MetaQuestPro | Self::AppleVisionPro
        )
    }
}

/// VR controller hand
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hand {
    Left,
    Right,
}

/// VR controller
#[derive(Debug, Clone)]
pub struct VRController {
    /// Hand
    pub hand: Hand,
    /// Position
    pub position: Vec3,
    /// Rotation
    pub rotation: Quat,
    /// Velocity
    pub velocity: Vec3,
    /// Angular velocity
    pub angular_velocity: Vec3,
    /// Is tracking valid
    pub tracking_valid: bool,
    /// Button states
    pub buttons: VRButtons,
    /// Thumbstick
    pub thumbstick: [f32; 2],
    /// Trigger
    pub trigger: f32,
    /// Grip
    pub grip: f32,
}

/// VR button states
#[derive(Debug, Clone, Copy, Default)]
pub struct VRButtons {
    /// A/X button
    pub a_x: bool,
    /// B/Y button
    pub b_y: bool,
    /// Thumbstick click
    pub thumbstick_click: bool,
    /// Menu button
    pub menu: bool,
    /// Trigger touched
    pub trigger_touch: bool,
    /// Thumbstick touched
    pub thumbstick_touch: bool,
}

/// Hand tracking data
#[derive(Debug, Clone)]
pub struct HandTracking {
    /// Hand
    pub hand: Hand,
    /// Is tracking valid
    pub valid: bool,
    /// Joint positions (25 joints)
    pub joints: Vec<HandJoint>,
    /// Pinch strength (thumb to each finger)
    pub pinch: [f32; 4],
    /// Grab strength
    pub grab: f32,
}

/// Hand joint
#[derive(Debug, Clone, Copy)]
pub struct HandJoint {
    /// Position
    pub position: Vec3,
    /// Rotation
    pub rotation: Quat,
    /// Radius
    pub radius: f32,
}

/// Eye tracking data
#[derive(Debug, Clone)]
pub struct EyeTracking {
    /// Left eye gaze origin
    pub left_origin: Vec3,
    /// Left eye gaze direction
    pub left_direction: Vec3,
    /// Right eye gaze origin
    pub right_origin: Vec3,
    /// Right eye gaze direction
    pub right_direction: Vec3,
    /// Combined gaze origin
    pub combined_origin: Vec3,
    /// Combined gaze direction
    pub combined_direction: Vec3,
    /// Left eye openness (0-1)
    pub left_openness: f32,
    /// Right eye openness (0-1)
    pub right_openness: f32,
    /// Pupil diameter left (mm)
    pub left_pupil_diameter: f32,
    /// Pupil diameter right (mm)
    pub right_pupil_diameter: f32,
    /// Tracking confidence
    pub confidence: f32,
}

/// VR head pose
#[derive(Debug, Clone)]
pub struct VRHeadPose {
    /// Position
    pub position: Vec3,
    /// Rotation
    pub rotation: Quat,
    /// Velocity
    pub velocity: Vec3,
    /// Angular velocity
    pub angular_velocity: Vec3,
    /// Is tracking valid
    pub tracking_valid: bool,
}

/// VR view
#[derive(Debug, Clone)]
pub struct VRView {
    /// Eye offset
    pub eye_offset: Vec3,
    /// Projection matrix
    pub projection: Mat4,
    /// View matrix
    pub view: Mat4,
    /// Field of view (left, right, up, down)
    pub fov: [f32; 4],
}

/// VR session
pub struct VRSession {
    /// Headset type
    pub headset: VRHeadset,
    /// Is active
    pub active: bool,
    /// Head pose
    pub head: VRHeadPose,
    /// Left controller
    pub left_controller: Option<VRController>,
    /// Right controller
    pub right_controller: Option<VRController>,
    /// Left hand tracking
    pub left_hand: Option<HandTracking>,
    /// Right hand tracking
    pub right_hand: Option<HandTracking>,
    /// Eye tracking
    pub eye_tracking: Option<EyeTracking>,
    /// Left eye view
    pub left_view: VRView,
    /// Right eye view
    pub right_view: VRView,
    /// Play space bounds (corners)
    pub play_space: Vec<Vec3>,
    /// Floor height
    pub floor_height: f32,
    /// IPD (mm)
    pub ipd: f32,
}

impl VRSession {
    /// Create new session
    #[must_use]
    pub fn new(headset: VRHeadset) -> Self {
        Self {
            headset,
            active: false,
            head: VRHeadPose {
                position: Vec3::ZERO,
                rotation: Quat::IDENTITY,
                velocity: Vec3::ZERO,
                angular_velocity: Vec3::ZERO,
                tracking_valid: false,
            },
            left_controller: None,
            right_controller: None,
            left_hand: None,
            right_hand: None,
            eye_tracking: None,
            left_view: VRView::default(),
            right_view: VRView::default(),
            play_space: Vec::new(),
            floor_height: 0.0,
            ipd: 63.0,
        }
    }

    /// Get combined view matrix
    #[must_use]
    pub fn combined_view(&self) -> Mat4 {
        Mat4::from_rotation_translation(
            self.head.rotation.inverse(),
            -self.head.position,
        )
    }

    /// Get controller
    #[must_use]
    pub fn controller(&self, hand: Hand) -> Option<&VRController> {
        match hand {
            Hand::Left => self.left_controller.as_ref(),
            Hand::Right => self.right_controller.as_ref(),
        }
    }

    /// Has foveated rendering support
    #[must_use]
    pub fn foveated_rendering(&self) -> bool {
        self.eye_tracking.is_some()
    }
}

impl Default for VRView {
    fn default() -> Self {
        Self {
            eye_offset: Vec3::ZERO,
            projection: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            fov: [-45.0, 45.0, 45.0, -45.0],
        }
    }
}

/// AR session (for passthrough/mixed reality)
pub struct ARSession {
    /// Is active
    pub active: bool,
    /// Camera intrinsics
    pub camera_intrinsics: CameraIntrinsics,
    /// Detected planes
    pub planes: Vec<ARPlane>,
    /// Tracked anchors
    pub anchors: Vec<ARAnchor>,
    /// Light estimation
    pub light_estimate: Option<ARLightEstimate>,
    /// Depth available
    pub depth_available: bool,
    /// Hand occlusion available
    pub hand_occlusion: bool,
}

/// Camera intrinsics
#[derive(Debug, Clone)]
pub struct CameraIntrinsics {
    /// Focal length
    pub focal_length: [f32; 2],
    /// Principal point
    pub principal_point: [f32; 2],
    /// Resolution
    pub resolution: [u32; 2],
}

/// AR plane
#[derive(Debug, Clone)]
pub struct ARPlane {
    /// Unique ID
    pub id: u64,
    /// Center position
    pub center: Vec3,
    /// Rotation
    pub rotation: Quat,
    /// Extents
    pub extents: [f32; 2],
    /// Plane type
    pub plane_type: ARPlaneType,
    /// Vertices
    pub vertices: Vec<Vec3>,
}

/// AR plane type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ARPlaneType {
    Horizontal,
    Vertical,
    Ceiling,
    Wall,
    Floor,
    Table,
    Seat,
    Door,
    Window,
}

/// AR anchor
#[derive(Debug, Clone)]
pub struct ARAnchor {
    /// Unique ID
    pub id: u64,
    /// Position
    pub position: Vec3,
    /// Rotation
    pub rotation: Quat,
    /// Is tracking
    pub tracking: bool,
}

/// AR light estimate
#[derive(Debug, Clone)]
pub struct ARLightEstimate {
    /// Ambient intensity
    pub ambient_intensity: f32,
    /// Ambient color temperature
    pub color_temperature: f32,
    /// Main light direction
    pub main_light_direction: Vec3,
    /// Main light intensity
    pub main_light_intensity: f32,
    /// Spherical harmonics (9 RGB coefficients)
    pub spherical_harmonics: [[f32; 3]; 9],
}
