/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
*/
export function greet(name: string): void;
/**
* @returns {string}
*/
export function getState(): string;
/**
* @param {number} x
* @param {number} y
*/
export function openFields(x: number, y: number): void;
/**
* @param {number} x
* @param {number} y
*/
export function toggleFlag(x: number, y: number): void;
/**
* @param {number} x
* @param {number} y
* @param {number} mine
* @returns {string}
*/
export function newField(x: number, y: number, mine: number): string;
/**
* @returns {boolean}
*/
export function isLost(): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly greet: (a: number, b: number) => void;
  readonly getState: (a: number) => void;
  readonly openFields: (a: number, b: number) => void;
  readonly toggleFlag: (a: number, b: number) => void;
  readonly newField: (a: number, b: number, c: number, d: number) => void;
  readonly isLost: () => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
