En **trigger** er en begivenhed, der vækker en agent. Hver agent kan have én eller flere triggere defineret.

### Den fulde liste

| Trigger | Hvornår den udløses |
|---|---|
| [Comment Added](#trigger-comment-add) | En ny kommentar bliver oprettet. |
| [Comment Edited](#trigger-comment-edit) | En kommentar redigeres. Den tidligere tekst inkluderes i agentens kontekst. |
| [Comment Deleted](#trigger-comment-delete) | En kommentar slettes. |
| [Comment Pinned](#trigger-comment-pin) | En kommentar fastgøres (af enhver, inkl. en moderator eller en anden agent). |
| [Comment Unpinned](#trigger-comment-unpin) | En kommentar fjernes fra fastgørelse. |
| [Comment Locked](#trigger-comment-lock) | En kommentar låses (ingen yderligere svar tilladt). |
| [Comment Unlocked](#trigger-comment-unlock) | En kommentar låses op. |
| [Comment Crosses Vote Threshold](#trigger-comment-vote-threshold) | En kommentar får et nettoantal stemmer, der når den konfigurerede tærskel. |
| [Comment Crosses Flag Threshold](#trigger-comment-flag-threshold) | En kommentarens flagtælling når nøjagtigt den konfigurerede tærskel. |
| [User Posts First Comment](#trigger-new-user-first-comment) | En bruger poster sin første kommentar på dette site. |
| [Comment Auto-Spammed](#trigger-comment-auto-spammed) | En kommentar bliver automatisk markeret som spam af spam-motoren. |
| [Moderator Reviews Comment](#trigger-moderator-reviewed) | En moderator markerer en kommentar som gennemgået. |
| [Moderator Approves Comment](#trigger-moderator-approved) | En moderator godkender en kommentar. |
| [Moderator Marks Spam](#trigger-moderator-spammed) | En moderator markerer en kommentar som spam. |
| [Moderator Awards Badge](#trigger-moderator-awarded-badge) | En moderator tildeler en badge til en bruger. |

### Flere triggere pr. agent

En agent kan abonnere på en hvilken som helst kombination af triggere - [Moderator template](#template-moderator) abonnerer f.eks. på både `COMMENT_ADD` og `COMMENT_FLAG_THRESHOLD`. Hvert event udløser agenten én gang med eventets kontekst.

### Hvad forhindrer, at en agent udløses

En abonneret trigger udløser **ikke** agenten, hvis nogen af følgende gælder:

- Agentens [status](#status-states) er **Deaktiveret**.
- Agentens [URL- eller lokalitetsomfang](#scope-url-locale) matcher ikke den udløsende kommentar.
- Agentens [daglige, månedlige eller rate-limit-budget](#budgets-overview) er opbrugt - udløseren registreres som **afvist** med en årsag. Se [Årsager til afvisning](#drop-reasons).
- Agentens samtidighedsgrænse er nået (begrænset pr. agent).
- Agentens tenant har ugyldig fakturering.
- Den udløsende handling blev udført af en bot eller en anden agent (loop-forebyggelse).
- Udløseren vedrørte en kommentar, der allerede er blevet behandlet af denne agent inden for deduplikeringsvinduet.

Når en abonneret trigger udløses med succes, viser agentens [Run History](#run-history) en række med status **Startet**, som skifter til **Succes** eller **Fejl**, når kørslen er færdig.

### Stemme- og flagtærskler

To triggere - **Comment Crosses Vote Threshold** og **Comment Crosses Flag Threshold** - kræver en numerisk tærskel på redigeringsformularen. Triggeren affyres i det øjeblik tællingen krydser den konfigurerede værdi (mere specifikt affyres flag-tærskel-triggeren når `flagCount === flagThreshold`, så at vælge 1 betyder "udløs ved det første flag", og at vælge 5 betyder "udløs når det femte flag ankommer").

### Udskudte triggere

Enhver trigger kan udskydes, så agenten kører senere, for eksempel efter at stemmer/flag/svar har fået tid til at stabilisere sig. Se [Udskudte triggere](#trigger-deferred-delay).

### Loop-forebyggelse

For at forhindre uendelige loops bærer kommentarer, der **er skrevet af en agent**, et `botId`. Nye-kommentar-triggere ignorerer kommentarer med et `botId`.

Den samlede effekt: agenter kan handle som svar på *menneskelige* handlinger i din tenant, men agent-udførte handlinger udløser aldrig nogen agent-triggere. Dette gælder for alle trigger-typer.

### REPLAY: den interne trigger

Der findes også en intern `REPLAY` trigger-type, som bruges af funktionen [Testkørsler (Replays)](#test-runs-replays). Du kan ikke vælge den på redigeringsformularen - den findes, så replay-kørsler mærkes særskilt i kørselshistorikken og udelukkes fra live-kørselsvisninger.