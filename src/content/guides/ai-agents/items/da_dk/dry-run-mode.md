**Tørløb** er sikkerhedstilstanden, som alle nye agenter starter i. Agenten kører end-to-end bortset fra den del, hvor den rører ved dit fællesskab.

### Hvad kører i tørløb

- Triggers udløses normalt.
- Agentens prompt, [community guidelines](#community-guidelines), og [context](#context-options) sammensættes.
- LLM'en bliver kaldt.
- Modellen vælger tool-kald og leverer begrundelser + tillidsvurderinger.
- Kørslen optages med et **Tørløb**-mærke, så den klart adskiller sig fra live-kørsler.

### Hvad kører ikke i tørløb

- Ingen kommentar postes, ingen stemme afgives, ingen kommentar fastgøres/af-fastgøres/låses/oplåses.
- Ingen kommentar markeres som spam, godkendes eller gennemgås.
- Ingen bruger bliver udelukket, advaret eller tildelt en badge.
- Ingen e-mail sendes.
- Intet hukommelse skrives. (Ja - inklusive hukommelse. Tørløbsagenter kan ikke forurene den delte hukommelsespool med hypotetiske beslutninger.)
- Ingen webhooks udløses for tool-aktioner. (Trigger-niveauets `trigger.succeeded` / `trigger.failed` webhooks udløses stadig, og payloaden indeholder `wasDryRun: true`. Se [Webhook Payloads](#webhook-payloads).)

### Hvad det koster

Tørløb kører **det samme LLM-opkald**, som en aktiveret (Enabled) kørsel ville gøre. Tokens opkræves, [budget caps](#budgets-overview) gælder, og kørslerne tæller mod de daglige/månedlige grænser pr. agent og pr. lejer.

Den omkostning er prisen for at få en nøjagtig forhåndsvisning. En "spring LLM-opkaldet over"-tilstand ville ikke give dig noget signal om, hvordan agenten ville opføre sig.

### Gennemgang af tørløbsresultater

I [Run History](#run-history) er tørløbskørsler markeret med **Tørløb**-mærket i statuskolonnen. Handlingerne inde i hver kørsel ser identiske ud med live-handlinger - samme tool-navn, samme argumenter, samme begrundelse og tillid - bortset fra at ingen af dem faktisk fandt sted.

[Analytics page](#analytics-page) opdeler "tørløb vs live" kørsler per måned, så du kan se, hvor meget af din tokenforbrug der blev brugt på observation.

### Skift ud af tørløb

Rediger agenten og ændr **Status** til **Aktiveret**. Næste trigger kører live.

Du kan også skifte den anden vej - Aktiveret tilbage til Tørløb - hvis agenten begynder at gøre ting, du ikke bryder dig om. Der er ingen straf.

### Genafspilninger tvinger tørløb

Funktionen [Test Runs (Replays)](#test-runs-replays) kører agenten mod historiske kommentarer **altid i tørløb**, uanset agentens gemte status. Genafspilninger kan ikke udføre reelle handlinger på tidligere kommentarer. Dette er med vilje - genafspilning er et forhåndsvisningsværktøj, ikke et moderationværktøj.