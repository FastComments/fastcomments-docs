Questa pagina tratta l'aggiunta di FastComments a un corso Brightspace dopo che un amministratore ha registrato lo strumento e creato una deployment. Se lo strumento non è ancora registrato, consultare prima la guida di registrazione D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments incorporato come argomento di unità in Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments in esecuzione all'interno di un'unità Brightspace, mostrando commenti a thread e un selettore di menzioni @-mention" />
</div>

Brightspace fornisce due esperienze di creazione dei contenuti: **Classic Content** e la **New Content Experience** (nota anche come **Lessons**). Entrambe espongono FastComments, ma i percorsi dei menu differiscono. Ogni sezione sottostante copre entrambe dove si discostano.

#### Individuare lo strumento FastComments

Lo strumento FastComments appare in due punti all'interno dell'editor dei contenuti di un corso:

1. Il selettore di attività, raggiungibile dal pulsante **Add Existing** di un modulo/unità (nelle versioni più vecchie di Brightspace etichettato **Add Existing Activities**). FastComments appare direttamente nel selettore nelle build correnti di Brightspace; nelle versioni più vecchie è annidato sotto un sottomenu **External Learning Tools**. Qualsiasi percorso aggiunge FastComments come argomento indipendente.
2. La finestra di dialogo **Insert Stuff** all'interno dell'editor HTML, sotto **LTI Advantage**. Questo incorpora FastComments inline in un argomento HTML tramite il flusso di deep linking LTI.

