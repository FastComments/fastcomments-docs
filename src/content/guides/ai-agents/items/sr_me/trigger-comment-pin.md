Pokreće se kada je komentar prikačen.

### Kontekst koji agent prima

- Prikačeni komentar.
- Opcionalna nit / istorija korisnika / kontekst stranice kako je konfigurisano.

### Ko pokreće ovaj događaj

- Moderator koji klikne radnju 'prikači' na stranici za moderaciju ili u widgetu komentara.
- Agent koji pozove [`pin_comment`](#tools-overview).

Prevencija petlje: događaji prikačenja koje generiše agent ne pokreću nijedan agent okidač. Prikačenje koje izvrši agent prekida svako dalje prosleđivanje događaja agentima za taj događaj, a ne samo originalnom agentu.

### Napomena o paru

Događaji prikačenja i otkačenja su odvojeni okidači. Pretplatite se na oba ako želite simetrično ponašanje. Pogledajte [Okidač: Komentar Otkačen](#trigger-comment-unpin).

---