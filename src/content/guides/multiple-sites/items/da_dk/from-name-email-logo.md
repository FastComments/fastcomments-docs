Nogle gange er FastComments nødt til at sende e-mails til dine brugere, især hvis du ikke bruger Secure SSO.

Eksempler på dette er verifikation af deres konto eller aktivitet, når de kommenterer for første gang. FastComments
sender dem også notifikationer om svar på deres kommentarer.

Når FastComments sender e-mails til dine brugere, bruger vi et standard From Name og Email på `FastComments Robot` og `noreply@fastcomments.com`.

Vi bruger også vores eget logo i sidefoden af disse e-mails.

Hvis du har FastComments Flex eller Pro, kan det hele tilpasses pr. domæne via "My Domains-siden":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; alt='Formular til e-mailindstillinger pr. domæne med felterne From Name, From Email og logo-upload'; title='Tilpasning af From Name, Email og logo' app-screenshot-end]

Når du tilpasser logoet, der vises i e-mails, skal du sørge for, at den størrelse, du uploader, er den samme størrelse, som du vil vise i sidefoden af e-mailen.

### Ved tilpasning af `From Domain`

Hvis du tilpasser `From Domain`, skal e-mailudbydere og -klienter vide, at FastComments er autoriseret til at sende e-mails på dine vegne. Ellers vil
det sandsynligvis resultere i, at e-mails ryger i spam, hvis man definerer `From Domain` uden at følge nedenstående trin.

#### 1. Opsæt SPF

For at tillade FastComments sikkert at sende e-mail som dit domæne, skal du sørge for at tilføje en SPF-record, der tillader os at gøre det.

Sørg for, at der er SPF-records, der tillader `mail.fastcomments.com` og `sib.fastcomments.com` at sende post som dit domæne.

Mere information om, hvordan du gør dette, findes her: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Opsæt DKIM

Ud over SPF bør du opsætte DKIM. Når din DNS-konfiguration er klar, kan du klikke på "Show Advanced" på domænekonfigurationssiden
for at vise DKIM-indstillingerne pr. domæne.

Du kan også [kalde API'en](/guide-api.html#domain-config-structure) for at sætte DKIM-konfigurationen op.

### Unsubscribe-links

Ved brug af SSO kan unsubscribe-funktionerne, der bruges i e-mails og notifikationer, tilpasses [via DomainConfigs API'en](/guide-api.html#domain-config-structure).

### Email Link Obfuscation

Hvis dit websteds domæne-omdømme får notifikations-e-mails til at ryge i spam, kan du route "se kommentar"-knapperne gennem `fastcomments.com` i stedet for at linke direkte til din side. Mailbox-udbydere vurderer hvert link i e-mailens brødtekst mod destinationens omdømme, så når dit domæne bliver markeret, bidrager de blotte links til spam-scoren uanset hvor ren din afsendelsesopsætning er.

Aktivér dette under "Show Advanced" på My Domains-siden, i sektionen "Email Link Obfuscation". Indstillingen er pr. domæne.

Når det er aktiveret, omskrives links i mention-, reply-, new-comment-, subscribed-page-, profile-comment- og digest-e-mails til korte tokens, der omdirigerer til den oprindelige side ved klik. Destinationen er bundet til din tenant: omdirigeringen videresender kun til URL'er, hvis host matcher et af dine konfigurerede domæner, og tokens udløber automatisk efter 30 dage.

Oplevelsen efter klik er uændret. Læsere lander stadig på din side med kommentaren scrollet ind i syne.