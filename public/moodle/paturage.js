class Matrix {
	// matrices are all 4x4, and are initialized as the identity matrix
	// I won't comment on the code here all that much because it's pretty much just computations

	constructor(template) {
		// if we pass a template matrix, copy it
		// otherwise, initialize it to the 4x4 identity matrix

		if (template) {
			this.data = JSON.parse(JSON.stringify(template.data)) // I hate javascript 🙂
			return
		}

		this.data = [
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 0.0, 1.0],
		]
	}

	multiply(left) {
		let right = new Matrix(this)

		for (let i = 0; i < 4; i++) {
			for (let j = 0; j < 4; j++) {
				this.data[i][j] =
					left.data[0][j] * right.data[i][0] +
					left.data[1][j] * right.data[i][1] +
					left.data[2][j] * right.data[i][2] +
					left.data[3][j] * right.data[i][3]
			}
		}
	}

	scale(x, y, z) {
		for (let i = 0; i < 4; i++) {
			this.data[0][i] *= x
			this.data[1][i] *= y
			this.data[2][i] *= z
		}
	}

	translate(x, y, z) {
		for (let i = 0; i < 4; i++) {
			this.data[3][i] +=
				this.data[0][i] * x +
				this.data[1][i] * y +
				this.data[2][i] * z
		}
	}

	rotate(theta, x, y, z) {
		// theta represents the angle we want to rotate by
		// xyz represents the eigenvector of the matrix transformation of the rotation

		// normalize xyz

		let mag = Math.sqrt(x * x + y * y + z * z)

		x /= -mag
		y /= -mag
		z /= -mag

		let s = Math.sin(theta)
		let c = Math.cos(theta)
		let one_minus_c = 1 - c

		let xx = x * x, yy = y * y, zz = z * z
		let xy = x * y, yz = y * z, zx = z * x
		let xs = x * s, ys = y * s, zs = z * s

		let rotation = new Matrix()

		rotation.data[0][0] = (one_minus_c * xx) + c
		rotation.data[0][1] = (one_minus_c * xy) - zs
		rotation.data[0][2] = (one_minus_c * zx) + ys

		rotation.data[1][0] = (one_minus_c * xy) + zs
		rotation.data[1][1] = (one_minus_c * yy) + c
		rotation.data[1][2] = (one_minus_c * yz) - xs

		rotation.data[2][0] = (one_minus_c * zx) - ys
		rotation.data[2][1] = (one_minus_c * yz) + xs
		rotation.data[2][2] = (one_minus_c * zz) + c

		rotation.data[3][3] = 1

		rotation.multiply(this)
		this.data = rotation.data

		//this.multiply(rotation)
	}

	rotate_2d(yaw, pitch) {
		this.rotate(yaw, 0, 1, 0)
		this.rotate(-pitch, Math.cos(yaw), 0, Math.sin(yaw))
	}

	frustum(left, right, bottom, top, near, far) {
		let dx = right - left
		let dy = top - bottom
		let dz = far - near

		// clear out matrix

		for (let i = 0; i < 4; i++) {
			for (let j = 0; j < 4; j++) {
				this.data[i][j] = 0
			}
		}

		this.data[0][0] = 2 * near / dx
		this.data[1][1] = 2 * near / dy

		this.data[2][0] = (right + left) / dx
		this.data[2][1] = (top + bottom) / dy
		this.data[2][2] = -(near + far)  / dz

		this.data[2][3] = -1
		this.data[3][2] = -2 * near * far / dz
	}

	perspective(fovy, aspect, near, far) {
		let y = Math.tan(fovy / 2) / 2
		let x = y / aspect

		this.frustum(-x * near, x * near, -y * near, y * near, near, far)
	}
}

var identity = new Matrix()

class Model {
	// this class handles the buffer creation, rendering, and texturing of models

