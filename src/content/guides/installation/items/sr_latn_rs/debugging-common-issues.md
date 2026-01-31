Evo nekoliko simptoma koje često viđamo i uobičajena rešenja.

### "This is a demo" Message

Ovo se prikazuje kada ste kopirali kod widgeta sa naše početne stranice, koja koristi naš demo
tenant. Da biste koristili svoj tenant, kopirajte kod widgeta sa [ovde](https://fastcomments.com/auth/my-account/get-acct-code).

### "FastComments cannot load on this domain" Error

FastComments mora znati koje domene posedujete kako bi autentifikovao zahteve povezane
sa vašim nalogom. [Pogledajte našu dokumentaciju](/guide-multiple-sites.html#add-domains-to-account) da vidite kako
da rešite ovu grešku (jednostavno dodajte tačan subdomen + domen na vaš nalog).

Imajte na umu da bi se ovo trebalo desiti samo nakon isteka probnog perioda. Tokom probnog perioda, svi zahtevi sa novih domena
će automatski biti dodati na vaš nalog.

### Migrated Comments Not Showing for Custom Installations

Obično se ovo dešava kada su uvezeni komentari vezani za `Page ID`, a vi prosleđujete URL
(ili nijednu vrednost, u kom slučaju podrazumevano je URL stranice).

Možete ovo otkloniti tako što ćete [izvesti svoje komentare](https://fastcomments.com/auth/my-account/manage-data/export) i pogledati kolonu `URL ID` (trenutno Kolona `B`).

Uverite se da su vrednosti koje vidite u koloni `URL ID` iste vrednosti koje prosleđujete u konfiguraciji widgeta
kao parametar `urlId`.

Za dalje objašnjenje, pokušajte da pročitate našu [How Comments are Tied to Pages and Articles documentation](/guide-customizations-and-configuration.html#url-id).

Ako ništa ne pomogne, [kontaktirajte nas](https://fastcomments.com/auth/my-account/help).

### Comment Widget Not Showing

Ako se komentar widget ne prikazuje, proverite Chrome developer konzolu za greške.

Za većinu pogrešnih konfiguracija, komentar widget će bar prikazati grešku na stranici ako je
u stanju da se učita. Ako se ništa ne vidi, obično je to indikator skript greške.

### Desired Configuration Not Working as Expected

Isprobajte našu [Chrome ekstenziju](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) da vidite koju
konfiguraciju komentar widget dobija. Ako ništa ne pomogne, napravite screenshot onoga što Chrome ekstenzija prikazuje
i [kontaktirajte nas](https://fastcomments.com/auth/my-account/help).

### Comments Missing on Same URL With Different Hash Bang

By default, FastComments will use the page URL for the "bucket" where comments are stored. If your URLs include `#hashbangs`, and these `#hashbangs`
should not be part of the identifier that identifies a comment thread, we can simply ignore the hash bang value, for example:

[inline-code-attrs-start title = 'Primer ignorisanja hash-bangova'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

Imajte na umu da će nakon ove promene morati da se izvrši migracija za postojeće komentare. [U vezi sa tim, kontaktirajte nas.](https://fastcomments.com/auth/my-account/help)

### URL Query Parameters Affecting Widget

By default, FastComments will use the page URL for the "bucket" where comments are stored. If your URLs include query parameters
that should not be part of the identifier that identifies a comment thread, we can simply ignore them, for example:

[inline-code-attrs-start title = 'Primer ignorisanja parametara upita'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

Imajte na umu da će nakon ove promene morati da se izvrši migracija za postojeće komentare. [U vezi sa tim, kontaktirajte nas.](https://fastcomments.com/auth/my-account/help)

### Not Receiving Emails

U FastComments ulažemo mnogo truda da obezbedimo što pouzdaniju isporuku mejlova. Ipak, neki provajderi e-pošte su poznato teško dostupni za pouzdanu isporuku. Proverite svoj spam
folder za poruke od fastcomments.com.

Ako nas [kontaktirate](https://fastcomments.com/auth/my-account/help) obično možemo pružiti
više informacija zašto možda ne vidite mejlove od nas.