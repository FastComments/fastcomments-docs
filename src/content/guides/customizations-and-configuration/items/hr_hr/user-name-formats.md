---
Prema zadanim postavkama, FastComments će prikazati ime korisnika onako kako ga je unio, ili kako je proslijeđeno putem SSO-a.

Međutim, možda je poželjno maskirati ili prikazati ime korisnika na drugačiji način. Na primjer, ako je ime korisnika Allen Rex, možda želite prikazati samo "Allen R.".

Ovo se može učiniti bez koda u sučelju za prilagodbu widgeta, pod postavkom pod nazivom `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Padajući izbornik Formata imena komentatora otvoren s izborima poput Capitalize, Last Initial i All Initials'; title='Promijeni format imena' app-screenshot-end]

Dostupni formati su:

- Capitalize (prikazuje primjer korisnika kao Example User)
- Last Initial (prikazuje Example User kao Example U.)
- All Initials (prikazuje Example User kao E. U.)
- Show "Anonymous"

Učinak promjene je trenutni. Korisnici će i dalje vidjeti svoje puno korisničko ime na vrhu područja za komentar, za sebe, ali njihovi komentari će prikazivati modificirano korisničko ime.

Korisnička imena maskiraju se na poslužitelju radi zaštite korisnika.
---