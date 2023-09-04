pub static mut g_failure_reason: &str = "";

pub unsafe fn err(s: &'static str) -> i32 {
    g_failure_reason = s;
    return 0;
}