Una volta che un amministratore ha registrato FastComments come strumento LTI 1.3 Advantage e ha approvato le policy dell'istituzione, gli insegnanti lo aggiungono ai corsi tramite i punti di posizionamento standard di Blackboard. I passaggi esatti differiscono tra Ultra Course View e Original Course View, quindi entrambi sono descritti di seguito.

#### Ultra Course View

Ultra Course View è il valore predefinito in Blackboard Learn SaaS a partire dal 2026.

1. Apri il corso e vai alla pagina **Course Content**.
2. Posiziona il cursore o tocca dove vuoi che il thread dei commenti appaia nell'outline e fai clic sul pulsante viola **+** (Aggiungi contenuto).
3. Scegli **Content Market**. Il pannello Content Market elenca tutti gli strumenti LTI approvati e i posizionamenti Building Block per la tua istituzione.
4. Trova la tessera **FastComments** e cliccala. Blackboard crea un elemento di contenuto nella posizione in cui hai aperto il menu **+**.
5. L'elemento appare nell'outline come voce "Visible to students" per impostazione predefinita per gli insegnanti che hanno **Hide from students** disattivato come impostazione personale. Se il tuo valore predefinito è **Hidden**, l'elemento viene creato nascosto e attivi il selettore di visibilità sulla riga dell'elemento quando sei pronto.
6. Per rinominare l'elemento, clicca il titolo nell'outline e digita un'etichetta nuova. Il titolo che gli studenti vedono nell'outline è indipendente dall'identificatore del thread FastComments, quindi rinominare è sicuro in qualsiasi momento.

Se non vedi **Content Market** come opzione, la tua istituzione ha nascosto il posizionamento. Puoi anche raggiungere lo stesso selettore tramite **More tools** nello stesso menu **+** sotto il gruppo **LTI Tools**.

#### Original Course View

Original Course View è ancora supportato in Learn SaaS e rimane l'esperienza primaria per i siti Learn 9.1 self-hosted sulla linea di rilascio Q4 2024 CU.

1. Apri il corso ed entra in una **Content Area** (ad esempio, l'area predefinita **Information** o **Content** nel menu del corso).
2. Attiva **Edit Mode** con l'interruttore nell'angolo in alto a destra della pagina.
3. Clicca **Build Content** nella barra delle azioni.
4. Nel sottomenu **Learning Tools**, clicca **FastComments**. Il sottomenu Learning Tools viene popolato dai posizionamenti degli strumenti LTI 1.3 dopo che un amministratore ha registrato lo strumento. Se non lo vedi, vedi la sezione dei problemi comuni qui sotto.
5. Nel modulo **Create FastComments**, imposta:
   - **Name**: l'etichetta che gli studenti vedono nell'area di contenuto.
   - **Description**: testo opzionale mostrato sopra il thread incorporato.
   - **Permit Users to View this Content**: interruttore di disponibilità Sì/No.
   - **Track Number of Views**: abilita se desideri le statistiche di visualizzazione per elemento di Blackboard. FastComments gestisce le proprie analitiche in modo indipendente.
   - **Date and Time Restrictions**: finestre opzionali **Display After** / **Display Until**.
6. Invia. Lo strumento appare come elemento cliccabile nell'area di contenuto.

#### Incorporamento all'interno di un elemento o documento

In entrambe le viste del corso, gli insegnanti incorporano FastComments inline all'interno del corpo di un Item, Documento o di qualsiasi campo rich-text tramite il pulsante LTI Advantage dell'Editor di contenuti.

Ultra Course View:

1. Crea o modifica un **Document**.
2. Clicca **Add content** all'interno del corpo del documento dove vuoi che il thread appaia.
3. Nella barra degli strumenti dell'editor, apri il menu **Insert content** e clicca **Content Market** (il punto di ingresso LTI Advantage / Deep Linking).
4. Scegli **FastComments**. FastComments restituisce un payload di deep-link e Blackboard inserisce un blocco incorporato nel corpo del documento nella posizione del cursore.
5. Salva il documento. Gli studenti vedono il thread renderizzato inline mentre scorrono oltre.

Original Course View:

1. Modifica qualsiasi elemento con un corpo in rich-text.
2. Nella barra degli strumenti del Content Editor, clicca sull'icona plus **Add Content** e scegli **Content Market** (etichettato **Add Content from External Tool** nelle CU Q4 2024 più vecchie).
3. Scegli **FastComments**. L'editor inserisce un blocco segnaposto che fa riferimento alla risorsa deep-linked.
4. Invia l'elemento.

Ogni embed deep-link produce il proprio thread FastComments, quindi un Item con due blocchi FastComments incorporati ha due flussi di commenti indipendenti.

#### Visibilità, condizioni di rilascio e restrizioni di gruppo

Gli elementi di contenuto FastComments si comportano come qualsiasi altro elemento di contenuto di Blackboard per le regole di controllo degli accessi applicate su di essi.

- Ultra: clicca il selettore di visibilità sulla riga (**Visible to students**, **Hidden from students**, **Conditional availability**). La disponibilità condizionale supporta finestre di data/ora, regole di performance rispetto agli elementi del registro voti e regole sui membri rispetto ai gruppi del corso.
- Original: apri il menu contestuale dell'elemento e scegli **Adaptive Release** o **Adaptive Release: Advanced** per limitare lo strumento per data, appartenenza, voto o stato di revisione. Usa **Set Group Availability** sull'elemento per limitarne l'accesso a gruppi specifici del corso.

FastComments rispetta qualunque limite imponga Blackboard. Se Blackboard nasconde l'elemento a uno studente, il lancio LTI non avviene mai per quello studente e lui/lei non compare nella vista dei moderatori.

#### Comportamento nel registro voti

FastComments non riporta voti tramite LTI Advantage Assignment and Grade Services. Nessuna colonna dei voti viene creata automaticamente per gli elementi di contenuto FastComments.

Se il tuo tenant Blackboard è configurato per creare automaticamente una colonna del registro voti per ogni nuovo elemento di contenuto indipendentemente dai metadati di valutazione, apparirà comunque una colonna vuota. Per nasconderla:

- Ultra: apri il **Gradebook**, clicca l'intestazione della colonna, scegli **Edit**, e disattiva **Show to students** oltre a **Include in calculations**. Oppure usa **Delete** se la tua istituzione consente la cancellazione delle colonne per elementi non valutati.
- Original: apri il **Grade Center**, clicca il chevron della colonna, scegli **Hide from Users (on/off)** e opzionalmente **Hide from Instructor View** sotto **Column Organization**.

#### Cosa vedono gli studenti

Quando uno studente apre l'elemento FastComments o scorre fino a un blocco incorporato:

1. Blackboard avvia il messaggio LTI 1.3 verso FastComments. Lo studente viene autenticato tramite SSO usando la sua identità Blackboard (nome, email, avatar, ruolo) senza vedere un modulo di login.
2. Il thread dei commenti viene renderizzato nell'iframe. Threading, risposte, menzioni e reazioni sono tutte disponibili in base alle impostazioni del widget dei commenti configurate in FastComments.
3. I loro commenti vengono attribuiti al loro account Blackboard. Se lo studente modifica il proprio nome o la foto in Blackboard in seguito, il prossimo lancio aggiorna il profilo FastComments.

Mapping dei ruoli da Blackboard a FastComments:

- **System Administrator** e **Course Builder** mappano a FastComments **admin**.
- **Instructor** e **Teaching Assistant** mappano a FastComments **moderator**.
- **Student**, **Guest**, e **Observer** mappano a FastComments **commenter**.

I moderatori vedono i controlli di moderazione (pin, hide, ban, delete) inline su ogni commento del thread.

#### Scoping dei thread

FastComments delimita ogni thread tramite **(Blackboard host, course ID, resource link ID)**. Due elementi FastComments nello stesso corso producono due thread. Lo stesso elemento copiato in due shell di corso diverse (per esempio, tramite copia del corso) produce due thread, perché Blackboard emette un nuovo resource link ID durante la copia. Per mantenere un thread condiviso tra copie del corso, usa Deep Linking con un URN di thread esplicito configurato in FastComments prima di avviare la copia.

#### Problemi specifici di Blackboard

**La tessera FastComments manca dal menu Build Content (Original) o da Content Market (Ultra).** L'amministratore ha approvato lo strumento ma ha lasciato una policy dell'istituzione che blocca il posizionamento rilevante. Vai su **Administrator Panel** > **Integrations** > **LTI Tool Providers**, modifica la voce FastComments e conferma che i posizionamenti **Course Content Tool** (Original) e **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) sono abilitati. Salva e aggiorna la pagina del corso.

