use std::{str, time::Duration};

use mlua::prelude::{Lua, LuaResult, LuaString, LuaTable};

// struct TimerEntry {
//     story_key: String,
//     start_time: String,
// }

// The main function
#[mlua::lua_module]
fn timer(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("start_timer", lua.create_function(start_timer)?)?;
    exports.set("end_timer", lua.create_function(end_timer)?)?;
    Ok(exports)
}

fn start_timer(lua: &Lua, story_key: String) -> LuaResult<LuaString> {
    let foo = lua.create_string(&story_key);
    foo

    // TODO(robin): ask user for story key
    // TODO(robin): enter key value into a hash store. story_key -> current time
}

// TODO(robin): figure out if this function return type is needed
fn end_timer(lua: &Lua, story_key: String) -> LuaResult<LuaString> {
    let foo = lua.create_string(&story_key);

    let _total_time = calculate_worked_time("foo", "baz");
    // TODO(robin): get start_date from hash map with story_key and then compare that start time to
    // the current_time
    foo
}

fn calculate_worked_time(start_time: &str, end_time: &str) -> Duration {
    todo!();
    // TODO(robin): figure out correct input for dates
    // calculate amount of time worked
    // save time to db or file?
}
