Gli **strumenti** di un agente sono le azioni che può compiere. Il modulo di modifica dell'agente ha una sezione **Allowed tool calls** dove selezioni gli strumenti che questo agente è autorizzato a usare, e una sezione **Approvals** dove selezioni le azioni che richiedono l'approvazione umana prima che abbiano effetto.

Ci sono tre livelli per qualsiasi strumento:

- **Disallowed** - l'agente non può vederlo né usarlo.
- **Allowed, no approval** - l'agente lo usa direttamente. Registrato nella cronologia delle esecuzioni.
- **Allowed, with approval** - la chiamata dell'agente viene messa in coda per la revisione umana e viene eseguita solo quando un umano la approva.

Gli strumenti disabilitati sono silenziosi: l'agente non può richiederli e la piattaforma li rifiuta categoricamente. Gli strumenti soggetti ad approvazione passano sempre attraverso la [approvals inbox](#approval-workflow).

### Tracciamento delle azioni

Ogni azione intrapresa dall'agente viene registrata con una breve giustificazione (1-2 frasi che spiegano il perché) e un punteggio di confidenza (0.0-1.0). Entrambi compaiono nella [Run Detail View](#run-detail-view) e su ogni [approval](#approval-workflow). La ricerca nella memoria è l'unica eccezione in sola lettura: non viene registrata come azione ed è sempre disponibile indipendentemente dalla allowlist.

### Riferimento degli strumenti

#### Posting comments

Permette all'agente di pubblicare un commento come se fosse lui. Il commento viene mostrato pubblicamente sotto il nome visualizzato dell'agente. Utilizzato dagli agenti di benvenuto e dai riassuntori. Reversibile - qualsiasi moderatore può rimuovere un commento inappropriato. Mettilo dietro approvazione se la tua community richiede che ogni messaggio rivolto al pubblico sia revisionato da un umano.

#### Editing a comment

