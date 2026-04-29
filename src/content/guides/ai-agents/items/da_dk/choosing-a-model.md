Hver agent kører mod en af to LLM-modeller, valgt i **Model**-sektionen på redigeringsformularen.

### De to muligheder

- **GLM 5.1 (DeepInfra) - Mere intelligent, en smule langsommere** - standardvalget. Højere ræsonnementskvalitet, en smule langsommere pr. kald. Anbefales til moderationsagtige agenter (`Moderator`-skabelon, alt der kalder `ban_user` eller `mark_comment_spam`), hvor omkostningen ved et forkert kald er høj.

- **GPT-OSS 120B Turbo (DeepInfra) - Hurtigere** - hurtigere pr. kald, lavere latenstid. Anbefales til høj-volumen, lavrisiko-agenter (velkomsthilsen, trådfæstning), hvor du ønsker svar inden for sekunder, og konsekvenserne af et forkert kald er mindre.

Begge modeller understøtter function calling, begge kører via den samme OpenAI-kompatible API, og begge deler de samme skemaer pr. værktøj - så du kan skifte en gemt agent mellem dem når som helst uden andre konfigurationsændringer.

### Forskelle i omkostninger

De to modeller har forskellige omkostninger pr. token. Agentens [budget caps](#budgets-overview) er angivet i din kontovaluta, ikke i tokens, så et skift fra den ene model til den anden ændrer, hvor mange kørsler der passer inden for dine daglige og månedlige grænser. Siden [Run History](#run-history) viser pr.-kørsel-omkostningen i din valuta, når en kørsel er afsluttet - at observere et par kørsler efter et skift er den nemmeste måde at vurdere den nye forbrændingsrate på.

### Tokens pr. kørsel

Modelens forbrug af responstokens logges ved hver trigger som **tokensUsed**. Det medtages i `trigger.succeeded`- og `trigger.failed`-webhook-payloads (se [Webhook Payloads](#webhook-payloads)) og vises i [Run Detail View](#run-detail-view). Mængden afhænger af:

- Hvor meget [Context](#context-options) du inkluderer - tråd-kontekst, brugerhistorik og sidemetadata tilføjer alle tokens.
- Hvor lange dine [Initial prompt](#personality-prompt) og [Community guidelines](#community-guidelines) er.
- Hvor mange værktøjer agenten kalder i en enkelt kørsel (hvert værktøjskald og dets resultat går tur-retur gennem modellen).

**Maksimalt antal tokens pr. trigger** (default 20,000) er den øvre grænse pr. kørsel, sat per agent.

### Skifte modeller

Du kan skifte modeller i redigeringsformularen når som helst. Eksisterende kørselslog og analyser beholder deres oprindelige token- og omkostningstal - de registreres ved kørslen. Den nye model gælder kun for kørsler, der starter efter du har gemt.

Der findes ikke en 'use whichever model is cheaper' mulighed. Valget er eksplicit pr. agent.