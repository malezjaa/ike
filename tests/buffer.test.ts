import { describe, it, expect } from '@std/test';
import { isAscii, isUtf8 } from '@std/buffer';

const encoder = new TextEncoder();

describe('Base64 encoding and decoding', () => {
  it('should encode a string to base64 using btoa', () => {
    const originalString = 'Hello, World!';
    const base64String = btoa(originalString);
    expect(base64String).toBe('SGVsbG8sIFdvcmxkIQ==');
  });

  it('should decode a base64 string to original string using atob', () => {
    const base64String = 'SGVsbG8sIFdvcmxkIQ==';
    const decodedString = atob(base64String);
    expect(decodedString).toBe('Hello, World!');
  });

  it('should handle empty strings', () => {
    const originalString = '';
    const base64String = btoa(originalString);
    expect(base64String).toBe('');

    const decodedString = atob(base64String);
    expect(decodedString).toBe(originalString);
  });

  it('should throw an error for non-base64 strings in atob', () => {
    expect(() => {
      atob('not_base64');
    }).toThrow();
  });
});

describe('isAscii', () => {
  it('should return true for ASCII strings', () => {
    const arr = encoder.encode('Hello, World!');
    expect(isAscii(arr)).toBe(true);
  });

  it('should return false for non-ASCII strings', () => {
    const arr = encoder.encode('你好，世界！');
    expect(isAscii(arr)).toBe(false);
  });

  it('should throw if the input is not a TypedArray or ArrayBuffer', () => {
    // @ts-ignore
    expect(() => isAscii('Hello, World!')).toThrow();
  });
});

describe('isUtf8', () => {
  it('should return true for valid UTF-8 strings', () => {
    const arr = encoder.encode('Hello, World!');
    expect(isUtf8(arr)).toBe(true);
  });

  it('should return false for invalid UTF-8 strings', () => {
    const arr = new Uint8Array([0xff]);
    expect(isUtf8(arr)).toBe(false);
  });

  it('should throw if the input is not a TypedArray or ArrayBuffer', () => {
    // @ts-ignore
    expect(() => isUtf8('Hello, World!')).toThrow();
  });
});
