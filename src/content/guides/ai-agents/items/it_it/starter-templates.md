FastComments include quattro template starter così non devi scrivere un agente funzionante da zero. Sono raggiungibili dalla [pagina Agenti AI](https://fastcomments.com/auth/my-account/ai-agents) cliccando **Sfoglia template**.

Quando scegli un template:

1. L'agente viene creato con **Status: Dry Run** e un nome interno basato sul template (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`). Se quel nome è già usato nel tuo tenant, viene aggiunto un suffisso numerico.
2. Arrivi direttamente al modulo di modifica con tutto precompilato - prompt, triggers, azioni consentite e eventuali soglie. Un banner nella parte superiore recita "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Niente è ancora abilitato. L'agente non agirà finché non salvi e mantieni il dry-run attivo (per osservare) o non lo cambi in Enabled.

### I quattro template

- **[Moderatore](#template-moderator)** - esamina commenti nuovi e segnalati, avverte i trasgressori alla prima infrazione e scala al ban solo dopo un avvertimento. Si attiva su nuovi commenti e su flag-threshold crossings (default flag threshold: 3). Strumenti consentiti: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Saluto di benvenuto](#template-welcome-greeter)** - risponde calorosamente ai commentatori alla prima esperienza con un breve e personale messaggio di benvenuto. Si attiva su new-user-first-comment. Strumento consentito: `write_comment`.

- **[Fissatore dei commenti migliori](#template-top-comment-pinner)** - fissa i commenti substantivi di livello superiore una volta che superano una soglia di voti (predefinita: 10), rimuovendo prima il commento precedentemente fissato. Si attiva su vote-threshold crossings. Strumenti consentiti: `pin_comment`, `unpin_comment`.

- **[Riassuntore del thread](#template-thread-summarizer)** - pubblica un riassunto neutro di un solo paragrafo nelle discussioni lunghe dopo un ritardo, quindi lo fissa. Si attiva su nuovi commenti con un differimento di 30 minuti in modo che la discussione si stabilizzi prima di essere riassunta. Strumenti consentiti: `write_comment`, `pin_comment`, `unpin_comment`.

### Personalizzare un template

I template sono punti di partenza, non contratti. È previsto che tu:

- Modifichi il **Prompt iniziale** per adattarlo alla voce della tua community.
- Aggiunga o rimuova **Triggers** per adeguare la frequenza con cui l'agente dovrebbe eseguire.
- Aggiunga **Approvazioni** per qualsiasi azione sensibile - raccomandiamo vivamente di mettere `ban_user` dietro approvazione per i template in stile moderatore.
- Aggiunga le **Linee guida della community** in modo che l'agente applichi la tua policy scritta in modo coerente. Vedi [Linee guida della community](#community-guidelines).
- Imposti per agente i **Budget** appropriati in base al numero di trigger che prevedi.

Il template è solo un veicolo che precompila valori predefiniti sensati; una volta salvato, l'agente è tuo.