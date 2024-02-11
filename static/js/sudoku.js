import { emitter, events } from "./global";

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

class Numbers extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        this.addEventListener("click", this.handleCellClick.bind(this))
    }

    handleCellClick(ev) {
        const target = ev.target;
        if (target.classList.contains("cell")) {
            const num = target.text;
            console.log("t", target, target.InnerHTML);
            console.log("selected", num);
            target.classList.toggle("active");
            emitter.emit(events.SELECT_NUM, num);
        }
    }

    disconnectedCallback() {
        this.removeEventListener("click", this.handleCellClick.bind(this));
    }

}

customElements.define("sudoku-numbers", Numbers);

document.addEventListener("DOMContentLoaded", () => {
    console.log("we are loaded");
})
