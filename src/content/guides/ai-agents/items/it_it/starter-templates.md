FastComments ships five starter templates so you do not have to write a working agent from scratch. They are reachable from the [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) by clicking **Browse templates**.

When you pick a template:

1. The agent is created with **Status: Dry Run** and an internal name based on the template (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). If that name is taken on your tenant, a numeric suffix is added.
2. You land directly on the edit form with everything pre-filled - prompt, triggers, allowed actions, and any thresholds. A banner across the top reads "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Nothing is enabled yet. The agent will not act until you save and either keep dry-run on (to observe) or flip to Enabled.

### I cinque template

- **[Moderatore](#template-moderator)** - esamina i commenti nuovi e quelli segnalati, avverte i trasgressori alla prima infrazione ed escale al ban solo dopo un avviso. Si attiva su nuovi commenti e quando viene superata la soglia di segnalazioni (soglia predefinita: 3). Strumenti consentiti: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Accoglienza di benvenuto](#template-welcome-greeter)** - risponde calorosamente ai commentatori alla prima partecipazione con un breve messaggio di benvenuto personale. Si attiva su new-user-first-comment. Strumento consentito: `write_comment`.

- **[Pin dei commenti in evidenza](#template-top-comment-pinner)** - fissa i commenti di livello superiore sostanziali quando superano una soglia di voti (predefinita: 10), rimuovendo prima il pin dal commento precedentemente fissato. Si attiva al superamento della soglia di voti. Strumenti consentiti: `pin_comment`, `unpin_comment`.

- **[Riepilogatore del thread](#template-thread-summarizer)** - pubblica un riepilogo neutro, in un unico paragrafo, per thread lunghi dopo un ritardo, quindi lo fissa. Si attiva sui nuovi commenti con una differenza di 30 minuti in modo che la discussione si stabilizzi prima del riepilogo. Strumenti consentiti: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Rilevatore di gaslighting](#template-gaslight-detector)** - monitora le modifiche ai commenti per riscritture nel corso della discussione che distorcono le risposte, ripristina il testo originale e invia un DM all'autore. Si attiva sulle modifiche ai commenti. Strumenti consentiti: `edit_comment`, `warn_user`, `send_dm`.

### Personalizzare un template

I template sono punti di partenza, non contratti. Si prevede che tu:

- Modifichi il **Initial prompt** per adattarlo alla voce della tua community.
- Aggiunga o rimuova **Triggers** per regolare la frequenza con cui l'agente deve eseguirsi.
- Aggiunga **Approvals** per qualsiasi azione sensibile - raccomandiamo fortemente di mettere `ban_user` dietro approvazione per i template in stile moderatore.
- Aggiunga le **Community guidelines** in modo che l'agente applichi consistentemente la tua policy scritta. Vedi [Community Guidelines](#community-guidelines).
- Imposti per agente dei **Budgets** appropriati al numero di trigger che prevedi.

Il template è solo un veicolo che precompila valori predefiniti sensati; una volta salvato, l'agente è tuo.