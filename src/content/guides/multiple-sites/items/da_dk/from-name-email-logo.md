Nogle gange er FastComments nødt til at sende e-mails til dine brugere, især hvis du ikke bruger Secure SSO.

Eksempler på dette omfatter at bekræfte deres konto eller aktivitet, når de kommenterer første gang. FastComments sender dem også notifikationer ved svar på deres kommentarer.

Når FastComments sender e-mails til dine brugere, bruger vi som standard et afsendernavn og en e-mail på `FastComments Robot` og `noreply@fastcomments.com`.

Vi bruger også vores eget logo i bunden af disse e-mails.

Hvis du har FastComments Flex eller Pro, kan dette alle tilpasses pr. domæne via siden "My Domains":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Når du tilpasser logoet, der vises i e-mails, skal du sikre, at størrelsen på det, du uploader, er den samme som den, du ønsker vist i e-mailens bund.

### When Customizing The `From Domain`

Hvis du tilpasser `From Domain`, skal e-mailudbydere og -klienter vide, at FastComments er autoriseret til at sende e-mails på dine vegne. Ellers vil det sandsynligvis resultere i, at e-mails havner i spam, hvis du definerer `From Domain` uden at følge trinene nedenfor.

#### 1. Opsæt SPF

For at tillade FastComments sikkert at sende e-mails på vegne af dit domæne, skal du sikre, at du tilføjer en SPF-post, der tillader os det.

Sørg for, at der findes SPF-poster, som tillader `mail.fastcomments.com` og `sib.fastcomments.com` at sende mail som dit domæne.

Mere information om, hvordan man gør dette, findes her: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Opsæt DKIM

Ud over SPF bør du opsætte DKIM. Når din DNS-konfiguration er klar, kan du klikke på "Show Advanced" på siden med domænekonfigurationer for at vise DKIM-indstillinger pr. domæne.

Du kan også [invoke the API](/guide-api.html#domain-config-structure) for at indstille DKIM-konfigurationen.

### Unsubscribe Links

Når du bruger SSO, kan afmeldingsfunktionerne, som bruges i e-mails og notifikationer, tilpasses [via the DomainConfigs API](/guide-api.html#domain-config-structure).