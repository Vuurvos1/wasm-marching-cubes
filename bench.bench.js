import { bench } from 'vitest';
import * as THREE from 'three';
import { MarchingCubes } from 'three/examples/jsm/objects/MarchingCubes.js';
import { marching_cubes } from './src/lib/wasm/marching_cubes/marching_cubes';

const resolution = 48;

bench('wasm marching cubes', () => {
	marching_cubes(resolution);
});

bench('three marching cubes', () => {
	const material = new THREE.MeshNormalMaterial();
	const effect = new MarchingCubes(resolution, material, false, false, 100000);
	effect.addBall(0.5, 0.5, 0.5, 15, 0);
	effect.update();
});
