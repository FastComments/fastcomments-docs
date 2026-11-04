FastComments include cinque modelli di partenza così non devi scrivere un agente funzionante da zero. Sono raggiungibili dalla [pagina AI Agents](https://fastcomments.com/auth/my-account/ai-agents) cliccando **Browse templates**.

Quando scegli un modello:

1. L'agente viene creato con **Status: Dry Run** e un nome interno basato sul modello (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Se quel nome è già in uso sul tuo tenant, viene aggiunto un suffisso numerico.
2. Arrivi direttamente sul modulo di modifica con tutto precompilato - prompt, trigger, azioni consentite ed eventuali soglie. Un banner in alto recita "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Nulla è ancora abilitato. L'agente non agirà finché non salvi e non mantieni la dry-run attiva (per osservare) oppure passi a Enabled.

### I cinque modelli

- **[Moderator](#template-moderator)** - esamina i commenti nuovi e segnalati, avverte i trasgressori alla prima infrazione, e passa al ban solo dopo un avvertimento. Si attiva sui nuovi commenti e sul superamento della soglia di segnalazioni (soglia predefinita: 3). Strumenti consentiti: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - risponde calorosamente a chi commenta per la prima volta con un breve benvenuto personale. Si attiva su new-user-first-comment. Strumento consentito: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - fissa i commenti di primo livello sostanziosi una volta superata una soglia di voti (predefinita: 10), rimuovendo prima il commento precedentemente fissato. Si attiva sul superamento della soglia di voti. Strumenti consentiti: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - pubblica un riepilogo neutrale di un solo paragrafo sui thread lunghi dopo un ritardo, poi lo fissa. Si attiva sui nuovi commenti con un rinvio di 30 minuti, così il thread si assesta prima del riepilogo. Strumenti consentiti: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - monitora le modifiche ai commenti per individuare riscritture a metà discussione che alterano le risposte, ripristina il testo originale e invia un DM all'autore. Si attiva sulle modifiche ai commenti. Strumenti consentiti: `edit_comment`, `warn_user`, `send_dm`.

### Personalizzare un modello

I modelli sono punti di partenza, non contratti. È previsto che tu:

- Modifichi l'**Initial prompt** per adattarlo alla voce della tua community.
- Aggiunga o rimuova **Triggers** in base alla frequenza con cui l'agente dovrebbe essere eseguito.
- Aggiunga **Approvals** per qualsiasi azione sensibile - raccomandiamo vivamente di sottoporre `ban_user` ad approvazione per i modelli in stile moderatore.
- Aggiunga **Community guidelines** così l'agente applica la tua politica scritta in modo coerente. Vedi [Community Guidelines](#community-guidelines).
- Imposti **Budgets** per agente adeguati al numero di trigger previsti.

Il modello è solo un veicolo che precompila valori predefiniti sensati; una volta salvato, l'agente è tuo.