	constructor(gl, model, tex_path) {
		// gl:       instance of WebGLRenderingContext
		// model:    the model we wanna load (these are simple JS objects located in public/static/models)
		// tex_path: the path to the image we want to use as a texture (these are found in public/static/textures)

		this.model = model

		// create vertex buffer

		this.vbo = gl.createBuffer()
		gl.bindBuffer(gl.ARRAY_BUFFER, this.vbo)
		gl.bufferData(gl.ARRAY_BUFFER, this.model.vertices, gl.STATIC_DRAW)

		// create index buffer

		this.ibo = gl.createBuffer()
		gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.ibo)
		gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, this.model.indices, gl.STATIC_DRAW)

		// load texture

		let tex = gl.createTexture()
		this.tex = tex

		const image = new Image()
		image.src = tex_path

		image.onload = function() {
			// set the contents of the texture object to our loaded image data

			gl.bindTexture(gl.TEXTURE_2D, tex)
			gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGB, gl.RGB, gl.UNSIGNED_BYTE, image)

			// set the minification/magnification filtering of our texture
			// we want to bilinearly interpolate between mipmap levels when minifying, and bilinearly interpolate between textures when magnifying

			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR)
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR)

			// WebGL 1.0 can be picky about non-POT textures, but here, all our textures are guaranteed POT, so we can generate mipmaps with no issue

			gl.generateMipmap(gl.TEXTURE_2D)
		}
	}

	draw(gl, render_state, rot_matrix, model_matrix) {
		// gl:           instance of WebGLRenderingContext
		// render_state: the render_state object
		// model_matrix: the model matrix to use to transform the model

		// bind texture
		// simply, set the active texture slot, pass it to the sampler uniform, and then bind the texture to that slot

		const slot = 0

		gl.activeTexture(gl.TEXTURE0 + slot)
		gl.uniform1i(render_state.sampler_uniform, slot)

		gl.bindTexture(gl.TEXTURE_2D, this.tex)

		// pass the model matrix of our model (so that's like its own translation/rotation/scale) to the model uniform
		// also inverse (only taking into account rotation) for calculating normals

		gl.uniformMatrix4fv(render_state.model_uniform, false, model_matrix.data.flat())
		gl.uniformMatrix4fv(render_state.rot_uniform, false, rot_matrix.data.flat())

		// set buffers up for drawing
		// the attribute layout here is as follows (in total, we use 8 32-bit floats per attribute so 32 bytes total):
		// 0: 3 32-bit floats at offset 0 for the vertex positions
		// 1: 2 32-bit floats at offset 12 for the texture coordinates
		// 2: 3 32-bit floats at offset 20 for the normal vectors

		let float_size = this.model.vertices.BYTES_PER_ELEMENT

		gl.bindBuffer(gl.ARRAY_BUFFER, this.vbo)
		gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.ibo)

		gl.enableVertexAttribArray(render_state.pos_attr)
		gl.enableVertexAttribArray(render_state.tex_coord_attr)
		gl.enableVertexAttribArray(render_state.normal_attr)

		gl.vertexAttribPointer(render_state.pos_attr,       3, gl.FLOAT, gl.FALSE, float_size * 8, float_size * 0)
		gl.vertexAttribPointer(render_state.tex_coord_attr, 2, gl.FLOAT, gl.FALSE, float_size * 8, float_size * 3)
		gl.vertexAttribPointer(render_state.normal_attr,    3, gl.FLOAT, gl.FALSE, float_size * 8, float_size * 5)

		// finally, actually draw the model

		gl.drawElements(gl.TRIANGLES, this.model.indices.length, gl.UNSIGNED_SHORT, 0)
	}
}

const TAU = Math.PI * 2
const GRAVITY = invert_gravity ? 0.3 : -32
const JUMP_HEIGHT = 0.7
const BOUNDS = 2.3
const SHADOW_SIZE = 2
const SPEED = 3 * cow_speed

function abs_min(x, y) {
	if (Math.abs(x) < Math.abs(y)) {
		return x;
	}

	return y;
}

