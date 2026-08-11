---
Po defaultu, FastComments prikazuje ime korisnika onako kako je uneto, ili kako je prosleđeno putem SSO.

Međutim, može biti poželjno maskirati ili prikazati ime korisnika na drugačiji način. Na primer, ako je ime korisnika Allen Rex, možda
želite da prikažete samo "Allen R.".

Ovo se može uraditi bez koda u UI‑u za prilagođavanje widgeta, pod podešavanjem pod nazivom `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Padajući meni Formata imena komentatora otvoren sa izborima kao što su Capitalize, Last Initial i All Initials'; title='Promeni format imena' app-screenshot-end]

Dostupni formati su:

- Capitalize (prikazati primer korisnika kao Example User)
- Last Initial (prikazati Example User kao Example U.)
- All Initials (prikazati Example User kao E. U.)
- Show "Anonymous"

Efekat promene je trenutni. Korisnici će i dalje videti svoje puno korisničko ime na vrhu oblasti za komentar, za sebe, ali njihovi komentari će prikazivati
izmenjeno korisničko ime.

Korisnička imena se maskiraju na serveru radi zaštite korisnika.

---