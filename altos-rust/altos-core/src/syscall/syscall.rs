/*
* Copyright (C) 2017 AltOS-Rust Team
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

//pub const SYS_NEW_TASK: u32 = 0;
pub const SYS_EXIT: u32 = 1;
pub const SYS_SCHED_YIELD: u32 = 2;
pub const SYS_SLEEP: u32 = 3;
pub const SYS_SLEEP_FOR: u32 = 4;
pub const SYS_WAKE: u32 = 5;
//pub const SYS_TICK: u32 = 6;
pub const SYS_MX_LOCK: u32 = 7;
pub const SYS_MX_TRY_LOCK: u32 = 8;
pub const SYS_MX_UNLOCK: u32 = 9;
pub const SYS_CV_WAIT: u32 = 10;
pub const SYS_CV_BROADCAST: u32 = 11;

pub enum SystemCall {
    //NewTask,
    Exit,
    SchedYield,
    Sleep,
    SleepFor,
    Wake,
    //SystemTick,
    MutexLock,
    MutexTryLock,
    MutexUnlock,
    CondVarWait,
    CondVarBroadcast,
}

impl SystemCall {
    pub fn call_number(&self) -> u32 {
        use self::SystemCall::*;

        match *self {
            //NewTask => SYS_NEW_TASK,
            Exit => SYS_EXIT,
            SchedYield => SYS_SCHED_YIELD,
            Sleep => SYS_SLEEP,
            SleepFor => SYS_SLEEP_FOR,
            Wake => SYS_WAKE,
            //SystemTick => SYS_TICK,
            MutexLock => SYS_MX_LOCK,
            MutexTryLock => SYS_MX_TRY_LOCK,
            MutexUnlock => SYS_MX_UNLOCK,
            CondVarWait => SYS_CV_WAIT,
            CondVarBroadcast => SYS_CV_BROADCAST,
        }
    }
}
