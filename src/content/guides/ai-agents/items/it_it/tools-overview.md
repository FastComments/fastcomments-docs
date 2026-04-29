Un agente dispone di **strumenti** che sono le azioni che può intraprendere. Il modulo di modifica dell'agente ha una sezione **Chiamate strumento consentite** dove selezioni gli strumenti che questo agente è autorizzato a usare, e una sezione **Approvazioni** dove selezioni le azioni che richiedono l'approvazione umana prima di avere effetto.

Esistono tre livelli per qualsiasi strumento:

- **Non consentito** - l'agente non può vederlo né usarlo.
- **Consentito, senza approvazione** - l'agente lo usa direttamente. Registrato nella cronologia esecuzioni.
- **Consentito, con approvazione** - la chiamata dell'agente viene messa in coda per la revisione umana e viene eseguita solo quando un umano approva.

Gli strumenti non consentiti sono silenziosi: l'agente non può richiederli e la piattaforma li rifiuta categoricamente. Gli strumenti soggetti ad approvazione passano sempre attraverso l'[inbox delle approvazioni](#approval-workflow).

### Traccia di audit su ogni azione

Ogni azione che l'agente compie è registrata con una breve giustificazione (1-2 frasi che spiegano il motivo) e un punteggio di confidenza (0.0-1.0). Entrambi compaiono nella [Vista dettagli dell'esecuzione](#run-detail-view) e su ogni [approvazione](#approval-workflow). La ricerca nella memoria è l'unica eccezione in sola lettura: non viene registrata come azione ed è sempre disponibile indipendentemente dalla allowlist.

### Riferimento degli strumenti

#### Pubblicare commenti

Permette all'agente di pubblicare un commento come se fosse lui stesso. Il commento viene mostrato pubblicamente con il nome visualizzato dell'agente. Utilizzato dagli agenti greeter e summarizer. Reversibile - qualsiasi moderatore può rimuovere un commento inappropriato. Di solito consentito senza approvazione; richiedi approvazione se la tua community necessita che ogni messaggio rivolto al pubblico sia revisionato da un umano.

#### Modificare un commento

Permette all'agente di riscrivere il testo di un commento nell'ambito. Il testo originale è preservato nel registro di audit del commento. Da riservare a casi ristretti - redazione di PII che un utente ha divulgato, o modifica della propria precedente risposta dell'agente. Non per riscrivere opinioni o attenuare il tono. **Valuta fortemente di richiederne l'approvazione.** Vedi [Modifica commento](#tool-edit-comment) per la pagina completa.

#### Votare i commenti

Permette all'agente di votare a favore o contro un commento. Il voto conta nel totale dei voti del commento come qualsiasi altro voto. La maggior parte delle community preferisce che i bot non votino; non è abilitato in nessun template di partenza. Se lo consenti, il voto è reversibile.

#### Pinnare / rimuovere pin a un commento

Permette all'agente di fissare un commento in cima alla pagina o rimuovere il pin da uno già fissato. La piattaforma non applica una regola di un pin per thread, quindi un agente che pinns dovrebbe essere istruito a rimuovere prima il commento precedentemente pinnato. Utilizzato dal template Top Comment Pinner. Reversibile; di solito consentito senza approvazione.

#### Bloccare / sbloccare un commento

Permette all'agente di impedire ulteriori risposte sotto un commento, o ripristinare le risposte. Il commento bloccato resta visibile. Utile per raffreddare discussioni accese, abbinato a uno sblocco differito. Reversibile ma visibile alla tua community; considera di richiedere approvazione nelle community ad alto rischio.

#### Segnare / rimuovere segnalazione spam

Permette all'agente di segnare un commento come spam (nascondendolo ai lettori e alimentando il classificatore di spam) o di cancellare tale segnalazione. Lo strumento fondamentale per qualsiasi agente di moderazione. Reversibile. Valuta fortemente di richiedere approvazione nelle prime settimane mentre costruisci fiducia nell'agente.

#### Approvare / disapprovare un commento

Permette all'agente di mostrare un commento in attesa ai lettori, o nasconderne uno già visibile. Più utile nei tenant che trattengono nuovi commenti per la revisione dei moderatori. Alto impatto quando si disapprova un commento già visibile - considera di richiedere approvazione.

#### Contrassegnare un commento come revisionato

Uno strumento per lo stato delle code: contrassegna un commento come "un moderatore (o agente) ha esaminato questo". Non cambia la visibilità. Basso rischio; raramente soggetto ad approvazione.

#### Assegnare un badge

Permette all'agente di assegnare a un utente un badge che hai configurato per il tuo tenant. Reversibile da un moderatore. Raramente soggetto ad approvazione. Quando questo strumento è abilitato, l'agente può vedere i badge del tuo tenant e scegliere quello giusto da solo, quindi non è necessario incollare gli identificatori dei badge nelle linee guida della community o nel prompt iniziale. Se vuoi indirizzare quale badge venga assegnato per quale comportamento, riferisciti ai badge tramite la loro **Etichetta di visualizzazione** nel prompt.

#### Inviare email

Permette all'agente di inviare un'email in plain-text all'autore di un commento nell'ambito del trigger. L'agente non vede mai l'indirizzo email del destinatario - sceglie un commento e la piattaforma consegna all'indirizzo che il commentatore ha lasciato quando ha pubblicato. L'indirizzo mittente è il mittente brandizzato del tuo tenant (con DKIM) quando il dominio del commento corrisponde a un dominio configurato, altrimenti viene usato il valore predefinito della piattaforma. Usa con parsimonia - l'email è lo strumento con la più alta frizione e le email sbagliate sono difficili da annullare. Valuta fortemente di richiedere approvazione, e instrada le email di approvazione a chiunque gestisca la casella di posta che l'agente finirà per contattare.

#### Salvare / cercare nella memoria dell'agente

Due strumenti accoppiati che leggono e scrivono in un pool di note condivise sull'utente per cui è scattato un trigger. La memoria è condivisa tra tutti gli agenti nel tuo tenant, quindi le note di un agente di triage informano le decisioni di un agente moderatore. La ricerca è in sola lettura ed è sempre disponibile; il salvataggio è raramente soggetto ad approvazione. Vedi [Sistema di memoria dell'agente](#agent-memory-system) per il design completo.

#### Avvertire un utente

Invia un messaggio privato di avvertimento a un utente riguardo a uno specifico commento, e registra in modo atomico l'avvertimento nella memoria dell'agente. La policy di escalation della piattaforma è costruita attorno a questo strumento - prima avvertire, bandire solo se l'utente recidiva. Meno comunemente soggetto ad approvazione rispetto a `ban_user`, ma considera di richiedere approvazione durante le prime settimane di vita di un agente. Vedi [Avverti utente](#tool-warn-user) per la pagina completa.

#### Bannare un utente

Lo strumento più consequenziale che un agente può chiamare. Bannare un utente per una durata fissa, opzionalmente come shadow ban, opzionalmente bannando anche l'IP, opzionalmente cancellando tutti i commenti dell'utente. Le due opzioni distruttive (IP, elimina-tutti) sono sottoposte a opt-in extra nel modulo di modifica. Anche se il modello allucinasse il parametro, la piattaforma rifiuta i valori a cui non hai dato il consenso. Nella regione UE, tutti i ban richiedono l'approvazione umana (vedi [Conformità articolo 17 DSA UE](#eu-dsa-compliance)). Valuta fortemente di richiedere approvazione ovunque. Vedi [Banna utente](#tool-ban-user) per la pagina completa.

### Sotto-opzioni dello strumento Ban

Lo strumento Ban espone due opzioni distruttive - delete-all-comments e ban-by-IP - che sono nascoste al modello del tutto fino a quando non le abiliti tramite la sezione **Opzioni Ban** nel modulo di modifica. Anche se il modello inventasse il parametro, la piattaforma rifiuta valori a cui non hai aderito. Vedi [Banna utente](#tool-ban-user).