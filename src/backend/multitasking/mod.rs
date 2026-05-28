use crate::backend::multitasking::TaskState::READY;
use crate::log;
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
}

#[repr(C, packed)]
pub struct NewTaskFrame {
    // Layout matching 'pusha' (pushed manually in our assembly ISR)
    edi: u32,
    esi: u32,
    ebp: u32,
    esp: u32, // Ignored by popa, but holds space
    ebx: u32,
    edx: u32,
    ecx: u32,
    eax: u32,
    // Hardware frame (Pushed automatically by CPU on an interrupt)
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

    /// Executed on every timer tick. Saves current stack pointer,
    /// finds next ready task, and returns the new stack pointer.
    pub(crate) fn switch_next(&mut self, current_esp: u32) -> u32 {
        let current_idx = self.current_task_index;

        // Save the stack pointer of the task that was just running
        if let Some(task) = &mut self.tasks[current_idx] {
            task.esp = current_esp;
        }

        // Round-robin selection for the next task
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

// Low-level context hooks called by Assembly/Hardware
unsafe extern "C" {
    pub fn launch_first_task(new_esp: u32) -> !;
    pub fn timer_interrupt_handler();
}

#[unsafe(no_mangle)]
pub extern "C" fn handle_preemptive_switch(current_esp: u32) -> u32 {
    SCHEDULER.lock().switch_next(current_esp)
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
        "iret",
        in(reg) first_esp,
        options(noreturn)
        )
    }
}

global_asm!(
    ".global timer_interrupt_entry",
    "timer_interrupt_entry:",
    "    pusha",


    "    mov ax, 0x10",
    "    mov ds, ax",
    "    mov es, ax",
    "    mov fs, ax",
    "    mov gs, ax",

    "    mov eax, esp",
    "    push eax",
    "    call timer_handler_inner",
    "    add esp, 4",
    "    mov esp, eax",
    "    popa",
    "    iret",
);

pub fn create_task(entry_point: fn(), id: usize) -> Task {
    let stack_size = 8192; // 8 Ko
    let mut stack = vec![0u8; stack_size];

    let stack_ptr = stack.as_mut_ptr() as u32;
    let stack_bottom = (stack_ptr + stack_size as u32) & !0xF; // Alignement sur 16 octets
    let mut esp = stack_bottom;

    unsafe {
        // --- Trame IRET stricte pour le Ring 0 (3 éléments SEULEMENT) ---
        esp -= 4; *(esp as *mut u32) = 0x202;      // EFLAGS (Interrupts activées)
        esp -= 4; *(esp as *mut u32) = 0x08;       // CS (Kernel Code)
        esp -= 4; *(esp as *mut u32) = entry_point as u32; // EIP

        // --- Trame POPA ---
        esp -= 4; *(esp as *mut u32) = 0;          // EAX
        esp -= 4; *(esp as *mut u32) = 0;          // ECX
        esp -= 4; *(esp as *mut u32) = 0;          // EDX
        esp -= 4; *(esp as *mut u32) = 0;          // EBX
        esp -= 4; *(esp as *mut u32) = 0;          // ESP (ignoré par popa)
        esp -= 4; *(esp as *mut u32) = 0;          // EBP
        esp -= 4; *(esp as *mut u32) = 0;          // ESI
        esp -= 4; *(esp as *mut u32) = 0;          // EDI
    }

    let current_cr3: u32;
    unsafe {
        core::arch::asm!("mov {}, cr3", out(reg) current_cr3);
    }

    Task {
        id: id as u32,
        esp,
        cr3: current_cr3,
        state: TaskState::READY,
        stack,
    }
}