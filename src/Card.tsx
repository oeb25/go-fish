// https://www.me.uk/cards/makeadeck.cgi

import C1B from "./assets/poker-qr/1B.svg";
import C2H from "./assets/poker-qr/2H.svg";
import C3H from "./assets/poker-qr/3H.svg";
import C4S from "./assets/poker-qr/4S.svg";
import C6C from "./assets/poker-qr/6C.svg";
import C7D from "./assets/poker-qr/7D.svg";
import C8H from "./assets/poker-qr/8H.svg";
import C9S from "./assets/poker-qr/9S.svg";
import CJC from "./assets/poker-qr/JC.svg";
import CKD from "./assets/poker-qr/KD.svg";
import CQH from "./assets/poker-qr/QH.svg";
import CTS from "./assets/poker-qr/TS.svg";
import C1J from "./assets/poker-qr/1J.svg";
import C2J from "./assets/poker-qr/2J.svg";
import C3S from "./assets/poker-qr/3S.svg";
import C5C from "./assets/poker-qr/5C.svg";
import C6D from "./assets/poker-qr/6D.svg";
import C7H from "./assets/poker-qr/7H.svg";
import C8S from "./assets/poker-qr/8S.svg";
import CAC from "./assets/poker-qr/AC.svg";
import CJD from "./assets/poker-qr/JD.svg";
import CKH from "./assets/poker-qr/KH.svg";
import CQS from "./assets/poker-qr/QS.svg";
import C2B from "./assets/poker-qr/2B.svg";
import C2S from "./assets/poker-qr/2S.svg";
import C4C from "./assets/poker-qr/4C.svg";
import C5D from "./assets/poker-qr/5D.svg";
import C6H from "./assets/poker-qr/6H.svg";
import C7S from "./assets/poker-qr/7S.svg";
import C9C from "./assets/poker-qr/9C.svg";
import CAD from "./assets/poker-qr/AD.svg";
import CJH from "./assets/poker-qr/JH.svg";
import CKS from "./assets/poker-qr/KS.svg";
import CTC from "./assets/poker-qr/TC.svg";
import C2C from "./assets/poker-qr/2C.svg";
import C3C from "./assets/poker-qr/3C.svg";
import C4D from "./assets/poker-qr/4D.svg";
import C5H from "./assets/poker-qr/5H.svg";
import C6S from "./assets/poker-qr/6S.svg";
import C8C from "./assets/poker-qr/8C.svg";
import C9D from "./assets/poker-qr/9D.svg";
import CAH from "./assets/poker-qr/AH.svg";
import CJS from "./assets/poker-qr/JS.svg";
import CQC from "./assets/poker-qr/QC.svg";
import CTD from "./assets/poker-qr/TD.svg";
import C2D from "./assets/poker-qr/2D.svg";
import C3D from "./assets/poker-qr/3D.svg";
import C4H from "./assets/poker-qr/4H.svg";
import C5S from "./assets/poker-qr/5S.svg";
import C7C from "./assets/poker-qr/7C.svg";
import C8D from "./assets/poker-qr/8D.svg";
import C9H from "./assets/poker-qr/9H.svg";
import CAS from "./assets/poker-qr/AS.svg";
import CKC from "./assets/poker-qr/KC.svg";
import CQD from "./assets/poker-qr/QD.svg";
import CTH from "./assets/poker-qr/TH.svg";

const HEARTHS = [
  CAH,
  C2H,
  C3H,
  C4H,
  C5H,
  C6H,
  C7H,
  C8H,
  C9H,
  CJH,
  CKH,
  CQH,
  CTH,
];
const SPADES = [
  CAS,
  C2S,
  C3S,
  C4S,
  C5S,
  C6S,
  C7S,
  C8S,
  C9S,
  CJS,
  CKS,
  CQS,
  CTS,
];
export const CLUBS = [
  CAC,
  C2C,
  C3C,
  C4C,
  C5C,
  C6C,
  C7C,
  C8C,
  C9C,
  CJC,
  CKC,
  CQC,
  CTC,
];
export const DIAMONDS = [
  CAD,
  C2D,
  C3D,
  C4D,
  C5D,
  C6D,
  C7D,
  C8D,
  C9D,
  CJD,
  CKD,
  CQD,
  CTD,
];

export const CARDS = [...SPADES, ...HEARTHS, ...DIAMONDS, ...CLUBS];

export const BACKS = [C1B, C2B];
export const JOKERS = [C1J, C2J];
