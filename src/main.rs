use rocket::{get, post, launch, routes, serde::{json::Json, Serialize}, fs::FileServer};
use lazy_static::lazy_static;
use std::sync::Mutex;

// Define memory block structure
#[derive(Serialize, Clone)]
struct MemoryBlock {
    id: u32,
    size: u32,
    allocated: bool,
    address: usize,
}

// Global Mutex to store memory blocks
lazy_static! {
    static ref MEMORY: Mutex<Vec<MemoryBlock>> = Mutex::new({
        let mut address_counter = 0x1000; // Starting memory address
        let mut blocks = vec![
            MemoryBlock { id: 1, size: 16, allocated: false, address: 0 },
            MemoryBlock { id: 2, size: 32, allocated: false, address: 0 },
            MemoryBlock { id: 3, size: 64, allocated: false, address: 0 },
            MemoryBlock { id: 4, size: 128, allocated: false, address: 0 },
            MemoryBlock { id: 5, size: 256, allocated: false, address: 0 }
        ];
        for block in &mut blocks {
            block.address = address_counter;
            address_counter += block.size as usize; // Increment address for next block
        }
        blocks
    });
}

// Route to get memory status
#[get("/status")]
fn status() -> Json<Vec<MemoryBlock>> {
    let memory = MEMORY.lock().unwrap();
    Json(memory.clone())
}

// GET Route to allocate memory (allocates first free block)
#[post("/allocate")]
fn allocate() -> Json<Option<MemoryBlock>> {
    let mut memory = MEMORY.lock().unwrap();
    if let Some(block) = memory.iter_mut().find(|b| !b.allocated) {
        block.allocated = true;
        return Json(Some(block.clone()));
    }
    Json(None) // No available memory blocks
}

// GET Route to free memory (by ID)
#[post("/free/<id>")]
fn free(id: u32) -> Json<bool> {
    let mut memory = MEMORY.lock().unwrap();
    if let Some(block) = memory.iter_mut().find(|b| b.id == id) {
        block.allocated = false;
        return Json(true);
    }
    Json(false) // Block ID not found
}

#[post("/freeAll")]
fn free_all() -> Json<Vec<MemoryBlock>> {
    let mut memory = MEMORY.lock().unwrap();
    for block in memory.iter_mut() {
        block.allocated = false;
    }
    Json(memory.clone())
}

#[post("/reset_memory")]
fn reset_memory() -> Json<Vec<MemoryBlock>> {
    let mut memory = MEMORY.lock().unwrap();
    let mut address_counter = 0x1000; // Reset address counter
    *memory = vec![
        MemoryBlock { id: 1, size: 16, allocated: false, address: 0 },
        MemoryBlock { id: 2, size: 32, allocated: false, address: 0 },
        MemoryBlock { id: 3, size: 64, allocated: false, address: 0 },
        MemoryBlock { id: 4, size: 128, allocated: false, address: 0 },
        MemoryBlock { id: 5, size: 256, allocated: false, address: 0 }
    ];
    for block in memory.iter_mut() {
        block.address = address_counter;
        address_counter += block.size as usize;
    }
    Json(memory.clone())
}

// Launch Rocket server
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("static"))
        .mount("/api", routes![status, allocate, free, free_all, reset_memory])
}
