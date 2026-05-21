Blok **FastComments** je glavni pripomoček za komentiranje. Dodajte ga v predloge objav na blogu, predloge izdelkov ali katero koli drugo stran, kjer želite razpravo ali klepet v živo.

### Dodaj blok

1. Odprite urejevalnik teme Shopify (**Online Store > Themes > Customize**).
2. Izberite predlogo, na kateri želite komentare: **Blog post**, **Product**, ali katero koli drugo stran ali predlogo sekcije.
3. V sekciji, kjer želite, da se komentarji prikažejo, kliknite **Add block**.
4. Pod **Apps** izberite **FastComments**.
5. Kliknite **Save**.

Blok se prikaže takoj. Vnos Tenant ID ni potreben; najemnik vaše trgovine je samodejno povezan ob namestitvi aplikacije.

### Nastavitve

| Setting | Kaj naredi | Privzeto |
|---|---|---|
| Tenant ID (neobvezno) | Prepiše, kateremu FastComments najemniku se blok prikaže. Pustite prazno, da uporabite najemnika, ki je samodejno konfiguriran za trgovino. Ročni Tenant ID poiščete na fastcomments.com/auth/my-account/api-secret. | (prazno) |
| SSO | Samodejno prijavi obiskovalca v njihov Shopify račun kupca pred komentiranjem. Glejte [Samodejna prijava strank Shopify](/guide-installation-shopify.html#shopify-sso). | Vključeno |
| Slog komentiranja | **Gnezdeno** za gnezdene odgovore in glasovanja, ali **Pretakanje** za klepet v realnem času. | Gnezdeno |
| Prilagojen URL ID | Prepiše samodejno zaznan identifikator strani. Uporabite to, kadar želite, da si dve URL naslovu delita en komentarni tok. | (samodejno zaznano) |

### Kako je izbran identifikator strani

Vsak komentarni tok je povezan z ID-jem URL. Blok izbere enega samodejno:

- **Blog post template:** `shopify-article-{article.id}`, ki ostane enak tudi ob spremembah prispevka in naslova.
- **Product template:** `shopify-product-{product.id}`, ki ostane enak tudi ob spremembah prispevka in naslova.
- **Other templates:** pot zahteve (request path).

Če nastavite **Custom URL ID**, se namesto tega uporabi ta vrednost. Uporabite isti Custom URL ID v več blokih (na primer na lokalizirani različici strani izdelka), da delijo en komentarni tok.

### Gnezdeno proti Pretakanju

**Gnezdeno** je privzeto. Obiskovalci se odgovarjajo med seboj, glasujejo in orodja za moderacijo delujejo kot pričakovano. Najbolje za objave na blogu in ocene izdelkov.

**Pretakanje** odstrani gnezdenje in prikazuje nove komentarje v realnem času, ko so objavljeni, podobno klepetu. Najbolje za predstavitve izdelkov, dogodke v živo in skupnostne strani.

### Več blokov na isti strani

Blok je mogoče dodati večkrat v isto predlogo. Na primer povzetek ocen na vrhu strani izdelka in blok FastComments na dnu. Bloki delijo ID URL, zato povzetek odraža komentarje spodaj.

### Nasveti

- Blok se v predogledu urejevalnika teme skrije z rumenim obvestilom, če ne najde najemnika. Če se to pojavi v vaši živi trgovini, znova namestite aplikacijo FastComments.
- Na strani izdelka blok FastComments lahko služi tudi kot pripomoček za ocene izdelkov. Združite ga s **FastComments - Reviews Summary** za povzetek z zvezdicami na vrhu strani.