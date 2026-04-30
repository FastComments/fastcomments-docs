FastComments levert vijf start-sjablonen zodat je niet vanaf nul een werkende agent hoeft te schrijven. Ze zijn bereikbaar vanaf de [AI Agents-pagina](https://fastcomments.com/auth/my-account/ai-agents) door te klikken op **Sjablonen bekijken**.

Wanneer je een sjabloon kiest:

1. De agent wordt aangemaakt met **Status: Dry Run** en een interne naam gebaseerd op het sjabloon (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Als die naam al bestaat op jouw tenant, wordt er een numerieke suffix toegevoegd.
2. Je komt direct op het bewerkformulier terecht met alles vooraf ingevuld - prompt, triggers, toegestane acties en eventuele drempelwaarden. Een banner bovenaan geeft weer "Gemaakt vanuit het {templateName}-sjabloon. Controleer de instellingen hieronder, en zet de status op Enabled wanneer je klaar bent."
3. Nog niets is ingeschakeld. De agent zal niet handelen totdat je opslaat en ofwel dry-run aanhoudt (om te observeren) of overschakelt naar Enabled.

### De vijf sjablonen

- **[Moderator](#template-moderator)** - beoordeelt nieuwe en gemarkeerde opmerkingen, waarschuwt beginnende overtreders bij de eerste keer en escaleert naar een ban pas na een waarschuwing. Triggert bij nieuwe opmerkingen en bij flag-threshold crossings (default flag threshold: 3). Toegestane tools: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - reageert hartelijk naar eerstegangsreactoren met een korte, persoonlijke welkomstboodschap. Triggert op new-user-first-comment. Toegestane tool: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - zet inhoudelijke top-level reacties vast zodra ze een vote threshold overschrijden (default: 10), waarbij eerst de eerder vastgezette reactie wordt losgezet. Triggert op vote-threshold crossings. Toegestane tools: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - plaatst een neutrale, eén-paragraafsamenvatting in lange threads na een vertraging en zet deze daarna vast. Triggert op nieuwe opmerkingen met een uitstel van 30 minuten zodat de thread tot rust komt voordat deze wordt samengevat. Toegestane tools: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - houdt bewerkingen van opmerkingen in de gaten voor herschrijvingen midden in een thread die reacties vervormen, herstelt de originele tekst en stuurt de auteur een DM. Triggert op comment edits. Toegestane tools: `edit_comment`, `warn_user`, `send_dm`.

### Een sjabloon aanpassen

Sjablonen zijn beginpunten, geen contracten. Van jou wordt verwacht dat je:

- Pas de **Initial prompt** aan zodat deze past bij de toon van je community.
- Voeg of verwijder **Triggers** om te bepalen hoe vaak de agent moet draaien.
- Voeg **Approvals** toe voor gevoelige acties - we raden sterk aan om `ban_user` achter een goedkeuring te plaatsen voor moderator-achtige sjablonen.
- Voeg **Community guidelines** toe zodat de agent je geschreven beleid consequent toepast. Zie [Gemeenschapsrichtlijnen](#community-guidelines).
- Stel per-agent **Budgets** in die passen bij het verwachte aantal triggers.

Het sjabloon is slechts een middel dat zinnige standaardwaarden vooraf invult; eenmaal opgeslagen is de agent van jou.