'use strict'

function Client () {
  this.install = function (host) {
    console.info('Client', 'Installing..')

    this.renderer = new Renderer(this)

    // add canvas to the DOM
    host.appendChild(this.renderer.el);

    document.addEventListener('keypress', (e) => { handleKeyPress(e) }, false );
    document.addEventListener('mousedown', (e) => mouseDown(e, this), false)
    document.addEventListener('mousemove', (e) => { }, false)
    document.addEventListener('mouseup', (e) => mouseUp(e, this), false)


    //window.addEventListener('resize', (e) => { this.onResize() }, false)
    window.addEventListener('dragover', (e) => {  })
  }

  this.start = () => {
    console.log('Client', 'Starting..')

    this.renderer.start()

    setInterval(() => { this.update() }, 33) // redraw at 30hz
    setTimeout(() => { document.body.className += ' ready' }, 250)
  }

  this.update = () => {
    this.renderer.update()
  }

  this.clear = () => {
  }

  this.reset = () => {
  }

  this.getPadding = () => {
    return { x: 0, y: 0 }
  }

  this.getWindowSize = () => {
    return { width: window.innerWidth, height: window.innerHeight }
  }

  this.getProjectSize = () => {
    return this.tool.settings.size
  }

  this.getPaddedSize = () => {
    const rect = this.getWindowSize()
    const pad = this.getPadding()
    return { width: step(rect.width - pad.x, 15), height: step(rect.height - pad.y, 15) }
  }

  this.handleKeyPress = (e) => {
    if (e.key === "q") {
      alert("hello");
    }
    else if (e.key === "a") {

    }
    else if (e.key === "w") {

    }
    else if (e.key === "s") {

    }
    else if (e.key === "e") {

    }
    else if (e.key === "d") {

    }
    else if (e.key === "r") {

    }    else if (e.key === "f") {

    }
  }
  var hold = false
  function mouseDown(e, client) {
    hold = true
    client.renderer.hold = hold
    buttonPresses(e, client);
  }

  function mouseUp(e, client){
    hold = false
    client.renderer.hold = hold
    buttonUnpresses(client);
  }




  function sizeOffset (a, b) { return { width: a.width - b.width, height: a.height - b.height } }
  function step (v, s) { return Math.round(v / s) * s }
}
