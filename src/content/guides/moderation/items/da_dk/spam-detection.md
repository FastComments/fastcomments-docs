Som standard leveres FastComments med en trænbar spamdetektion.

Når du modererer kommentarer og markerer dem som **Spam**, eller markerer kommentarer der automatisk er fundet som **Spam** som **Not Spam**, vil spamdetektionssystemet lære af disse handlinger for mere præcist at bestemme, hvad du ønsker skal være spam.

Kommentarer markeret som **Spam** vil ikke blive godkendt automatisk, så de vil ikke blive vist, før de eksplicit er markeret som **Not Spam**.

Spam Detection kan deaktiveres via Comment Moderation Settings page.

### Forskellige spamdetektorer

FastComments understøtter tre måder at detektere spam på:

1. En traditionel Naïve-Bayes-klassifikator, der løbende trænes, og som deles på tværs af alle FastComments.com tenants.
2. En traditionel Naïve-Bayes-klassifikator, der løbende trænes, og som er **isolated** til din tenant.
3. Brug af ChatGPT 4.

Alle har adgang til de delte og isolerede Naïve-Bayes-klassifikatorer.

ChatGPT 4-muligheden kan vælges i Comment Moderation Settings page, hvis du er på Flex billing, da den faktureres baseret på anvendte tokens.

### Trust Factor

FastComments justerer spamfilteret for en bruger baseret på, hvor meget vedkommende er betroet for det pågældende site.

For eksempel, hvis administratorer har fastgjort mange af deres kommentarer, så er de sandsynligvis en meget pålidelig bruger. Eller hvis de har været medlem af sitet i lang tid og har mange kommentarer, kan deres trust factor også være høj.

### SSO

Kommentarer indsendt af SSO-brugere kan blive betragtet som spam og vil blive kontrolleret som sådan. Undtagelsen er, hvis SSO-brugeren har samme e-mail som en tenant-bruger, der har en eller flere af følgende tilladelser:

- Account Owner
- Super Admin
- Comment Moderator Admin

SSO-brugere med disse tilladelser vil ikke få deres kommentarer tjekket for spam.

### Gentagne beskeder

FastComments vil opdage og forhindre gentagne beskeder. Det vil også opdage gentagne beskeder, der er meget ens, for at hjælpe med at forhindre spam. Dette kan ikke deaktiveres, da det forhindrer vores platform i at blive misbrugt. Hvis du har en høj trust factor, tages dette i betragtning ved forebyggelse af gentagne beskeder.