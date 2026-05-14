Questa pagina spiega come aggiungere FastComments a un corso Brightspace dopo che un amministratore ha registrato lo strumento e creato una deployment. Se lo strumento non è ancora registrato, vedere prima la guida alla registrazione D2L.

Brightspace fornisce due esperienze di authoring dei contenuti: **Classic Content** e la **New Content Experience** (chiamata anche **Lessons**). Entrambe espongono FastComments, ma i percorsi di menu differiscono. Ogni sezione qui sotto copre entrambe dove divergono.

#### Individuare lo strumento FastComments

Lo strumento FastComments appare in due punti all'interno dell'editor dei contenuti del corso:

1. Il selettore di attività, raggiungibile dal pulsante **Add Existing** di un modulo/unità (etichettato **Add Existing Activities** nelle versioni precedenti di Brightspace). FastComments appare direttamente nel selettore nelle build attuali di Brightspace; nelle versioni più vecchie è nidificato sotto un sottomenu **External Learning Tools**. Entrambi i percorsi aggiungono FastComments come argomento autonomo.
2. La finestra di dialogo **Insert Stuff** all'interno dell'editor HTML, sotto **LTI Advantage**. Questo incorpora FastComments inline in un argomento HTML tramite il flusso di deep linking LTI.

Se FastComments non appare in nessuno dei due selettori, la deployment non è abilitata per l'unità organizzativa che contiene il corso. Chiedere all'amministratore di Brightspace di aprire **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > strumento FastComments > **View Deployments**, aprire la deployment e aggiungere l'unità organizzativa del corso (o un'unità organizzativa superiore) sotto **Org Units**.

#### Aggiungere FastComments come argomento in un modulo

Classic Content:

1. Aprire il corso e cliccare **Content** nella navbar.
2. Selezionare il modulo che deve contenere la discussione (o crearne uno tramite **Add a module**).
3. Cliccare **Add Existing** (Brightspace più vecchio: **Add Existing Activities** > **External Learning Tools**).
4. Nel selettore, cliccare **FastComments**. Brightspace crea un argomento nel modulo e ritorna alla vista dei contenuti.
5. Cliccare il nuovo argomento. Rinominarlo con qualcosa di descrittivo come `FastComments Discussion` utilizzando l'editor in linea del titolo.

New Content Experience (Lessons):

1. Aprire il corso e cliccare **Content**.
2. Aprire l'unità e la lesson che devono contenere la discussione.
3. Cliccare **Add** > **Existing Activity** e selezionare **FastComments** (Brightspace più vecchio: nidificato sotto **External Learning Tools**).
4. L'attività viene aggiunta alla lesson.
5. Cliccare il titolo dell'attività per rinominarla.

La prima volta che un qualsiasi utente (istruttore o studente) apre l'argomento, FastComments inizializza il thread per quel resource link. Il thread è vincolato all'ID del resource link, quindi rinominare o spostare l'argomento non cambia quale thread viene caricato.

#### Incorporare FastComments inline in un argomento HTML

Usare questo flusso quando si desidera che i commenti appaiano sotto una lettura, un video o altro contenuto all'interno della stessa pagina dell'argomento invece che come argomento separato.

