Una volta che un amministratore ha registrato FastComments come strumento LTI 1.3 Advantage e ha approvato le policy dell'istituzione, gli insegnanti lo aggiungono ai corsi tramite i punti di inserimento standard di Blackboard. I passaggi esatti differiscono tra Ultra Course View e Original Course View, quindi entrambi sono descritti di seguito.

#### Ultra Course View

Ultra Course View è l'impostazione predefinita in Blackboard Learn SaaS a partire dal 2026.

1. Apri il corso e vai alla pagina **Course Content**.
2. Passa il mouse o tocca il punto in cui vuoi che compaia il thread di commenti nella struttura e clicca il pulsante viola **+** (Aggiungi contenuto).
3. Scegli **Content Market**. Il pannello Content Market elenca tutti gli strumenti LTI approvati e i placement dei Building Block per la tua istituzione.
4. Trova il riquadro **FastComments** e cliccalo. Blackboard crea un elemento di contenuto nella posizione in cui hai aperto il menu **+**.
5. L'elemento compare nella struttura come voce "Visible to students" per impostazione predefinita per gli insegnanti che hanno l'opzione **Hide from students** disattivata come preferenza personale. Se la tua impostazione predefinita è **Hidden**, l'elemento viene creato nascosto e puoi modificare il selettore di visibilità nella riga dell'elemento quando sei pronto.
6. Per rinominare l'elemento, clicca il titolo nella struttura e digita una nuova etichetta. Il titolo che gli studenti vedono nella struttura è indipendente dall'identificatore del thread di FastComments, quindi rinominare è sicuro in qualsiasi momento.

Se non vedi **Content Market** come opzione, la tua istituzione ha nascosto il placement. Puoi raggiungere lo stesso selettore anche tramite **More tools** nello stesso menu **+**, sotto il gruppo **LTI Tools**.

#### Original Course View

Original Course View è ancora supportato in Learn SaaS e rimane l'esperienza principale per i siti Learn 9.1 self-hosted sulla linea di rilascio Q4 2024 CU.

