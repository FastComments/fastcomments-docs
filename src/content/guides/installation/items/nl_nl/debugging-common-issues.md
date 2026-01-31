Hier volgen enkele symptomen die we vaak tegenkomen, en gebruikelijke oplossingen. 

### "Dit is een demo" Melding

Dit wordt getoond wanneer je de widgetcode van onze startpagina hebt gekopieerd, die onze demo-tenant gebruikt. Om je eigen tenant te gebruiken, kopieer je de widgetcode van [hier](https://fastcomments.com/auth/my-account/get-acct-code).

### Foutmelding "FastComments kan niet laden op dit domein"

FastComments moet weten welke domeinen van jou zijn om verzoeken die aan jouw account zijn gekoppeld te verifiëren. [Raadpleeg onze documentatie](/guide-multiple-sites.html#add-domains-to-account) om te zien hoe je deze fout kunt oplossen (voeg simpelweg het exacte subdomein + domein aan je account toe).

Houd er rekening mee dat dit alleen zou moeten optreden nadat de proefperiode is afgelopen. Tijdens de proefperiode worden verzoeken van nieuwe domeinen automatisch aan je account toegevoegd.

### Gemigreerde reacties worden niet weergegeven bij aangepaste installaties

Meestal gebeurt dit wanneer de geïmporteerde reacties zijn gekoppeld aan een `Page ID`, en jij een URL doorgeeft (of geen waarde doorgeeft, in welk geval dit standaard de pagina-URL is).

Je kunt dit debuggen door je reacties te [exporteren](https://fastcomments.com/auth/my-account/manage-data/export) en de `URL ID`-kolom te bekijken (momenteel Kolom `B`).

Zorg ervoor dat de waarden die je in de `URL ID`-kolom ziet dezelfde waarden zijn die je doorgeeft aan de widgetconfiguratie als de `urlId`-parameter.

Voor nadere uitleg, lees onze [How Comments are Tied to Pages and Articles documentation](/guide-customizations-and-configuration.html#url-id).

Als niets helpt, [neem contact met ons op](https://fastcomments.com/auth/my-account/help).

### Reactie-widget wordt niet weergegeven

Als de reactie-widget niet wordt weergegeven, controleer dan de Chrome-ontwikkelaarsconsole op fouten.

Bij de meeste verkeerd geconfigureerde situaties zal de reactie-widget in ieder geval een fout op de pagina tonen als deze kan laden. Niets zien is meestal een aanwijzing voor een scriptfout.

### Gewenste configuratie werkt niet zoals verwacht

Probeer onze [Chrome-extensie](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) om te zien welke configuratie aan de reactie-widget wordt doorgegeven. Als alles faalt, maak een screenshot van wat de Chrome-extensie aangeeft en [neem contact met ons op](https://fastcomments.com/auth/my-account/help).

### Reacties ontbreken op dezelfde URL met verschillende hashbangs

Standaard gebruikt FastComments de pagina-URL voor de "bucket" waar reacties worden opgeslagen. Als je URL's `#hashbangs` bevatten, en deze `#hashbangs` geen deel zouden moeten uitmaken van de identificatie van een reactie-thread, kunnen we de hashbang-waarde eenvoudig negeren, bijvoorbeeld:

[inline-code-attrs-start title = 'Voorbeeld Hashbangs negeren'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Houd er rekening mee dat na het aanbrengen van deze wijziging een migratie moet worden uitgevoerd voor bestaande reacties. [Neem hiervoor contact met ons op.](https://fastcomments.com/auth/my-account/help)

### URL-queryparameters die de widget beïnvloeden

Standaard gebruikt FastComments de pagina-URL voor de "bucket" waar reacties worden opgeslagen. Als je URL's queryparameters bevatten die geen deel zouden moeten uitmaken van de identificatie van een reactie-thread, kunnen we deze eenvoudig negeren, bijvoorbeeld:

[inline-code-attrs-start title = 'Queryparameters negeren'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Houd er rekening mee dat na het aanbrengen van deze wijziging een migratie moet worden uitgevoerd voor bestaande reacties. [Neem hiervoor contact met ons op.](https://fastcomments.com/auth/my-account/help)

### Geen e-mails ontvangen

Bij FastComments doen we veel moeite om ervoor te zorgen dat onze e-mailbezorging zo betrouwbaar mogelijk is. Sommige e-mailproviders zijn echter berucht moeilijk te bereiken. Controleer je spammap op berichten van fastcomments.com.

Als je [contact met ons opneemt](https://fastcomments.com/auth/my-account/help), kunnen we gewoonlijk meer inzicht geven in waarom je mogelijk geen e-mails van ons ziet.

---