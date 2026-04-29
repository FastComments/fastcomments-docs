Pokreće se kada je komentar zaključan.

### Kontekst koji agent prima

- Zaključani komentar.
- Opcionalna istorija teme / korisnika / kontekst stranice, kako je konfigurisano.

### Ko pokreće ovo

- Moderator koji koristi akciju zaključavanja na stranici za moderaciju ili widgetu za komentare.

### Uobičajene upotrebe

- **Obavesti recenzente** - događaj zaključavanja često sledi uzavrelu diskusiju; webhook ka vašem Slack kanalu za moderaciju može omogućiti ljudima da preuzmu ostatak.
- **Sprovođenje perioda hlađenja** - zakažite a [deferred trigger](#trigger-deferred-delay) na posebnom agentu koji će, 24 sata nakon zaključavanja, razmotriti da li otključati.

### Pair

Pogledajte [Trigger: Comment Unlocked](#trigger-comment-unlock) za odgovarajući okidač.

---