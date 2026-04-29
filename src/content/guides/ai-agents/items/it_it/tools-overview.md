Un agente dispone di **strumenti** che sono le azioni che può compiere. Il modulo di modifica dell'agente ha una sezione **Chiamate utensili consentite** in cui selezioni gli strumenti che questo agente può usare, e una sezione **Approvazioni** in cui selezioni le azioni che devono richiedere l'approvazione umana prima di avere effetto.

Ci sono tre livelli per qualsiasi strumento:

- **Non consentito** - l'agente non può vederlo né usarlo.
- **Consentito, senza approvazione** - l'agente lo usa direttamente. Registrato nella cronologia delle esecuzioni.
- **Consentito, con approvazione** - la chiamata dell'agente viene messa in coda per la revisione umana e viene eseguita solo quando un umano la approva.

Gli strumenti non consentiti sono silenziosi: l'agente non può richiederli e la piattaforma li rifiuta categoricamente. Gli strumenti soggetti ad approvazione passano sempre per la [casella delle approvazioni](#approval-workflow).

### Traccia di controllo per ogni azione

Ogni azione che l'agente compie viene registrata con una breve giustificazione (1-2 frasi che spiegano il motivo) e un punteggio di confidenza (0.0-1.0). Entrambi appaiono nella [Vista dettagli esecuzione](#run-detail-view) e su ogni [approvazione](#approval-workflow). La ricerca nella memoria è l'unica eccezione in sola lettura: non viene registrata come azione ed è sempre disponibile indipendentemente dalla lista consentita.

### Riferimento agli strumenti

#### Pubblicare commenti

Permette all'agente di pubblicare un commento come se fosse lui stesso. Il commento viene mostrato pubblicamente sotto il nome visualizzato dell'agente. Usato dagli agenti di accoglienza e di riepilogo. Reversibile - qualsiasi moderatore può rimuovere un commento inappropriato. Di solito consentito senza approvazione; mettilo sotto controllo se la tua comunità necessita che ogni messaggio esposto al pubblico sia revisionato da un umano.

#### Modificare un commento

Permette all'agente di riscrivere il testo di un commento nell'ambito del suo permesso. Il testo originale viene preservato nel registro di audit del commento. Riservalo a casi ristretti - la redazione di dati personali sensibili che un utente ha divulgato, o la correzione della propria risposta precedente da parte dell'agente. Non per riscrivere opinioni o attenuare il tono. **Valuta fortemente di metterlo sotto approvazione.** Vedi [Modifica commento](#tool-edit-comment) per la pagina completa.

#### Votare i commenti

Permette all'agente di esprimere un voto positivo o negativo su un commento. Il voto conta nel totale dei voti del commento come qualsiasi altro voto. La maggior parte delle community preferisce che i bot non votino; non abilitato in nessun modello iniziale. Se lo permetti, il voto è reversibile.

#### Fissare / rimuovere la fissazione di un commento

Permette all'agente di fissare un commento in cima alla pagina o di rimuovere la fissazione da uno già fissato. La piattaforma non applica una regola di un solo pin per thread, quindi un agente che fissa dovrebbe essere istruito a rimuovere prima la fissazione del commento precedente. Usato dal modello Top Comment Pinner. Reversibile; di solito consentito senza approvazione.

#### Bloccare / sbloccare un commento

Permette all'agente di impedire ulteriori risposte sotto un commento, o di ripristinare le risposte. Il commento bloccato rimane visibile. Utile per raffreddare thread accesi, abbinato a uno sblocco differito. Reversibile ma visibile alla tua comunità; considera di metterlo sotto approvazione nelle comunità ad alto rischio.

#### Segnalare / rimuovere segnalazione spam

Permette all'agente di contrassegnare un commento come spam (nascondendolo ai lettori e alimentando il classificatore di spam) o di rimuovere tale flag. Lo strumento fondamentale per qualsiasi agente di moderazione. Reversibile. Valuta fortemente di metterlo sotto approvazione nelle prime settimane finché non costruisci fiducia nell'agente.

#### Approva / disapprova un commento

Permette all'agente di mostrare un commento in attesa ai lettori, o di nascondere uno già visibile. Più utile per tenant che trattengono i nuovi commenti per la revisione del moderatore. Ha un alto impatto quando si disapprova un commento visibile - considera di metterlo sotto approvazione.

#### Contrassegnare un commento come revisionato

Uno strumento per lo stato della coda: contrassegna un commento come "un moderatore (o agente) ha esaminato questo". Non cambia la visibilità. Basso impatto; raramente soggetto ad approvazione.

#### Assegnare un badge

Permette all'agente di assegnare a un utente un badge dalla configurazione dei badge del tuo tenant. Reversibile da un moderatore. Raramente soggetto ad approvazione. L'agente deve conoscere l'ID del badge, quindi includi gli ID rilevanti nelle tue [linee guida della comunità](#community-guidelines) o nel [prompt iniziale](#personality-prompt).

#### Inviare email

Permette all'agente di inviare un'email in testo semplice da `noreply@fastcomments.com` a un indirizzo scelto da lui. Usalo con parsimonia - l'email è lo strumento con la massima frizione e le email errate sono difficili da annullare. Valuta fortemente di metterlo sotto approvazione, e instrada le approvazioni via email a chiunque possieda la casella che l'agente finirà per contattare.

#### Salvare / cercare nella memoria dell'agente

Due strumenti accoppiati che leggono e scrivono in un pool di note condiviso riguardo l'utente per cui è scattato un trigger. La memoria è condivisa tra tutti gli agenti nel tuo tenant, quindi le note di un agente di triage informano le decisioni di un agente moderatore. La ricerca è in sola lettura ed è sempre disponibile; il salvataggio è raramente soggetto ad approvazione. Vedi il [Sistema di memoria dell'agente](#agent-memory-system) per il progetto completo.

#### Avvertire un utente

Invia un messaggio diretto privato di avvertimento a un utente riguardo a un commento specifico e registra atomica-mente l'avvertimento nella memoria dell'agente. La politica di escalation della piattaforma è costruita attorno a questo strumento - avvertire prima, bannare solo se l'utente recidiva. Meno frequentemente soggetto ad approvazione rispetto a `ban_user`, ma considera di metterlo sotto controllo durante le prime settimane della vita di un agente. Vedi [Avverti utente](#tool-warn-user) per la pagina completa.

#### Bannare un utente

Lo strumento più consequenziale che un agente può chiamare. Bannare un utente con una durata fissa, opzionalmente come shadow ban, opzionalmente bannando anche l'IP, opzionalmente eliminando anche tutti i commenti dell'utente. Le due opzioni distruttive (IP, elimina-tutto) sono soggette a opt-in aggiuntivi nel modulo di modifica. Nella regione UE, tutti i ban richiedono l'approvazione umana (vedi [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Valuta fortemente di metterlo sotto approvazione ovunque. Vedi [Banna utente](#tool-ban-user) per la pagina completa.

### Sotto-opzioni dello strumento Ban

Lo strumento Ban espone due opzioni distruttive - delete-all-comments e ban-by-IP - che sono nascoste al modello fino a quando non le abiliti tramite la sezione **Opzioni di ban** nel modulo di modifica. Anche se il modello allucina il parametro, la piattaforma rifiuta i valori che non hai abilitato. Vedi [Banna utente](#tool-ban-user).