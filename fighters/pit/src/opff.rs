use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn upperdash_arm_jump_and_aerial_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32, id: usize) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S || (status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END && frame > 5.0) {
        if frame > 27.0 {
            if moveset_utils::jump_checker_buffer(boma, cat1) {
                if situation_kind == *SITUATION_KIND_AIR {
                    if hdr::get_jump_count(boma) < hdr::get_jump_count_max(boma) - 1 &&  !VarModule::is_flag(boma.object(), vars::common::SIDE_SPECIAL_CANCEL) {
                        VarModule::on_flag(boma.object(), vars::common::SIDE_SPECIAL_CANCEL);
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                    }
                } else if situation_kind == *SITUATION_KIND_GROUND {
                    if facing * stick_x < 0.0 {
                        PostureModule::reverse_lr(boma);
                        // Does this need PostureModule::update_rot_y_lr(boma)?
                    }
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
    }
}


pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    upperdash_arm_jump_and_aerial_cancel(boma, status_kind, situation_kind, cat[0], stick_x, facing, frame, id);
    pits::moveset(boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
}

#[utils::opff(FIGHTER_KIND_PIT )]
pub fn pit_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		pit_frame(fighter)
    }
}

pub unsafe fn pit_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}