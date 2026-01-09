Skript za svaku ekstenziju se preuzima i poziva pre nego što vidžet komentara počne da preuzima prvi skup komentara i prikazuje korisnički interfejs.

Pri početnom učitavanju, sledeći podaci biće prikačeni na objekat ekstenzije:

- `config` - Referenca na objekat `config`.
- `translations` - Referenca na objekat `translations`.
- `commentsById` - Referenca na sve komentare po id.
- `root` - Referenca na root DOM čvor.

Ekstenzije bi trebalo da prepišu željene funkcije, koje će vidžet komentara pozivati u odgovarajućim trenucima.