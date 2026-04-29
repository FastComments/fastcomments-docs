Een **AI-agent** is een autonome werker, beperkt tot je FastComments-tenant, die gebeurtenissen in je community bewaakt en namens jou actie onderneemt.

Elke agent heeft drie dingen die je kunt beheren:

1. **Een persoonlijkheid.** Een vrije-tekst initiële prompt die toon, rol en beslissingsstijl definieert ("Je bent een warme community-gastheer", "Je handhaaft de communityregels maar neigt naar waarschuwing boven verbanning", enzovoort).
2. **Een of meer triggers.** Een lijst met gebeurtenissen die de agent wakker maken - een nieuw comment, een comment dat een stem- of flag-drempel overschrijdt, een moderatoractie, iemands eerste comment op de site, en andere. De volledige lijst staat in [Overzicht triggergebeurtenissen](#triggers-overview).
3. **Een allowlist van tools.** Wat de agent mag doen - een comment plaatsen, stemmen, pinnen, vergrendelen, als spam markeren, een gebruiker verbannen, waarschuwen via DM, een badge uitreiken, e-mail verzenden, gedeeld geheugen opslaan en doorzoeken. De volledige lijst staat in [Overzicht toegestane tool-oproepen](#tools-overview).

Wanneer een trigger afgaat, ontvangt de agent een contextbericht dat beschrijft wat er gebeurde (het commentaar, de pagina, optionele thread-/gebruiker-/paginacontext) en wordt hij geprompt met zijn initiële prompt en jouw communityrichtlijnen. Vervolgens roept hij tools aan om te handelen, en legt bij elke oproep een rechtvaardiging en een betrouwbaarheidsniveau vast.

### Agenten draaien asynchroon

Agenten **blokkeren nooit de gebruikersactie die hen heeft geactiveerd**. Een lezer plaatst een comment, het comment wordt opgeslagen en getoond in de thread, het antwoord wordt teruggegeven, en pas *daarna* draait de agent erop - direct of na een geconfigureerde vertraging (zie [Uitgestelde triggers](#trigger-deferred-delay)). Niets wat de agent doet voegt latentie toe aan de gebruikerservaring.

### Waarom je ze zou gebruiken

- **Modereren op schaal.** Markeer duidelijke spam en verbied terugkerende overtreders zonder de queue de klok rond in de gaten te houden.
- **Nieuwe reageerders verwelkomen.** Reageer in jouw toon naar eerstereageerders.
- **Het beste content naar boven halen.** Pin inhoudelijke top-level comments zodra ze een stemdrempel passeren.
- **Je richtlijnen consistent handhaven.** Pas dezelfde beleidsregels toe op elke twijfelachtige comment.
- **Lange threads samenvatten.** Plaats neutrale samenvattingen van meer-pagina debatten.

### Wat je de controle geeft

- **Dry-run-modus.** Elke nieuwe agent wordt geleverd in **Dry-run**: hij verwerkt triggers, draait het model en registreert wat hij *zou* doen, maar onderneemt geen echte acties. Zie [Dry-Run-modus](#dry-run-mode).
- **Goedkeuringen.** Elke subset van acties kan achter menselijke goedkeuring worden geplaatst. Zie [Goedkeuringsworkflow](#approval-workflow).
- **Budgetten per agent en per account.** Harde dagelijkse en maandelijkse limieten. Zie [Overzicht budgetten](#budgets-overview).
- **Tool-allowlist.** Niet-toegestane tools worden uit het modelpalet verwijderd - de agent kan ze letterlijk niet aanvragen. Zie [Overzicht toegestane tool-oproepen](#tools-overview).
- **Auditvelden bij elke actie.** Het model moet een rechtvaardiging en een betrouwbaarheidsniveau opnemen. Beide verschijnen in de run-tijdlijn en bij elke goedkeuring. Zie [Run-detailweergave](#run-detail-view).
- **EU DSA Artikel 17.** In de EU-regio worden volledig geautomatiseerde verbanningen geblokkeerd. Zie [EU DSA Artikel 17 naleving](#eu-dsa-compliance).
- **Geen training op je data.** FastComments gebruikt providers die niet trainen op je prompts of comments.

### Hoe ze passen naast menselijke moderatie

Agenten en menselijke moderators delen hetzelfde comments-platform: agenten voeren acties uit via dezelfde kanalen (goedkeuren, spam, verbannen, badge, pinnen, vergrendelen, schrijven) en die acties verschijnen in dezelfde [Reactielogboeken](/guide-moderation.html#comment-logs), op dezelfde [Moderatiepagina](/guide-moderation.html#moderate-comments-page) en in dezelfde notificatiestromen. Agenten en mensen zien elkaars werk en kunnen op elkaar reageren - moderatoracties zijn zelf geldige agent-triggers (zie [Trigger: Door moderator beoordeelde opmerking](#trigger-moderator-reviewed) en aanverwante triggers).