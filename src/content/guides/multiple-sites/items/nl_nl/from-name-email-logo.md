Soms moet FastComments e-mails sturen naar uw gebruikers, vooral als u geen Secure SSO gebruikt.

Voorbeelden hiervan zijn het verifiëren van hun account of activiteit bij het voor het eerst reageren. FastComments
stuurt ze ook meldingen voor antwoorden op hun reacties.

Wanneer FastComments e-mails naar uw gebruikers stuurt, zullen we een standaard Afzendernaam en e-mailadres gebruiken: `FastComments Robot` en `noreply@fastcomments.com`.

We gebruiken daarnaast ons eigen logo in de voettekst van deze e-mails.

Als u FastComments Flex of Pro heeft, kan dit allemaal per domein worden aangepast via de pagina "My Domains page":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Wanneer u het logo aanpast dat in e-mails wordt weergegeven, zorg ervoor dat de grootte die u uploadt dezelfde is als de grootte die u in de voettekst van de e-mail wilt tonen.

### Wanneer u het `From Domain` aanpast

Als u het `From Domain` aanpast, moeten e-mailproviders en -clients weten dat FastComments gemachtigd is om namens u e-mails te verzenden. Anders,
zal het definiëren van het `From Domain` zonder de onderstaande stappen waarschijnlijk resulteren in e-mails die in de spam belanden.

#### 1. SPF instellen

Om FastComments toe te staan veilig e-mails namens uw domein te verzenden, zorgt u ervoor dat u een SPF-record toevoegt dat ons dit toestaat.

Zorg ervoor dat er SPF-records zijn die toestaan dat `mail.fastcomments.com` en `sib.fastcomments.com` e-mail verzenden namens uw domein.

Meer informatie over hoe u dit doet vindt u hier: https://mailtrap.io/blog/multiple-spf-records/

#### 2. DKIM instellen

Naast SPF moet u DKIM instellen. Zodra uw DNS-configuratie klaar is, kunt u op "Geavanceerd weergeven" klikken op de pagina met domeinconfiguraties
om de DKIM-instellingen per domein te tonen.

U kunt ook de [API aanroepen](/guide-api.html#domain-config-structure) om de DKIM-configuratie in te stellen.

### Afmeldlinks

Wanneer u SSO gebruikt, kunnen de afmeldfuncties die in e-mails en meldingen worden gebruikt, worden aangepast [via de DomainConfigs API](/guide-api.html#domain-config-structure).