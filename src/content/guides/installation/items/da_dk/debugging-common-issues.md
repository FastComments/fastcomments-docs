Her er nogle symptomer, vi ofte ser, samt almindelige løsninger.

### "This is a demo"-besked

Dette vises, når du har kopieret widget-koden fra vores forside, som bruger vores demo-
tenant. For at bruge din egen tenant skal du kopiere widget-koden fra [her](https://fastcomments.com/auth/my-account/get-acct-code).

### "FastComments cannot load on this domain"-fejl

FastComments skal vide, hvilke domæner du ejer, for at autentificere forespørgsler knyttet til
din konto. [Se vores dokumentation](/guide-multiple-sites.html#add-domains-to-account) for at se, hvordan du
løser denne fejl (tilføj blot det nøjagtige subdomæne + domæne til din konto).

Bemærk, at dette normalt kun opstår, efter prøveperioden er slut. I prøveperioden tilføjes forespørgsler fra nye domæner
automatisk til din konto.

### Migrerede kommentarer vises ikke for brugerdefinerede installationer

Normalt sker dette, når de importerede kommentarer er knyttet til en `Page ID`, og du sender en URL
(eller ingen værdi, i hvilket tilfælde den som standard er sidens URL).

Du kan fejlfinde dette ved at [eksportere dine kommentarer](https://fastcomments.com/auth/my-account/manage-data/export) og se kolonnen `URL ID` (i øjeblikket kolonne `B`).

Sørg for, at værdierne i kolonnen `URL ID` er de samme værdier, som du sender til widget-
konfigurationen som `urlId`-parameteren.

For yderligere forklaring kan du læse vores [dokumentation om, hvordan kommentarer knyttes til sider og artikler](/guide-customizations-and-configuration.html#url-id).

Hvis alt andet fejler, kan du [kontakte os](https://fastcomments.com/auth/my-account/help).

### Kommentar-widget vises ikke

Hvis kommentar-widget'en ikke vises, så tjek Chrome-udviklerkonsollen for fejl.

Ved de fleste fejlkonfigurationer vil kommentar-widget'en i det mindste vise en fejl på siden, hvis den
kan indlæses. Hvis der ikke vises noget, er det normalt tegn på en scripting-fejl.

### Ønsket konfiguration virker ikke som forventet

Prøv vores [Chrome-udvidelse](https://chromewebstore.google.com/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj) for at se, hvilken
konfiguration kommentar-widget'en modtager. Hvis alt fejler, så tag et skærmbillede af, hvad Chrome-udvidelsen viser,
og [kontakt os](https://fastcomments.com/auth/my-account/help).

### Kommentarer mangler på samme URL med forskellig hash bang

Som standard bruger FastComments side-URL'en som "bucket", hvor kommentarer gemmes. Hvis dine URL'er indeholder `#hashbangs`, og disse `#hashbangs`
ikke skal være en del af identifikatoren, der identificerer en kommentartråd, kan vi blot ignorere hash bang-værdien, for eksempel:

[inline-code-attrs-start title = 'Eksempel på ignorering af hash bangs'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Bemærk, at der efter denne ændring skal udføres en migration af eksisterende kommentarer. [For det, kontakt os.](https://fastcomments.com/auth/my-account/help)

### URL-queryparametre påvirker widget'en

Som standard bruger FastComments side-URL'en som "bucket", hvor kommentarer gemmes. Hvis dine URL'er indeholder queryparametre,
der ikke skal være en del af identifikatoren, der identificerer en kommentartråd, kan vi blot ignorere dem, for eksempel:

[inline-code-attrs-start title = 'Ignorer queryparametre'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Bemærk, at der efter denne ændring skal udføres en migration af eksisterende kommentarer. [For det, kontakt os.](https://fastcomments.com/auth/my-account/help)

### Modtager ikke e-mails

Hos FastComments lægger vi meget arbejde i at sikre, at vores e-mail-levering er så pålidelig som
mulig. Dog er nogle e-mail-udbydere notorisk svære at levere pålideligt til. Tjek din spam-
mappe for beskeder fra fastcomments.com.

Hvis du [kontakter os](https://fastcomments.com/auth/my-account/help), kan vi normalt give
mere indsigt i, hvorfor du muligvis ikke ser e-mails fra os.