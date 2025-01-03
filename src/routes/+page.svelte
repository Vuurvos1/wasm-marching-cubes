<script lang="ts">
	import * as THREE from 'three';
	import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
	import { ViewportGizmo } from 'three-viewport-gizmo';
	import { marching_cubes, Metaball } from '$lib/wasm/marching_cubes/marching_cubes';
	import { onMount } from 'svelte';

	import { MarchingCubes } from 'three/examples/jsm/objects/MarchingCubes.js';

	const resolution = 24;

	let canvas: HTMLCanvasElement;

	let meshTime = $state(0);

	function generateGeometry() {
		const startTime = performance.now();

		const balls: Metaball[] = [
			new Metaball(0.25, 0.25, 0.25, 0.65, 1.25),
			new Metaball(-0.25, -0.25, -0.25, 0.65, 1.25)
		];

		const { vertices, indices, normals } = marching_cubes(resolution, balls, 1.55);
		const geometry = new THREE.BufferGeometry();
		geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3, false));
		geometry.setIndex(new THREE.BufferAttribute(new Uint16Array(indices), 1));
		geometry.setAttribute('normal', new THREE.BufferAttribute(new Float32Array(normals), 3));
		console.log('vertices', vertices.length / 3); // 28536

		// geometry.computeVertexNormals();

		const endTime = performance.now();
		meshTime = endTime - startTime;

		return geometry;
	}

	onMount(() => {
		// Create the scene
		const scene = new THREE.Scene();
		scene.background = new THREE.Color(0x333333);

		// Add lighting
		const light = new THREE.DirectionalLight(0xffffff, 1);
		light.position.set(5, 5, 5);
		scene.add(light);

		const ambientLight = new THREE.AmbientLight(0x404040); // soft white light
		scene.add(ambientLight);

		// Create a camera
		const camera = new THREE.PerspectiveCamera(
			75,
			window.innerWidth / window.innerHeight,
			0.1,
			1000
		);
		camera.position.z = 5;

		// Create a renderer and attach it to the canvas
		const renderer = new THREE.WebGLRenderer({ canvas });
		renderer.setSize(window.innerWidth, window.innerHeight);

		// Generate marching cubes geometry
		const geometry = generateGeometry();

		const material = new THREE.MeshLambertMaterial({ color: 0x00ff00 });
		// const material = new THREE.MeshLambertMaterial({ color: 0x00ff00, wireframe: true });
		// const material = new THREE.MeshNormalMaterial();

		const cube = new THREE.Mesh(geometry, material);
		cube.scale.set(2, 2, 2);

		scene.add(cube);

		// vanilla marching cubes
		const effect = new MarchingCubes(resolution + 2, material, false, false, 100000);
		effect.addBall(0.25, 0.25, 0.25, 9, 1);
		effect.addBall(0.75, 0.75, 0.75, 9, 1);
		effect.update();
		effect.translateX(-2);
		console.info('three vertices', effect.geometry.attributes.position.count);
		scene.add(effect);

		const grid = new THREE.GridHelper(10, 10, 0xffffff, 0x555555);
		scene.add(grid);

		const controls = new OrbitControls(camera, renderer.domElement);
		const gizmo = new ViewportGizmo(camera, renderer);
		gizmo.attachControls(controls);

		let frame: number;

		// Animation loop
		function animate() {
			frame = requestAnimationFrame(animate);

			// generateGeometry();

			// Render the scene
			renderer.render(scene, camera);
			gizmo.render();
		}

		// Handle window resizing
		window.addEventListener('resize', () => {
			camera.aspect = window.innerWidth / window.innerHeight;
			camera.updateProjectionMatrix();
			renderer.setSize(window.innerWidth, window.innerHeight);
			gizmo.update();
		});

		animate();

		return () => {
			cancelAnimationFrame(frame);

			window.removeEventListener('resize', () => {
				camera.aspect = window.innerWidth / window.innerHeight;
				camera.updateProjectionMatrix();
				renderer.setSize(window.innerWidth, window.innerHeight);
				gizmo.update();
			});
		};
	});
</script>

<svelte:head>
	<title>Marching cubes</title>
</svelte:head>

<canvas bind:this={canvas} class="absolute -z-10 block h-screen w-screen"></canvas>

<div class="p-3 text-white">
	<h1 class="font-semi-bold text-xl">WASM marching cubes</h1>
	<!-- <pre>Mesh time: {(meshTime / 1000).toFixed(3)}s</pre> -->
</div>
