'use strict'


function Client () {
  this.install = function (host) {
    console.info('Client', 'Installing..')

    this.renderer = new Renderer(this)
    
    host.appendChild(this.renderer.el);

    // host.appendChild(this.renderer.el)

    document.addEventListener('keypress', (e) => { this.handleKeyPress(e) }, false );
    // document.addEventListener('mousedown', (e) => { this.cursor.down(e) }, false)
    // document.addEventListener('mousemove', (e) => { this.cursor.move(e) }, false)
    // document.addEventListener('contextmenu', (e) => { this.cursor.alt(e) }, false)
    // document.addEventListener('mouseup', (e) => { this.cursor.up(e) }, false)
    // document.addEventListener('copy', (e) => { this.copy(e) }, false)
    // document.addEventListener('cut', (e) => { this.cut(e) }, false)
    // document.addEventListener('paste', (e) => { this.paste(e) }, false)
    // window.addEventListener('resize', (e) => { this.onResize() }, false)
    // window.addEventListener('dragover', (e) => { e.stopPropagation(); e.preventDefault(); e.dataTransfer.dropEffect = 'copy' })
    // window.addEventListener('drop', this.onDrop)
  }

  this.start = () => {
    console.log('Client', 'Starting..')

    this.renderer.start()

    setInterval(() => { this.update() }, 33) // redraw at 30hz
    setTimeout(() => { document.body.className += ' ready' }, 250)
  }

  this.update = () => {
    console.log('Client', 'Update...')
    this.renderer.update()
  }

  this.clear = () => {
  }

  this.reset = () => {
  }

  this.whenOpen = (file, data) => {
  }

  this.getPadding = () => {
    return { x: 60, y: 90 }
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
    }
    else if (e.key === "f") {
    } 
  }

  function sizeOffset (a, b) { return { width: a.width - b.width, height: a.height - b.height } }
  function step (v, s) { return Math.round(v / s) * s }
}
