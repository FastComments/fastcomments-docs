---
Skripta za svako proširenje dohvaća se i poziva prije nego što widget za komentare počne dohvaćati prvi skup komentara i prikazivati korisničko sučelje.

Pri početnom učitavanju, sljedeći će se podaci pridružiti objektu proširenja:

- `config` - Referenca na objekt `config`.
- `translations` - Referenca na objekt `translations`.
- `commentsById` - Referenca na sve komentare po id-u.
- `root` - Referenca na korijenski DOM čvor.

Proširenja trebaju prebrisati željene funkcije koje će widget za komentare pozivati u odgovarajućim trenucima.

---