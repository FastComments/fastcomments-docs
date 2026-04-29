En **godkendelse** er et værktøjskald i kø, som kræver, at et menneske godkender eller afviser, før platformen udfører det.

### Konfiguration af godkendelser

På agentens redigeringsformular viser sektionen **Approvals** hvert værktøj, som agenten har lov til at påkalde (allowlisten), og lader dig sætte flueben ved de værktøjer, der skal gennemgås af et menneske. Værktøjer uden flueben udføres med det samme. Værktøjer med flueben sættes i kø.

Værktøjer, der ikke er tilladte, bliver *afvist direkte*, ikke sat i kø - godkendelser gælder kun for værktøjer, der ellers er tilladt.

### Hvad sker der, når en gated handling udløses

1. Agenten vælger et værktøjskald (f.eks. `ban_user`) med argumenter, begrundelse og konfidens.
2. I stedet for at køre det, sætter platformen en godkendelse i kø i `PENDING`-tilstand med værktøjets navn, argumenter, begrundelse, konfidens og et kontekstøjebliksbillede, der beskriver triggeren, der producerede det.
3. Notifikationer sendes til anmeldere (se [Godkendelsesnotifikationer](#approval-notifications)).
4. Agentens kørsel afsluttes og registreres - handlingen vises som **Pending approval** i [Visning af kørselsdetaljer](#run-detail-view).

### Gennemgang af godkendelser

Godkendelses-indbakken på [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) viser ventende, godkendte, afviste og udførelsesfejlede godkendelser. For hver:

- **Værktøjsnavn og argumenter** - præcis hvad agenten ønsker at gøre.
- **Agentens begrundelse** - den begrundelse agenten leverede.
- **Konfidens** - agentens egenvurderede konfidens, 0.0 til 1.0.
- **Kontekstsnapshot** - kommentaren, siden og brugeren, som handlingen retter sig mod.

To knapper: **Godkend** og **Afvis**. Godkend kører faktisk værktøjet; Afvis forkaster det.

### Tilstande for godkendelsesstatus

En godkendelse bevæger sig gennem disse tilstande:

| Status | Betydning |
|---|---|
| `PENDING` | Afventer menneskelig beslutning. |
| `APPROVED` | Et menneske godkendte, og handlingen kørte. |
| `REJECTED` | Et menneske afviste. Handlingen blev ikke kørt. |
| `EXECUTION_FAILED` | Et menneske godkendte, men handlingen fejlede (f.eks. var målekommentaren allerede slettet). |
| `EXECUTING` | Transient: et menneske klikkede Godkend, og handlingen kører. Bruges til at serialisere samtidige godkendelsesklik, så et værktøj med reelle sideeffekter aldrig kører to gange. |

`EXECUTING`-tilstanden er vigtig, når to anmeldere klikker Godkend samtidigt - én vinder, den anden ser, at godkendelsen allerede er rykket videre.

### Hvad der bliver ryddet op

Ventende godkendelser forbliver ventende, indtil der træffes en beslutning. De udløber automatisk **efter 90 dage** fra oprettelsen. Godkendelser i enhver anden tilstand fjernes også efter 90 dage af hensyn til lagerhygiejne.

Felterne "decided by" / "decided at" / "executed at" / "execution result" på godkendelsen udfyldes, efterhånden som godkendelsen bevæger sig gennem tilstandene - alle synlige i indbakke-detaljevisningen.

### Webhook-integration

To webhook-hændelser udløses, efterhånden som godkendelser ændrer tilstand:

- **`approval.requested`** - ved indsættelse i PENDING.
- **`approval.decided`** - ved overgang til APPROVED, REJECTED eller EXECUTION_FAILED.

Begge er signeret som alle andre webhooks. Se [Webhook Events](#webhook-events) og [Webhook Payloads](#webhook-payloads).

### Hærdning: known-tool gate

Godkendelsessystemet nægter at sætte i kø ethvert værktøjsnavn, der ikke er et anerkendt agentværktøj. Dette er en defense-in-depth-kontrol: selv hvis en fremtidig kodevej sender et LLM-afledt værktøjsnavn ind i godkendelsesflowet, kan den streng aldrig lande som en køet godkendelse. Sættet af kendte værktøjsnavne er det samme sæt, der er opført i [Tools Reference](#tools-overview).

### Typiske gating-mønstre

- **Helt ny moderationsagent** - gate `ban_user`, `mark_comment_spam`, `mark_comment_approved`, `send_email`. Hold øje med indbakken i et par uger, og fjern derefter gating fra værktøjer med lav fejlrate.
- **Langsigtet moderationsagent** - behold `ban_user` og alle irreversible handlinger (`deleteAllUsersComments`, `banIP`) gated for altid.
- **EU-regionen** - `ban_user` er låst af Artikel 17 uanset, hvad du markerer. Se [EU DSA Artikel 17 Overholdelse](#eu-dsa-compliance).

### Hvad godkendelser **ikke** gør

- De forsinker ikke agentens andre værktøjskald. Agentens kørsel kan påkalde flere værktøjer, og kun de gatede sættes i kø - resten udføres som normalt.
- De ruller ikke agentens kørsel tilbage, hvis mennesket afviser. Den ikke-gatede del af kørslen er allerede udført.