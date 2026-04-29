Pokreće se kada je komentar zaključan.

### Kontekst koji agent prima

- Zaključani komentar.
- Opcionalna povijest niti / korisnika / kontekst stranice prema konfiguraciji.

### Tko to pokreće

- Moderator koji koristi radnju zaključavanja na stranici za moderaciju ili u widgetu komentara.

### Uobičajene upotrebe

- **Obavijesti recenzente** - događaj zaključavanja često slijedi žestoku nit; webhook prema vašem Slack kanalu za moderaciju može omogućiti ljudima da preuzmu ostatak.
- **Provođenje perioda hlađenja** - zakažite [odgođeni okidač](#trigger-deferred-delay) na zasebnom agentu koji, 24 sata nakon zaključavanja, razmatra treba li otključati.

### Par

Pogledajte [Okidač: Komentar otključan](#trigger-comment-unlock) za zrcalni okidač.

---