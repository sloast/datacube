extends Node3D

var rng = RandomNumberGenerator.new()

enum Face {
	U,
	D,
	L,
	R,
	F,
	B,
}

const speed = 0.05

var char2face = {"u":Face.U, "d":Face.D, "l":Face.L, "r":Face.R, "f":Face.F, "b":Face.B}

const scene = preload("res://mesh_instance_3d.tscn")
const white: Material = preload("res://materials/white.tres")
const yellow: Material = preload("res://materials/yellow.tres")
const blue: Material = preload("res://materials/blue.tres")
const red: Material = preload("res://materials/red.tres")
const orange: Material = preload("res://materials/orange.tres")
const green: Material = preload("res://materials/green.tres")

var fcolors: Array[Material] = [white, yellow, green, blue, red, orange]

@onready var viscube = $VisualCube

const offsets = [[-1,-1],[0,-1],[1,-1],[1,0],[1,1],[0,1],[-1,1],[-1,0]]
var colors: Array[Material] = [red, white, green, white, blue, white, yellow, white]

var facelets: Array[MeshInstance3D] = []

func _rotate_r(array: Array, n: int) -> Array:
	var new_array = []
	for i in range(array.size()):
		new_array.append(array[(i-n) % array.size()])
	return new_array

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	rng.randomize()
	
	_make_face(Vector3.UP, Vector3.ZERO, white)
	_make_face(Vector3.DOWN, Vector3(PI, 0, 0), yellow, 2, true)
	_make_face(Vector3.LEFT, Vector3(-PI/2, PI/2, 0), green, 2)
	_make_face(Vector3.RIGHT, Vector3(PI/2, PI/2, 0), blue, 4, true)
	_make_face(Vector3.BACK, Vector3(-PI/2, 0, 0), red, 2, true)
	_make_face(Vector3.FORWARD, Vector3(PI/2, 0, 0), orange, 4)

	#await parse("RUR'U' RUR'U' RUR'U' RUR'U' RUR'U' RUR'U'")
	#await parse("R2 U (R U R' U') R' U' (R' U R')")

func parse(str: String, vc=viscube) -> void:
	var flag = false
	var face = null
	for c in str:
		if c.to_lower() not in "udlrfb23'":
			continue
		var read = true
		if flag:
			var n = 1
			if c == "2":
				n = 2
				read = false
			elif c in "'3":
				n = 3
				read = false
			await sleep(.05)
			vc.rotate(face, n)
			flag = false
		if read:
			if c.to_lower() not in char2face:
				continue
			face = char2face[c.to_lower()]
			flag = true
	if flag:
		await sleep(.05)
		vc.rotate(face, 1)

func shuffle(moves: int, vc=viscube) -> void:
	var i = 0
	while moves == -1 || i < moves:
		var face = rng.randi_range(0, 5)
		var n = rng.randi_range(0,2)
		vc.rotate(face,n)

func sleep(sec: float):
	await get_tree().create_timer(sec).timeout


func _make_facelet(axis: Vector3, rotation_: Vector3, offset: Array, color: Material) -> Node:
	var pos = Vector3(axis * 1.5)
	if pos.x == 0:
		pos.x = offset[0]
	else:
		pos.y = offset[0]
	if pos.z == 0:
		pos.z = offset[1]
	else:
		pos.y = offset[1]
	
	var instance = scene.instantiate() as MeshInstance3D
	instance.position = pos
	instance.rotation = rotation_
	instance.set_surface_override_material(0, color)
	add_child(instance)
	return instance

func _make_face(axis: Vector3, rotation_: Vector3, center: Material, r_offset: int=0, rev:bool=false) -> void:
	_make_facelet(axis, rotation_, [0,0], center)
	var offsets_copy = offsets
	if rev:
		offsets_copy = _rotate_r(offsets_copy, -1)
		offsets_copy.reverse()
	for i in range(8):
		var instance = _make_facelet(axis, rotation_, offsets_copy[(i+r_offset)%offsets_copy.size()], colors[i])
		facelets.append(instance)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	var face = rng.randi_range(0, 5)
	var n = rng.randi_range(0,2)
	viscube.rotate(face,n)
	var state = viscube.get_state()
	for i in range(state.size()):
		facelets[i].set_surface_override_material(0, fcolors[state[i]])