class Cow {
	// an instance of a cow is an individual who kind of jumps around the place aimlessly

	constructor(model, age, shadow) {
		// model:  designated cow model (either this.holstein, this.jersey, or this.bbb)
		// age:    the age of the cow (this controls its size; smaller cows are younger obviously)
		// shadow: the shadow model (simply this.shadow in all cases)

		this.model = model
		this.age = age // controls the size of the cow
		this.shadow = shadow

		// give random initial rotation and position

		this.target_rot = Math.random() * TAU

		this.rot = this.target_rot
		this.pos = [Math.random() * BOUNDS * 2 - BOUNDS, 0, Math.random() * BOUNDS * 2 - BOUNDS]

		// a few physics values

		this.vel = [0, 0, 0]
		this.grounded = false

		this.jump_height = JUMP_HEIGHT
	}

	force_jump(height) {
		this.vel[1] = Math.sqrt(-2 * GRAVITY * height)
	}

	jump() {
		// jump if not grounded

		if (!this.grounded) {
			return
		}

		this.force_jump(this.jump_height)
	}

	update(dt) {
		// dt: time delta between frames (in seconds)
		// "AI" computation

		if (Math.random() < 0.5 * dt) {
			this.jump()
		}

		this.rot += (this.target_rot - this.rot) * dt * 5

		// physics computation
		// really nothing complicated here, formulas come from a video of mine: https://www.youtube.com/watch?v=YG3Gd7Vr93o
		// first, calculate friction/drag coefficients

		// No advertising allowed Mr. Wibo
		// - The FBI & CIA & NSA

		let friction = [1.8, this.vel[1] > 0 ? 0 : 0.4, 1.8]

		if (this.grounded) {
			friction = [3, 3, 3]
		}

		// apply input acceleration & adjust for friction/drag
		// here, we want our cow moving in its rotation direction at all times

		this.vel[0] -= Math.cos(this.target_rot + 6.28 / 4) * friction[0] * dt * SPEED * this.age / 50
		this.vel[2] += Math.sin(this.target_rot + 6.28 / 4) * friction[2] * dt * SPEED * this.age / 50

		// apply velocity, gravity acceleration, and friction/drag

		this.pos = this.pos.map((pos, i) => pos + this.vel[i] * dt)
		this.vel[1] += GRAVITY * dt
		this.vel = this.vel.map((vel, i) => vel - abs_min(vel * friction[i] * dt, vel))

		// check collisions (nothing complicated, just check if we're past ground/boundaries and reset on respective axes)

		this.grounded = false

		if (this.pos[1] < 0 || isNaN(this.pos[1])) {
			this.grounded = true
			this.pos[1] = 0
		}

		if (this.pos[0] > BOUNDS || this.pos[2] > BOUNDS || this.pos[0] < -BOUNDS || this.pos[2] < -BOUNDS) {
			this.target_rot = Math.random() * TAU

			this.pos[0] = Math.max(Math.min(this.pos[0], BOUNDS), -BOUNDS)
			this.pos[2] = Math.max(Math.min(this.pos[2], BOUNDS), -BOUNDS)

			this.vel = [0, 0, 0]
		}

		// create a model matrix for rendering cow depending on its position/age/rotation

		this.model_matrix = new Matrix()
		this.scale = 0.2 + this.age / 100

		this.model_matrix.translate(...this.pos)
		this.model_matrix.rotate(this.rot, 0, 1, 0)
		this.model_matrix.scale(this.scale, this.scale, this.scale)

		// just rotation matrix

		this.rot_matrix = new Matrix()
		this.rot_matrix.rotate(this.rot, 0, 1, 0)
	}

	draw(gl, render_state) {
		// gl:           instance of WebGLRenderingContext
		// render_state: the render_state object

		this.model.draw(gl, render_state, this.rot_matrix, this.model_matrix)
	}

