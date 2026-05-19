Una volta che un amministratore ha registrato FastComments come strumento LTI 1.3 Advantage e approvato le policy dell'istituzione, gli istruttori lo aggiungono ai corsi attraverso i punti di posizionamento standard di Blackboard. I passaggi esatti differiscono tra Ultra Course View e Original Course View, quindi entrambi sono coperti di seguito.

#### Ultra Course View

Ultra Course View è la visualizzazione predefinita in Blackboard Learn SaaS a partire dal 2026.

1. Apri il corso e vai alla pagina **Course Content**.
2. Posiziona il cursore o tocca dove vuoi che il thread dei commenti venga inserito nell'indice e clicca il pulsante viola **+** (Add content).
3. Scegli **Content Market**. Il pannello Content Market elenca ogni strumento LTI approvato e i posizionamenti Building Block per la tua istituzione.
4. Trova la tessera **FastComments** e cliccala. Blackboard crea un elemento di contenuto nella posizione in cui hai aperto il menu **+**.
5. Per impostazione predefinita, l'elemento appare nell'indice come voce "Visible to students" per gli istruttori che hanno **Hide from students** disattivato come impostazione personale predefinita. Se il tuo default è **Hidden**, l'elemento viene creato nascosto e puoi attivare il selettore di visibilità sulla riga dell'elemento quando sei pronto.
6. Per rinominare l'elemento, clicca il titolo nell'indice e digita una nuova etichetta. Il titolo che gli studenti vedono nell'indice è indipendente dall'identificatore del thread FastComments, quindi rinominare è sicuro in qualsiasi momento.

Se non vedi **Content Market** come opzione, la tua istituzione ha nascosto quel posizionamento. Puoi anche raggiungere lo stesso selettore tramite **More tools** nello stesso menu **+** sotto il gruppo **LTI Tools**.

#### Original Course View

Original Course View è ancora supportato in Learn SaaS e rimane l'esperienza principale per i siti Learn 9.1 self-hosted sulla linea di release CU Q4 2024.

1. Apri il corso ed entra in un'area di contenuto (**Content Area**) (per esempio, l'area predefinita **Information** o **Content** nel menu del corso).
2. Attiva **Edit Mode** con l'interruttore in alto a destra della pagina.
3. Clicca **Build Content** nella barra delle azioni.
4. Nel sottomenu **Learning Tools**, clicca **FastComments**. Il sottomenu Learning Tools viene popolato dai posizionamenti degli strumenti LTI 1.3 dopo che un amministratore ha registrato lo strumento. Se non lo vedi, consulta la sezione dei problemi noti qui sotto.
5. Nel modulo **Create FastComments**, imposta:
   - **Name**: l'etichetta che gli studenti vedono nell'area di contenuto.
   - **Description**: testo opzionale mostrato sopra il thread incorporato.
   - **Permit Users to View this Content**: interruttore di disponibilità Sì/No.
   - **Track Number of Views**: abilita se desideri le statistiche di visualizzazione per elemento di Blackboard. FastComments gestisce le proprie analitiche in modo indipendente.
   - **Date and Time Restrictions**: finestre opzionali **Display After** / **Display Until**.
6. Invia. Lo strumento appare come elemento cliccabile nell'area di contenuto.

#### Embedding Inside an Item or Document

In entrambe le visualizzazioni del corso, gli istruttori incorporano FastComments inline all'interno del corpo di un Item, Document o di qualsiasi campo rich-text tramite il pulsante LTI Advantage dell'Editor di Contenuti.

Ultra Course View:

1. Crea o modifica un **Document**.
2. Clicca **Add content** all'interno del corpo del documento dove vuoi che appaia il thread.
3. Nella barra degli strumenti dell'editor, apri il menu **Insert content** e clicca **Content Market** (il punto di ingresso LTI Advantage / Deep Linking).
4. Seleziona **FastComments**. FastComments restituisce un payload di deep-link e Blackboard inserisce un blocco incorporato nel corpo del documento nella posizione del cursore.
5. Salva il documento. Gli studenti vedono il thread renderizzato inline mentre scorrono la pagina.

Original Course View:

1. Modifica qualsiasi elemento con un corpo rich-text.
2. Nella barra degli strumenti dell'Editor di Contenuti, clicca l'icona più **Add Content** e scegli **Content Market** (etichettato **Add Content from External Tool** nelle CU Q4 2024 più datate).
3. Seleziona **FastComments**. L'editor inserisce un blocco segnaposto che fa riferimento alla risorsa deep-linked.
4. Invia l'elemento.

Ogni embed deep-link genera il proprio thread FastComments, quindi un Item con due blocchi FastComments incorporati avrà due flussi di commenti indipendenti.

#### Visibility, Release Conditions, and Group Restrictions

Gli elementi di contenuto FastComments si comportano come qualsiasi altro elemento di contenuto Blackboard per le regole di controllo accessi applicate su di essi.

- Ultra: clicca il selettore di visibilità sulla riga (**Visible to students**, **Hidden from students**, **Conditional availability**). La disponibilità condizionale supporta finestre di data/ora, regole basate sulle prestazioni rispetto ai criteri del gradebook e regole di appartenenza rispetto ai gruppi del corso.
- Original: apri il menu contestuale dell'elemento e scegli **Adaptive Release** o **Adaptive Release: Advanced** per limitare lo strumento per data, appartenenza, voto o stato di revisione. Usa **Set Group Availability** sull'elemento per limitarne l'accesso a specifici gruppi del corso.

FastComments rispetta qualunque limitazione decida Blackboard. Se Blackboard nasconde l'elemento a uno studente, il lancio LTI non avviene per quello studente e lui/lei non appare nella vista moderatore.

