Aktivira se kada je komentar prikvačen.

### Kontekst koji agent prima

- Prikvačeni komentar.
- Opcionalna istorija teme / korisnika / kontekst stranice kako je konfigurisano.

### Ko pokreće ovo

- Moderator koji klikne na radnju prikvačivanja na stranici za moderaciju ili u vidžetu komentara.
- Agent koji pozove [`pin_comment`](#tools-overview).

Prevencija petlje: događaji prikvačenja koje potiče agent ne pokreću nijedan okidač agenta. Prikvačivanje koje izvrši agent onemogućava svako slanje događaja agentima za taj događaj, i to ne samo za agenta koji ga je pokrenuo.

### Napomena o paru

Događaji prikvačenja i odkačivanja su zasebni okidači. Pretplatite se na oba ako želite simetrično ponašanje. Pogledajte [Okidač: Komentar odkačen](#trigger-comment-unpin).