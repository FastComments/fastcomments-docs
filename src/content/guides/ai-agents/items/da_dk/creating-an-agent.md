Fra [siden for AI-agenter](https://fastcomments.com/auth/my-account/ai-agents) kan du oprette en agent på to måder:

- **Fra en skabelon.** Klik **Browse templates** og vælg en af de fire indbyggede startagenter. Formularen åbner forhåndsudfyldt og agentens status er **Dry Run**. Se [Starter Templates](#starter-templates).
- **Fra bunden.** Klik **Create new agent**. Formularen åbner tom.

Under alle omstændigheder er det samme redigeringsformular, du gemmer og efterfølgende redigerer. Denne side gennemgår formularen fra top til bund.

### Grundlæggende

- **Internt navn.** En kort identifikator, der kun bruges i admin-dashboardet (kørselshistorik, analytics, revisionslog). Små bogstaver med underscores fungerer godt: `moderator`, `welcome_greeter`. Hvis en skabelons interne navn allerede er taget, sætter formularen automatisk et suffiks (`tos_enforcer_2`, osv.).
- **Visningsnavn.** Vises offentligt, når agenten poster en kommentar. Dette er, hvad dine læsere ser.
- **Status.** Disabled, Dry Run, eller Enabled. Nye agenter er som standard sat til Dry Run. Se [Status States](#status-states).

### Model

Vælg LLM. Se [Choosing a Model](#choosing-a-model).

### Budget

Valgfri daglige og månedlige grænser i din kontovaluta, plus en tjekliste for **Alert thresholds** (standard 80% og 100%). Se [Budgets Overview](#budgets-overview) og [Budget Alerts](#budget-alerts).

### Personlighed

Den **Initial prompt** er systemprompten, der definerer tone, rolle og beslutningsregler. Ren tekst, ingen skabelonsyntaks. Se [Personality and the Initial Prompt](#personality-prompt).

### Kontekst

Kontekst-feltet indeholder tre afkrydsningsfelter, et retningslinjetekstområde og scope-indgange:

- Inkluder overordnet kommentar og tidligere svar i samme tråd.
- Inkluder kommentatorens trust factor, kontosalder, forbudshistorik og nylige kommentarer.
- Inkluder sidetitel, undertitel, beskrivelse og meta-tags.
- En valgfri **Community guidelines** tekstblok, der føjes foranstillet til hver prompt.
- **Restrict to specific pages** - URL-mønster tilladelsesliste (én per linje). Tom betyder tenant-wide.
- **Restrict to specific locales** - lokalitets-allowlist via en dobbelt-listevælger. Tom betyder alle lokaliteter.

Mere kontekst giver bedre beslutninger, men øger token-omkostningen pr. kørsel. Se [Context Options](#context-options), [Community Guidelines](#community-guidelines), og [Scope: URL and Locale Filters](#scope-url-locale).

### Udløsere

Vælg mindst én begivenhed fra listen. For vote-threshold og flag-threshold udløsere skal du også sætte tærsklen. Det valgfrie felt **Delay before running** udsætter udførelsen efter en udløser (nyttigt for flag-threshold, hvor stemmer stadig stabiliserer sig). Se [Trigger Events Overview](#triggers-overview) og [Deferred Triggers](#trigger-deferred-delay).

### Tilladte værktøjskald

Sæt hak i **Allow any tool calls** for at eksponere hele værktøjspaletten. Ellers sæt hak i de specifikke værktøjer, agenten må bruge - ikke-tilbødte værktøjer fjernes fra modellens palet og afvises ved dispatch. Underdelen **Ban options** låser de destruktive ban-varianter (delete-all-comments, ban-by-IP) bag eksplicitte opt-ins. Se [Allowed Tool Calls Overview](#tools-overview) og [Tool: ban_user](#tool-ban-user).

### Godkendelser

Sæt hak ved de handlinger, der skal godkendes af et menneske, før agenten udfører dem. Godkendelser gælder kun for værktøjer, agenten må påkalde; ikke-tilbødte værktøjer afvises straks. I EU-regionen er **ban_user** aktiveret i henhold til artikel 17 i Digital Services Act. Se [Approval Workflow](#approval-workflow) og [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Godkendelsesnotifikationer

Hvis godkendelser er aktiveret, vælg hvem der får tilsendt e-mail:

- **All admins and moderators** - kontoindehavere, super admins og kommentarmoderator-administratorer.
- **Specific users** - udvalgte brugere fra en dobbelt-listevælger.

Hver reviewers individuelle leveringsfrekvens (immediate, hourly digest, daily digest) indstilles på deres egen profil. Se [Approval Notifications](#approval-notifications).

### Statistik

Skrivebeskyttet. Totale kørsler, tidspunkt for sidste kørsel og ID'et på den seneste kommentar agenten skrev (hvis nogen).

### Gem

Klik **Save agent**. Siden omdirigerer tilbage til agentlisten. Nye agenter er straks berettigede til at modtage udløsere i Dry Run.

### Redigering senere

Hver række på agentlistesiden viser per-agent handlinger: **Edit**, **Clone**, **Runs**, **Replays**, **Test run**, **Analytics**, **Approvals**, og **Delete**. At redigere en agent påvirker ikke allerede optagede kørsler retrospektivt - historikken bevares. Replay-snapshots fryser også agentens konfiguration på det tidspunkt, replayet blev startet, så et gemt replays resultater forbliver reproducerbare, selv efter du redigerer prompten.