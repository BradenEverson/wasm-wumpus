/* tslint:disable */
/* eslint-disable */
export enum Action {
  Move = 0,
  Shoot = 1,
}
export enum Direction {
  North = 0,
  South = 1,
  East = 2,
  West = 3,
}
export enum Entity {
  Arrow = 0,
  Wumpus = 1,
  BigBat = 2,
  BottomlessPit = 3,
  Player = 4,
  Empty = 5,
}
export class GameSession {
  free(): void;
  /**
   * @param {number} bats
   * @param {number} pits
   * @param {number} arrows
   * @returns {GameSession | undefined}
   */
  static new(bats: number, pits: number, arrows: number): GameSession | undefined;
  /**
   * @param {Action} action
   * @param {Direction} direction
   */
  perform_action(action: Action, direction: Direction): void;
  /**
   * @returns {boolean | undefined}
   */
  get_state(): boolean | undefined;
  /**
   * @returns {string}
   */
  render(): string;
  /**
   * @returns {Entity | undefined}
   */
  get_curr_room(): Entity | undefined;
  /**
   * @returns {number}
   */
  arrows_left(): number;
  /**
   * @returns {boolean}
   */
  was_carried(): boolean;
  /**
   * @returns {(string)[]}
   */
  get_status_messages(): (string)[];
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_gamesession_free: (a: number, b: number) => void;
  readonly gamesession_new: (a: number, b: number, c: number) => number;
  readonly gamesession_perform_action: (a: number, b: number, c: number) => void;
  readonly gamesession_get_state: (a: number) => number;
  readonly gamesession_render: (a: number, b: number) => void;
  readonly gamesession_get_curr_room: (a: number) => number;
  readonly gamesession_arrows_left: (a: number) => number;
  readonly gamesession_was_carried: (a: number) => number;
  readonly gamesession_get_status_messages: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
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