1. Apri il corso ed entra in una **Content Area** (ad esempio, l'area predefinita **Information** o **Content** nel menu del corso).
2. Attiva la **Edit Mode** con l'interruttore in alto a destra della pagina.
3. Clicca **Build Content** nella barra delle azioni.
4. Sotto il sottomenu **Learning Tools**, clicca **FastComments**. Il sottomenu Learning Tools viene popolato dai placement degli strumenti LTI 1.3 dopo che un amministratore ha registrato lo strumento. Se non lo vedi, consulta la sezione sui problemi noti qui sotto.
5. Nel modulo **Create FastComments**, imposta:
   - **Name**: l'etichetta che gli studenti vedono nella content area.
   - **Description**: testo opzionale mostrato sopra il thread incorporato.
   - **Permit Users to View this Content**: interruttore di disponibilità Sì/No.
   - **Track Number of Views**: attivalo se desideri le statistiche di visualizzazione per elemento di Blackboard. FastComments esegue le proprie analisi in modo indipendente.
   - **Date and Time Restrictions**: finestre opzionali **Display After** / **Display Until**.
6. Invia. Lo strumento appare come elemento cliccabile nella content area.

#### Incorporamento all'interno di un elemento o documento

In entrambe le visualizzazioni del corso, gli insegnanti incorporano FastComments in linea nel corpo di un Item, Document o qualsiasi campo di testo formattato tramite il pulsante LTI Advantage dell'editor di contenuti.

Ultra Course View:

1. Crea o modifica un **Document**.
2. Clicca **Add content** all'interno del corpo del documento dove vuoi che appaia il thread.
3. Nella barra degli strumenti dell'editor, apri il menu **Insert content** e clicca **Content Market** (il punto di ingresso LTI Advantage / Deep Linking).
4. Seleziona **FastComments**. FastComments restituisce un payload di deep link e Blackboard inserisce un blocco incorporato nel corpo del documento nella posizione del cursore.
5. Salva il documento. Gli studenti vedono il thread renderizzato in linea mentre scorrono oltre.

Original Course View:

1. Modifica qualsiasi elemento con un corpo di testo formattato.
2. Nella barra degli strumenti dell'editor di contenuti, clicca l'icona più **Add Content** e scegli **Content Market** (etichettato **Add Content from External Tool** nelle versioni Q4 2024 CU più datate).
3. Seleziona **FastComments**. L'editor inserisce un blocco segnaposto che fa riferimento alla risorsa deep-linked.
4. Invia l'elemento.

Ogni incorporamento tramite deep link produce il proprio thread FastComments, quindi un Item con due blocchi FastComments incorporati ha due flussi di commenti indipendenti.

#### Visibilità, condizioni di rilascio e restrizioni di gruppo

Gli elementi di contenuto FastComments si comportano come qualsiasi altro elemento di contenuto di Blackboard per quanto riguarda le regole di controllo degli accessi sovrapposte.

- Ultra: clicca il selettore di visibilità sulla riga (**Visible to students**, **Hidden from students**, **Conditional availability**). La disponibilità condizionale supporta finestre di data/ora, regole di prestazione sugli elementi del registro dei voti e regole sui membri dei gruppi del corso.
- Original: apri il menu contestuale dell'elemento e scegli **Adaptive Release** o **Adaptive Release: Advanced** per limitare lo strumento per data, appartenenza, voto o stato di revisione. Usa **Set Group Availability** sull'elemento per limitarlo a gruppi specifici del corso.

FastComments rispetta qualsiasi decisione prenda il controllo di Blackboard. Se Blackboard nasconde l'elemento a uno studente, il lancio LTI non avviene mai per quello studente e non appare nella vista del moderatore.

#### Comportamento del registro dei voti

FastComments non riporta voti tramite LTI Advantage Assignment and Grade Services. Nessuna colonna di voto viene creata automaticamente per gli elementi di contenuto FastComments.

Se il tuo tenant Blackboard è configurato per creare automaticamente una colonna nel registro dei voti per ogni nuovo elemento di contenuto indipendentemente dai metadati di valutazione, apparirà comunque una colonna vuota. Per nasconderla:

- Ultra: apri il **Gradebook**, clicca l'intestazione della colonna, scegli **Edit** e disattiva **Show to students** e **Include in calculations**. Oppure usa **Delete** se la tua istituzione consente l'eliminazione delle colonne per gli elementi non valutati.
- Original: apri il **Grade Center**, clicca il chevron della colonna, scegli **Hide from Users (on/off)** e, facoltativamente, **Hide from Instructor View** sotto **Column Organization**.

#### Cosa vedono gli studenti

Quando uno studente apre l'elemento FastComments o scorre fino a un blocco incorporato:

1. Blackboard lancia il messaggio LTI 1.3 verso FastComments. Lo studente accede tramite SSO utilizzando la sua identità Blackboard (nome, email, avatar, ruolo) senza vedere un modulo di accesso.
2. Il thread di commenti viene renderizzato nell'iframe. Thread, risposte, menzioni e reazioni sono tutti disponibili in base alle impostazioni del widget dei commenti configurate in FastComments.
3. I loro commenti sono attribuiti al loro account Blackboard. Se lo studente modifica successivamente il proprio nome o la propria foto in Blackboard, il lancio successivo aggiorna il profilo FastComments.

Mappatura dei ruoli da Blackboard a FastComments:

- **System Administrator** e **Course Builder** corrispondono a **admin** di FastComments.
- **Instructor** e **Teaching Assistant** corrispondono a **moderator** di FastComments.
- **Student**, **Guest** e **Observer** corrispondono a **commenter** di FastComments.

I moderatori vedono i controlli di moderazione (pin, nascondi, banna, elimina) in linea su ogni commento nel thread.

#### Bloccare l'accesso pubblico (consigliato)

Per impostazione predefinita, i dati dei commenti di FastComments sono leggibili pubblicamente. Chiunque riesca a indovinare l'URL di un thread o un endpoint API può visualizzarne i commenti, anche al di fuori di Blackboard. Per le discussioni di corso, quasi certamente vorrai limitare la visualizzazione ai soli studenti iscritti.

Apri la tua <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pagina di personalizzazione del widget</a> e crea una regola con **Require SSO To View Comments** abilitato, quindi imposta il livello di sicurezza su **Secure SSO** in modo che i thread possano essere caricati solo tramite il lancio LTI firmato.

Consulta [Protezione dei thread di commenti con Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) per la guida completa, incluso come limitare la regola a un singolo dominio o pagina.

#### Ambito dei thread

FastComments definisce l'ambito di ogni thread tramite **(host Blackboard, ID corso, ID resource link)**. Due elementi FastComments nello stesso corso producono due thread. Lo stesso elemento copiato in due shell di corso (ad esempio, tramite copia del corso) produce due thread, perché Blackboard genera un nuovo resource link ID durante la copia. Per mantenere un thread condiviso tra le copie del corso, usa il Deep Linking con un URN di thread esplicito configurato in FastComments prima di eseguire la copia.

#### Problemi specifici di Blackboard

**Riquadro FastComments mancante nel menu Build Content (Original) o nel Content Market (Ultra).** L'amministratore ha approvato lo strumento ma ha lasciato una policy dell'istituzione che blocca il placement pertinente. Vai a **Administrator Panel** > **Integrations** > **LTI Tool Providers**, modifica la voce FastComments e conferma che entrambi i placement **Course Content Tool** (Original) e **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) siano abilitati. Salva e aggiorna la pagina del corso.

