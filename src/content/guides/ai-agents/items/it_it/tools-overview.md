Gli **strumenti** di un agente sono le azioni che può intraprendere. Il modulo di modifica dell'agente ha una sezione **Chiamate degli strumenti consentite** dove selezioni gli strumenti che questo agente è autorizzato a usare, e una sezione **Approvazioni** dove selezioni le azioni che richiedono l'approvazione di un umano prima di avere effetto.

Ci sono tre livelli per ogni strumento:

- **Non consentito** - l'agente non può vederlo né usarlo.
- **Consentito, senza approvazione** - l'agente lo usa direttamente. Registrato nella cronologia delle esecuzioni.
- **Consentito, con approvazione** - la chiamata dell'agente viene messa in coda per la revisione umana e viene eseguita solo quando un umano la approva.

Gli strumenti non consentiti sono silenziosi: l'agente non può richiederli e la piattaforma li rifiuta categoricamente. Gli strumenti soggetti ad approvazione passano sempre attraverso il [vassoio delle approvazioni](#approval-workflow).

### Tracciamento (audit) di ogni azione

Ogni azione intrapresa dall'agente viene registrata con una breve giustificazione (1-2 frasi che spiegano il perché) e un punteggio di confidenza (0.0-1.0). Entrambi appaiono nella [Vista dettagli esecuzione](#run-detail-view) e su ogni [approvazione](#approval-workflow). La ricerca nella memoria è l'unica eccezione in sola lettura: non viene registrata come azione ed è sempre disponibile indipendentemente dalla allowlist.

### Riferimento agli strumenti

#### Pubblicare commenti

Permette all'agente di pubblicare un commento come se fosse lui stesso. Il commento viene mostrato pubblicamente sotto il nome visualizzato dell'agente. Usato dagli agenti greeter e summarizer. Reversibile - qualsiasi moderatore può rimuovere un commento sbagliato. Di solito consentito senza approvazione; abilitalo con approvazione se la tua community richiede che ogni messaggio pubblico sia revisionato da un umano.

#### Votare i commenti

Permette all'agente di votare a favore o contro un commento. Il voto conta nel totale dei voti del commento come qualsiasi altro voto. La maggior parte delle community preferisce che i bot non votino; non è abilitato in nessun template di avvio. Se lo permetti, il voto è reversibile.

#### Fissare / rimuovere la fissazione di un commento

Permette all'agente di fissare un commento in cima alla pagina o rimuovere la fissazione di uno già fissato. La piattaforma non applica una regola di un solo pin per thread, quindi un agente che fissa dovrebbe essere istruito a rimuovere prima il commento precedentemente fissato. Usato dal template Top Comment Pinner. Reversibile; di solito consentito senza approvazione.

#### Bloccare / sbloccare un commento

Permette all'agente di impedire ulteriori risposte sotto un commento, o ripristinare le risposte. Il commento bloccato rimane visibile. Utile per raffreddare discussioni accese, da abbinare a uno sblocco differito. Reversibile ma visibile alla tua community; considera di richiedere approvazione nelle community ad alto rischio.

#### Segnalare / rimuovere segnalazione spam

Permette all'agente di segnare un commento come spam (nascondendolo ai lettori e alimentando il classificatore di spam) o di rimuovere tale flag. Lo strumento fondamentale per qualsiasi agente di moderazione. Reversibile. Considera fortemente di richiedere approvazione nelle prime settimane mentre acquisisci fiducia nell'agente.

#### Approvare / revocare l'approvazione di un commento

Permette all'agente di mostrare ai lettori un commento in attesa, o nascondere uno già visibile. Più utile per tenant che tengono i nuovi commenti in attesa per la revisione dei moderatori. Ad alto impatto quando si revoca l'approvazione di un commento visibile - considera di richiedere approvazione.

#### Segnare un commento come revisionato

Uno strumento per lo stato della coda: marca un commento come "un moderatore (o agente) ha esaminato questo". Non cambia la visibilità. Basso impatto; raramente soggetto ad approvazione.

#### Assegnare un badge

Permette all'agente di assegnare a un utente un badge dalla configurazione badge del tuo tenant. Reversibile da un moderatore. Raramente soggetto ad approvazione. L'agente deve conoscere l'ID del badge, quindi includi gli ID pertinenti nelle tue [linee guida della community](#community-guidelines) o nel [prompt iniziale](#personality-prompt).

#### Inviare email

Permette all'agente di inviare un'email in testo semplice da `noreply@fastcomments.com` a un indirizzo da lui selezionato. Usa con parsimonia: l'email è lo strumento con maggior attrito e le email sbagliate sono difficili da annullare. Considera fortemente di richiedere approvazione e indirizza le approvazioni delle email a chi gestisce la casella di posta che l'agente finirà per contattare.

#### Salvare / cercare nella memoria dell'agente

Due strumenti abbinati che leggono e scrivono in un pool condiviso di note sull'utente per cui è scattato un trigger. La memoria è condivisa tra tutti gli agenti del tuo tenant, quindi le note di un agente di triage informano le decisioni di un agente moderatore. La ricerca è in sola lettura ed è sempre disponibile; il salvataggio è raramente soggetto ad approvazione. Vedi il [Sistema di memoria dell'agente](#agent-memory-system) per il progetto completo.

#### Avvertire un utente

Invia un messaggio privato (DM) di avvertimento a un utente su uno specifico commento e registra in modo atomico l'avvertimento nella memoria dell'agente. La policy di escalation della piattaforma è costruita attorno a questo strumento: prima avvisa, banna solo in caso di recidiva. Meno comunemente soggetto ad approvazione rispetto a `ban_user`, ma considera di applicare una limitazione nelle prime settimane di vita di un agente. Vedi [Warn user](#tool-warn-user) per la pagina completa.

#### Bannare un utente

Lo strumento più consequenziale che un agente può invocare. Banna un utente per una durata fissa, opzionalmente come shadow ban, opzionalmente bannando anche l'IP, opzionalmente cancellando tutti i commenti dell'utente. Le due opzioni distruttive (IP, delete-all) sono soggette a opt-in aggiuntive nel modulo di modifica. Nella regione UE, tutti i ban richiedono l'approvazione umana (vedi [Conformità all'articolo 17 del DSA UE](#eu-dsa-compliance)). Considera fortemente di richiedere approvazione ovunque. Vedi [Bannare un utente](#tool-ban-user) per la pagina completa.

### Sott0-opzioni dello strumento Ban

Lo strumento Ban espone due opzioni distruttive - delete-all-comments e ban-by-IP - che sono nascoste al modello del tutto fino a quando non le abiliti tramite la sezione **Ban options** nel modulo di modifica. Anche se il modello immagina il parametro, la piattaforma rifiuta i valori che non hai abilitato. Vedi [Bannare un utente](#tool-ban-user).