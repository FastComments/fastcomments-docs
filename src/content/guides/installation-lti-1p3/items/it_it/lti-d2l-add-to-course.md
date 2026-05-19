Questa pagina tratta l'aggiunta di FastComments a un corso Brightspace dopo che un amministratore ha registrato lo strumento e creato una deployment. Se lo strumento non è ancora registrato, consulta prima la guida di registrazione D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments embedded as a unit topic in Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments incorporato all'interno di un'unità Brightspace, mostra commenti threadati e un selettore di menzioni @-mention" />
</div>

Brightspace offre due esperienze di creazione dei contenuti: **Classic Content** e la **New Content Experience** (chiamata anche **Lessons**). Entrambe espongono FastComments, ma i percorsi nei menu differiscono. Ogni sezione qui sotto copre entrambe le esperienze quando divergono.

#### Locate the FastComments Tool

Lo strumento FastComments appare in due punti all'interno dell'editor dei contenuti di un corso:

1. Il selettore di attività, raggiungibile dal pulsante **Add Existing** del modulo/unità (etichettato **Add Existing Activities** nelle versioni precedenti di Brightspace). FastComments appare direttamente nel selettore nelle build correnti di Brightspace; nelle versioni più vecchie è annidato sotto un sottomenu **External Learning Tools**. Entrambe le vie aggiungono FastComments come topic autonomo.
2. La finestra di dialogo **Insert Stuff** all'interno dell'editor HTML, sotto **LTI Advantage**. Questo incorpora FastComments inline in un topic HTML tramite il flusso di deep linking LTI.

Se FastComments non appare in nessuno dei due selettori, la deployment non è abilitata per l'org unit che contiene il corso. Chiedi al tuo amministratore Brightspace di aprire **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, aprire la deployment e aggiungere l'org unit del corso (o un org unit padre) sotto **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Apri il corso e clicca **Content** nella barra di navigazione.
2. Seleziona il modulo che deve contenere la discussione (o creane uno tramite **Add a module**).
3. Clicca **Add Existing** (Brightspace più vecchio: **Add Existing Activities** > **External Learning Tools**).
4. Nel selettore, clicca **FastComments**. Brightspace crea un topic nel modulo e ti riporta alla vista dei contenuti.
5. Clicca il nuovo topic. Rinominalo con qualcosa di descrittivo come `FastComments Discussion` usando l'editor del titolo inline.

New Content Experience (Lessons):

1. Apri il corso e clicca **Content**.
2. Apri l'unità e la lesson che dovrebbero contenere la discussione.
3. Clicca **Add** > **Existing Activity** e seleziona **FastComments** (Brightspace più vecchio: annidato sotto **External Learning Tools**).
4. L'attività viene aggiunta alla lesson.
5. Clicca il titolo dell'attività per rinominarla.

La prima volta che un utente (istruttore o studente) apre il topic, FastComments inizializza il thread per quel resource link. Il thread è legato all'ID del resource link, quindi rinominare o spostare il topic non modifica quale thread viene caricato.

#### Embed FastComments Inline in an HTML Topic

Usa questo flusso quando vuoi che i commenti appaiano sotto una lettura, un video o altro contenuto all'interno della stessa pagina del topic anziché come topic separato.

