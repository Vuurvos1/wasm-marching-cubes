<script lang="ts">
	import * as THREE from 'three';
	import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
	import { ViewportGizmo } from 'three-viewport-gizmo';
	import { List, Pane, Slider, type ListOptions } from 'svelte-tweakpane-ui';
	import { onMount } from 'svelte';

	import { MarchingCubes } from 'three/examples/jsm/objects/MarchingCubes.js';

	async function loadWasm() {
		const { marching_cubes, visualize_sdf, Metaball } = await import(
			'$lib/wasm/marching_cubes/marching_cubes'
		);
		return { marching_cubes, visualize_sdf, Metaball };
	}

	let wasm: Awaited<ReturnType<typeof loadWasm>> | undefined = $state();

	let canvas: HTMLCanvasElement;

	let resolution = $state(24);
	let threshold = $state(1.5);

	let ballSize = $state(0.5);
	let ballStrength = $state(2.5);

	type MaterialOptions = 'Lambert' | 'Normal' | 'Wireframe';
	const materialOptions: ListOptions<MaterialOptions> = {
		Lambert: 'Lambert',
		Normal: 'Normal',
		Wireframe: 'Wireframe'
	};
	let materialSelection = $state<MaterialOptions>('Lambert');

	let meshMaterial = $derived.by(() => {
		if (materialSelection === 'Normal') {
			return new THREE.MeshNormalMaterial();
		} else if (materialSelection === 'Wireframe') {
			return new THREE.MeshLambertMaterial({ color: 0x00ff00, wireframe: true });
		}
		return new THREE.MeshLambertMaterial({ color: 0x00ff00 });
	});

	let scene: THREE.Scene | undefined = $state();

	// visualize sdf
	$effect(() => {
		if (!scene || !wasm) return;

		const sdfPoints = scene.getObjectByName('sdfPoints');
		if (sdfPoints) {
			scene.remove(sdfPoints);
		}

		const balls = [
			new wasm.Metaball(0.25, 0.25, 0.25, ballSize, ballStrength),
			new wasm.Metaball(0.75, 0.75, 0.75, ballSize, ballStrength)
		];

		const { vertices } = wasm.visualize_sdf(resolution, balls, threshold);
		const sdfGeometry = new THREE.BufferGeometry();
		sdfGeometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3, false));

		const sdfMaterial = new THREE.PointsMaterial({ size: 0.02 });
		const newSdfPoints = new THREE.Points(sdfGeometry, sdfMaterial);
		newSdfPoints.name = 'sdfPoints';
		newSdfPoints.scale.set(2, 2, 2);
		newSdfPoints.translateX(2);
		newSdfPoints.translateY(-1);
		newSdfPoints.translateZ(-1);

		scene.add(newSdfPoints);
	});

	// marching cubes
	$effect(() => {
		if (!scene || !wasm) return;

		const cube = scene.getObjectByName('marchingCubes');
		if (cube) {
			scene.remove(cube);
		}

		const balls = [
			new wasm.Metaball(0.25, 0.25, 0.25, ballSize, ballStrength),
			new wasm.Metaball(0.75, 0.75, 0.75, ballSize, ballStrength)
		];

		console.time('wasm marching cubes');
		const { vertices, indices, normals } = wasm.marching_cubes(resolution, balls, threshold);
		console.timeEnd('wasm marching cubes');
		const geometry = new THREE.BufferGeometry();
		geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3, false));
		geometry.setIndex(new THREE.BufferAttribute(new Uint16Array(indices), 1));
		geometry.setAttribute('normal', new THREE.BufferAttribute(new Float32Array(normals), 3));
		console.log('vertices', vertices.length / 3);

		const newCube = new THREE.Mesh(geometry, meshMaterial);
		newCube.name = 'marchingCubes';
		newCube.scale.set(2, 2, 2);
		newCube.translateX(-1);
		newCube.translateY(-1);
		newCube.translateZ(-1);

		scene.add(newCube);
	});

	onMount(() => {
		async function load() {
			wasm = await loadWasm();
		}
		load();

		// Create the scene
		scene = new THREE.Scene();
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

		// vanilla marching cubes
		const material = new THREE.MeshLambertMaterial({ color: 0x00ff00 });
		console.time('three marching cubes');
		const effect = new MarchingCubes(resolution + 2, material, false, false, 100000);
		effect.addBall(0.25, 0.25, 0.25, 9, 1);
		effect.addBall(0.75, 0.75, 0.75, 9, 1);
		effect.update();
		console.timeEnd('three marching cubes');
		effect.translateX(-3);
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

			// Render the scene
			if (scene) renderer.render(scene, camera);
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

<Pane position="draggable" x={24} y={1000}>
	<Slider
		min={8}
		max={64}
		step={1}
		format={(v) => v.toFixed(0)}
		label="Resolution"
		bind:value={resolution}
	></Slider>

	<Slider
		min={0}
		max={3}
		step={0.01}
		format={(v) => v.toFixed(2)}
		label="Threshold"
		bind:value={threshold}
	></Slider>

	<Slider
		min={0}
		max={1}
		step={0.01}
		format={(v) => v.toFixed(2)}
		label="Size"
		bind:value={ballSize}
	></Slider>

	<Slider
		min={0}
		max={50}
		step={0.01}
		format={(v) => v.toFixed(2)}
		label="Strength"
		bind:value={ballStrength}
	></Slider>`

	<List options={materialOptions} bind:value={materialSelection} label="Material"></List>
</Pane>

<canvas bind:this={canvas} class="absolute block h-screen w-screen"></canvas>

<div class="p-3 text-white">
	<h1 class="font-semi-bold text-xl">WASM marching cubes</h1>
</div>
