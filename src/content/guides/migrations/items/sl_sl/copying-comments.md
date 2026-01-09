Če je treba podatke premakniti, FastComments zagotavlja samoobslužno orodje za premikanje komentarjev med stranmi in članki.

Tako izgleda obrazec za kopiranje komentarjev:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Filling out the "From" Fields

Da določimo, od kod premakniti komentarje, potrebujemo le izvorni `URL ID`.

Če v konfiguraciji vtičnika za komentarje ne pošiljate vrednosti za `urlId`, bo to "čista" različica URL-ja strani.

Z izvozom lahko vidite, katere vrednosti imajo vaši komentarji za `URL ID`.

### Filling out the "To" Fields

Da določimo, kam premakniti komentarje, moramo poznati ciljni `URL ID` in `URL`.

`URL ID` bo vedro, v katerega komentar spada. Polje `URL` se uporablja, da lahko do komentarja neposredno dostopate iz e-poštnih sporočil in orodij za moderiranje.

#### WordPress

Če uporabljate WordPress, bi na primer v orodju za migracijo v polja To/From `URL ID` vnesli ID-je člankov namesto URL-ja.