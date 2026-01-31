Evo nekih simptoma na koje često nailazimo i uobičajenih rješenja. 

### "This is a demo" Message

Ovo se prikazuje kada ste kopirali kod widgeta sa naše početne stranice, koja koristi naš demo tenant. Da biste koristili svoj tenant, kopirajte kod widgeta sa [ovdje](https://fastcomments.com/auth/my-account/get-acct-code).

### "FastComments cannot load on this domain" Error

FastComments mora znati koji domeni pripadaju vama kako bi autentifikovao zahtjeve povezane sa vašim nalogom. [Pogledajte našu dokumentaciju](/guide-multiple-sites.html#add-domains-to-account) da vidite kako riješiti ovu grešku (jednostavno dodajte tačan subdomen + domen na vaš nalog).

Obratite pažnju da bi se ovo trebalo pojaviti samo nakon isteka probnog perioda. Tokom probnog perioda, svi zahtjevi sa novih domena će automatski biti dodani na vaš nalog.

### Migrated Comments Not Showing for Custom Installations

Obično se ovo dešava kada su uvezeni komentari vezani za `Page ID`, a vi prosljeđujete URL (ili nijednu vrijednost, u kom slučaju se koristi zadani URL stranice).

Možete ovo otkloniti tako što ćete [izvesti svoje komentare](https://fastcomments.com/auth/my-account/manage-data/export) i pogledati stupac `URL ID` (trenutno stupac `B`).

Osigurajte da su vrijednosti koje vidite u stupcu `URL ID` iste vrijednosti koje prosljeđujete konfiguraciji widgeta kao parametar `urlId`.

Za daljnje objašnjenje, pokušajte pročitati našu dokumentaciju [How Comments are Tied to Pages and Articles](/guide-customizations-and-configuration.html#url-id).

Ako ništa drugo ne pomogne, [obratite nam se](https://fastcomments.com/auth/my-account/help).

### Comment Widget Not Showing

Ako se komentar widget ne prikazuje, provjerite Chrome konzolu za razvojne programere zbog grešaka.

Za većinu pogrešnih konfiguracija, widget će bar prikazati grešku na stranici ako je uopšte može učitati. To što se ne vidi ništa obično je indikacija greške u skripti.

### Desired Configuration Not Working as Expected

Isprobajte našu [Chrome ekstenziju](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) da vidite koju konfiguraciju widget prima. Ako ništa ne pomogne, napravite snimak ekrana onoga što Chrome ekstenzija prikazuje i [obratite nam se](https://fastcomments.com/auth/my-account/help).

### Comments Missing on Same URL With Different Hash Bang

Po defaultu, FastComments će koristiti URL stranice za "bucket" gdje se komentari pohranjuju. Ako vaši URL-ovi uključuju `#hashbangs`, i ti `#hashbangs`
ne bi trebali biti dio identifikatora koji određuje thread komentara, možemo jednostavno ignorisati vrijednost hash banga, na primjer:

[inline-code-attrs-start title = 'Primjer ignorisanja hash-bangova'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Obratite pažnju da će nakon ove promjene migracija morati biti izvršena za postojeće komentare. [Za to, obratite nam se.](https://fastcomments.com/auth/my-account/help)

### URL Query Parameters Affecting Widget

Po defaultu, FastComments će koristiti URL stranice za "bucket" gdje se komentari pohranjuju. Ako vaši URL-ovi uključuju query parametre
koji ne bi trebali biti dio identifikatora koji određuje thread komentara, možemo ih jednostavno ignorisati, na primjer:

[inline-code-attrs-start title = 'Ignoriši parametre upita'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Obratite pažnju da će nakon ove promjene migracija morati biti izvršena za postojeće komentare. [Za to, obratite nam se.](https://fastcomments.com/auth/my-account/help)

### Not Receiving Emails

U FastComments ulažemo mnogo truda da naša dostava e-poruka bude što pouzdanija. Međutim, nekim provajderima e-pošte je izuzetno teško pouzdano dostaviti poruke. Provjerite svoju mapu neželjene pošte za poruke od fastcomments.com.

Ako nam se [obratite](https://fastcomments.com/auth/my-account/help) obično možemo pružiti više informacija zašto možda ne vidite naše e-poruke.