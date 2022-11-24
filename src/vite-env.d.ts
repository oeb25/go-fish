/// <reference types="vite/client" />

declare const tag: unique symbol;
declare type Rank =
  | "RA"
  | "R2"
  | "R3"
  | "R4"
  | "R5"
  | "R6"
  | "R7"
  | "R8"
  | "R9"
  | "R10"
  | "RJ"
  | "RQ"
  | "RK";
// declare type Suit = number & { readonly [tag]: "Suit" };
declare type Card = number & { readonly [tag]: "Card" };
declare type PlayerId = number & { readonly [tag]: "Card" };
declare type Cards = Card[];
declare type Ranks = string[] & { readonly [tag]: "Ranks" };
