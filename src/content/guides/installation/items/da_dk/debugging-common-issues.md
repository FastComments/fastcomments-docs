---
Her er nogle symptomer, vi ofte ser, og almindelige løsninger. 

### "This is a demo" Message

Dette vises, når du har kopieret widget-koden fra vores startside, som bruger vores demo
tenant. For at bruge din tenant skal du kopiere widget-koden fra [her](https://fastcomments.com/auth/my-account/get-acct-code).

### "FastComments cannot load on this domain" Error

FastComments skal vide, hvilke domæner der ejes af dig for at godkende forespørgsler tilknyttet
din konto. [Se vores dokumentation](/guide-multiple-sites.html#add-domains-to-account) for at se, hvordan
du løser denne fejl (tilføj blot det præcise subdomæne + domæne til din konto).

Bemærk, at dette kun burde forekomme efter prøveperioden er slut. I prøveperioden vil forespørgsler fra nye domæner
automatisk blive tilføjet til din konto.

### Migrated Comments Not Showing for Custom Installations

Normalt sker dette, når de importerede kommentarer er knyttet til en `Page ID`, og du sender en URL
(eller ingen værdi, i hvilket tilfælde den som standard bruger sidens URL).

Du kan fejlsøge dette ved at [eksportere dine kommentarer](https://fastcomments.com/auth/my-account/manage-data/export) og se på `URL ID` kolonnen (i øjeblikket Kolonne `B`).

Sørg for, at de værdier, du ser i `URL ID` kolonnen, er de samme værdier, du sender til widget-
konfigurationen som `urlId` parameteren.

For yderligere forklaring kan du læse vores [How Comments are Tied to Pages and Articles documentation](/guide-customizations-and-configuration.html#url-id).

Hvis intet hjælper, [kontakt os](https://fastcomments.com/auth/my-account/help).

### Comment Widget Not Showing

Hvis kommentar-widgetten ikke vises, tjek Chrome udviklerkonsollen for fejl.

Ved de fleste fejlkonfigurationer vil kommentar-widgetten i det mindste vise en fejl på siden, hvis den
kan indlæses. At intet vises er normalt et tegn på en script-fejl.

### Desired Configuration Not Working as Expected

Prøv vores [Chrome-udvidelse](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) for at se, hvilken
konfiguration kommentar-widgetten får. Hvis intet hjælper, tag et skærmbillede af, hvad Chrome-udvidelsen siger
og [kontakt os](https://fastcomments.com/auth/my-account/help).

### Comments Missing on Same URL With Different Hash Bang

Som standard bruger FastComments sidens URL for den "bucket", hvor kommentarer gemmes. Hvis dine URL'er indeholder `#hashbangs`, og disse `#hashbangs`
ikke bør være en del af identifikatoren, der identificerer en kommentartråd, kan vi simpelt ignorere hashbang-værdien, for eksempel:

[inline-code-attrs-start title = 'Ignorer hashbangs-eksempel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Bemærk, at efter at have foretaget denne ændring, skal der udføres en migration for eksisterende kommentarer. [Til det, kontakt os.](https://fastcomments.com/auth/my-account/help)

### URL Query Parameters Affecting Widget

Som standard bruger FastComments sidens URL for den "bucket", hvor kommentarer gemmes. Hvis dine URL'er indeholder forespørgselsparametre,
der ikke bør være en del af identifikatoren, som identificerer en kommentartråd, kan vi simpelt ignorere dem, for eksempel:

[inline-code-attrs-start title = 'Ignorer forespørgselsparametre'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Bemærk, at efter at have foretaget denne ændring, skal der udføres en migration for eksisterende kommentarer. [Til det, kontakt os.](https://fastcomments.com/auth/my-account/help)

### Not Receiving Emails

Hos FastComments gør vi en stor indsats for at sikre, at vores e-mails leveres så pålideligt som
muligt. Dog er nogle e-mailudbydere notorisk svære at levere til pålideligt vis. Tjek din spam-
mappe for beskeder fra fastcomments.com.

Hvis du [kontakter os](https://fastcomments.com/auth/my-account/help) kan vi som regel give
mere indsigt i, hvorfor du måske ikke modtager e-mails fra os.

---