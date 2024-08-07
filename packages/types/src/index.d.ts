import './modules/buffer';
import './modules/test';
import './modules/inspect';
import './modules/assert';
import './modules/_internal_';
import './url';
import './console';
import './headers';
import type Path from './modules/path';

/**
 * The Meta interface contains information about the current file and its paths.
 */
interface Meta {
  /**
   * The name of the current file, e.g., index.ts.
   * @example 'index.ts'
   */
  file: string;
  /**
   * The absolute path to the current file.
   * @example '/user/project/src/index.ts'
   */
  path: string;
  /**
   * An alias for `path`. Contains the same value as `path`.
   * @example '/user/project/src/index.ts'
   */
  filename: string;
  /**
   * The absolute path to the directory containing the current file.
   * @example '/user/project/src'
   */
  dir: string;
  /**
   * An alias for `dir`. Contains the same value as `dir`.
   * @example '/user/project/src'
   */
  dirname: string;
}

type Os =
  | 'linux'
  | 'macos'
  | 'ios'
  | 'freebsd'
  | 'dragonfly'
  | 'netbsd'
  | 'openbsd'
  | 'solaris'
  | 'android'
  | 'windows';

interface Ike {
  /**
   * Provides information for the module about itself.
   */
  meta: Meta;

  /**
   * Exit the process with optional exit code.
   *
   * @param code Exit code.
   * @returns void
   */
  exit(code?: number): never;

  /**
   * Set an exit code for the process.
   *
   * @param code Exit code.
   * @returns void
   * @throws Error if the code is not a number or no arguments are provided.
   */
  setExitCode(code: number): void;

  /**
   * Global exit code
   *
   * @default 0
   * @returns number
   */
  exitCode: number;

  /**
   * Returns group ID of the process.
   *
   * Only available on Unix platforms. Returns null on Windows.
   */
  gid(): number | null;

  /**
   * Returns process ID of the process.
   */
  pid: number;

  /**
   * Returns user ID of the process.
   *
   * Only available on Unix platforms. Returns null on Windows.
   */
  uid(): number | null;

  /**
   * Checks if current operating system is Windows.
   *
   * @returns boolean
   */
  isWindows(): boolean;

  /**
   * Checks if current operating system is macOS.
   *
   * @returns boolean
   */
  isMacOS(): boolean;

  /**
   * Checks if current operating system is Linux.
   *
   * @returns boolean
   */
  isLinux(): boolean;

  /**
   * Returns the current working directory.
   *
   * @returns string
   */
  cwd(): string;

  /**
   * Returns the name of the current operating system.
   *
   * @example 'linux'
   */
  os: Os;

  /**
   * Returns the version of the Ike runtime.
   *
   * @example '0.1.0'
   */
  version: string;

  /**
   * Takes a string and parses it as TOML.
   *
   * @param tomlString TOML string to parse
   * @returns unknown
   */
  parseToml(tomlString: string): any;

  /**
   * Synchronously reads a file and returns entire content as array of bytes.
   *
   * Files are resolved using current working directory
   *
   * Let's say we have this structure:
   * - src
   *   - index.ts
   *   - file.txt
   *
   *  And we are running the script from parent directory:
   *  `ike run src/index.ts`:
   *  ```ts
   *  const content = Ike.readFileSync("file.txt");
   *  ```
   *
   *  This will result in error because path will be resolved as `{cwd}/file.txt` which is not correct.
   *
   *  @example
   *  ```ts
   *  const content = Ike.readFileSync("file.txt");
   *  console.log(new TextDecoder().decode(content));
   *  ```
   *
   * @param path Path to the file
   * @returns Uint8Array Content of the file as array of bytes
   */
  readFileSync(path: string): Uint8Array;

  /**
   * Synchronously reads a file and returns entire content as string.
   *
   *  @example
   *  ```ts
   *  const content = Ike.readFileSync("file.txt");
   *  console.log(content);
   *  ```
   *
   * @param path Path to the file
   * @returns string Content of the file as string
   */
  readTextFileSync(path: string): string;

  /**
   * Synchronously creates a new directory.
   *
   * @param path Path to the directory
   * @param opts Options for creating the directory
   * @returns void
   */
  createDirSync(path: string, opts?: { recursive?: boolean }): void;

  /**
   * Synchronously removes a directory.
   *
   * @param path Path to the directory
   * @returns void
   */
  removeDirSync(path: string): void;

  /**
   * Returns path to the executable.
   *
   * @returns string
   */
  which(
    executable: string,
    opts?: {
      /**
       * Current working directory.
       */
      cwd?: string;
      path?: string;
    },
  ): string | null;

  /**
   * Utilities for working with paths.
   *
   * @example
   * ```ts
   * console.log(Ike.path.join("path", "to", "file.txt"));
   * ```
   *
   * Can be also used as a module
   *
   * @example
   * ```ts
   * import { join } from "path";
   * console.log(join("path", "to", "file.txt"));
   * ```
   *
   */
  path: Path;

  /**
   * Environment variables.
   *
   * @example
   * ```ts
   * console.log(Ike.env.PATH);
   * ```
   */
  env: Record<string, string>;
}

declare namespace Ike {
  const meta: Meta;
}

declare global {
  const Ike: Ike;

  /**
   * Function allows to call internal rust functions.
   *
   * @internal
   */
  function $rustFunction(name: string): Function;
}

export {};
