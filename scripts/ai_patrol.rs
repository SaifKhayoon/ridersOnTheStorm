
use std::collections::HashMap;

pub struct AiPatrol {
  // NPC's current position and orientation
  position: (f32, f32, f32),
  rotation: f32,
  // NPC's patrol path and current waypoint
  path: Vec<(f32, f32, f32)>,
  waypoint_index: usize,
  // NPC's available actions and their associated costs and benefits
  actions: HashMap<Action, (f32, f32)>,
  // NPC's current state and associated values
  state: HashMap<State, f32>,
}

impl AiPatrol {
  pub fn new(path: Vec<(f32, f32, f32)>) -> Self {
    Self {
      position: (0.0, 0.0, 0.0),
      rotation: 0.0,
      path,
      waypoint_index: 0,
      actions: HashMap::new(),
      state: HashMap::new(),
    }
  }

  pub fn update(&mut self) {
    // Check if the NPC has reached the current waypoint
    let waypoint = self.path[self.waypoint_index];
    if self.position == waypoint {
      // Move to the next waypoint if the NPC has reached the current one
      self.waypoint_index = (self.waypoint_index + 1) % self.path.len();
    }

    // Choose the best action based on the NPC's current position and waypoint
    let best_action = self.choose_best_action();

    // Take the chosen action
    self.take_action(best_action);
  }

  fn choose_best_action(&self) -> Action {
    let mut best_action = None;
    let mut best_utility = std::f32::NEG_INFINITY;

    for (action, (cost, benefit)) in &self.actions {
      // Calculate the utility of each action based on the NPC's current position and waypoint
      let utility = self.calculate_utility(cost, benefit);

      if utility > best_utility {
        // Update the best action and utility if this action has a higher utility
        best_action = Some(*action);
        best_utility = utility;
      }
    }

    best_action.unwrap()
  }

  fn take_action(&mut self, action: Action) {
    match action {
      Action::MoveForward => {
        // Move the NPC forward in the direction it is facing
        self.position.0 += self.rotation.cos();
        self.position.1 += self.rotation.sin();
      }
      Action::RotateClockwise => {
        // Rotate the NPC clockwise
        self.rotation += std::f32::consts::PI / 2.0;
      }
      Action::RotateCounterClockwise => {
        // Rotate the NPC counter-clockwise
        self.rotation -= std::f32::consts::PI / 2.0;
      }
      // Add additional actions as needed
      _ => {}
    }
  }
}

enum Action {
  MoveForward,
  RotateClockwise,
  RotateCounterClockwise,
  // Add additional actions as needed
}

enum State {
  DistanceToWaypoint,
  // Add additional states as needed
}
