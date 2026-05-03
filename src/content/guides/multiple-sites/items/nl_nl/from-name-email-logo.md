Soms moet FastComments e-mails naar uw gebruikers sturen, vooral als u geen Secure SSO gebruikt.

Voorbeelden hiervan zijn het verifiëren van hun account of activiteit bij hun eerste reactie. FastComments stuurt ze ook meldingen voor reacties op hun opmerkingen.

Wanneer FastComments e-mails naar uw gebruikers stuurt, gebruiken we standaard de afzendernaam en e-mailadres `FastComments Robot` en `noreply@fastcomments.com`.

We gebruiken ook ons eigen logo in de voettekst van deze e-mails.

Als u FastComments Flex of Pro heeft, kan dit allemaal per domein worden aangepast via de pagina "My Domains":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Wanneer u het logo in e-mails aanpast, zorg ervoor dat de grootte die u uploadt dezelfde is als de grootte die u in de voettekst van de e-mail wilt weergeven.

### Wanneer u de `From Domain` aanpast

Als u de `From Domain` aanpast, moeten e-mailproviders en -clients weten dat FastComments gemachtigd is om namens u e-mails te verzenden. Als u de `From Domain` instelt en onderstaande stappen niet volgt, is de kans groot dat e-mails in de spam belanden.

#### 1. SPF instellen

Om FastComments veilig e-mails te laten verzenden namens uw domein, moet u een SPF-record toevoegen dat ons hiervoor toestemming geeft.

Zorg ervoor dat er SPF-records zijn die toestaan dat `mail.fastcomments.com` en `sib.fastcomments.com` e-mail verzenden namens uw domein.

Meer informatie over hoe u dit doet vindt u hier: https://mailtrap.io/blog/multiple-spf-records/

#### 2. DKIM instellen

Naast SPF moet u ook DKIM instellen. Zodra uw DNS-configuratie gereed is, kunt u op "Toon geavanceerde opties" klikken op de pagina met domeinconfiguraties om de DKIM-instellingen per domein weer te geven.

U kunt ook de [API](/guide-api.html#domain-config-structure) aanroepen om de DKIM-configuratie in te stellen.

### Afmeldlinks

Wanneer u SSO gebruikt, kunnen de afmeldfuncties die in e-mails en meldingen worden gebruikt worden aangepast [via de DomainConfigs API](/guide-api.html#domain-config-structure).

### Verduistering van e-maillinks

Als de domeinreputatie van uw site ervoor zorgt dat meldings-e-mails in de spam terechtkomen, kunt u de knoppen "view comment" via `fastcomments.com` laten lopen in plaats van direct naar uw pagina te linken. Mailproviders beoordelen elke link in de e-mailinhoud op basis van de reputatie van de bestemming, dus als uw domein wordt gemarkeerd dragen de blote links bij aan de spam-score, ongeacht hoe goed uw verzendconfiguratie is.

Schakel dit in onder "Toon geavanceerde opties" op de pagina "My Domains", in de sectie "Verduistering van e-maillinks". De instelling geldt per domein.

Wanneer ingeschakeld worden links in mention, reply, new-comment, subscribed-page, profile-comment en digest e-mails herschreven naar korte tokens die bij klikken doorgaan naar de originele pagina. De bestemming is gebonden aan uw tenant: de redirect stuurt alleen door naar URL's waarvan de host overeenkomt met een van uw geconfigureerde domeinen, en tokens verlopen automatisch na 30 dagen.

De ervaring na het doorklikken blijft ongewijzigd. Lezers landen nog steeds op uw pagina waarbij de opmerking in beeld wordt gebracht.