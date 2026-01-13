Hier zijn enkele symptomen die we vaak tegenkomen en veelvoorkomende oplossingen.

### "This is a demo" Bericht

Dit wordt getoond wanneer u de widgetcode van onze homepage hebt gekopieerd, die onze demo-tenant gebruikt. Om uw tenant te gebruiken, kopieer de widgetcode van [hier](https://fastcomments.com/auth/my-account/get-acct-code).

### "FastComments cannot load on this domain" Fout

FastComments moet weten welke domeinen van u zijn om verzoeken te authenticeren die aan uw account zijn gekoppeld. [Bekijk onze documentatie](/guide-multiple-sites.html#add-domains-to-account) om te zien hoe u deze fout kunt oplossen (voeg simpelweg het exacte subdomein + domein toe aan uw account).

Let op dat dit alleen zou moeten optreden nadat de proefperiode is afgelopen. Tijdens de proefperiode worden alle verzoeken van nieuwe domeinen automatisch aan uw account toegevoegd.

### Gemigreerde Reacties Worden Niet Getoond voor Aangepaste Installaties

Dit gebeurt meestal wanneer de geïmporteerde reacties gekoppeld zijn aan een `Page ID`, en u een URL doorgeeft (of geen waarde, in welk geval de pagina-URL standaard wordt gebruikt).

U kunt dit debuggen door [uw reacties te exporteren](https://fastcomments.com/auth/my-account/manage-data/export) en de `URL ID` kolom te bekijken (momenteel Kolom `B`).

Zorg ervoor dat de waarden die u ziet in de `URL ID` kolom dezelfde waarden zijn die u doorgeeft aan de widgetconfiguratie als de `urlId` parameter.

Voor verdere uitleg, probeer onze [documentatie over Hoe Reacties aan Pagina's en Artikelen zijn Gekoppeld](/guide-customizations-and-configuration.html#url-id) te lezen.

Als al het andere faalt, [neem contact met ons op](https://fastcomments.com/auth/my-account/help).

### Reactiewidget Wordt Niet Getoond

Als de reactiewidget niet wordt getoond, controleer de Chrome-ontwikkelaarsconsole op fouten.

Voor de meeste misconfiguraties zal de reactiewidget ten minste een fout op de pagina tonen als deze kan laden. Niets zien is meestal een indicatie van een scriptfout.

### Gewenste Configuratie Werkt Niet Zoals Verwacht

Probeer onze [Chrome-extensie](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) om te zien welke configuratie aan de reactiewidget wordt doorgegeven. Als alles faalt, maak een screenshot van wat de Chrome-extensie zegt en [neem contact met ons op](https://fastcomments.com/auth/my-account/help).

### Reacties Ontbreken op Dezelfde URL met Andere Hash Bang

Standaard gebruikt FastComments de pagina-URL voor de "bucket" waar reacties worden opgeslagen. Als uw URL's `#hashbangs` bevatten, en deze `#hashbangs` geen deel zouden moeten uitmaken van de identifier die een reactiethread identificeert, kunnen we simpelweg de hash bang-waarde negeren, bijvoorbeeld:

[inline-code-attrs-start title = 'Ignore Hash Bangs Example'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Let op dat na het maken van deze wijziging een migratie moet worden uitgevoerd voor bestaande reacties. [Neem hiervoor contact met ons op.](https://fastcomments.com/auth/my-account/help)

### URL Query Parameters Beïnvloeden Widget

Standaard gebruikt FastComments de pagina-URL voor de "bucket" waar reacties worden opgeslagen. Als uw URL's query-parameters bevatten die geen deel zouden moeten uitmaken van de identifier die een reactiethread identificeert, kunnen we ze simpelweg negeren, bijvoorbeeld:

[inline-code-attrs-start title = 'Ignore Query Parameters'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Let op dat na het maken van deze wijziging een migratie moet worden uitgevoerd voor bestaande reacties. [Neem hiervoor contact met ons op.](https://fastcomments.com/auth/my-account/help)

### Geen E-mails Ontvangen

Bij FastComments steken we veel werk in het zo betrouwbaar mogelijk maken van onze e-mailbezorging. Sommige e-mailproviders zijn echter notoir moeilijk betrouwbaar te bereiken. Controleer uw spammap op berichten van fastcomments.com.

Als u [contact met ons opneemt](https://fastcomments.com/auth/my-account/help) kunnen we meestal meer inzicht geven in waarom u mogelijk geen e-mails van ons ontvangt.
