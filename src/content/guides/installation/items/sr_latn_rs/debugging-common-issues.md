Evo nekih simptoma koje često viđamo, i uobičajenih rešenja.

### Poruka "This is a demo"

Ovo se prikazuje kada ste kopirali kod vidžeta sa naše početne stranice, koja koristi naš demo
tenant. Da biste koristili svoj tenant, kopirajte kod vidžeta sa [ovde](https://fastcomments.com/auth/my-account/get-acct-code).

### Greška "FastComments cannot load on this domain"

FastComments treba da zna koji domeni su vlasništvo vašeg naloga kako bi autentifikovao zahteve povezane
sa vašim nalogom. [Pogledajte našu dokumentaciju](/guide-multiple-sites.html#add-domains-to-account) da vidite kako
da rešite ovu grešku (jednostavno dodajte tačan subdomen + domen na svoj nalog).

Imajte na umu da bi se ovo trebalo dešavati tek nakon isteka probnog perioda. Tokom probnog perioda, svi zahtevi sa novih domena
će automatski biti dodati na vaš nalog.

### Migrirani komentari se ne prikazuju za prilagođene instalacije

Obično se ovo dešava kada su uvezeni komentari vezani za `Page ID`, a vi prosleđujete URL
(ili ne prosleđujete vrednost, u kom slučaju se podrazumeva URL stranice).

Ovo možete otkloniti [izvozom svojih komentara](https://fastcomments.com/auth/my-account/manage-data/export) i pregledom kolone `URL ID` (trenutno kolona `B`).

Proverite da li su vrednosti koje vidite u koloni `URL ID` iste kao vrednosti koje prosleđujete konfiguraciji vidžeta
kao parametar `urlId`.

Za dalje objašnjenje, pročitajte našu [dokumentaciju How Comments are Tied to Pages and Articles](/guide-customizations-and-configuration.html#url-id).

Ako ništa ne pomogne, [obratite nam se](https://fastcomments.com/auth/my-account/help).

### Vidžet za komentare se ne prikazuje

Ako se vidžet za komentare ne prikazuje, proverite Chrome developer konzolu za greške.

Za većinu nepravilnih konfiguracija, vidžet za komentare će bar prikazati grešku na stranici ako je
u stanju da se učita. Ako ne vidite ništa, to obično ukazuje na grešku u skripti.

### Željena konfiguracija ne radi kako se očekivalo

Isprobajte naš [Chrome ekstenziju](https://chromewebstore.google.com/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj) da vidite kakva
konfiguracija se prosleđuje vidžetu za komentare. Ako ništa ne pomogne, napravite snimak ekrana onoga što Chrome ekstenzija prikazuje
i [obratite nam se](https://fastcomments.com/auth/my-account/help).

### Komentari nedostaju na istom URL-u sa različitim hash bang-om

Podrazumevano, FastComments koristi URL stranice kao "bucket" u kom se čuvaju komentari. Ako vaši URL-ovi uključuju `#hashbangs`, a ti `#hashbangs`
ne bi trebalo da budu deo identifikatora koji identifikuje nit komentara, možemo jednostavno ignorisati vrednost hash bang-a, na primer:

[inline-code-attrs-start title = 'Primer ignorisanja hash bang-ova'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Imajte na umu da nakon ove izmene mora biti izvršena migracija za postojeće komentare. [Za to, obratite nam se.](https://fastcomments.com/auth/my-account/help)

### URL query parametri utiču na vidžet

Podrazumevano, FastComments koristi URL stranice kao "bucket" u kom se čuvaju komentari. Ako vaši URL-ovi uključuju query parametre
koji ne bi trebalo da budu deo identifikatora koji identifikuje nit komentara, možemo ih jednostavno ignorisati, na primer:

[inline-code-attrs-start title = 'Ignorisanje query parametara'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Imajte na umu da nakon ove izmene mora biti izvršena migracija za postojeće komentare. [Za to, obratite nam se.](https://fastcomments.com/auth/my-account/help)

### Ne primanje emailova

U FastComments uložili smo puno truda da osiguramo da je isporuka naših emailova što
pouzdanija. Međutim, neki email provajderi su notorno teški za pouzdano dostavljanje. Proverite svoj spam
folder za poruke sa fastcomments.com.

Ako nam [se obratite](https://fastcomments.com/auth/my-account/help), obično možemo pružiti
više uvida u to zašto možda ne vidite emailove od nas.