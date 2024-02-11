
console.log("we are working");

class Sudoku extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        this.addEventListener("click", this.handleCellClick.bind(this))
    }

    handleCellClick(ev) {
        const target = ev.target;
        if (target.classList.contains("cell")) {
            target.classList.toggle("active");
        }
    }

    disconnectedCallback() {
        this.removeEventListener("click", this.handleCellClick.bind(this));
    }

}

customElements.define("sudoku-board", Sudoku);

document.addEventListener("DOMContentLoaded", () => {
    console.log("we are loaded");
})
