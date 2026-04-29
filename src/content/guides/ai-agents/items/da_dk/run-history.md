Kørselshistorik er den pr.-agent log over hver udløser, der kørte. Tilgængelig fra agentlisten via **Kørsler**-knappen, eller direkte på `/auth/my-account/ai-agents/{agentId}/runs`.

### Hvad der er på siden

En pagineret tabel med en række pr. kørsel:

| Kolonne | Betydning |
|---|---|
| Date | Hvornår udløseren affyrede (eller hvornår den udsatte udløser kørte). |
| Status | **Startet**, **Succes**, eller **Fejl**. **Tørkørsel**-badge vises ved siden af, hvis kørslen var i tørkørsels-tilstand. |
| Cost | Pris pr. kørsel i din tenants valuta. Tom for igangværende (Startet) kørsler. |
| Actions | Antallet af værktøjsopkald i kørslen. |
| Details | En **Vis**-knap, der åbner [Run Detail View](#run-detail-view). |

### Betydning af statuser

- **Startet** - kørslen er i gang, eller den døde før færdiggørelse. En kørsel, der sidder fast i "Startet" usædvanligt længe, repræsenterer normalt en timeout på LLM-opkald.
- **Fejl** - kørslen blev gennemført, men fejlede et sted - LLM-opkald returnerede en fejl, en værktøjsdispatch mislykkedes osv. Detaljevisningen indeholder den specifikke fejl.
- **Succes** - kørslen blev gennemført uden fejl. Agenten kan have taget nul, ét eller mange handlinger.

### Tom tilstand

Når en agent ikke har nogen kørsler, viser siden: "Ingen kørsler endnu for denne agent. Aktiverede kørsler vises her, når en udløser affyrer; brug Testkørsel til at forhåndsvise, hvad denne agent ville gøre mod tidligere kommentarer."

Den sidste del er tilsigtet - [flowet for testkørsel](#test-runs-replays) er den anbefalede måde at udfylde Kørselshistorik på en ny agent.

### Hvad der ikke er på kørselshistoriksiden

- **Live-udløsere, der aldrig dispatcherede** - en udløser, der blev droppet på grund af budget, scope eller ratebegrænsning, vises ikke på denne side. De dukker op på [Analytics-siden](#analytics-page) under "Triggers skipped".
- **Godkendelser** - ventende godkendelser for handlinger taget i denne kørsel findes i [godkendelsesindbakken](#approval-workflow). Handlingen vises i kørselsdetaljevisningen som **Afventer godkendelse**.

### Retention

Enkeltstående kørselsregistre beholdes i 90 dage, hvorefter kørslen forsvinder fra historikken. Omkostninger og udløsertællinger fortsætter med at blive opsummeret i langsigtede analysetabeller, så [Analytics-siden](#analytics-page) stadig viser historiske totaler ud over den periode.

### Afspilninger

Afspilningsproducerede kørsler er udelukket fra live-kørselsvisningen som standard. [Testkørsler (afspilninger)](#test-runs-replays)-siden er hvor du ser disse.

### Filtrering på tværs af agenter

Kørselstabellen er pr.-agent. Der er ingen tvær-agent kørsel-visning - [Analytics-siden](#analytics-page) er tvær-agent oversigten. Hvis du har brug for at inspicere kørsler på tværs af flere agenter, er [Webhooks](#webhooks-overview) `trigger.succeeded` og `trigger.failed` events dem, du vil videresende til dit eget system.