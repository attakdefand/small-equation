/* tslint:disable */
/* eslint-disable */
/**
 * WASM wrapper for European Call Option pricing using Black-Scholes model
 */
export function price_european_call(underlying_price: number, strike_price: number, time_to_expiry: number, risk_free_rate: number, volatility: number): number;
/**
 * WASM wrapper for European Put Option pricing using Black-Scholes model
 */
export function price_european_put(underlying_price: number, strike_price: number, time_to_expiry: number, risk_free_rate: number, volatility: number): number;
/**
 * WASM wrapper for calculating Greeks
 */
export function calculate_greeks(underlying_price: number, strike_price: number, time_to_expiry: number, risk_free_rate: number, volatility: number): string;
/**
 * Initialize the WASM module
 */
export function main(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly price_european_call: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly price_european_put: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly calculate_greeks: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly main: () => void;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
