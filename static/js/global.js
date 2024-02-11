
const emitter = mitt();
console.log("emitter", emitter);

const store = {
    nextNum: null, 
}

const events = {
    SELECT_NUM: 'SELECT_NUM'
}

emitter.on(events.SELECT_NUM, (ev) => {
    console.log('ev', ev);
})

export { emitter, events };