**Errore "Tool not configured for this context" o "Tool is not deployed" al lancio.** L'ambito di deployment registrato durante la registrazione dinamica non corrisponde al contesto istituzionale a cui appartiene il corso. Nella voce provider dello strumento di Blackboard, verifica che il **Deployment ID** corrisponda a quello che FastComments mostra nella sua pagina di Configurazione LTI 1.3 per questo tenant. Se differiscono, elimina il posizionamento e riesegui la registrazione dinamica da un URL di registrazione nuovo ( <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ottienilo qui</a> ).

**L'altezza dell'iframe sembra fissa o il contenuto viene tagliato.** Alcuni tenant Blackboard vengono forniti con una Content Security Policy rigorosa che blocca il postMessage di ridimensionamento iframe LTI predefinito. FastComments invia sia il messaggio in stile Canvas `lti.frameResize` sia il messaggio secondo la specifica IMS `org.imsglobal.lti.frameResize` per massimizzare la compatibilità, ma un override CSP a livello tenant può bloccare il listener del parent. Chiedi al tuo amministratore di confermare che `*.fastcomments.com` sia nella allowlist degli strumenti LTI e che nessun header CSP personalizzato stia rimuovendo gli eventi postMessage. Il ridimensionamento funzionerà allora senza ulteriore configurazione.

**La copia del corso duplica i thread.** La copia del corso di Blackboard genera nuovi resource link ID per i posizionamenti LTI, quindi i corsi copiati iniziano con thread vuoti. Questo è previsto. Se hai bisogno che il corso copiato erediti il thread originale, configura Deep Linking con un URN di thread esplicito prima della copia, oppure contatta il supporto FastComments per rimappare gli ID dei thread in blocco.

**Lo studente vede un errore generico di Blackboard al lancio.** La causa è una claim `email` mancante o obsoleta. Conferma che la policy dell'istituzione per FastComments ha abilitato **Role**, **Name**, e **Email Address** sotto **User Fields to Send**. Salva, quindi rilancia in una nuova sessione del browser.