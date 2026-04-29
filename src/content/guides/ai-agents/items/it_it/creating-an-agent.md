Dalla [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) puoi creare un agente in due modi:

- **From a template.** Clicca **Browse templates** e scegli uno dei quattro agenti iniziali predefiniti. Il modulo viene caricato precompilato e lo stato dell'agente è **Modalità di prova**. Vedi [Starter Templates](#starter-templates).
- **From scratch.** Clicca **Create new agent**. Il modulo viene caricato vuoto.

In entrambi i casi, lo stesso modulo di modifica è quello che salverai e modificherai in seguito. Questa pagina descrive il modulo dall'alto verso il basso.

### Basics

- **Internal name.** Un identificatore breve usato solo nelle dashboard di amministrazione (cronologia esecuzioni, analytics, log di audit). Minuscolo con underscore funziona bene: `moderator`, `welcome_greeter`. Se il nome interno di un template è già occupato, il modulo aggiunge automaticamente un suffisso (`tos_enforcer_2`, ecc.).
- **Display name.** Viene mostrato pubblicamente ogni volta che l'agente pubblica un commento. Questo è ciò che vedono i tuoi lettori.
- **Status.** Disabilitato, Modalità di prova, o Abilitato. I nuovi agenti partono sempre in Modalità di prova. Vedi [Status States](#status-states).

### Model

Scegli l'LLM. Vedi [Choosing a Model](#choosing-a-model).

### Budget

Limiti giornalieri e mensili opzionali nella valuta del tuo account, più una checklist di **Alert thresholds** (impostazione predefinita 80% e 100%). Vedi [Budgets Overview](#budgets-overview) e [Budget Alerts](#budget-alerts).

### Personality

L'**Initial prompt** è il prompt di sistema che definisce tono, ruolo e regole decisionali. Testo semplice, nessuna sintassi di template. Vedi [Personality and the Initial Prompt](#personality-prompt).

### Context

Il gruppo di campi Contesto contiene tre caselle di controllo, un'area di testo per le linee guida e gli input di ambito:

- Includi il commento genitore e le risposte precedenti nello stesso thread.
- Includi il trust factor del commentatore, l'età dell'account, la cronologia dei ban e i commenti recenti.
- Includi il titolo della pagina, il sottotitolo, la descrizione e i meta tag.
- Un blocco di testo opzionale **Community guidelines** che viene anteposto a ogni prompt.
- **Restrict to specific pages** - allowlist di pattern URL (uno per riga). Vuoto significa a livello tenant.
- **Restrict to specific locales** - allowlist di locali tramite un selettore a doppia lista. Vuoto significa ogni locale.

Più contesto produce decisioni migliori ma aumenta il costo in token per esecuzione. Vedi [Context Options](#context-options), [Community Guidelines](#community-guidelines), e [Scope: URL and Locale Filters](#scope-url-locale).

### Triggers

Seleziona almeno un evento dalla lista. Per i trigger vote-threshold e flag-threshold devi anche impostare la soglia. Il campo opzionale **Delay before running** differisce l'esecuzione dopo il trigger (utile per le soglie di flag dove i voti sono ancora in fase di assestamento). Vedi [Trigger Events Overview](#triggers-overview) e [Deferred Triggers](#trigger-deferred-delay).

### Allowed tool calls

Seleziona **Allow any tool calls** per esporre l'intero insieme di strumenti. Altrimenti seleziona gli strumenti specifici che l'agente è autorizzato a usare - gli strumenti non consentiti vengono rimossi dalla palette del modello e rifiutati al momento della dispatch. La sotto-sezione **Opzioni di ban** limita le varianti distruttive di ban (delete-all-comments, ban-by-IP) dietro esplicite opzioni di consenso. Vedi [Allowed Tool Calls Overview](#tools-overview) e [Strumento: ban_user](#tool-ban-user).

### Approvals

Seleziona le azioni che devono essere approvate da un umano prima che l'agente le esegua. Le approvazioni si applicano solo agli strumenti che l'agente è autorizzato a invocare; gli strumenti non consentiti sono rifiutati immediatamente. Nella regione UE, **ban_user** è attivato in forza dell'Articolo 17 del Digital Services Act. Vedi [Approval Workflow](#approval-workflow) e [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Approval notifications

Se le approvazioni sono abilitate, scegli chi riceve le email:

- **All admins and moderators** - proprietari dell'account, super admins, e amministratori moderatori dei commenti.
- **Specific users** - selezionati manualmente tramite un selettore a doppia lista.

La frequenza di consegna individuale di ogni revisore (immediata, riepilogo orario, riepilogo giornaliero) è impostata nel proprio profilo. Vedi [Approval Notifications](#approval-notifications).

### Stats

Sola lettura. Totale esecuzioni, timestamp dell'ultima esecuzione e l'ID del commento più recente scritto dall'agente (se presente).

### Save

Clicca **Save agent**. La pagina viene reindirizzata alla lista degli agenti. I nuovi agenti sono immediatamente idonei a ricevere trigger in modalità di prova.

### Editing later

Ogni riga nella pagina della lista agenti espone azioni per agenti specifici: **Edit**, **Clone**, **Runs**, **Replays**, **Test run**, **Analytics**, **Approvals**, e **Delete**. Modificare un agente non influisce retroattivamente sulle esecuzioni già registrate - la cronologia è preservata. Gli snapshot di replay congelano anche la configurazione dell'agente al momento dell'avvio del replay, quindi i risultati di un replay salvato rimangono riproducibili anche dopo che hai modificato il prompt.