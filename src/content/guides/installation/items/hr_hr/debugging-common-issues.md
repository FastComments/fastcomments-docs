Ispod se nalaze neki simptomi koji se često pojavljuju i uobičajena rješenja. 

### "This is a demo" Poruka

Ovo se prikazuje kada ste kopirali kod widgeta s naše početne stranice, koja koristi naš demo tenant. Da biste koristili svoj tenant, kopirajte kod widgeta s [ovdje](https://fastcomments.com/auth/my-account/get-acct-code).

### "FastComments cannot load on this domain" Greška

FastComments mora znati koje domene pripadaju vama kako bi autentificirao zahtjeve povezane s vašim računom. [Pogledajte našu dokumentaciju](/guide-multiple-sites.html#add-domains-to-account) da vidite kako riješiti ovu grešku (jednostavno dodajte točan subdomen + domenu na svoj račun).

Imajte na umu da se ovo obično događa tek nakon isteka probnog razdoblja. Tijekom probnog razdoblja, svi zahtjevi s novih domena će se automatski dodati na vaš račun.

### Migrirani komentari se ne prikazuju za prilagođene instalacije

Obično se to događa kada su uvezeni komentari vezani uz `Page ID`, a vi prosljeđujete URL
(ili nijednu vrijednost, u kojem slučaju se koristi zadani URL stranice).

Možete debugirati ovo [izvozom svojih komentara](https://fastcomments.com/auth/my-account/manage-data/export) i pregledom stupca `URL ID` (trenutno stupac `B`).

Provjerite jesu li vrijednosti koje vidite u stupcu `URL ID` iste vrijednosti koje prosljeđujete konfiguraciji widgeta kao parametar `urlId`.

Za dodatno objašnjenje, pokušajte pročitati našu dokumentaciju [How Comments are Tied to Pages and Articles](/guide-customizations-and-configuration.html#url-id).

Ako ništa drugo ne pomogne, [obratite nam se](https://fastcomments.com/auth/my-account/help).

### Widget za komentare se ne prikazuje

Ako se widget za komentare ne prikazuje, provjerite Chrome konzolu za programere zbog grešaka.

Za većinu pogrešne konfiguracije, widget za komentare će se barem prikazati s greškom na stranici ako uspije učitati. Ako se ne vidi ništa, obično je to pokazatelj skriptne greške.

### Željena konfiguracija ne radi kako se očekuje

Isprobajte našu [Chrome ekstenziju](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) da vidite koju konfiguraciju widget za komentare zapravo prima. Ako ništa ne pomogne, snimite zaslon onoga što Chrome ekstenzija prikazuje i [obratite nam se](https://fastcomments.com/auth/my-account/help).

### Komentari nedostaju na istoj URL-u s različitim hash-bangom

Po defaultu, FastComments će koristiti URL stranice za "bucket" u kojem se komentari pohranjuju. Ako vaši URL-ovi uključuju `#hashbangs`, i ti `#hashbangs`
ne bi trebali biti dio identifikatora koji identificira thread komentara, možemo jednostavno ignorirati vrijednost hash banga, na primjer:

[inline-code-attrs-start title = 'Primjer ignoriranja hash-bangova'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Imajte na umu da će nakon ove promjene za postojeće komentare trebati izvršiti migraciju. [Za to, obratite nam se.](https://fastcomments.com/auth/my-account/help)

### Parametri upita u URL-u koji utječu na widget

Po defaultu, FastComments će koristiti URL stranice za "bucket" u kojem se komentari pohranjuju. Ako vaši URL-ovi uključuju query parametre
koji ne bi trebali biti dio identifikatora koji identificira thread komentara, možemo ih jednostavno ignorirati, na primjer:

[inline-code-attrs-start title = 'Ignoriraj parametre upita'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Imajte na umu da će nakon ove promjene za postojeće komentare trebati izvršiti migraciju. [Za to, obratite nam se.](https://fastcomments.com/auth/my-account/help)

### Ne primate e-poštu

U FastCommentsu ulažemo puno truda kako bismo osigurali da je slanje naših e-pošta što pouzdanije. Međutim, nekim pružateljima e-pošte je iznimno teško pouzdano dostaviti poruke. Provjerite svoju mapu neželjene pošte za poruke od fastcomments.com.

Ako nam [obratite se](https://fastcomments.com/auth/my-account/help) obično možemo dati
više informacija zašto možda ne vidite e-poruke od nas.

---