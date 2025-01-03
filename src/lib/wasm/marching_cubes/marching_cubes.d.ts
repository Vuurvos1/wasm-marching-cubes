/* tslint:disable */
/* eslint-disable */
export function marching_cubes(size: number, metaballs: (Metaball)[], threshold: number): GridData;
export class GridData {
  private constructor();
  free(): void;
  readonly vertices: Float32Array;
  readonly indices: Uint32Array;
  readonly normals: Float32Array;
}
export class Metaball {
  free(): void;
  constructor(x: number, y: number, z: number, radius: number, influence: number);
}
