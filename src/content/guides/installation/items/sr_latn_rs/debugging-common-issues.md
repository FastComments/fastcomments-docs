Evo nekih simptoma sa kojima se često susrećemo i uobičajenih rešenja.

### Poruka "Ovo je demo"

Ovo se prikazuje kada ste kopirali kod vidžeta sa naše početne stranice, koji koristi naš demo
zakupac. Za korišćenje vašeg zakupca, kopirajte kod vidžeta [odavde](https://fastcomments.com/auth/my-account/get-acct-code).

### Greška "FastComments ne može se učitati na ovom domenu"

FastComments mora znati koji domeni su u vašem vlasništvu kako bi autentifikovao zahteve povezane
sa vašim nalogom. [Pogledajte našu dokumentaciju](/guide-multiple-sites.html#add-domains-to-account) da biste videli kako
da rešite ovu grešku (jednostavno dodajte tačan poddomen + domen na vaš nalog).

Imajte na umu da bi se ovo trebalo desiti samo nakon isteka probnog perioda. Tokom probnog perioda, svi zahtevi sa novih domena
automatski će se dodati na vaš nalog.

### Migrirani komentari se ne prikazuju za prilagođene instalacije

Obično se to dešava kada su uvezeni komentari vezani za `Page ID`, a vi prosleđujete URL
(ili nikakvu vrednost, u kom slučaju se koristi URL stranice).

Ovo možete debagovati [izvozom vaših komentara](https://fastcomments.com/auth/my-account/manage-data/export) i pregledavanjem kolone `URL ID` (trenutno kolona `B`).

Osigurajte da su vrednosti koje vidite u koloni `URL ID` iste vrednosti koje prosleđujete konfiguraciji vidžeta
kao parametar `urlId`.

Za dodatno objašnjenje, pokušajte pročitati našu [dokumentaciju Kako su komentari vezani za stranice i članke](/guide-customizations-and-configuration.html#url-id).

Ako sve drugo ne uspe, [obratite nam se](https://fastcomments.com/auth/my-account/help).

### Vidžet za komentare se ne prikazuje

Ako se vidžet za komentare ne prikazuje, proverite Chrome developer konzolu za greške.

Za većinu pogrešnih konfiguracija, vidžet za komentare će barem prikazati grešku na stranici ako se
može učitati. Ne videti ništa obično ukazuje na skriptnu grešku.

### Željena konfiguracija ne radi kako se očekuje

Isprobajte naše [Chrome proširenje](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) da vidite koja
konfiguracija se prosleđuje vidžetu za komentare. Ako sve propadne, napravite snimak ekrana šta chrome proširenje kaže
i [obratite nam se](https://fastcomments.com/auth/my-account/help).

### Komentari nedostaju na istom URL-u sa različitim hash bang-om

Prema podrazumevanim podešavanjima, FastComments će koristiti URL stranice za "bucket" gde se komentari čuvaju. Ako vaši URL-ovi uključuju `#hashbang`-ove, i ti `#hashbang`-ovi
ne bi trebali biti deo identifikatora koji identifikuje nit komentara, možemo jednostavno ignorisati hash bang vrednost, na primer:

[inline-code-attrs-start title = 'Primer ignorisanja hash bang-ova'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
});
</script>
[inline-code-end]

Imajte na umu da će nakon ove promene biti potrebna migracija za postojeće komentare. [Za to nam se obratite.](https://fastcomments.com/auth/my-account/help)

### URL parametri upita utiču na vidžet

Prema podrazumevanim podešavanjima, FastComments će koristiti URL stranice za "bucket" gde se komentari čuvaju. Ako vaši URL-ovi uključuju parametre upita
koji ne bi trebali biti deo identifikatora koji identifikuje nit komentara, možemo ih jednostavno ignorisati, na primer:

[inline-code-attrs-start title = 'Ignoriši parametre upita'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
});
</script>
[inline-code-end]

Imajte na umu da će nakon ove promene biti potrebna migracija za postojeće komentare. [Za to nam se obratite.](https://fastcomments.com/auth/my-account/help)

### Ne primate e-pošte

U FastComments-u ulažemo mnogo truda kako bismo osigurali da je isporuka naših e-poruka što pouzdanija.
Međutim, nekim provajderima e-pošte je notorno teško pouzdano isporučivati. Proverite vašu spam
fasciklu za poruke od fastcomments.com.

Ako nam se [obratite](https://fastcomments.com/auth/my-account/help) obično možemo pružiti
više uvida zašto možda ne vidite e-pošte od nas.
