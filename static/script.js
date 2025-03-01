let memory = [
    { id: 1, size: 16, allocated: false },
    { id: 2, size: 32, allocated: false },
    { id: 3, size: 64, allocated: false },
    { id: 4, size: 128, allocated: false },
    { id: 5, size: 256, allocated: false }
];

function updateMemoryDisplay() {
    fetch("/api/status")
        .then(response => response.json())
        .then(apiMemory => {
            const container = document.getElementById("memory-container");
            container.innerHTML = "";

            memory.forEach(localBlock => {
                let apiBlock = apiMemory.find(b => b.id === localBlock.id);
                if (apiBlock) {
                    localBlock.allocated = apiBlock.allocated; // Sync allocation status
                }

                const div = document.createElement("div");
                div.classList.add("memory-block", localBlock.allocated ? "allocated" : "free");
                div.innerHTML = `<strong>${localBlock.size}B</strong><br>Addr: 0x${(localBlock.address || 0).toString(16)}`;
                div.onclick = () => showBlockDetails(localBlock);
                container.appendChild(div);
            });

            updateStatusBar();
        });
}

function allocateMemory() {
    let block = memory.find(b => !b.allocated);
    if (block) {
        block.allocated = true;
        
        // Send update to server
        fetch("/api/allocate", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ id: block.id })
        }).then(updateMemoryDisplay);

        document.getElementById("status").textContent = "Status: Memory allocated!";
    } else {
        document.getElementById("status").textContent = "Status: No available memory!";
    }
}

function freeMemory(id) {
    fetch(`/api/free/${id}`, { method: "POST" })  
        .then(response => response.json())
        .then(apiMemory => {
            memory.forEach(localBlock => {
                if (localBlock.id === id) {
                    localBlock.allocated = false;
                }
            });
            document.getElementById("status").textContent = `Status: Memory block ${id} freed`;
            updateMemoryDisplay();
        });
}

function freeAllMemory() {
    fetch(`/api/freeAll`, { method: "POST" })  
        .then(response => response.json())
        .then(memory => {
            memory.forEach(localBlock => {
                localBlock.allocated = false;
            });
            document.getElementById("status").textContent = `Status: All memory freed`;
            updateMemoryDisplay();
        })
        .catch(error => console.error("Error freeing all memory:", error));
}

function resetMemory() {
    fetch("/api/reset_memory", { method: "POST" })  // Corrected endpoint
        .then(response => response.json())
        .then(apiMemory => {
            memory.forEach(localBlock => {
                let apiBlock = apiMemory.find(b => b.id === localBlock.id);
                if (apiBlock) {
                    localBlock.allocated = apiBlock.allocated;
                    localBlock.address = apiBlock.address;
                }
            });
            document.getElementById("status").textContent = "Status: Memory reset!";
            updateMemoryDisplay();
        });
}

function showBlockDetails(block) {
    const details = document.getElementById("details-info");
    details.innerHTML = `
        <strong>ID:</strong> ${block.id} <br>
        <strong>Size:</strong> ${block.size} KB <br>
        <strong>Status:</strong> ${block.allocated ? "Allocated" : "Free"} <br>
        ${block.allocated ? `<button onclick="freeMemory(${block.id})">Free Memory</button>` : ""}
    `;
}

function updateStatusBar() {
    const allocatedCount = memory.filter(block => block.allocated).length;
    const totalCount = memory.length;
    const usage = (allocatedCount / totalCount) * 100;

    const statusBar = document.getElementById("status-bar");
    statusBar.style.width = usage + "%";
    statusBar.style.backgroundColor = usage > 75 ? "red" : usage > 50 ? "orange" : "green";
}

document.addEventListener("DOMContentLoaded", updateMemoryDisplay);