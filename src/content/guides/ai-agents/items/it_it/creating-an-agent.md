Dalla [pagina AI Agents](https://fastcomments.com/auth/my-account/ai-agents) puoi creare un agente in due modi:

- **Da un template.** Clicca **Browse templates** e scegli uno dei quattro agenti iniziali predefiniti. Il modulo viene precompilato e lo stato dell'agente è **Dry Run**. Vedi [Starter Templates](#starter-templates).
- **Da zero.** Clicca **Create new agent**. Il modulo si presenta vuoto.

In entrambi i casi, lo stesso modulo di modifica è ciò che salvi e modifichi in seguito. Questa pagina illustra il modulo dall'alto verso il basso.

### Informazioni di base

- **Nome interno.** Un identificatore breve usato solo nei dashboard di amministrazione (cronologia delle esecuzioni, analisi, log di audit). Funziona bene in minuscolo con underscore: `moderator`, `welcome_greeter`. Se il nome interno di un template è già in uso, il modulo aggiunge automaticamente un suffisso (`tos_enforcer_2`, ecc.).
- **Nome visualizzato.** Mostrato pubblicamente ogni volta che l'agente pubblica un commento. È ciò che vedono i tuoi lettori.
- **Stato.** Disabled, Dry Run o Enabled. I nuovi agenti partono sempre con Dry Run. Vedi [Status States](#status-states).

### Modello

Scegli l'LLM. Vedi [Choosing a Model](#choosing-a-model).

### Budget

Limiti giornalieri e mensili opzionali nella valuta del tuo account, più una checklist **Alert thresholds** (predefinita 80% e 100%). Vedi [Budgets Overview](#budgets-overview) e [Budget Alerts](#budget-alerts).

### Personalità

L'**Initial prompt** è il prompt di sistema che definisce tono, ruolo e regole decisionali. Testo semplice, senza sintassi di template. Vedi [Personality and the Initial Prompt](#personality-prompt).

### Contesto

Il fieldset Context ha tre caselle di controllo, un'area di testo per le linee guida e gli input per l'ambito:

- Includi il commento padre e le risposte precedenti nello stesso thread.
- Includi il trust factor del commentatore, l'età dell'account, la cronologia dei ban e i commenti recenti.
- Includi titolo della pagina, sottotitolo, descrizione e meta tag.
- Un blocco di testo opzionale **Community guidelines** che viene anteposto a ogni prompt.
- **Restrict to specific pages** - allowlist di pattern URL (uno per riga). Vuoto significa a livello di tenant.
- **Restrict to specific locales** - allowlist delle localizzazioni tramite un selettore a doppia lista. Vuoto significa tutte le localizzazioni.

Più contesto produce decisioni migliori ma aumenta il costo in token per esecuzione. Vedi [Context Options](#context-options), [Community Guidelines](#community-guidelines) e [Scope: URL and Locale Filters](#scope-url-locale).

### Trigger

Scegli almeno un evento dall'elenco. Per i trigger basati su soglia di voti e soglia di segnalazioni devi anche impostare la soglia. Il campo opzionale **Delay before running** rimanda l'esecuzione dopo l'attivazione di un trigger (utile per le soglie di segnalazioni quando i voti si stanno ancora stabilizzando). Vedi [Trigger Events Overview](#triggers-overview) e [Deferred Triggers](#trigger-deferred-delay).

### Chiamate agli strumenti consentite

Spunta **Allow any tool calls** per esporre l'intera palette di strumenti. Altrimenti spunta gli strumenti specifici che l'agente è autorizzato a usare - gli strumenti non consentiti vengono rimossi dalla palette del modello e rifiutati al momento dell'invocazione. La sottosezione **Ban options** protegge le varianti distruttive di ban (delete-all-comments, ban-by-IP) dietro consensi espliciti. Vedi [Allowed Tool Calls Overview](#tools-overview) e [Tool: ban_user](#tool-ban-user).

### Approvazioni

Spunta le azioni che devono essere approvate da un umano prima che l'agente le esegua. Le approvazioni si applicano solo agli strumenti che l'agente è autorizzato a invocare; gli strumenti non consentiti vengono rifiutati del tutto. Nella regione UE, **ban_user** è bloccato su attivo dall'Articolo 17 del Digital Services Act. Vedi [Approval Workflow](#approval-workflow) e [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Notifiche di approvazione

Se le approvazioni sono abilitate, scegli a chi inviare l'email:

- **Tutti gli amministratori e moderatori** - proprietari dell'account, super admin e admin moderatori dei commenti.
- **Utenti specifici** - selezionati manualmente da un selettore a doppia lista.

La frequenza di consegna individuale di ogni revisore (immediata, digest orario, digest giornaliero) è impostata sul proprio profilo. Vedi [Approval Notifications](#approval-notifications).

### Statistiche

Sola lettura. Esecuzioni totali, timestamp dell'ultima esecuzione e l'ID del commento più recente scritto dall'agente (se presente).

### Salvataggio

Clicca **Save agent**. La pagina reindirizza all'elenco degli agenti. I nuovi agenti sono immediatamente idonei a ricevere trigger in dry-run.

### Modifica successiva

Ogni riga nella pagina dell'elenco degli agenti espone azioni per singolo agente: **Edit**, **Clone**, **Runs**, **Replays**, **Test run**, **Analytics**, **Approvals** e **Delete**. La modifica di un agente non ha effetto retroattivo sulle esecuzioni già registrate - la cronologia è preservata. Anche gli snapshot di replay congelano la configurazione dell'agente al momento dell'avvio del replay, quindi i risultati di un replay salvato restano riproducibili anche dopo aver modificato il prompt.