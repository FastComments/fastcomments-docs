FastComments levert vier starttemplates zodat je niet vanaf nul een werkende agent hoeft te schrijven. Ze zijn bereikbaar vanaf de [AI Agents-pagina](https://fastcomments.com/auth/my-account/ai-agents) door te klikken op **Bekijk sjablonen**.

When you pick a template:

1. The agent is created with **Status: Dry Run** and an internal name based on the template (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`). If that name is taken on your tenant, a numeric suffix is added.
2. You land directly on the edit form with everything pre-filled - prompt, triggers, allowed actions, and any thresholds. A banner across the top reads "Gemaakt vanaf het {templateName}-sjabloon. Controleer de instellingen hieronder en zet de status op Enabled wanneer je klaar bent."
3. Nothing is enabled yet. The agent will not act until you save and either keep dry-run on (to observe) or flip to Enabled.

### The four templates

- **[Moderator](#template-moderator)** - beoordeelt nieuwe en gemarkeerde reacties, waarschuwt overtreders die voor het eerst de fout in gaan, en escaleert naar verbanning alleen nadat er een waarschuwing is gegeven. Triggert bij nieuwe reacties en wanneer de meldingsdrempel wordt overschreden (standaard meldingsdrempel: 3). Toegestane tools: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - reageert hartelijk op eerste reacties van gebruikers met een korte, persoonlijke welkomstboodschap. Triggert bij new-user-first-comment. Toegestane tool: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - zet inhoudelijke top-level reacties vast zodra ze een stemdrempel overschrijden (standaard: 10), en haalt eerst de eerder vastgezette reactie los. Triggert bij overschrijden van een stemdrempel. Toegestane tools: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - plaatst een neutrale, eendelige samenvatting in één alinea op lange threads na een vertraging, en zet deze daarna vast. Triggert bij nieuwe reacties met een uitstel van 30 minuten zodat de thread kan bedaren voordat er samengevat wordt. Toegestane tools: `write_comment`, `pin_comment`, `unpin_comment`.

### Customizing a template

Templates zijn beginpunten, geen contracten. Van je wordt verwacht dat je:

- Pas de **Initial prompt** aan zodat deze past bij de toon van je community.
- Voeg **Triggers** toe of verwijder ze om te bepalen hoe vaak de agent moet draaien.
- Voeg **Approvals** toe voor gevoelige acties - we raden sterk aan om `ban_user` achter goedkeuring te zetten voor moderator-achtige templates.
- Voeg **Gemeenschapsrichtlijnen** toe zodat de agent je geschreven beleid consistent toepast. Zie [Community Guidelines](#community-guidelines).
- Stel per-agent **Budgets** in die passen bij het aantal triggers dat je verwacht.

Het template is slechts een voertuig dat verstandige standaardwaarden invult; zodra je het opslaat is de agent van jou.