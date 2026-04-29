Een **goedkeuring** is een in de wachtrij geplaatste tool-aanroep die een menselijke goedkeuring of afwijzing vereist voordat het platform deze uitvoert.

### Goedkeuringen configureren

Op het agent-bewerkingsformulier toont de sectie **Approvals** elke tool die de agent mag aanroepen (de toegestane lijst, of allowlist) en laat je de tools aankruisen die door een mens beoordeeld moeten worden. Niet-aangekruiste tools worden onmiddellijk uitgevoerd. Aangekruiste tools worden in de wachtrij geplaatst.

Niet-toegestane tools worden *direct geweigerd*, niet in de wachtrij geplaatst - goedkeuringen zijn alleen van toepassing op tools die anderszins zijn toegestaan.

### Wat gebeurt er wanneer een afgeschermde actie wordt uitgevoerd

1. De agent kiest een tool-aanroep (bijv. `ban_user`) met argumenten, rechtvaardiging en vertrouwen.
2. In plaats van uit te voeren plaatst het platform een goedkeuring in de wachtrij in de status `PENDING` met de toolnaam, argumenten, rechtvaardiging, vertrouwen en een context-snapshot die de trigger beschrijft die het veroorzaakte.
3. Meldingen worden verzonden naar beoordelaars (zie [Goedkeuringsmeldingen](#approval-notifications)).
4. De run van de agent wordt voltooid en geregistreerd - de actie wordt weergegeven met **In afwachting van goedkeuring** in [Run Detail View](#run-detail-view).

### Goedkeuringen beoordelen

De goedkeuringsinbox op [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) toont goedkeuringen die in afwachting, goedgekeurd, afgewezen of waarvan de uitvoering is mislukt. Voor elk item:

- **Tool name and arguments** - precies wat de agent wil doen.
- **Agent reasoning** - de rechtvaardiging die de agent gaf.
- **Confidence** - het zelf-ingeschatte vertrouwen van de agent, 0.0 tot 1.0.
- **Context snapshot** - de opmerking, pagina en gebruiker waarop de actie is gericht.

Twee knoppen: **Goedkeuren** en **Afwijzen**. **Goedkeuren** voert de tool daadwerkelijk uit; **Afwijzen** verwijdert de aanvraag.

### Statussen van goedkeuring

Een goedkeuring doorloopt deze statussen:

| State | Meaning |
|---|---|
| `PENDING` | Wacht op menselijke beslissing. |
| `APPROVED` | Een mens heeft goedgekeurd en de actie is uitgevoerd. |
| `REJECTED` | Een mens heeft afgewezen. De actie is niet uitgevoerd. |
| `EXECUTION_FAILED` | Een mens heeft goedgekeurd maar de uitvoering is mislukt (bijv. de doelopmerking was al verwijderd). |
| `EXECUTING` | Transient: een mens klikte op Goedkeuren en de actie is bezig. Wordt gebruikt om gelijktijdige goedkeuringsklikken te serialiseren zodat een tool met echte bijwerkingen nooit twee keer draait. |

De `EXECUTING`-status is van belang wanneer twee beoordelaars gelijktijdig op 'Goedkeuren' klikken - één krijgt voorrang, de ander ziet dat de goedkeuring al is verschoven.

### Wat wordt opgeruimd

In afwachting zijnde goedkeuringen blijven in afwachting totdat er een beslissing is genomen. Ze verlopen automatisch na **90 dagen** na aanmaak. Goedkeuringen in eender welke andere status worden eveneens na 90 dagen verwijderd voor opslaghygiëne.

De velden van de goedkeuring "decided by" / "decided at" / "executed at" / "execution result" worden ingevuld naarmate de goedkeuring door de statussen heen beweegt - alles zichtbaar in de detailweergave van de inbox.

### Webhook-integratie

Twee webhook-events worden afgevuurd naarmate goedkeuringen van status veranderen:

- **`approval.requested`** - bij invoeging in `PENDING`.
- **`approval.decided`** - bij overgang naar `APPROVED`, `REJECTED`, of `EXECUTION_FAILED`.

Beide worden gesigneerd zoals alle andere webhooks. Zie [Webhook-evenementen](#webhook-events) en [Webhook Payloads](#webhook-payloads).

### Verharding: known-tool gate

Goedkeuringen weigeren het in de wachtrij plaatsen van elke toolnaam die geen erkende agent-tool is. Dit is een defense-in-depth-controle: zelfs als een toekomstige codepad een door een LLM afgeleide toolnaam in de goedkeuringsstroom doorgeeft, kan die string nooit als een in de wachtrij geplaatste goedkeuring terechtkomen. De set bekende toolnamen is dezelfde set die in [Tools Reference](#tools-overview) wordt vermeld.

### Veelvoorkomende gating-patronen

- **Brand-new moderation agent** - zet een gate op `ban_user`, `mark_comment_spam`, `mark_comment_approved`, `send_email`. Houd de inbox een paar weken in de gaten en verwijder daarna de gating voor tools met weinig fouten.
- **Long-term moderation agent** - houd `ban_user` en alle onomkeerbare acties (`deleteAllUsersComments`, `banIP`) voor altijd gegate.
- **EU region** - `ban_user` is ingeschakeld vanwege Artikel 17 ongeacht wat je aanvinkt. Zie [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Wat goedkeuringen **niet** doen

- Ze vertragen niet de andere tool-aanroepen van de agent. De run van de agent kan meerdere tools aanroepen, en alleen de gegateerde worden in de wachtrij geplaatst - de rest wordt normaal uitgevoerd.
- Ze draaien de run van de agent niet terug als een mens weigert. Het niet-gegateerde deel van de run is al voltooid.

---