	draw_shadow(gl, render_state) {
		// gl:           instance of WebGLRenderingContext
		// render_state: the render_state object

		const model_matrix = new Matrix(this.model_matrix)

		// reset the Y axis of the model matrix to render the shadow
		// we also need to set the shadow uniform depending on the height of the cow (higher means smaller shadow)
		// this is totally the right way of doing shadows and i perfectly know what i'm doing

		gl.uniform1f(render_state.shadow_uniform, Math.max(1 - this.pos[1] / this.jump_height * this.scale, 0.01))

		model_matrix.translate(0, (-this.pos[1] + 0.05) / this.scale, 0)
		this.shadow.draw(gl, render_state, this.rot_matrix, model_matrix)

		gl.uniform1f(render_state.shadow_uniform, -1)
	}
}

class Paturage {
	// actual rendering code
	// Paturage does all the WebGL setup and handles the main loop/cows

	constructor() {
		// webgl setup
		// this is all quite boilerplate-y stuff

		let canvas = document.getElementById("paturage")
		let error = document.getElementById("paturage-error")

		this.gl = canvas.getContext("webgl2") || canvas.getContext("experimental-webgl2") || canvas.getContext("webgl") || canvas.getContext("experimental-webgl");

		if (!this.gl || (!(this.gl instanceof WebGLRenderingContext) && !(this.gl instanceof WebGL2RenderingContext))) {
			error.hidden = false
			canvas.hidden = true

			return
		}

		this.x_res = this.gl.drawingBufferWidth
		this.y_res = this.gl.drawingBufferHeight

		this.gl.viewport(0, 0, this.x_res, this.y_res)
		this.gl.enable(this.gl.DEPTH_TEST)
		this.gl.blendFunc(this.gl.SRC_ALPHA, this.gl.ONE_MINUS_SRC_ALPHA)

		// load shader program
		// again, nothing interesting to comment on, this is all basically boilerplate

		const vert_shader = this.gl.createShader(this.gl.VERTEX_SHADER)
		const frag_shader = this.gl.createShader(this.gl.FRAGMENT_SHADER)

		this.gl.shaderSource(vert_shader, document.getElementById("moodle-vert-shader").innerHTML)
		this.gl.shaderSource(frag_shader, document.getElementById("moodle-frag-shader").innerHTML)

		this.gl.compileShader(vert_shader)
		this.gl.compileShader(frag_shader)

		this.program = this.gl.createProgram()

		this.gl.attachShader(this.program, vert_shader)
		this.gl.attachShader(this.program, frag_shader)

		this.gl.linkProgram(this.program)

		// MDN detaches the shaders first with 'gl.detachShader'
		// I'm not really sure what purpose this serves

		this.gl.deleteShader(vert_shader)
		this.gl.deleteShader(frag_shader)

		if (!this.gl.getProgramParameter(this.program, this.gl.LINK_STATUS)) {
			const log = this.gl.getProgramInfoLog(this.program)

			error.innerHTML = `Shader error: ${log}`
			error.hidden = false
		}

		// get attribute & uniform locations from program
		// we have to do this for attributes too, because WebGL 1.0 limits us to older shader models

		this.render_state = {
			pos_attr:        0, // this.gl.getAttribLocation (this.program, "a_pos"),
			tex_coord_attr:  1, // this.gl.getAttribLocation (this.program, "a_tex_coord"),
			normal_attr:     2, // this.gl.getAttribLocation (this.program, "a_normal"),

			model_uniform:   this.gl.getUniformLocation(this.program, "u_model"),
			rot_uniform:     this.gl.getUniformLocation(this.program, "u_rot"),
			vp_uniform:      this.gl.getUniformLocation(this.program, "u_vp"),
			sampler_uniform: this.gl.getUniformLocation(this.program, "u_sampler"),
			shadow_uniform:  this.gl.getUniformLocation(this.program, "u_shadow")
		}

		// load models

		const quad_model = {
			indices: new Uint16Array([0, 1, 2, 2, 3, 0]),
			vertices: new Float32Array([
				-SHADOW_SIZE, 0, -SHADOW_SIZE, 0, 0, 0, 1, 0, // 0
				+SHADOW_SIZE, 0, -SHADOW_SIZE, 1, 0, 0, 1, 0, // 1
				+SHADOW_SIZE, 0,  SHADOW_SIZE, 1, 1, 0, 1, 0, // 2
				-SHADOW_SIZE, 0,  SHADOW_SIZE, 0, 1, 0, 1, 0, // 3
			])
		}

		this.paturage = new Model(this.gl, paturage_model, "/public/moodle/textures/paturage.png")
		this.shadow = new Model(this.gl, quad_model, "/public/moodle/textures/shadow.png")

		this.holstein = new Model(this.gl, holstein_model, "/public/moodle/textures/holstein.png")
		this.jersey = new Model(this.gl, jersey_model, "/public/moodle/textures/jersey.png")
		this.bbb = new Model(this.gl, bbb_model, "/public/moodle/textures/bbb.png")

		// cows

		let cow_sum = Object.values(data).reduce((x, y) => x + y)
		this.cows = []

		for (let breed in data) {
			let cow_count = data[breed]

			let model = {
				"Holstein":         this.holstein,
				"Jersey":           this.jersey,
				"Blanc Bleu Belge": this.bbb,
			}[breed]

			for (let j = 0; j < cow_count; j++) {
				this.cows.push(new Cow(model, (7 + 160 / cow_sum) * (Math.random() / 2 + 0.5), this.shadow))
			}
		}

		// loop

		this.target_fov = TAU / 4
		this.fov = TAU / 5

		this.prev = 0
		requestAnimationFrame((now) => this.render(now))
	}

