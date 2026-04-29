Okida se kada je komentar zaključan.

### Kontekst koji agent prima

- Zaključani komentar.
- Opcionalna istorija teme / korisnika / kontekst stranice kako je konfigurisano.

### Ko pokreće ovaj događaj

- Moderator koji koristi radnju zaključavanja na stranici za moderaciju ili u widgetu za komentare.

### Uobičajene upotrebe

- **Obavestite pregledaoce** - događaj zaključavanja često sledi burnu nit; webhook ka vašem Slack kanalu za moderaciju može omogućiti ljudima da preuzmu ostatak.
- **Sprovođenje perioda hlađenja** - zakažite [deferred trigger](#trigger-deferred-delay) na odvojenom agentu koji, 24 sata nakon zaključavanja, razmatra da li otključati.

### Par

Pogledajte [Trigger: Comment Unlocked](#trigger-comment-unlock) za odgovarajući okidač.