#### Gradebook Behavior

FastComments non invia voti tramite LTI Advantage Assignment and Grade Services. Nessuna colonna di voto viene creata automaticamente per gli elementi di contenuto FastComments.

Se il tuo tenant Blackboard è configurato per creare automaticamente una colonna del gradebook per ogni nuovo elemento di contenuto indipendentemente dai metadati di valutazione, appare comunque una colonna vuota. Per nasconderla:

- Ultra: apri il **Gradebook**, clicca l'intestazione della colonna, scegli **Edit**, e disattiva **Show to students** oltre a **Include in calculations**. Oppure usa **Delete** se la tua istituzione consente la cancellazione delle colonne per elementi non valutati.
- Original: apri il **Grade Center**, clicca sul chevron della colonna, scegli **Hide from Users (on/off)**, e opzionalmente **Hide from Instructor View** sotto **Column Organization**.

#### What Students See

Quando uno studente apre l'elemento FastComments o scorri fino a un blocco incorporato:

1. Blackboard avvia il messaggio LTI 1.3 verso FastComments. Lo studente viene autenticato tramite SSO usando la propria identità Blackboard (nome, email, avatar, ruolo) senza visualizzare un modulo di login.
2. Il thread dei commenti viene renderizzato nell'iframe. Threading, risposte, menzioni e reazioni sono tutte disponibili in base alle impostazioni del widget dei commenti configurate in FastComments.
3. I loro commenti sono attribuiti al loro account Blackboard. Se lo studente modifica il proprio nome o la foto in Blackboard successivamente, il lancio successivo aggiorna il profilo FastComments.

Mappatura dei ruoli da Blackboard a FastComments:

- **System Administrator** e **Course Builder** si mappano su FastComments **admin**.
- **Instructor** e **Teaching Assistant** si mappano su FastComments **moderator**.
- **Student**, **Guest**, e **Observer** si mappano su FastComments **commenter**.

I moderatori vedono i controlli di moderazione (pin, hide, ban, delete) inline su ogni commento nel thread.

#### Lock Down Public Access (Recommended)

Per impostazione predefinita, i dati dei commenti FastComments sono leggibili pubblicamente. Chiunque possa indovinare l'URL di un thread o un endpoint API può visualizzare i suoi commenti, anche al di fuori di Blackboard. Per le discussioni di corso quasi certamente vorrai limitare la visualizzazione solo agli studenti iscritti.

Apri la tua <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pagina di personalizzazione del widget</a> e crea una regola con **Require SSO To View Comments** abilitato, quindi imposta il livello di sicurezza su **Secure SSO** in modo che i thread possano essere caricati solo tramite il lancio LTI firmato.

Vedi [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) per la procedura completa, incluso come limitare la regola a un singolo dominio o pagina.

#### Thread Scoping

FastComments scopa ogni thread per **(Blackboard host, course ID, resource link ID)**. Due elementi FastComments nello stesso corso producono due thread. Lo stesso elemento copiato in due istanze del corso (per esempio, tramite course copy) produce due thread, perché Blackboard emette un nuovo resource link ID durante la copia. Per mantenere un thread condiviso attraverso le copie del corso, usa Deep Linking con un URN di thread esplicito configurato in FastComments prima di eseguire la copia.

#### Blackboard-Specific Gotchas

**FastComments tile missing from the Build Content menu (Original) or Content Market (Ultra).** L'amministratore ha approvato lo strumento ma ha lasciato una policy dell'istituzione che blocca il posizionamento pertinente. Vai su **Administrator Panel** > **Integrations** > **LTI Tool Providers**, modifica la voce FastComments e conferma che siano abilitati i posizionamenti **Course Content Tool** (Original) e **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra). Salva e aggiorna la pagina del corso.

**"Tool not configured for this context" or "Tool is not deployed" error on launch.** L'ambito di deployment registrato durante la registrazione dinamica non corrisponde al contesto dell'istituzione a cui appartiene il corso. Nella voce del tool provider di Blackboard, verifica che il **Deployment ID** corrisponda a quello mostrato da FastComments nella sua pagina di configurazione LTI 1.3 per questo tenant. Se differiscono, elimina il posizionamento e riesegui la registrazione dinamica da un URL di registrazione nuovo (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ottienilo qui</a>).

**Iframe height looks fixed or content gets cut off.** Alcuni tenant Blackboard distribuiscono una Content Security Policy restrittiva che blocca il postMessage di ridimensionamento iframe LTI predefinito. FastComments emette sia il messaggio in stile Canvas `lti.frameResize` sia il messaggio conforme alla specifica IMS `org.imsglobal.lti.frameResize` per massimizzare la compatibilità, ma un override CSP a livello di tenant blocca il listener lato padre. Chiedi al tuo amministratore di confermare che `*.fastcomments.com` sia nella allowlist degli strumenti LTI e che nessun header CSP personalizzato stia rimuovendo gli eventi postMessage. Il ridimensionamento funzionerà poi senza ulteriore configurazione.

**Course copy duplicates threads.** La copia del corso di Blackboard emette nuovi resource link ID per i posizionamenti LTI, quindi i corsi copiati iniziano con thread vuoti. Questo è previsto. Se hai bisogno che il corso copiato erediti il thread originale, configura il Deep Linking con un URN di thread esplicito prima della copia, oppure contatta il supporto FastComments per rimappare gli ID dei thread in blocco.

**Student sees a generic Blackboard error on launch.** La causa è una claim `email` mancante o obsoleta. Conferma che la policy dell'istituzione per FastComments abbia abilitati **Role**, **Name**, e **Email Address** sotto **User Fields to Send**. Salva, quindi rilancia in una nuova sessione del browser.