Permette all'agente di riscrivere il testo di un commento entro l'ambito. Il testo originale viene preservato nel registro di controllo del commento. Riservalo a casi ristretti - la rimozione di PII che un utente ha divulgato, o l'adeguamento della propria precedente risposta dell'agente. Non per riscrivere opinioni o attenuare toni. Vedi [Edit comment](#tool-edit-comment) per la pagina completa.

#### Voting on comments

Permette all'agente di votare a favore o contro un commento. Il voto contribuisce al totale dei voti del commento come qualsiasi altro voto. La maggior parte delle community preferisce che i bot non votino; non è abilitato in nessun template di partenza. Se lo permetti, il voto è reversibile.

#### Pin / unpin a comment

Permette all'agente di appuntare (pin) un commento in cima alla pagina o rimuovere l'appuntamento su uno già appuntato. La piattaforma non applica una regola di un solo pin per thread, quindi un agente che esegue pin dovrebbe essere istruito a rimuovere prima il commento precedentemente appuntato. Per scoprire cosa è già appuntato sulla stessa pagina, l'agente può chiamare lo strumento in sola lettura `get_pinned_comments` (vedi sotto). Utilizzato dal template Top Comment Pinner.

#### Lock / unlock a comment

Permette all'agente di impedire ulteriori risposte sotto un commento, o ripristinare le risposte. Il commento bloccato rimane visibile. Utile per raffreddare thread accesi, da combinare con uno sblocco differito. Per scoprire cosa è attualmente bloccato sulla stessa pagina, l'agente può chiamare lo strumento in sola lettura `get_locked_comments` (vedi sotto).

#### Mark / unmark spam

Permette all'agente di contrassegnare un commento come spam (nascondendolo ai lettori e alimentando il classificatore di spam) oppure di rimuovere tale flag. Lo strumento principale per qualsiasi agente di moderazione. Reversibile.

#### Approve / un-approve a comment

Permette all'agente di mostrare un commento trattenuto ai lettori, o nascondere uno già visibile. Utile soprattutto per tenant che trattengono nuovi commenti per la revisione dei moderatori.

#### Mark a comment reviewed

Uno strumento di stato in coda: contrassegna un commento come "un moderatore (o agente) l'ha esaminato". Non cambia la visibilità. Bassa criticità; raramente sottoposto ad approvazione.

#### Award a badge

Permette all'agente di assegnare a un utente un badge che hai configurato per il tuo tenant. Reversibile da un moderatore. Quando questo strumento è abilitato, l'agente può vedere i badge del tuo tenant e scegliere quello giusto da solo, quindi non è necessario incollare gli identificatori dei badge nelle linee guida della community o nel prompt iniziale. Per indirizzare quale badge venga assegnato per quale comportamento, fai riferimento ai badge tramite la loro **Display Label** nel prompt.

#### Send email

Permette all'agente di inviare un'email in testo semplice all'autore di un commento nel campo d'azione del trigger. L'agente non vede mai l'indirizzo email del destinatario - sceglie un commento e la piattaforma consegna all'indirizzo che quel commentatore ha lasciato al momento della pubblicazione. L'indirizzo del mittente è il sender brandizzato del tuo tenant (con DKIM) quando il dominio del commento corrisponde a un dominio configurato, altrimenti viene usato il default della piattaforma. Usalo con parsimonia - l'email è lo strumento a più alta frizione e le email sbagliate sono difficili da annullare.

#### Save / search agent memory

Due strumenti abbinati che leggono e scrivono in un pool condiviso di note sull'utente per cui è scattato un trigger. La memoria è condivisa tra tutti gli agenti del tuo tenant, quindi le note di un agente di triage informano le decisioni di un agente moderatore. La ricerca è in sola lettura ed è sempre disponibile; il salvataggio è raramente soggetto ad approvazione. Vedi [Agent Memory System](#agent-memory-system) per il progetto completo.

#### Get pinned comments / Get locked comments

Due strumenti di scoperta in sola lettura che elencano i commenti appuntati (o bloccati) sulla stessa pagina (`urlId`) su cui è scattato il trigger. Non accettano argomenti - la pagina viene letta dal contesto del trigger, quindi l'agente non può pivotare verso altre pagine. Usali quando un agente deve agire su un commento che è già appuntato o bloccato - tipicamente la prima chiamata prima di `unpin_comment` o `unlock_comment`, o prima di appuntare un nuovo commento in modo che quello esistente possa essere rimosso per primo.

Ogni strumento è sottoposto separatamente nella sezione **Allowed tool calls** (l'amministratore seleziona `List pinned comments on the current page` o `List locked comments on the current page`). Non possono essere messe sotto approvazione - gli strumenti in sola lettura non hanno effetti collaterali da approvare. La loro chiamata non viene registrata come azione nella cronologia delle esecuzioni; solo la conseguente chiamata `unpin_comment` / `unlock_comment` / `pin_comment` (se presente) viene mostrata. La lista è limitata ai 20 risultati più recenti per chiamata.

Importante da capire: quando uno di questi strumenti restituisce un commentId, quel commentId viene aggiunto all'ambito per-esecuzione dell'agente, quindi la chiamata successiva `unpin_comment` / `unlock_comment` viene validata tramite il controllo di sicurezza sulla destinazione dello strumento della piattaforma. Senza aver prima chiamato lo strumento di scoperta, l'agente non può agire su commenti che non sono già nell'ambito immediato del trigger. Quindi un agente di tipo unpin normalmente ha entrambi gli strumenti abilitati (es. `get_pinned_comments` più `unpin_comment`).

#### Warn a user

Invia un DM privato di avviso a un utente riguardo a uno specifico commento, e registra in modo atomico l'avvertimento nella memoria dell'agente. La policy di escalation della piattaforma si basa su questo strumento - avvisa prima, banna solo se l'utente recidiva. Vedi [Warn user](#tool-warn-user) per la pagina completa.

#### Ban a user

Lo strumento più significativo che un agente può chiamare. Bannare un utente per una durata fissa, opzionalmente come shadow ban, opzionalmente bannando anche l'IP, opzionalmente cancellando tutti i commenti dell'utente. Le due opzioni distruttive (IP, elimina-tutto) sono messe dietro opt-in aggiuntivi nel modulo di modifica. Nella regione EU, tutti i ban richiedono approvazione umana (vedi [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Vedi [Ban user](#tool-ban-user) per la pagina completa.

### Opzioni secondarie dello strumento Ban

Lo strumento Ban espone due opzioni distruttive - delete-all-comments e ban-by-IP - che sono nascoste al modello fino a quando non le abiliti tramite la sezione **Ban options** nel modulo di modifica. Anche se il modello allucina il parametro, la piattaforma rifiuta valori che non hai optato per accettare. Vedi [Ban user](#tool-ban-user).