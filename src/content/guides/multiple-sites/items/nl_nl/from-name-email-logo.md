Soms moet FastComments e‑mails naar uw gebruikers sturen, vooral als u geen Secure SSO gebruikt.

Voorbeelden hiervan zijn het verifiëren van hun account of activiteit wanneer ze voor de eerste keer een reactie plaatsen. FastComments
stuurt hen ook meldingen voor antwoorden op hun reacties.

Wanneer FastComments e‑mails naar uw gebruikers stuurt, gebruiken we een standaard **From Name** en **From Email** van `FastComments Robot` en `noreply@fastcomments.com`.

We gebruiken ook ons eigen logo in de voettekst van deze e‑mails.

Als u FastComments Flex of Pro heeft, kan dit allemaal per domein worden aangepast via de **My Domains**‑pagina:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; alt='Per-domein e-mailinstellingen formulier met de velden Van Naam, Van E-mail en logo uploadvelden'; title='Aanpassen van Van Naam, E-mail en Logo' app-screenshot-end]

Wanneer u het logo dat in e‑mails wordt weergegeven aanpast, zorg er dan voor dat de grootte die u uploadt dezelfde is als de grootte die u in de voettekst van de e‑mail wilt tonen.

### Bij het aanpassen van het `From Domain`

Als u het `From Domain` aanpast, moeten e‑mailproviders en -clients weten dat FastComments gemachtigd is om e‑mails namens u te verzenden. Anders
zal het definiëren van het `From Domain` zonder de onderstaande stappen waarschijnlijk resulteren in e‑mails die in spam terechtkomen.

#### 1. Instellen van SPF

Om FastComments veilig e‑mail te laten verzenden als uw domein, moet u een SPF‑record toevoegen dat ons dit toestaat.

Zorg ervoor dat er SPF‑records zijn die `mail.fastcomments.com` en `sib.fastcomments.com` toestaan e‑mail te verzenden als uw domein.

Meer informatie over hoe u dit doet vindt u hier: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Instellen van DKIM

Naast SPF moet u DKIM instellen. Zodra uw DNS‑configuratie klaar is, kunt u op **Show Advanced** klikken op de pagina met domeinconfiguraties
om de DKIM‑instellingen per domein te bekijken.

U kunt ook de [API aanroepen](/guide-api.html#domain-config-structure) om de DKIM‑configuratie in te stellen.

### Afmeldlinks

Wanneer u SSO gebruikt, kunnen de afmeldfuncties die in e‑mails en meldingen worden gebruikt, worden aangepast [via de DomainConfigs API](/guide-api.html#domain-config-structure).

### E‑mail linkobfuscatie

Als de reputatie van het domein van uw site ervoor zorgt dat meldings‑e‑mails in spam terechtkomen, kunt u de “view comment”‑knoppen via `fastcomments.com` laten verlopen in plaats van direct naar uw pagina te linken. Mailbox‑providers scoren elke link in de e‑mailtekst op basis van de reputatie van de bestemming, dus wanneer uw domein wordt gemarkeerd, dragen de directe links bij aan de spamscore, ongeacht hoe schoon uw verzendconfiguratie is.

Schakel dit in onder **Show Advanced** op de My Domains‑pagina, in de sectie **Email Link Obfuscation**. Deze instelling is per domein.

Wanneer ingeschakeld, worden links in vermeldingen, antwoorden, nieuwe reacties, abonnementspagina’s, profielreacties en digest‑e‑mails herschreven naar korte tokens die bij klikken naar de oorspronkelijke pagina omleiden. De bestemming is gebonden aan uw tenant: de omleiding werkt alleen naar URL’s waarvan de host overeenkomt met een van uw geconfigureerde domeinen, en tokens verlopen automatisch na 30 dagen.

De door‑geklikte ervaring blijft ongewijzigd. Lezers komen nog steeds op uw pagina terecht met de reactie in beeld gescrold.