Se FastComments non appare in nessuno dei due selettori, la deployment non è abilitata per l'org unit che contiene il corso. Chiedere al proprio amministratore Brightspace di aprire **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, aprire la deployment e aggiungere l'org unit del corso (o un'org unit padre) sotto **Org Units**.

#### Aggiungere FastComments come argomento in un modulo

Classic Content:

1. Aprire il corso e fare clic su **Content** nella barra di navigazione.
2. Selezionare il modulo che deve contenere la discussione (o crearne uno tramite **Add a module**).
3. Fare clic su **Add Existing** (Brightspace più vecchio: **Add Existing Activities** > **External Learning Tools**).
4. Nel selettore, fare clic su **FastComments**. Brightspace crea un argomento nel modulo e ritorna alla vista contenuti.
5. Fare clic sul nuovo argomento. Rinominarlo con un titolo descrittivo come `FastComments Discussion` usando l'editor del titolo inline.

New Content Experience (Lessons):

1. Aprire il corso e fare clic su **Content**.
2. Aprire l'unità e la lesson che devono contenere la discussione.
3. Fare clic su **Add** > **Existing Activity** e selezionare **FastComments** (Brightspace più vecchio: annidato sotto **External Learning Tools**).
4. L'attività viene aggiunta alla lesson.
5. Fare clic sul titolo dell'attività per rinominarlo.

La prima volta che qualsiasi utente (istruttore o studente) apre l'argomento, FastComments inizializza il thread per quel resource link. Il thread è vincolato all'ID del resource link, quindi rinominare o spostare l'argomento non modifica quale thread viene caricato.

#### Incorporare FastComments inline in un argomento HTML

Usare questo flusso quando si desidera che i commenti appaiano sotto una lettura, un video o altro contenuto all'interno della stessa pagina dell'argomento anziché come argomento separato.

1. Aprire o creare un argomento HTML nel modulo/lesson.
2. Fare clic su **Edit HTML** per aprire l'editor HTML di Brightspace.
3. Posizionare il cursore dove deve comparire il thread di commenti.
4. Fare clic sul pulsante **Insert Stuff** (icona a forma di pezzo di puzzle nella barra degli strumenti dell'editor).
5. Nella finestra Insert Stuff, scorrere fino a **LTI Advantage** e fare clic su **FastComments**.
6. FastComments apre un selettore di deep linking. Confermare il posizionamento (le opzioni predefinite funzionano per discussioni sui contenuti); fare clic su **Insert** o **Continue**.
7. Brightspace ritorna all'editor HTML con un blocco segnaposto che rappresenta il lancio LTI. Fare clic su **Save and Close** sull'argomento.

Quando l'argomento viene caricato, Brightspace sostituisce il segnaposto con un iframe che avvia automaticamente FastComments tramite LTI. Gli studenti vedono il thread di discussione inline.

Un singolo argomento HTML può contenere più embed FastComments deep-linked. Ogni embed ottiene il proprio thread perché ogni deep link produce un resource link ID distinto.

#### Argomento del modulo vs Collegamento inline

Scegliere l'approccio **argomento del modulo** quando:

- La discussione è l'attività principale per quel passaggio del modulo.
- Si desidera che l'argomento appaia nella tabella dei contenuti di Brightspace, nel tracciamento del completamento e in Class Progress.

Scegliere l'approccio **embed inline** quando:

- I commenti devono comparire sotto altri contenuti nella stessa pagina.
- Non si desidera un elemento separato tracciabile per il completamento nella tabella dei contenuti.

#### Visibilità, Bozza e Condizioni di rilascio

Un nuovo argomento FastComments è visibile agli studenti per impostazione predefinita. Per nasconderlo mentre lo si configura:

1. Nell'editor dei contenuti, fare clic sul titolo dell'argomento (Classic) o sul menu a tre puntini sull'attività (New Content Experience).
2. Impostare lo stato su **Draft** (Classic) o disattivare la **Visibility** (New Content Experience).

Gli argomenti in bozza sono invisibili agli studenti. I docenti e i TA li vedono comunque con un badge "Draft".

Per limitare l'argomento a un gruppo o a una sezione specifica:

1. Aprire l'argomento.
2. Fare clic sul menu del titolo dell'argomento > **Edit Properties In-place** (Classic) o **Edit** > **Restrictions** (New Content Experience).
3. Sotto **Release Conditions**, fare clic su **Create**.
4. Scegliere **Group enrollment** o **Section enrollment**, selezionare il gruppo/la sezione e salvare.

Le condizioni di rilascio si sommano al mapping dei ruoli di FastComments. Gli studenti che non possono vedere l'argomento non ricevono un lancio LTI.

#### Cosa vedono gli studenti al primo avvio

Quando uno studente clicca l'argomento (o carica un argomento HTML con un embed):

1. Brightspace esegue il lancio LTI 1.3 in background.
2. FastComments riceve il nome dello studente, l'email, l'URL dell'avatar e il ruolo LMS, e li autentica automaticamente. Non viene mostrata alcuna richiesta di login FastComments.
3. Il thread di commenti per quel resource link viene renderizzato all'interno dell'iframe di Brightspace.

Mapping dei ruoli al lancio:

- Brightspace `Administrator` diventa un FastComments **admin** per il thread (piena moderazione, eliminazione, ban e accesso alla configurazione).
- Brightspace `Instructor` diventa un FastComments **moderator** (pin, nascondi, elimina, ban).
- Tutti gli altri ruoli (`Learner`, `TeachingAssistant`, ecc.) diventano commentatori standard.

I commenti sono attribuiti all'account Brightspace dello studente. Se lo studente modifica il proprio nome o avatar in Brightspace, il successivo lancio LTI sincronizza la modifica.

#### Altezza iframe e ridimensionamento

FastComments emette il postMessage `org.imsglobal.lti.frameResize` ad ogni rendering del thread e al variare del contenuto (nuovo commento, espansione delle risposte). Brightspace ascolta questo messaggio e regola l'altezza dell'iframe in modo che il thread non venga tagliato e non mostri una scrollbar interna.

Se l'iframe rimane su un'altezza fissa e breve:

- Confermare che il corso è caricato tramite HTTPS. Il listener postMessage di Brightspace rifiuta frame con contenuto misto.
- Confermare che nessuna estensione del browser stia bloccando il canale postMessage.
- Per embed inline in un argomento HTML, l'HTML circostante non deve avvolgere l'iframe in un contenitore ad altezza fissa. Rimuovere qualsiasi inline `style="height: ..."` dall'elemento parent.

#### Particolarità specifiche di Brightspace

**Tool not showing in the Add Existing picker.** La deployment non è abilitata per l'org unit di questo corso. Un amministratore deve aggiungere l'org unit (o un padre) alla lista Org Units della deployment. La sola registrazione dello strumento non è sufficiente; la deployment definisce quali corsi vedono lo strumento.

**`deployment_id` mismatch on launch.** FastComments TOFU-pins il primo `deployment_id` che rileva per una registrazione. Se un amministratore elimina la deployment originale e ne crea una nuova, i lanci dalla nuova deployment vengono rifiutati con un errore di mismatch della deployment. La soluzione è rieseguire la registrazione di FastComments (generare una nuova registration URL ed eseguire nuovamente la Dynamic Registration); il vecchio record di configurazione viene sostituito.

**Tool launches but shows "Invalid LTI launch".** Il corso si trova in una struttura tenant/org diversa da quella coperta dalla deployment, oppure la deployment è stata disabilitata dopo la registrazione. Ricontrollare **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > toggle **Enabled** e la lista delle org unit della deployment.

**Names and roles missing inside FastComments.** Brightspace invia i lanci LTI con i claims Names and Role Provisioning Services (NRPS). Se un corso è stato aggiornato da un vecchio link LTI 1.1, il lancio può mancare dei claim `name` e `email`. Riaggiungere l'argomento FastComments tramite **Add Existing** (non migrare il vecchio link) in modo che il lancio utilizzi LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** L'argomento HTML è stato inserito come un semplice `<iframe>` puntato verso FastComments invece che tramite **Insert Stuff** > **LTI Advantage**. I semplici iframe saltano il lancio LTI e portano gli utenti alla pagina pubblica di FastComments. Eliminare l'iframe e reinserirlo tramite il flusso Insert Stuff.