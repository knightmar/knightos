use crate::backend::multitasking::TaskState::READY;
use crate::log;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::arch::global_asm;
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
    UNINITIALIZED,
}

#[repr(C)]
pub struct CpuContext {
    edi: u32,
    esi: u32,
    ebx: u32,
    ebp: u32,
    eip: u32,
}

pub static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());

pub struct Scheduler {
    tasks: [Option<Task>; 4],
    current_task_index: usize,
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

    pub(crate) fn switch_next_pointers(&mut self) -> Option<(*mut u32, u32)> {
        let current_idx = self.current_task_index;
        let mut next_idx = None;

        for offset in 1..=self.tasks.len() {
            let idx = (current_idx + offset) % self.tasks.len();
            if let Some(task) = &self.tasks[idx] {
                if task.state == READY {
                    next_idx = Some(idx);
                    break;
                }
            }
        }

        let next_idx = next_idx?;
        self.current_task_index = next_idx;

        let current = self.tasks[current_idx].as_mut().unwrap();
        if current.state == TaskState::UNINITIALIZED {
            current.state = TaskState::READY;
        }

        let old_esp_ptr = &mut self.tasks[current_idx].as_mut().unwrap().esp as *mut u32;
        let new_esp = self.tasks[next_idx].as_ref().unwrap().esp;

        Some((old_esp_ptr, new_esp))
    }

    pub fn yield_now() {
        unsafe { core::arch::asm!("cli") }; // disable interrupts during switch
        let mut scheduler = SCHEDULER.lock();
        if let Some((old_esp, new_esp)) = scheduler.switch_next_pointers() {
            core::mem::drop(scheduler);
            unsafe {
                context_switch(old_esp, new_esp);
            }
        }

        unsafe { core::arch::asm!("sti") };
    }
}

unsafe extern "C" {
    fn context_switch(old_esp: *mut u32, new_esp: u32);
}
#[rustfmt::skip]
global_asm!(
    ".global context_switch",
    "context_switch:",
    "   push ebp",
    "   push ebx",
    "   push esi",
    "   push edi",
    "   mov eax,[esp + 20]",
    "   mov [eax],esp",
    "",
    "   mov esp, [esp + 24]",

    "   pop edi",
    "   pop esi",
    "   pop ebx",
    "   pop ebp",
    "   ret",
);

pub fn create_task(func: fn(), page_dir: u32) -> Task {
    let mut stack: Vec<u8> = vec![0; 4096];
    let stack_top = (stack.as_mut_ptr() as u32) + 4096u32;
    let context_ptr = stack_top - size_of::<CpuContext>() as u32;
    let context = CpuContext {
        edi: 0,
        esi: 0,
        ebx: 0,
        ebp: 0,
        eip: func as u32,
    };

    unsafe {
        *(context_ptr as *mut CpuContext) = context;
    }

    Task {
        id: 0,
        esp: context_ptr,
        cr3: page_dir,
        state: TaskState::READY,
        stack,
    }
}
