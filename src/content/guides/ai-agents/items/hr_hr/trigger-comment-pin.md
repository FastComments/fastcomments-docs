Pokreće se kada je komentar prikvačen.

### Kontekst koji agent prima

- Prikvačeni komentar.
- Opcionalni kontekst niti / povijest korisnika / kontekst stranice prema konfiguraciji.

### Tko pokreće ovo

- Moderator koji klikne radnju prikvačivanja na stranici za moderiranje ili u widgetu komentara.
- Agent koji poziva [`pin_comment`](#tools-overview).

Sprječavanje petlje: događaji prikvačenja koje je inicirao agent ne pokreću okidače agenata. Prikvačenje koje izvrši agent prekida svu distribuciju okidača agenata za taj događaj, ne samo za agenta koji ga je inicirao.

### Napomena o paru

Događaji prikvačenja i uklanjanja prikvačenja su odvojeni okidači. Pretplatite se na oba ako želite simetrično ponašanje. Pogledajte [Okidač: Komentar uklonjen s prikvačenja](#trigger-comment-unpin).