**Errore "Tool not configured for this context" o "Tool is not deployed" al lancio.** L'ambito di deployment registrato durante la registrazione dinamica non corrisponde al contesto dell'istituzione a cui appartiene il corso. Nella voce del tool provider di Blackboard, verifica che il **Deployment ID** corrisponda a ciò che FastComments mostra sulla sua pagina di configurazione LTI 1.3 per questo tenant. Se differiscono, elimina il placement ed esegui nuovamente la registrazione dinamica da un nuovo URL di registrazione (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ottienilo qui</a>).

**L'altezza dell'iframe sembra fissa o il contenuto viene tagliato.** Alcuni tenant Blackboard sono forniti con una Content Security Policy restrittiva che blocca il postMessage predefinito di ridimensionamento dell'iframe LTI. FastComments emette sia il messaggio in stile Canvas `lti.frameResize` sia il messaggio conforme alle specifiche IMS `org.imsglobal.lti.frameResize` per massimizzare la compatibilità, ma un override CSP a livello di tenant blocca il listener del genitore. Chiedi al tuo amministratore di confermare che `*.fastcomments.com` sia presente nella allowlist degli strumenti LTI e che nessun header CSP personalizzato stia eliminando gli eventi postMessage. Il ridimensionamento funzionerà quindi senza ulteriori configurazioni.

**La copia del corso duplica i thread.** La copia del corso di Blackboard genera nuovi resource link ID per i placement LTI, quindi i corsi copiati partono con thread vuoti. Questo è previsto. Se hai bisogno che il corso copiato erediti il thread originale, configura il Deep Linking con un URN di thread esplicito prima di copiare, oppure contatta il supporto FastComments per rimappare gli ID dei thread in blocco.

**Lo studente vede un errore generico di Blackboard al lancio.** La causa è un claim `email` mancante o non aggiornato. Conferma che la policy dell'istituzione per FastComments abbia **Role**, **Name** e **Email Address** abilitati sotto **User Fields to Send**. Salva, quindi esegui nuovamente il lancio in una nuova sessione del browser.