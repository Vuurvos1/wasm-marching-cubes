<script lang="ts">
	import * as THREE from 'three';
	import { marching_cubes } from '$lib/wasm/marching_cubes/marching_cubes';
	import { onMount } from 'svelte';

	let canvas: HTMLCanvasElement;

	onMount(() => {
		// Create the scene
		const scene = new THREE.Scene();

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

		// Create a cube
		const vertices = marching_cubes(1.0); // Generate vertices for a cube
		const geometry = new THREE.BufferGeometry();
		geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3, false));
		geometry.computeVertexNormals();

		// Add a spinning cube
		// const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
		const material = new THREE.MeshLambertMaterial({ color: 0x00ff00 });
		const cube = new THREE.Mesh(geometry, material);
		scene.add(cube);

		// Animation loop
		function animate() {
			requestAnimationFrame(animate);

			// Rotate the cube
			cube.rotation.x += 0.001;
			cube.rotation.y += 0.001;

			// Render the scene
			renderer.render(scene, camera);
		}

		// Handle window resizing
		window.addEventListener('resize', () => {
			camera.aspect = window.innerWidth / window.innerHeight;
			camera.updateProjectionMatrix();
			renderer.setSize(window.innerWidth, window.innerHeight);
		});

		animate();
	});
</script>

<canvas bind:this={canvas} class="absolute -z-10 block h-screen w-screen"></canvas>

<section class="text-white">
	<h1>Welcome to SvelteKit</h1>
</section>
