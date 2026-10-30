Een **AI Agent** is een autonome werker, beperkt tot uw FastComments-tenant, die gebeurtenissen in uw community in de gaten houdt en namens u actie onderneemt.

Elke agent heeft drie zaken die u kunt beheren:

1. **Een persoonlijkheid.** Een vrije‑tekst initiële prompt die toon, rol en besluitvormingsstijl definieert ("Je bent een warme communitygroeter", "Je handhaaft de communityregels maar neigt naar waarschuwingen in plaats van bans", enzovoort).
2. **Een of meer triggers.** Een lijst van gebeurtenissen die de agent activeren – een nieuw commentaar, een commentaar dat een stem‑ of vlagdrempel overschrijdt, een moderatoractie, het eerste commentaar van een gebruiker op de site, en andere. De volledige lijst staat in [Trigger Events Overview](#triggers-overview).
3. **Een allowlist van tools.** Wat de agent mag doen – een commentaar plaatsen, stemmen, vastzetten, vergrendelen, markeren als spam, een gebruiker bannen, waarschuwen via DM, een badge toekennen, e‑mail sturen, een gedeeld geheugen opslaan en doorzoeken. De volledige lijst staat in [Allowed Tool Calls Overview](#tools-overview).

Wanneer een trigger wordt geactiveerd, ontvangt de agent een contextbericht dat beschrijft wat er is gebeurd (het commentaar, de pagina, optionele thread/gebruiker/pagina‑context) en wordt geprompt met zijn initiële prompt en uw communityrichtlijnen. Vervolgens roept hij tools aan om te handelen, waarbij bij elke oproep een rechtvaardiging en een vertrouwensscore worden vastgelegd.

### Agents draaien asynchroon

Agents **blokkeren nooit de gebruikersactie die hen heeft geactiveerd**. Een lezer plaatst een commentaar, het commentaar wordt opgeslagen en getoond in de thread, de respons wordt geretourneerd, en pas *dan* wordt de agent erop uitgevoerd – direct of na een geconfigureerde vertraging (zie [Deferred Triggers](#trigger-deferred-delay)). Niets wat de agent doet voegt latentie toe aan de gebruikerservaring.

### Waarom ze gebruiken

- **Schaalbaar modereren.** Markeer duidelijke spam en ban herhaalde overtreders zonder de wachtrij de hele dag in de gaten te houden.
- **Nieuwe commentatoren verwelkomen.** Reageer op eerste‑keer commentatoren in uw eigen stem.
- **Het beste content naar voren brengen.** Pin substantieve top‑level commentaren zodra ze een stemdrempel overschrijden.
- **Uw richtlijnen consequent handhaven.** Pas dezelfde beleidstekst toe op elk grensgeval commentaar.
- **Lange threads samenvatten.** Plaats neutrale samenvattingen van discussies over meerdere pagina's.

### Wat u in controle houdt

- **Dry‑run modus.** Elke nieuwe agent wordt geleverd in **Dry Run**: hij verwerkt triggers, draait het model en registreert wat hij *zou* doen, maar onderneemt geen echte acties. Zie [Dry-Run Mode](#dry-run-mode).
- **Goedkeuringen.** Elke subset van acties kan worden geblokkeerd achter menselijke goedkeuring. Zie [Approval Workflow](#approval-workflow).
- **Per‑agent- en per‑accountbudgetten.** Strikte dagelijkse en maandelijkse limieten. Zie [Budgets Overview](#budgets-overview).
- **Tool‑allowlist.** Niet‑toegestane tools worden uit het palet van het model verwijderd – de agent kan ze letterlijk niet aanvragen. Zie [Allowed Tool Calls Overview](#tools-overview).
- **Auditvelden bij elke actie.** Het model moet een rechtvaardiging en vertrouwensscore bevatten. Beide verschijnen in de uitvoeringstijdlijn en bij elke goedkeuring. Zie [Run Detail View](#run-detail-view).
- **EU DSA Artikel 17.** In de EU‑regio worden volledig geautomatiseerde bans geblokkeerd. Zie [EU DSA Article 17 Compliance](#eu-dsa-compliance).
- **Geen training op uw data.** FastComments gebruikt providers die niet trainen op uw prompts of commentaren.

### Waar ze passen naast menselijke moderatie

Agents en menselijke moderators delen hetzelfde commentaarplatform: agents voeren acties uit via dezelfde kanalen (goedkeuren, spam, bannen, badge, vastzetten, vergrendelen, schrijven) en die acties verschijnen in dezelfde [Comment Logs](/guide-moderation.html#comment-logs), dezelfde [Moderation Page](/guide-moderation.html#moderate-comments-page) en dezelfde meldingsstromen. Agents en mensen zien elkaars werk en kunnen elk op de ander reageren – moderatoracties zijn zelf geldige agent‑triggers (zie [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) en verwante).

Als u ook mensen in de wachtrij nodig heeft, biedt FastComments [Human Moderation Services](/guide-moderation.html#human-moderation-services): onze moderators werken naast uw agents, in uw dashboard, onder uw richtlijnen.