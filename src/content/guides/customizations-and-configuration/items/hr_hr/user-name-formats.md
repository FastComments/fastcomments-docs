---
Po zadanom, FastComments će prikazati ime korisnika kako su ga unijeli ili kako nam je proslijeđeno putem SSO.

Ipak, može biti poželjno zamaskirati ili prikazati korisnikovo ime na drugačiji način. Na primjer, ako je korisnikovo ime Allen Rex, možda
želite prikazati samo "Allen R.".

To se može učiniti bez koda u sučelju za prilagodbu widgeta, pod postavkom nazvanom `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

Dostupni formati su:

- Početna velika slova (prikazuje Example User kao Example User)
- Inicijal prezimena (prikazuje Example User kao Example U.)
- Svi inicijali (prikazuje Example User kao E. U.)
- Prikaži "Anoniman"

Učinak promjene je trenutni. Korisnici će i dalje vidjeti svoje puno korisničko ime pri vrhu područja za komentare, za sebe, ali njihovi će komentari prikazivati
izmijenjeno korisničko ime.

Korisnička imena se maskiraju na strani poslužitelja radi zaštite korisnika.

---