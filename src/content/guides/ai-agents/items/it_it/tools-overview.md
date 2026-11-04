Gli **strumenti** di un agente sono le azioni che può compiere. Il modulo di modifica dell'agente contiene una sezione **Allowed tool calls** in cui selezioni gli strumenti che l'agente è autorizzato a usare, e una sezione **Approvals** in cui selezioni le azioni che richiedono l'approvazione di un essere umano prima di avere effetto.

Esistono tre livelli per ogni strumento:

- **Non consentito** - l'agente non può vederlo né usarlo.
- **Consentito, senza approvazione** - l'agente lo usa direttamente. Registrato nella cronologia delle esecuzioni.
- **Consentito, con approvazione** - la chiamata dell'agente viene messa in coda per la revisione umana e viene eseguita solo quando un essere umano approva.

Gli strumenti non consentiti sono silenziosi: l'agente non può richiederli e la piattaforma li rifiuta categoricamente. Gli strumenti con approvazione passano sempre attraverso la [cassetta postale delle approvazioni](#approval-workflow).

### Tracciamento di controllo su ogni azione

Ogni azione compiuta dall'agente viene registrata con una breve giustificazione (1-2 frasi che spiegano il perché) e un punteggio di confidenza (0.0-1.0). Entrambi compaiono nella [Run Detail View](#run-detail-view) e su ogni [approvazione](#approval-workflow). La ricerca nella memoria è l'unica eccezione di sola lettura: non viene registrata come azione ed è sempre disponibile indipendentemente dalla allowlist.

### Riferimento agli strumenti

#### Pubblicare commenti

Consente all'agente di pubblicare un commento per conto proprio. Il commento viene mostrato pubblicamente sotto il nome visualizzato dell'agente. Usato dagli agenti greeter e summarizer. Reversibile - qualsiasi moderatore può rimuovere un commento inappropriato. Richiedi l'approvazione se la tua community necessita che ogni messaggio pubblico sia revisionato da un essere umano.

#### Modificare un commento

Consente all'agente di riscrivere il testo di un commento nell'ambito di applicazione. Il testo originale viene conservato nel log di audit del commento. Riservalo a casi limitati - oscurare PII divulgati da un utente, o correggere una precedente risposta dell'agente stesso. Non per riscrivere opinioni o attenuare il tono. Vedi [Edit comment](#tool-edit-comment) per la pagina completa.

#### Votare i commenti

Consente all'agente di votare a favore o contro un commento. Il voto conta nel totale dei voti del commento come qualsiasi altro voto. La maggior parte delle community preferisce non avere bot che votano; non è abilitato in nessun template iniziale. Se lo consenti, il voto è reversibile.

#### Fissare / sfissare un commento

Consente all'agente di fissare un commento in cima alla pagina o di sfissarne uno già fissato. La piattaforma non impone una regola di un solo pin per thread, quindi un agente che fissa commenti dovrebbe essere istruito a sfissare prima il commento fissato in precedenza. Per scoprire cosa è già fissato sulla stessa pagina, l'agente può chiamare lo strumento di sola lettura `get_pinned_comments` (vedi sotto). Usato dal template Top Comment Pinner.

#### Bloccare / sbloccare un commento

Consente all'agente di impedire ulteriori risposte sotto un commento, o di ripristinarle. Il commento bloccato resta visibile. Utile per i periodi di raffreddamento nei thread accesi, abbinato a uno sblocco differito. Per scoprire cosa è attualmente bloccato sulla stessa pagina, l'agente può chiamare lo strumento di sola lettura `get_locked_comments` (vedi sotto).

#### Segnalare / rimuovere la segnalazione spam

Consente all'agente di segnalare un commento come spam (nascondendolo ai lettori e alimentando il classificatore dello spam) o di rimuovere tale segnalazione. Lo strumento fondamentale per qualsiasi agente di moderazione. Reversibile.

#### Approvare / disapprovare un commento

Consente all'agente di mostrare ai lettori un commento in attesa, o di nasconderne uno già visibile. Più utile sui tenant che trattengono i nuovi commenti per la revisione del moderatore.

#### Segnare un commento come revisionato

Uno strumento di stato della coda: segna un commento come "un moderatore (o un agente) l'ha esaminato". Non modifica la visibilità. Basso rischio; raramente sottoposto ad approvazione.

#### Assegnare un badge