1. Apri o crea un topic HTML nel modulo/lesson.
2. Clicca **Edit HTML** per aprire l'editor HTML di Brightspace.
3. Posiziona il cursore dove deve apparire il thread dei commenti.
4. Clicca il pulsante **Insert Stuff** (icona a forma di pezzo di puzzle nella barra degli strumenti dell'editor).
5. Nella finestra Insert Stuff, scorri fino a **LTI Advantage** e clicca **FastComments**.
6. FastComments apre un selettore di deep linking. Conferma il posizionamento (le opzioni di default funzionano per le discussioni sui contenuti); clicca **Insert** o **Continue**.
7. Brightspace ritorna all'editor HTML con un blocco segnaposto che rappresenta il lancio LTI. Clicca **Save and Close** sul topic.

Quando il topic viene caricato, Brightspace sostituisce il segnaposto con un iframe che avvia automaticamente FastComments tramite LTI. Gli studenti vedono il thread della discussione inline.

Un singolo topic HTML può contenere più embed FastComments deep-linked. Ogni embed ottiene il proprio thread perché ogni deep link produce un resource link ID distinto.

#### Module Topic vs Inline Quicklink

Scegli l'approccio del **module topic** quando:

- La discussione è l'attività principale per quel passaggio nel modulo.
- Vuoi che il topic appaia nella table of contents di Brightspace, nel completion tracking e in Class Progress.

Scegli l'approccio **inline embed** quando:

- I commenti devono stare sotto altri contenuti nella stessa pagina.
- Non vuoi un elemento separato tracciabile per il completamento nella table of contents.

#### Visibility, Draft, and Release Conditions

Un nuovo topic FastComments è visibile agli studenti per impostazione predefinita. Per nasconderlo mentre lo configuri:

1. Nell'editor dei contenuti, clicca il titolo del topic (Classic) o il menu a tre puntini sull'attività (New Content Experience).
2. Imposta lo stato su **Draft** (Classic) o disattiva la **Visibility** (New Content Experience).

I topic in Draft sono invisibili agli studenti. I docenti e gli assistenti didattici li vedono comunque con un badge "Draft".

Per limitare il topic a un gruppo o a una sezione specifica:

1. Apri il topic.
2. Clicca il menu del titolo del topic > **Edit Properties In-place** (Classic) o **Edit** > **Restrictions** (New Content Experience).
3. Sotto **Release Conditions**, clicca **Create**.
4. Scegli **Group enrollment** o **Section enrollment**, seleziona il gruppo/sezione e salva.

Le condizioni di rilascio si sommano al mapping dei ruoli di FastComments. Gli studenti che non possono vedere il topic non ricevono un lancio LTI.

#### What Students See on First Launch

Quando uno studente clicca il topic (o carica un topic HTML con un embed):

1. Brightspace esegue il lancio LTI 1.3 in background.
2. FastComments riceve il nome dello studente, l'email, l'URL dell'avatar e il ruolo nell'LMS, e li autentica automaticamente. Non viene mostrata alcuna richiesta di login a FastComments.
3. Il thread dei commenti per quel resource link viene renderizzato all'interno dell'iframe Brightspace.

Mapping dei ruoli al lancio:

- Brightspace `Administrator` diventa un FastComments **admin** per il thread (moderazione completa, delete, ban e accesso alla configurazione).
- Brightspace `Instructor` diventa un FastComments **moderator** (pin, hide, delete, ban).
- Tutti gli altri ruoli (`Learner`, `TeachingAssistant`, ecc.) diventano commentatori standard.

I commenti sono attribuiti all'account Brightspace dello studente. Se lo studente modifica il proprio nome o avatar in Brightspace, il successivo lancio LTI sincronizza la modifica.

#### Iframe Height and Resize

FastComments emette il postMessage `org.imsglobal.lti.frameResize` ad ogni render del thread e quando il contenuto cambia (nuovo commento, espansione delle risposte). Brightspace ascolta questo messaggio e regola l'altezza dell'iframe in modo che il thread non venga tagliato e non mostri una scrollbar interna.

Se l'iframe rimane a un'altezza fissa e ridotta:

- Conferma che il corso sia caricato tramite HTTPS. Il listener postMessage di Brightspace rifiuta frame con contenuti misti.
- Conferma che nessuna estensione del browser stia bloccando il canale postMessage.
- Per gli embed inline in un topic HTML, l'HTML circostante non deve avvolgere l'iframe in un contenitore ad altezza fissa. Rimuovi qualsiasi inline `style="height: ..."` dall'elemento padre.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** La deployment non è abilitata per l'org unit di questo corso. Un amministratore deve aggiungere l'org unit (o un padre) alla lista **Org Units** della deployment. La sola registrazione dello strumento non basta; la deployment determina quali corsi vedono lo strumento.

**`deployment_id` mismatch on launch.** FastComments memorizza in modo TOFU il primo `deployment_id` che riceve per una registration. Se un amministratore elimina la deployment originale e ne crea una nuova, i lanci dalla nuova deployment vengono rifiutati con un errore di mismatch della deployment. La soluzione è re-registrare FastComments (genera una nuova registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) ed eseguire nuovamente la Dynamic Registration); il vecchio record di configurazione verrà sostituito.

**Tool launches but shows "Invalid LTI launch".** Il corso si trova in una struttura tenant/org diversa rispetto a quella coperta dalla deployment, oppure la deployment è stata disabilitata dopo la registrazione. Ricontrolla **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > il toggle **Enabled** e la lista degli org unit della deployment.

**Names and roles missing inside FastComments.** Brightspace include nei lanci LTI le claims di Names and Role Provisioning Services (NRPS). Se un corso è stato aggiornato da un vecchio link LTI 1.1, il lancio potrebbe mancare delle claims `name` e `email`. Riaggiungi il topic FastComments tramite **Add Existing** (non migrare il vecchio link) in modo che il lancio utilizzi LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** Il topic HTML è stato inserito come un normale `<iframe>` che punta a FastComments invece di essere inserito tramite **Insert Stuff** > **LTI Advantage**. I normali iframe saltano il lancio LTI e portano gli utenti sulla pagina pubblica di FastComments. Elimina l'iframe e reinseriscilo tramite il flusso Insert Stuff.