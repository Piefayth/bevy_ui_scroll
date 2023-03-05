pub mod components;
pub mod scroll_bar;
pub mod scroll_content;
pub mod scroll_indicator;
pub mod scroll_container;
pub mod styles;

use scroll_bar::*;
use scroll_content::*;
use scroll_indicator::*;
use scroll_container::*;

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy, SystemSet)]
pub enum ScrollSystemSet {
    Create,
    Interact,
    Update,
    Constrain,
    Propagate,
    Extract,
}

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PropagateSchedule;

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

pub struct UiScrollPlugin;

impl Plugin for UiScrollPlugin {
    fn build(&self, app: &mut App) {
        use ScrollSystemSet::*;

        app
            .add_schedule(PropagateSchedule, Schedule::new())
            .configure_sets((
                Create,
                Interact,
                Update,
                Constrain,
                Propagate,
                Extract
            ).chain())
            // .add_system(
            //     run_ui_propagate_schedule.in_set(Propagate)
            // )
            .add_plugin(ScrollBarPlugin)
            .add_plugin(ScrollContentPlugin)
            .add_plugin(ScrollIndicatorPlugin)
            .add_plugin(ScrollContainerPlugin);
    }
}

pub fn run_ui_propagate_schedule(world: &mut World) {
    // let mut schedules = world.resource_mut::<Schedules>();

    // let propagate_schedule = schedules
    //     .get(&PropagateSchedule)
    //     .unwrap_or_else(|| panic!(""));

    // for system in propagate_schedule.graph().systems() {
    //     system.1.get_last_change_tick();
    // }

    // let before_schedule_tick = world.change_tick();
    //    world.run_schedule(PropagateSchedule);
    // let after_schedule_tick = world.change_tick();
    // for entity in world.iter_entities() {
    //     //entity.get_change_ticks::<ScrollBarWidget>().unwrap().is_changed(before_schedule_tick, after_schedule_tick);
    // }
}