1. Aprire o creare un argomento HTML nel modulo/lesson.
2. Cliccare **Edit HTML** per aprire l'editor HTML di Brightspace.
3. Posizionare il cursore dove deve apparire il thread dei commenti.
4. Cliccare il pulsante **Insert Stuff** (icona a forma di pezzo di puzzle nella toolbar dell'editor).
5. Nella finestra Insert Stuff, scorrere fino a **LTI Advantage** e cliccare **FastComments**.
6. FastComments apre un selettore di deep linking. Confermare il posizionamento (le opzioni predefinite vanno bene per discussioni di contenuto); cliccare **Insert** o **Continue**.
7. Brightspace ritorna all'editor HTML con un blocco segnaposto che rappresenta il lancio LTI. Cliccare **Save and Close** sull'argomento.

Quando l'argomento viene caricato, Brightspace sostituisce il segnaposto con un iframe che avvia automaticamente FastComments tramite LTI. Gli studenti vedono il thread della discussione inline.

Un singolo argomento HTML può contenere più embed FastComments deep-linked. Ogni embed ottiene il suo thread perché ogni deep link genera un ID resource link distinto.

#### Argomento del modulo vs collegamento rapido inline

Scegliere l'approccio **argomento del modulo** quando:

- La discussione è l'attività principale per quel passaggio nel modulo.
- Si desidera che l'argomento compaia nella table of contents di Brightspace, nel tracciamento del completamento e in Class Progress.

Scegliere l'approccio **embed inline** quando:

- I commenti devono trovarsi sotto altri contenuti nella stessa pagina.
- Non si desidera un elemento separato tracciabile per il completamento nella table of contents.

#### Visibilità, Bozza e Condizioni di rilascio

Un nuovo argomento FastComments è visibile agli studenti per impostazione predefinita. Per nasconderlo mentre lo si configura:

1. Nell'editor dei contenuti, cliccare il titolo dell'argomento (Classic) o il menu a tre puntini sull'attività (New Content Experience).
2. Impostare lo stato su **Draft** (Classic) o disattivare la **Visibility** (New Content Experience).

Gli argomenti in bozza sono invisibili agli studenti. I docenti e i TA li vedono comunque con un badge "Draft".

Per limitare l'argomento a un gruppo o sezione specifica:

1. Aprire l'argomento.
2. Cliccare il menu del titolo dell'argomento > **Edit Properties In-place** (Classic) o **Edit** > **Restrictions** (New Content Experience).
3. Sotto **Release Conditions**, cliccare **Create**.
4. Scegliere **Group enrollment** o **Section enrollment**, selezionare il gruppo/sezione e salvare.

Le condizioni di rilascio si sommano al mapping dei ruoli di FastComments. Gli studenti che non possono vedere l'argomento non ricevono un lancio LTI.

#### Cosa vedono gli studenti al primo lancio

Quando uno studente clicca l'argomento (o carica un argomento HTML con un embed):

1. Brightspace esegue il lancio LTI 1.3 in background.
2. FastComments riceve il nome dello studente, l'email, l'URL dell'avatar e il ruolo LMS, e li autentica automaticamente. Non viene richiesto alcun login a FastComments.
3. Il thread dei commenti per quel resource link viene visualizzato all'interno dell'iframe di Brightspace.

Mapping dei ruoli al lancio:

- Brightspace `Administrator` diventa un FastComments **amministratore** per il thread (piena moderazione, eliminazione, ban e accesso alla configurazione).
- Brightspace `Instructor` diventa un FastComments **moderatore** (pin, nascondi, elimina, ban).
- Tutti gli altri ruoli (`Learner`, `TeachingAssistant`, ecc.) diventano commentatori standard.

I commenti sono attribuiti all'account Brightspace dello studente. Se lo studente modifica il proprio nome o avatar in Brightspace, il successivo lancio LTI sincronizza la modifica.

#### Altezza iframe e ridimensionamento

FastComments emette il postMessage `org.imsglobal.lti.frameResize` ad ogni render del thread e al cambiamento del contenuto (nuovo commento, espansione risposte). Brightspace ascolta questo messaggio e regola l'altezza dell'iframe in modo che il thread non venga ritagliato e non venga mostrata una scrollbar interna.

Se l'iframe rimane a un'altezza fissa e ridotta:

- Confermare che il corso sia caricato via HTTPS. Il listener postMessage di Brightspace rifiuta frame con contenuto misto.
- Confermare che nessuna estensione del browser stia bloccando il canale postMessage.
- Per gli embed inline in un argomento HTML, l'HTML circostante non deve avvolgere l'iframe in un contenitore ad altezza fissa. Rimuovere qualsiasi `style="height: ..."` inline dall'elemento genitore.

#### Problemi specifici di Brightspace

**Strumento non visibile nel selettore Add Existing.** La deployment non è abilitata per l'unità organizzativa di questo corso. Un amministratore deve aggiungere l'unità organizzativa (o un genitore) alla lista **Org Units** della deployment. La sola registrazione dello strumento non è sufficiente; la deployment determina quali corsi vedono lo strumento.

**Mismatch di `deployment_id` al lancio.** FastComments TOFU-pins il primo `deployment_id` che vede per una registrazione. Se un amministratore elimina la deployment originale e ne crea una nuova, i lanci dalla nuova deployment vengono rifiutati con un errore di mismatch di deployment. La soluzione è re-registrare FastComments (generare una nuova registration URL ed eseguire nuovamente la Dynamic Registration); il vecchio record di configurazione viene sostituito.

**Lo strumento si avvia ma mostra "Invalid LTI launch".** Il corso si trova in una struttura tenant/org diversa da quella coperta dalla deployment, oppure la deployment è stata disabilitata dopo la registrazione. Ricontrollare **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > toggle **Enabled** e la lista delle org unit della deployment.

**Nomi e ruoli mancanti dentro FastComments.** Brightspace fornisce i lanci LTI con le claim Names and Role Provisioning Services (NRPS). Se un corso è stato aggiornato da un vecchio link LTI 1.1, il lancio può mancare delle claim `name` e `email`. Riaggiungere l'argomento FastComments tramite **Add Existing** (non migrare il vecchio link) in modo che il lancio usi LTI 1.3.

**L'embed mostra una schermata di login invece della SSO automatica.** L'argomento HTML è stato inserito come un normale `<iframe>` che punta a FastComments invece che tramite **Insert Stuff** > **LTI Advantage**. I normali iframe saltano il lancio LTI e portano gli utenti sulla pagina pubblica di FastComments. Eliminare l'iframe e reinserirlo tramite il flusso Insert Stuff.