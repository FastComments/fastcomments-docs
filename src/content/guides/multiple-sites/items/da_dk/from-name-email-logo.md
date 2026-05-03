Nogle gange skal FastComments sende e-mails til dine brugere, især hvis du ikke bruger Secure SSO.

Eksempler på dette inkluderer at bekræfte deres konto eller aktivitet, når de kommenterer for første gang. FastComments
sender dem også notifikationer om svar på deres kommentarer.

Når FastComments sender e-mails til dine brugere, bruger vi som standard From Name og Email `FastComments Robot` og `noreply@fastcomments.com`.

Vi bruger også vores eget logo i footeren af disse e-mails.

Hvis du har FastComments Flex eller Pro, kan dette alt sammen tilpasses på domæneniveau via "My Domains page":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Når du tilpasser logoet, der vises i e-mails, skal du sikre, at den størrelse, du uploader, er den samme størrelse, du ønsker vist i e-mailens footer.

### Når du tilpasser `From Domain`

Hvis du tilpasser `From Domain`, skal e-mailudbydere og klienter vide, at FastComments er autoriseret til at sende e-mails på dine vegne. Ellers vil det at definere `From Domain` uden at følge nedenstående trin sandsynligvis resultere i, at e-mails ryger i spam.

#### 1. Opsæt SPF

For at tillade FastComments sikkert at sende e-mail som dit domæne, skal du sørge for at tilføje en SPF-record, der tillader os at gøre det.

Sørg for, at der er SPF-records, der tillader `mail.fastcomments.com` og `sib.fastcomments.com` at sende mail som dit domæne.

Mere information om, hvordan du gør dette, findes her: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Opsæt DKIM

Udover SPF bør du også opsætte DKIM. Når din DNS-konfiguration er klar, kan du klikke på "Show Advanced" på siden for domænekonfigurationer for at vise DKIM-indstillingerne per domæne.

Du kan også [kalde API'en](/guide-api.html#domain-config-structure) for at indstille DKIM-konfigurationen.

### Afmeldingslinks

Når du bruger SSO, kan afmeldingsfunktionerne, der bruges i e-mails og notifikationer, tilpasses [via the DomainConfigs API](/guide-api.html#domain-config-structure).

### E-mail-linkopfuskning

Hvis dit sites domænerytelse medfører, at notifikations-e-mails lander i spam, kan du rute "view comment"-knapperne gennem `fastcomments.com` i stedet for at linke direkte til din side. Mailudbydere vurderer hvert link i e-mailens indhold i forhold til destinationens omdømme, så når dit domæne bliver flaget, bidrager de bare links til spam-scoret uanset hvor ren din afsenderopsætning er.

Aktivér dette under "Show Advanced" på My Domains page, i sektionen "Email Link Obfuscation". Indstillingen er per domæne.

Når det er aktiveret, omskrives links i mention, reply, new-comment, subscribed-page, profile-comment, og digest e-mails til korte tokens, der omdirigerer til den oprindelige side ved klik. Destinationen er bundet til din tenant: omdirigeringen videresender kun til URL'er hvis host matcher et af dine konfigurerede domæner, og tokens udløber automatisk efter 30 dage.

Den gennemklikkede oplevelse er uændret. Læsere lander stadig på din side med kommentaren scrollet i fokus.