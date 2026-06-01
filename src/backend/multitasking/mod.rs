use crate::backend::multitasking::TaskState::READY;
use crate::log;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::PartialEq;
use spin::Mutex;

pub struct Task {
    pub(crate) id: u32,
    pub(crate) esp: u32,
    pub(crate) cr3: u32,
    pub(crate) state: TaskState,
    pub(crate) stack: Vec<u8>,
}

#[derive(PartialEq)]
pub enum TaskState {
    READY,
    SUSPENDED,
}

#[repr(C, packed)]
pub struct NewTaskFrame {
    edi: u32,
    esi: u32,
    ebp: u32,
    esp: u32, // Ignored by popa, but holds space
    ebx: u32,
    edx: u32,
    ecx: u32,
    eax: u32,
    eip: u32,
    cs: u32,
    eflags: u32,
}

pub static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());

pub struct Scheduler {
    pub(crate) tasks: [Option<Task>; 4],
    pub(crate) current_task_index: usize,
}

impl Scheduler {
    const fn new() -> Scheduler {
        Self {
            tasks: [const { None }; 4],
            current_task_index: 0,
        }
    }

    pub(crate) fn add_task(&mut self, task: Task) -> Result<(), &'static str> {
        if let Some(p) = self.tasks.iter_mut().find(|x| x.is_none()) {
            *p = Some(task);
            Ok(())
        } else {
            Err("Scheduler full")
        }
    }

    /// basic round robin scheduler
    pub(crate) fn switch_next(&mut self, current_esp: u32) -> u32 {
        let current_idx = self.current_task_index;

        if let Some(task) = &mut self.tasks[current_idx] {
            task.esp = current_esp;
        }

        let mut next_idx = current_idx;
        for offset in 1..=self.tasks.len() {
            let idx = (current_idx + offset) % self.tasks.len();
            if let Some(task) = &self.tasks[idx] {
                if task.state == READY {
                    next_idx = idx;
                    break;
                }
            }
        }

        self.current_task_index = next_idx;
        self.tasks[next_idx].as_ref().unwrap().esp
    }
}

pub fn start_scheduler() -> ! {
    let first_esp = {
        let scheduler = SCHEDULER.lock();
        scheduler.tasks[0].as_ref().unwrap().esp
    };

    unsafe {
        core::arch::asm!(
        "mov esp, {0}",
        "popa",
        "iretd",
        in(reg) first_esp,
        options(noreturn)
        )
    }
}

pub fn create_task(entry_point: fn(), id: usize) -> Task {
    log!(Info, "CREATE: entry_point = {:#x}", entry_point as u32);
    let stack_size = 8192; // 8 Ko
    let mut stack = vec![0u8; stack_size];

    let stack_ptr = stack.as_mut_ptr() as u32;
    let stack_bottom = (stack_ptr + stack_size as u32) & !0xF;
    let mut esp = stack_bottom;

    unsafe {
        esp -= 4;
        *(esp as *mut u32) = 0x202;
        esp -= 4;
        *(esp as *mut u32) = 0x08;
        esp -= 4;
        *(esp as *mut u32) = entry_point as u32;

        esp -= 4;
        *(esp as *mut u32) = 0; // EAX
        esp -= 4;
        *(esp as *mut u32) = 0; // ECX
        esp -= 4;
        *(esp as *mut u32) = 0; // EDX
        esp -= 4;
        *(esp as *mut u32) = 0; // EBX
        esp -= 4;
        *(esp as *mut u32) = 0; // ESP
        esp -= 4;
        *(esp as *mut u32) = 0; // EBP
        esp -= 4;
        *(esp as *mut u32) = 0; // ESI
        esp -= 4;
        *(esp as *mut u32) = 0; // EDI
    }

    use crate::backend::serial::LogLevel::Info;
    log!(Info, "CREATE DEBUG: entry_point = {:#x}, prepared esp = {:#x}", entry_point as u32, esp);

    let current_cr3: u32;
    unsafe {
        core::arch::asm!("mov {}, cr3", out(reg) current_cr3);
    }



    Task {
        id: id as u32,
        esp,
        cr3: current_cr3,
        state: READY,
        stack,
    }
}
