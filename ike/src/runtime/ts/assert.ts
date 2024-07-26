import { inspect } from "inspect";

export class AssertionError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "AssertionError";
  }
}

export const assertEquals = (actual: any, expected: any, msg: string) => {
  const actualStr = inspect(actual);
  const expectedStr = inspect(expected);
  if (actualStr == expectedStr) {
    return;
  }
  throw new AssertionError(
    `Expected ${expectedStr}, but got ${actualStr}. ${msg}`
  );
};

export const assert = (condition: boolean, msg: string) => {
  if (condition) {
    return;
  }
  throw new AssertionError(msg);
};
