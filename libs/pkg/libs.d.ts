/* tslint:disable */
/* eslint-disable */
/**
* @param {string} plaintext
* @param {string} key
* @returns {string}
*/
export function vigenere_encode(plaintext: string, key: string): string;
/**
* @param {string} ciphertext
* @param {string} key
* @returns {string}
*/
export function vigenere_decode(ciphertext: string, key: string): string;
/**
* @param {string} text
* @param {number} shift
* @returns {string}
*/
export function caesar_encode(text: string, shift: number): string;
/**
* @param {string} text
* @param {number} shift
* @returns {string}
*/
export function caesar_decode(text: string, shift: number): string;
