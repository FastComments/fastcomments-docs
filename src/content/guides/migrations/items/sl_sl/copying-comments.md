V primeru, da je treba podatke premikati, FastComments nudi orodje za samopostrežno premikanje komentarjev med stranmi in članki.

Tukaj je, kako izgleda obrazec za kopiranje komentarjev:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; alt='Obrazec za kopiranje komentarjev s poljem ID URL-ja iz in poljema ID URL-ja ter URL-ja za cilj'; title='Obrazec za kopiranje komentarjev' app-screenshot-end]

### Izpolnjevanje polj "Od"

Da bi določili, od kod premikati komentarje, preprosto potrebujemo izvorni `URL ID`.

Če v konfiguraciji pripomočka za komentarje ne podajate vrednosti za `urlId`, bo to čista različica URL-ja strani.

Vrednosti `URL ID`, ki jih imajo vaši komentarji, lahko vidite z njihovim izvozom.

### Izpolnjevanje polj "Do"

Da bi določili, kam premikati komentarje, moramo poznati ciljni `URL ID` in `URL`.

`URL ID` bo koš, v katerega gre komentar. Polje `URL` se uporablja, da lahko neposredno odprete komentar iz e‑pošte in orodij za moderiranje.

#### WordPress

Če uporabljate WordPress, bi na primer v polja To/From `URL ID` v orodju za migracijo vnesli ID-je člankov, namesto URL-ja.