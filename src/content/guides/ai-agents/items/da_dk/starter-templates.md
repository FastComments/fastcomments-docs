FastComments leveres med fem startskabeloner, så du ikke behøver at skrive en fungerende agent fra bunden. De er tilgængelige fra [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) ved at klikke på **Browse templates**.

Når du vælger en skabelon:

1. Agenten oprettes med **Status: Testkørsel** og et internt navn baseret på skabelonen (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Hvis det navn allerede er taget på din tenant, tilføjes et numerisk suffiks.
2. Du lander direkte på redigeringsformularen med alt forudfyldt - prompt, triggers, tilladte handlinger og eventuelle tærskler. En banner øverst lyder "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Intet er aktiveret endnu. Agenten vil ikke handle, før du gemmer og enten beholder testkørsel tændt (for at observere) eller skifter til Aktiveret.

### De fem skabeloner

- **[Moderator](#template-moderator)** - gennemgår nye og markerede kommentarer, advarer førstegangsovertrædere og eskalerer til udelukkelse (ban) kun efter en advarsel. Trigger på new comments og på flag-threshold crossings (standard flag threshold: 3). Tilladte værktøjer: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Velkomsthilsen](#template-welcome-greeter)** - svarer varmt til førstegangskommentatorer med en kort, personlig velkomst. Trigger på new-user-first-comment. Tilladt værktøj: `write_comment`.

- **[Fastgør topkommentar](#template-top-comment-pinner)** - fastgør substantielle top-level kommentarer, når de krydser en stemmetærskel (standard: 10), og fjerner først den tidligere fastgjorte kommentar. Trigger på vote-threshold crossings. Tilladte værktøjer: `pin_comment`, `unpin_comment`.

- **[Trådsummerer](#template-thread-summarizer)** - poster et neutralt, ét-afsnits resumé på lange tråde efter en forsinkelse og fastgør det derefter. Trigger på new comments med en 30-minutters udsættelse, så tråden kan roe sig, før der opsummeres. Tilladte værktøjer: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight-detektor](#template-gaslight-detector)** - overvåger redigeringer af kommentarer for midt-tråds omskrivninger, der forvansker svar, gendanner den oprindelige tekst og sender en DM til forfatteren. Trigger på comment edits. Tilladte værktøjer: `edit_comment`, `warn_user`, `send_dm`.

### Tilpasning af en skabelon

Skabeloner er udgangspunkter, ikke kontrakter. Du forventes at:

- Justere **Indledende prompt** så den matcher din fællesskabsstemme.
- Tilføje eller fjerne **Udløsere** for at passe til, hvor ofte agenten skal køre.
- Tilføje **Godkendelser** for enhver følsom handling - vi anbefaler kraftigt at kræve godkendelse for `ban_user` i moderator-typer af skabeloner.
- Tilføje **Community guidelines** så agenten anvender din skrevne politik konsekvent. Se [Community Guidelines](#community-guidelines).
- Sætte per-agent **Budgetter** passende til, hvor mange triggers du forventer.

Skabelonen er blot et køretøj, der forudfylder fornuftige standarder; når den er gemt, er agenten din.