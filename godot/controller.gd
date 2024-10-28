extends Node3D

const scene = preload("res://mesh_instance_3d.tscn")
const white: Material = preload("res://materials/white.tres")
const yellow: Material = preload("res://materials/yellow.tres")
const blue: Material = preload("res://materials/blue.tres")
const red: Material = preload("res://materials/red.tres")
const orange: Material = preload("res://materials/orange.tres")
const green: Material = preload("res://materials/green.tres")

const offsets = [[-1,-1],[0,-1],[1,-1],[1,0],[1,1],[0,1],[-1,1],[-1,0]]
var bcolors: Array[Material] = [red, white, green, white, blue, white, yellow, white]

func _rotate_r(array: Array, n: int) -> Array:
	var new_array = []
	for i in range(array.size()):
		new_array.append(array[(i-n) % array.size()])
	return new_array

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	const u_rot = 0
	const r_rot = 0
	const f_rot = 4
	
	_make_face(Vector3.UP, Vector3.ZERO, white, _rotate_r(bcolors, u_rot))
	_make_face(Vector3.DOWN, Vector3(PI, 0, 0), yellow, _rotate_r(bcolors, u_rot+4))
	_make_face(Vector3.RIGHT, Vector3(PI/2, PI/2, 0), blue, _rotate_r(bcolors, r_rot), true)
	_make_face(Vector3.LEFT, Vector3(-PI/2, PI/2, 0), green, _rotate_r(bcolors, r_rot+4), true)
	_make_face(Vector3.BACK, Vector3(-PI/2, 0, 0), red, _rotate_r(bcolors, f_rot), true)
	_make_face(Vector3.FORWARD, Vector3(PI/2, 0, 0), orange, _rotate_r(bcolors, f_rot+4), true)

func _make_facelet(axis: Vector3, rotation_: Vector3, offset: Array, color: Material) -> void:
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

func _make_face(axis: Vector3, rotation_: Vector3, center_color: Material, colors: Array = bcolors, reverse: bool = false) -> void:
	_make_facelet(axis, rotation_, [0,0], center_color)
	if reverse:
		colors = _rotate_r(colors,-1)
		colors.reverse()
	for i in range(8):
		_make_facelet(axis, rotation_, offsets[i], colors[i])

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
