Un'**approvazione** è una chiamata a uno strumento accodata che richiede l'approvazione o il rifiuto da parte di un umano prima che la piattaforma la esegua.

### Configurazione delle approvazioni

Nel modulo di modifica dell'agente, la sezione **Approvazioni** elenca ogni strumento che l'agente è autorizzato a invocare (la allowlist) e ti consente di selezionare quelli che devono essere revisionati da un umano. Gli strumenti non selezionati vengono eseguiti immediatamente. Gli strumenti selezionati sono accodati.

Gli strumenti non consentiti vengono *rifiutati direttamente*, non accodati - le approvazioni si applicano solo agli strumenti che sono comunque consentiti.

### Cosa accade quando scatta un'azione sottoposta a controllo

1. L'agente sceglie una chiamata a uno strumento (es. `ban_user`) con argomenti, giustificazione e confidenza.
2. Invece di eseguire, la piattaforma accoda un'approvazione in stato `PENDING` con il nome dello strumento, gli argomenti, la giustificazione, la confidenza e un'istantanea del contesto che descrive il trigger che l'ha generata.
3. Vengono inviate notifiche ai revisori (vedi [Notifiche di approvazione](#approval-notifications)).
4. L'esecuzione dell'agente si completa e viene registrata - l'azione è mostrata con **Pending approval** nella [Vista dettagli esecuzione](#run-detail-view).

### Revisione delle approvazioni

La inbox delle approvazioni su [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) elenca le approvazioni in sospeso, approvate, respinte e con esecuzione fallita. Per ciascuna:

- **Nome dello strumento e argomenti** - esattamente ciò che l'agente vuole fare.
- **Motivazione dell'agente** - la giustificazione fornita dall'agente.
- **Confidenza** - la confidenza auto-valutata dell'agente, da 0.0 a 1.0.
- **Snapshot del contesto** - il commento, la pagina e l'utente su cui è diretta l'azione.

Due pulsanti: **Approva** e **Rifiuta**. Approva esegue realmente lo strumento; Rifiuta scarta.

### Stati di approvazione

Un'approvazione attraversa questi stati:

| Stato | Significato |
|---|---|
| `PENDING` | In attesa della decisione umana. |
| `APPROVED` | Un umano ha approvato e l'azione è stata eseguita. |
| `REJECTED` | Un umano ha rifiutato. L'azione non è stata eseguita. |
| `EXECUTION_FAILED` | Un umano ha approvato ma l'azione è fallita (es., il commento target era già stato eliminato). |
| `EXECUTING` | Transitorio: un umano ha cliccato Approva e l'azione è in esecuzione. Usato per serializzare clic Approva concorrenti in modo che uno strumento con effetti reali non venga eseguito due volte. |

Lo stato `EXECUTING` è importante quando due revisori cliccano Approva simultaneamente - uno "vince", l'altro vede che l'approvazione è già cambiata.

### Cosa viene ripulito

Le approvazioni in sospeso restano tali fino a decisione. Scadono automaticamente dopo **90 giorni** dalla creazione. Le approvazioni in qualsiasi altro stato vengono anch'esse rimosse dopo 90 giorni per igiene dello storage.

I campi "deciso da" / "deciso il" / "eseguito il" / "risultato esecuzione" dell'approvazione vengono popolati man mano che l'approvazione attraversa gli stati - tutti visibili nella vista dettaglio dell'inbox.

### Integrazione Webhook

Due eventi webhook vengono inviati quando le approvazioni cambiano stato:

- **`approval.requested`** - alla creazione in `PENDING`.
- **`approval.decided`** - alla transizione verso APPROVED, REJECTED o EXECUTION_FAILED.

Entrambi sono firmati come ogni altro webhook. Vedi [Eventi Webhook](#webhook-events) e [Payload Webhook](#webhook-payloads).

### Hardening: gate per strumenti conosciuti

Le approvazioni rifiutano di accodare qualsiasi nome di strumento che non sia un tool riconosciuto dall'agente. Questo è un controllo di difesa in profondità: anche se in futuro un percorso di codice passasse un nome di strumento derivato da un LLM nel flusso di approvazione, quella stringa non potrà mai finire come approvazione accodata. L'insieme dei nomi degli strumenti noti è lo stesso elenco riportato in [Riferimento strumenti](#tools-overview).

### Modelli comuni di controllo

- **Agente di moderazione appena creato** - sottoporre a gate `ban_user`, `mark_comment_spam`, `mark_comment_approved`, `send_email`. Monitora l'inbox per alcune settimane, poi rimuovi il gating dagli strumenti a basso tasso di errore.
- **Agente di moderazione a lungo termine** - mantieni `ban_user` e qualsiasi azione irreversibile (`deleteAllUsersComments`, `banIP`) sottoposte a gate per sempre.
- **Regione UE** - `ban_user` è obbligatoriamente attivato dall'Articolo 17 indipendentemente da cosa selezioni. Vedi [Conformità Articolo 17 DSA UE](#eu-dsa-compliance).

### Cosa le approvazioni **non** fanno

- Non ritardano le altre chiamate a strumenti dell'agente. L'esecuzione dell'agente può invocare diversi strumenti e solo quelli sottoposti a gate vengono accodati - il resto viene eseguito normalmente.
- Non annullano l'esecuzione dell'agente se l'umano rifiuta. La parte non sottoposta a gate dell'esecuzione è già stata eseguita.

---