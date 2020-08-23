'use strict'

/* global Acels */
/* global Theme */
/* global Source */
/* global History */

/* global Manager */
/* global Renderer */
/* global Tool */
/* global Interface */
/* global Picker */
/* global Cursor */

/* global FileReader */

function Client () {
  this.install = function (host) {
    console.info('Client', 'Installing..')

    this.adsr = new ADSR(this)
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

    this.adsr.start()
    this.renderer.start()

    setInterval(() => { this.renderer.update() }, 33) // redraw at 30hz
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
      this.renderer.incParamWave(1)
    }
    else if (e.key === "a") {
      this.renderer.incParamWave(-1)
    }
    else if (e.key === "w") {
      this.renderer.incParamSub(1)
    }
    else if (e.key === "s") {
      this.renderer.incParamSub(-1)
    } 
    else if (e.key === "e") {
      this.renderer.incParamRelation(1)
    }
    else if (e.key === "d") {
      this.renderer.incParamRelation(-1)
    } 
    else if (e.key === "r") {
      this.renderer.incParamFilter(1)
    }
    else if (e.key === "f") {
      this.renderer.incParamFilter(-1)
    } 
  }

  function sizeOffset (a, b) { return { width: a.width - b.width, height: a.height - b.height } }
  function step (v, s) { return Math.round(v / s) * s }
}