	render(now) {
		// timing

		const dt = (now - this.prev) / 1000
		this.prev = now

		const time = now / 1000

		// create matrices

		const multiplier = dt * 5

		if (multiplier > 1) {
			this.fov = this.target_fov
		}

		else {
			this.fov += (this.target_fov - this.fov) * multiplier
		}

		let proj_matrix = new Matrix()
		proj_matrix.perspective(this.fov, this.y_res / this.x_res, 2, 20)

		let view_matrix = new Matrix()

		view_matrix.translate(0, 0, -6)
		view_matrix.rotate_2d(time / 3, -0.5)

		let vp_matrix = new Matrix(view_matrix)
		vp_matrix.multiply(proj_matrix)

		// actually render

		this.gl.clearColor(0.0, 0.0, 0.0, 0.0)
		this.gl.clear(this.gl.COLOR_BUFFER_BIT | this.gl.DEPTH_BUFFER_BIT)

		this.gl.useProgram(this.program)
		this.gl.uniformMatrix4fv(this.render_state.vp_uniform, false, vp_matrix.data.flat())

		this.paturage.draw(this.gl, this.render_state, identity, identity /* no special transformation for paturage */)

		// first, update cows

		for (let cow of this.cows) {
			cow.update(dt)
		}

		// then, render their shadows

		this.gl.enable(this.gl.BLEND)
		this.gl.disable(this.gl.DEPTH_TEST)

		for (let cow of this.cows) {
			cow.draw_shadow(this.gl, this.render_state)
		}

		this.gl.disable(this.gl.BLEND)
		this.gl.enable(this.gl.DEPTH_TEST)

		// finally, render the cows themselves

		for (let cow of this.cows) {
			cow.draw(this.gl, this.render_state)
		}

		requestAnimationFrame((now) => this.render(now))
	}

	click() {
		this.fov = TAU / 4.3

		for (let cow of this.cows) {
			if (invert_gravity) {
				cow.vel[1] = -Math.random()
				continue
			}

			cow.force_jump(Math.random() * JUMP_HEIGHT * 2)
		}
	}
}

// create a new instance of Paturage when the page loads

var paturage

window.addEventListener("load", () => {
	paturage = new Paturage()
}, false)
