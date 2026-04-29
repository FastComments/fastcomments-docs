FastComments leverer fire startskabeloner, så du ikke behøver at skrive en fungerende agent fra bunden. De er tilgængelige fra [AI Agents-siden](https://fastcomments.com/auth/my-account/ai-agents) ved at klikke på **Gennemse skabeloner**.

Når du vælger en skabelon:

1. Agenten oprettes med **Status: Testkørsel** og et internt navn baseret på skabelonen (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`). Hvis det navn allerede er taget på din tenant, tilføjes et numerisk suffiks.
2. Du lander direkte på redigeringsformularen med alt forudfyldt - prompt, triggere, tilladte handlinger og eventuelle tærskelværdier. Et banner øverst lyder "Oprettet ud fra {templateName}-skabelonen. Gennemgå indstillingerne nedenfor, og skift derefter status til Aktiveret, når du er klar."
3. Intet er aktiveret endnu. Agenten vil ikke handle, før du gemmer og enten beholder testkørsel tændt (for at observere) eller skifter til Aktiveret.

### De fire skabeloner

- **[Moderator](#template-moderator)** - gennemgår nye og flaggede kommentarer, advarer førstegangsforseelser, og eskalerer til udelukkelse kun efter en advarsel. Udløser ved nye kommentarer og når flag-tærskler krydses (standard flag-tærskel: 3). Tilladte værktøjer: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - svarer venligt på førstegangs-kommentarer fra nye brugere med en kort, personlig velkomst. Udløser ved førstegangs-kommentar fra nye brugere. Tilladt værktøj: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - fastgør væsentlige top‑niveau kommentarer, når de overstiger en stemmetærskel (standard: 10), og fjerner først fastgørelsen af den tidligere fastgjorte kommentar. Udløser når stemmegrænsen overskrides. Tilladte værktøjer: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - offentliggør et neutralt, ét‑afsnits resumé på lange tråde efter en forsinkelse og fastgør det derefter. Udløser ved nye kommentarer med en 30‑minutters udsættelse, så tråden kan falde til ro, før den opsummeres. Tilladte værktøjer: `write_comment`, `pin_comment`, `unpin_comment`.

### Tilpasning af en skabelon

Skabeloner er udgangspunkter, ikke kontrakter. Du forventes at:

- Tilpasse **Startprompt** så den matcher din fællesskabsstemme.
- Tilføje eller fjerne **Udløsere** for at bestemme, hvor ofte agenten skal køre.
- Tilføje **Godkendelser** for enhver følsom handling - vi anbefaler stærkt at placere `ban_user` bag en godkendelse for moderator‑stil skabeloner.
- Tilføje **Fællesskabsretningslinjer** så agenten anvender din skrevne politik konsekvent. Se [Community Guidelines](#community-guidelines).
- Sætte per‑agent **Budgetter** passende i forhold til, hvor mange udløsere du forventer.

Skabelonen er blot et værktøj, der forudfylder fornuftige standarder; når den er gemt, er agenten din.