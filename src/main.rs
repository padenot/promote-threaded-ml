
use audio_thread_priority::*;
use std::env;


fn main() {
    let argz: Vec<String> = env::args().into_iter().collect::<Vec<String>>();
    println!("Attempting to promote pid: {}, tid: {}", argz[1], argz[2]);
    let pid : i32 = argz[1].parse::<i32>().unwrap();
    let tid : i64 = argz[2].parse::<i64>().unwrap();
    let a = audio_thread_priority::RtPriorityThreadInfo::new(pid, tid);
    let rv = promote_thread_to_real_time(a, 2048, 48000);
    match rv {
        Ok(_) => {
            println!("Ok, thread {} of process {} is now real-time", tid, pid);
        },
        Err(e) => {
            eprint!("Err: {}", e.to_string());
        }
    }
}
