import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
	plugins: [wasm(), topLevelAwait(), sveltekit()],

	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	},

	optimizeDeps: {
		exclude: ['@syntect/wasm']
	}
});
