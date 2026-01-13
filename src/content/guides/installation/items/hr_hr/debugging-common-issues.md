Ovdje su neki simptomi s kojima se često susrećemo i uobičajena rješenja.

### Poruka "Ovo je demo"

Ovo se prikazuje kada ste kopirali kod widgeta s naše početne stranice, koji koristi naš demo
tenant. Za korištenje vašeg tenanta, kopirajte kod widgeta [odavde](https://fastcomments.com/auth/my-account/get-acct-code).

### Greška "FastComments ne može se učitati na ovoj domeni"

FastComments mora znati koje domene su u vašem vlasništvu kako bi autentificirao zahtjeve povezane
s vašim računom. [Pogledajte našu dokumentaciju](/guide-multiple-sites.html#add-domains-to-account) da biste vidjeli kako
riješiti ovu grešku (jednostavno dodajte točnu poddomenu + domenu na vaš račun).

Imajte na umu da bi se ovo trebalo dogoditi samo nakon isteka probnog razdoblja. Tijekom probnog razdoblja, svi zahtjevi s novih domena
automatski će se dodati na vaš račun.

### Migrirani komentari se ne prikazuju za prilagođene instalacije

Obično se to događa kada su uvezeni komentari vezani za `Page ID`, a vi prosljeđujete URL
(ili nikakvu vrijednost, u kojem slučaju se koristi URL stranice).

Ovo možete debugirati [izvozom vaših komentara](https://fastcomments.com/auth/my-account/manage-data/export) i pregledavanjem stupca `URL ID` (trenutno stupac `B`).

Osigurajte da su vrijednosti koje vidite u stupcu `URL ID` iste vrijednosti koje prosljeđujete konfiguraciji widgeta
kao parametar `urlId`.

Za dodatno objašnjenje, pokušajte pročitati našu [dokumentaciju Kako su komentari vezani za stranice i članke](/guide-customizations-and-configuration.html#url-id).

Ako sve drugo ne uspije, [obratite nam se](https://fastcomments.com/auth/my-account/help).

### Widget za komentare se ne prikazuje

Ako se widget za komentare ne prikazuje, provjerite Chrome developer konzolu za greške.

Za većinu pogrešnih konfiguracija, widget za komentare će barem prikazati grešku na stranici ako se
može učitati. Ne vidjeti ništa obično ukazuje na skriptnu grešku.

### Željena konfiguracija ne radi kako se očekuje

Isprobajte naše [Chrome proširenje](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) da vidite koja
konfiguracija se prosljeđuje widgetu za komentare. Ako sve propadne, napravite snimku zaslona što chrome proširenje kaže
i [obratite nam se](https://fastcomments.com/auth/my-account/help).

### Komentari nedostaju na istom URL-u s različitim hash bangom

Prema zadanim postavkama, FastComments će koristiti URL stranice za "bucket" gdje se komentari pohranjuju. Ako vaši URL-ovi uključuju `#hashbangove`, i ti `#hashbangovi`
ne bi trebali biti dio identifikatora koji identificira nit komentara, možemo jednostavno ignorirati hash bang vrijednost, na primjer:

[inline-code-attrs-start title = 'Primjer ignoriranja hash bangova'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Imajte na umu da će nakon ove promjene biti potrebna migracija za postojeće komentare. [Za to nam se obratite.](https://fastcomments.com/auth/my-account/help)

### URL parametri upita utječu na widget

Prema zadanim postavkama, FastComments će koristiti URL stranice za "bucket" gdje se komentari pohranjuju. Ako vaši URL-ovi uključuju parametre upita
koji ne bi trebali biti dio identifikatora koji identificira nit komentara, možemo ih jednostavno ignorirati, na primjer:

[inline-code-attrs-start title = 'Ignoriraj parametre upita'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Imajte na umu da će nakon ove promjene biti potrebna migracija za postojeće komentare. [Za to nam se obratite.](https://fastcomments.com/auth/my-account/help)

### Ne primate e-mailove

U FastCommentsu ulažemo puno truda kako bismo osigurali da je isporuka naših e-mailova što pouzdanija.
Međutim, nekim pružateljima e-mail usluga notorno je teško pouzdano isporučivati. Provjerite vašu spam
mapu za poruke od fastcomments.com.

Ako nam se [obratite](https://fastcomments.com/auth/my-account/help) obično možemo pružiti
više uvida zašto možda ne vidite e-mailove od nas.
