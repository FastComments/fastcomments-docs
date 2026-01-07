Her er nogle symptomer, vi ofte støder på, og almindelige løsninger.

### "This is a demo" besked

Dette vises, når du har kopieret widget-koden fra vores hjemmeside, som bruger vores demo
tenant. For at bruge din tenant, kopier widget-koden fra [her](https://fastcomments.com/auth/my-account/get-acct-code).

### "FastComments cannot load on this domain" fejl

FastComments skal vide, hvilke domæner der tilhører dig, for at autentificere anmodninger forbundet
med din konto. [Se vores dokumentation](/guide-multiple-sites.html#add-domains-to-account) for at se, hvordan
du løser denne fejl (tilføj blot det præcise underdomæne + domæne til din konto).

Bemærk at dette kun bør forekomme efter prøveperioden er udløbet. Under prøveperioden vil alle anmodninger fra nye domæner
automatisk blive tilføjet til din konto.

### Migrerede kommentarer vises ikke for tilpassede installationer

Dette sker normalt, når de importerede kommentarer er knyttet til et `Page ID`, og du sender en URL
(eller ingen værdi, i hvilket tilfælde den bruger sidens URL).

Du kan fejlsøge dette ved at [eksportere dine kommentarer](https://fastcomments.com/auth/my-account/manage-data/export) og se på `URL ID`-kolonnen (i øjeblikket kolonne `B`).

Sørg for at værdierne du ser i `URL ID`-kolonnen er de samme værdier, du sender til widget-konfigurationen
som `urlId`-parameteren.

For yderligere forklaring, prøv at læse vores [dokumentation om hvordan kommentarer er knyttet til sider og artikler](/guide-customizations-and-configuration.html#url-id).

Hvis alt andet fejler, [kontakt os](https://fastcomments.com/auth/my-account/help).

### Kommentar-widget vises ikke

Hvis kommentar-widget'en ikke vises, tjek Chrome developer-konsollen for fejl.

For de fleste fejlkonfigurationer vil kommentar-widget'en i det mindste vise en fejl på siden, hvis den
kan indlæses. At se ingenting er normalt en indikation af en scripting-fejl.

### Ønsket konfiguration virker ikke som forventet

Prøv vores [Chrome-udvidelse](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) for at se, hvilken
konfiguration kommentar-widget'en modtager. Hvis alt fejler, tag et screenshot af hvad chrome-udvidelsen siger
og [kontakt os](https://fastcomments.com/auth/my-account/help).

### Kommentarer mangler på samme URL med forskellig hash bang

Som standard vil FastComments bruge sidens URL til den "bucket", hvor kommentarer gemmes. Hvis dine URL'er inkluderer `#hashbangs`, og disse `#hashbangs`
ikke skal være en del af identifikatoren, der identificerer en kommentartråd, kan vi blot ignorere hash bang-værdien, for eksempel:

[inline-code-attrs-start title = 'Ignorer hash bangs eksempel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Bemærk at efter denne ændring skal der udføres en migration for eksisterende kommentarer. [For det, kontakt os.](https://fastcomments.com/auth/my-account/help)

### URL-forespørgselsparametre påvirker widget'en

Som standard vil FastComments bruge sidens URL til den "bucket", hvor kommentarer gemmes. Hvis dine URL'er inkluderer forespørgselsparametre
der ikke skal være en del af identifikatoren, der identificerer en kommentartråd, kan vi blot ignorere dem, for eksempel:

[inline-code-attrs-start title = 'Ignorer forespørgselsparametre'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Bemærk at efter denne ændring skal der udføres en migration for eksisterende kommentarer. [For det, kontakt os.](https://fastcomments.com/auth/my-account/help)

### Modtager ikke e-mails

Hos FastComments arbejder vi hårdt på at sikre, at vores e-mail-levering er så pålidelig som
muligt. Dog er nogle e-mail-udbydere notorisk svære at levere til pålideligt. Tjek din spam-mappe
for beskeder fra fastcomments.com.

Hvis du [kontakter os](https://fastcomments.com/auth/my-account/help) kan vi normalt give
mere indsigt i, hvorfor du måske ikke ser e-mails fra os.
