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

/// System call number for `exit(void)`
pub const SYS_EXIT: u32 = 0;

/// System call number for `sched_yield(void)`
pub const SYS_SCHED_YIELD: u32 = 1;

/// System call number for `sleep(wchan)`
pub const SYS_SLEEP: u32 = 2;

/// System call number for `sleep_for(wchan, delay)`
pub const SYS_SLEEP_FOR: u32 = 3;

/// System call number for `wake(wchan)`
pub const SYS_WAKE: u32 = 4;

/// System call number for `mutex_lock(lock)`
pub const SYS_MX_LOCK: u32 = 5;

/// System call number for `mutex_try_lock(lock)`
pub const SYS_MX_TRY_LOCK: u32 = 6;

/// System call number for `mutex_unlock(lock)`
pub const SYS_MX_UNLOCK: u32 = 7;

/// System call number for `condvar_wait(condvar, lock)`
pub const SYS_CV_WAIT: u32 = 8;

/// System call number for `condvar_broadcast(lock)`
pub const SYS_CV_BROADCAST: u32 = 9;
