Prema zadatim postavkama, FastComments će prikazati ime korisnika onako kako su ga uneli, ili kako nam je prosleđeno putem SSO.

Međutim, može biti poželjno maskirati ili prikazati ime korisnika na drugačiji način. Na primer, ako je ime korisnika Allen Rex, možda
želite da prikažete samo "Allen R.".

Ovo je moguće uraditi bez koda u korisničkom interfejsu za prilagođavanje widgeta (Widget Customization UI), u podešavanju nazvanom `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

Dostupni formati su:

- Capitalize (prikazuje primer korisnika kao Example User)
- Last Initial (prikazuje Example User kao Example U.)
- All Initials (prikazuje Example User kao E. U.)
- Show "Anonymous"

Efekat promene se primenjuje odmah. Korisnici će i dalje videti svoje puno korisničko ime na vrhu oblasti za komentare, za sebe, ali njihovi komentari će prikazivati
izmenjeno korisničko ime.

Korisnička imena se maskiraju na strani servera radi zaštite korisnika.