Consente all'agente di assegnare a un utente un badge che hai configurato per il tuo tenant. Reversibile da un moderatore. Quando questo strumento è abilitato, l'agente può vedere i badge del tuo tenant e scegliere quello giusto da solo, quindi non è necessario incollare identificatori di badge nelle linee guida della community o nel prompt iniziale. Per orientare quale badge viene assegnato per quale comportamento, riferisciti ai badge tramite la loro **Display Label** nel prompt.

#### Inviare email

Consente all'agente di inviare un'email in testo semplice all'autore di un commento nell'ambito del trigger. L'agente non vede mai l'indirizzo email del destinatario - sceglie un commento e la piattaforma invia all'indirizzo che quel commentatore ha lasciato quando ha pubblicato. L'indirizzo mittente è quello brandizzato del tuo tenant (con DKIM) quando il dominio del commento corrisponde a un dominio configurato, altrimenti quello predefinito della piattaforma. Usalo con parsimonia - l'email è lo strumento con maggiore attrito e le email sbagliate sono difficili da annullare.

#### Salvare / cercare nella memoria dell'agente

Due strumenti abbinati che leggono e scrivono in un pool condiviso di note sull'utente per cui un trigger è scattato. La memoria è condivisa tra tutti gli agenti del tuo tenant, quindi le note di un agente di triage informano le decisioni di un agente moderatore. La ricerca è di sola lettura e sempre disponibile; il salvataggio è raramente sottoposto ad approvazione. Vedi [Agent Memory System](#agent-memory-system) per il design completo.

#### Get pinned comments / Get locked comments

Due strumenti di scoperta di sola lettura che elencano i commenti fissati (o bloccati) sulla stessa pagina (`urlId`) su cui il trigger è scattato. Non accettano argomenti - la pagina viene letta dal contesto del trigger, quindi l'agente non può spostarsi su altre pagine. Usali quando un agente deve agire su un commento già fissato o bloccato - tipicamente la prima chiamata prima di `unpin_comment` o `unlock_comment`, o prima di fissare un nuovo commento così che quello esistente possa essere sfissato prima.

Ogni strumento è sottoposto ad approvazione separatamente in **Allowed tool calls** (l'amministratore seleziona `List pinned comments on the current page` o `List locked comments on the current page`). Non possono essere sottoposti ad approvazione - gli strumenti di sola lettura non hanno effetti collaterali da approvare. Chiamarli non viene registrato come azione nella cronologia delle esecuzioni; solo la chiamata risultante `unpin_comment` / `unlock_comment` / `pin_comment` (se presente) compare. L'elenco è limitato ai 20 risultati più recenti per chiamata.

Importante capire: quando uno di questi strumenti restituisce un commentId, quel commentId viene aggiunto all'ambito per-esecuzione dell'agente, così la chiamata successiva `unpin_comment` / `unlock_comment` viene validata dal controllo di sicurezza tool-target della piattaforma. Senza prima chiamare lo strumento di scoperta, l'agente non può agire su commenti che non sono già nell'ambito immediato del trigger. Quindi un agente di tipo unpin tipicamente ha entrambi gli strumenti abilitati (es. `get_pinned_comments` più `unpin_comment`).

#### Avvisare un utente

Invia un avviso privato in DM a un utente riguardo a un commento specifico, e registra atomicamente l'avviso nella memoria dell'agente. La politica di escalation della piattaforma è costruita attorno a questo strumento - prima avvisare, bannare solo se l'utente recidiva. Vedi [Warn user](#tool-warn-user) per la pagina completa.

#### Bannare un utente

Lo strumento più consequenziale che un agente possa chiamare. Banna un utente con una durata fissa, opzionalmente come shadow ban, opzionalmente bannando anche l'IP, opzionalmente eliminando anche tutti i commenti dell'utente. Le due opzioni distruttive (IP, delete-all) sono sottoposte ad approvazione tramite opt-in aggiuntivi nel modulo di modifica. Nella regione UE, tutti i ban richiedono l'approvazione umana (vedi [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Vedi [Ban user](#tool-ban-user) per la pagina completa.

### Sotto-opzioni dello strumento di ban

Lo strumento di ban espone due opzioni distruttive - delete-all-comments e ban-by-IP - che sono nascoste completamente al modello finché non le attivi tramite la sezione **Ban options** del modulo di modifica. Anche se il modello allucina il parametro, la piattaforma rifiuta i valori che non hai attivato. Vedi [Ban user](#tool-ban-user).