Questa pagina tratta l'aggiunta di FastComments a un corso Brightspace dopo che un amministratore ha registrato lo strumento e creato una deployment. Se lo strumento non è ancora registrato, consultare prima la guida alla registrazione D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments incorporato come argomento di unità in Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments in esecuzione all'interno di un'unità Brightspace, che mostra commenti threadati e un selettore per le menzioni @-mention" />
</div>

Brightspace fornisce due esperienze di creazione dei contenuti: **Classic Content** e la **New Content Experience** (detta anche **Lessons**). Entrambe espongono FastComments, ma i percorsi dei menu differiscono. Ogni sezione qui sotto copre entrambe le opzioni quando divergono.

#### Locate the FastComments Tool

Lo strumento FastComments appare in due punti all'interno dell'editor dei contenuti del corso:

1. Il selettore di attività, raggiungibile dal pulsante **Add Existing** del modulo/unità (etichettato **Add Existing Activities** nelle versioni più vecchie di Brightspace). FastComments appare direttamente nel selettore nelle build correnti di Brightspace; nelle versioni più vecchie è annidato sotto un sottomenu **External Learning Tools**. Entrambe le vie aggiungono FastComments come argomento indipendente.
2. La finestra di dialogo **Insert Stuff** all'interno dell'editor HTML, sotto **LTI Advantage**. Questo incorpora FastComments inline in un argomento HTML tramite il flusso di deep linking LTI.

Se FastComments non appare in nessuno dei due selettori, la deployment non è abilitata per l'unità organizzativa che contiene il corso. Chiedere all'amministratore Brightspace di aprire **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, aprire la deployment e aggiungere l'unità organizzativa del corso (o una unità superiore) sotto **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Aprire il corso e cliccare **Content** nella barra di navigazione.
2. Selezionare il modulo che dovrebbe contenere la discussione (o crearne uno tramite **Add a module**).
3. Cliccare **Add Existing** (Brightspace più vecchio: **Add Existing Activities** > **External Learning Tools**).
4. Nel selettore, cliccare **FastComments**. Brightspace crea un argomento nel modulo e ritorna alla vista contenuti.
5. Cliccare il nuovo argomento. Rinominarlo con qualcosa di descrittivo come `FastComments Discussion` usando l'editor del titolo inline.

New Content Experience (Lessons):

1. Aprire il corso e cliccare **Content**.
2. Aprire l'unità e la lesson che dovrebbero contenere la discussione.
3. Cliccare **Add** > **Existing Activity** e selezionare **FastComments** (Brightspace più vecchio: annidato sotto **External Learning Tools**).
4. L'attività viene aggiunta alla lesson.
5. Cliccare il titolo dell'attività per rinominarlo.

La prima volta che qualsiasi utente (istruttore o studente) apre l'argomento, FastComments inizializza il thread per quel resource link. Il thread è vincolato all'ID del resource link, quindi rinominare o spostare l'argomento non cambia quale thread viene caricato.

#### Embed FastComments Inline in an HTML Topic

Usare questo flusso quando si desidera che i commenti appaiano sotto un testo, un video o altro contenuto all'interno della stessa pagina argomento invece che come argomento separato.

1. Aprire o creare un argomento HTML nel modulo/lesson.
2. Cliccare **Edit HTML** per aprire l'editor HTML di Brightspace.
3. Posizionare il cursore dove deve apparire il thread di commenti.
4. Cliccare il pulsante **Insert Stuff** (icona a forma di pezzo di puzzle nella barra dell'editor).
5. Nella finestra Insert Stuff, scorrere fino a **LTI Advantage** e cliccare **FastComments**.
6. FastComments apre un selettore di deep linking. Confermare il posizionamento (le opzioni predefinite funzionano per le discussioni sul contenuto); cliccare **Insert** o **Continue**.
7. Brightspace ritorna all'editor HTML con un blocco segnaposto che rappresenta il lancio LTI. Cliccare **Save and Close** sull'argomento.

Quando l'argomento viene caricato, Brightspace sostituisce il segnaposto con un iframe che avvia automaticamente FastComments tramite LTI. Gli studenti vedono il thread di discussione inline.

Un singolo argomento HTML può contenere più embed deep-linked di FastComments. Ogni embed ottiene il proprio thread perché ogni deep link produce un diverso resource link ID.

#### Module Topic vs Inline Quicklink

Scegliere l'approccio **module topic** quando:

- La discussione è l'attività principale per quel passaggio nel modulo.
- Si desidera che l'argomento appaia nella table of contents di Brightspace, nel tracciamento del completamento e in Class Progress.

Scegliere l'approccio **inline embed** quando:

- I commenti devono essere posizionati sotto altro contenuto sulla stessa pagina.
- Non si desidera un elemento separato tracciabile per il completamento nella table of contents.

#### Visibility, Draft, and Release Conditions

Un nuovo argomento FastComments è visibile agli studenti per impostazione predefinita. Per nasconderlo mentre lo si prepara:

1. Nell'editor dei contenuti, cliccare il titolo dell'argomento (Classic) o il menu a tre puntini sull'attività (New Content Experience).
2. Impostare lo status su **Draft** (Classic) o disattivare la **Visibility** (New Content Experience).

Gli argomenti in Draft sono invisibili agli studenti. I docenti e gli assistenti didattici li vedono comunque con un badge "Draft".

Per limitare l'argomento a un gruppo o a una sezione specifica:

1. Aprire l'argomento.
2. Cliccare il menu del titolo dell'argomento > **Edit Properties In-place** (Classic) o **Edit** > **Restrictions** (New Content Experience).
3. Sotto **Release Conditions**, cliccare **Create**.
4. Scegliere **Group enrollment** o **Section enrollment**, selezionare il gruppo/sezione e salvare.

Le release conditions si sommano alla mappatura dei ruoli di FastComments. Gli studenti che non possono vedere l'argomento non ricevono un lancio LTI.

#### What Students See on First Launch

Quando uno studente clicca l'argomento (o carica un argomento HTML con un embed):

1. Brightspace esegue il lancio LTI 1.3 in background.
2. FastComments riceve il nome dello studente, l'email, l'URL dell'avatar e il ruolo LMS, e li autentica automaticamente. Non viene richiesto alcun login FastComments.
3. Il thread di commenti per quel resource link viene renderizzato all'interno dell'iframe di Brightspace.

Mappatura dei ruoli al lancio:

- Brightspace `Administrator` diventa un amministratore FastComments (**admin**) per il thread (piena moderazione, eliminazione, ban e accesso alla configurazione).
- Brightspace `Instructor` diventa un moderatore FastComments (**moderator**) (pin, nascondi, elimina, ban).
- Tutti gli altri ruoli (`Learner`, `TeachingAssistant`, ecc.) diventano commentatori standard.

I commenti sono attribuiti all'account Brightspace dello studente. Se lo studente modifica il suo nome o avatar in Brightspace, il successivo lancio LTI sincronizzerà la modifica.

#### Lock Down Public Access (Recommended)

Per impostazione predefinita, i dati dei commenti FastComments sono leggibili pubblicamente. Chiunque riesca a indovinare l'URL di un thread o un endpoint API può visualizzarne i commenti, anche al di fuori di Brightspace. Per le discussioni di corso quasi certamente si vorrà limitare la visualizzazione ai soli partecipanti iscritti.

Aprire la propria <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pagina di personalizzazione del widget</a> e creare una regola con **Require SSO To View Comments** abilitato, quindi impostare il livello di sicurezza su **Secure SSO** in modo che i thread possano essere caricati solo tramite il lancio LTI firmato.

Consultare [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) per la procedura completa, incluso come limitare la regola a un singolo dominio o pagina.

#### Iframe Height and Resize

FastComments emette il postMessage `org.imsglobal.lti.frameResize` ad ogni render del thread e al cambiare del contenuto (nuovo commento, espansione delle risposte). Brightspace ascolta questo messaggio e regola l'altezza dell'iframe in modo che il thread non venga tagliato e non mostri una barra di scorrimento interna.

Se l'iframe rimane a un'altezza fissa e bassa:

- Confermare che il corso sia caricato tramite HTTPS. Il listener postMessage di Brightspace rifiuta frame con contenuti misti.
- Confermare che nessuna estensione del browser stia bloccando il canale postMessage.
- Per gli embed inline in un argomento HTML, l'HTML circostante non deve avvolgere l'iframe in un contenitore ad altezza fissa. Rimuovere qualsiasi `style="height: ..."` inline dall'elemento padre.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** La deployment non è abilitata per l'unità organizzativa di questo corso. Un amministratore deve aggiungere l'unità organizzativa (o una parent) alla lista **Org Units** della deployment. La sola registrazione dello strumento non è sufficiente; la deployment definisce quali corsi vedono lo strumento.

**`deployment_id` mismatch on launch.** FastComments applica TOFU al primo `deployment_id` che incontra per una registrazione. Se un amministratore elimina la deployment originale e ne crea una nuova, i lanci dalla nuova deployment vengono rifiutati con un errore di mismatch della deployment. La soluzione è registrare nuovamente FastComments (generare una nuova URL di registrazione (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ottienila qui</a>) ed eseguire di nuovo la Dynamic Registration); il vecchio record di configurazione viene sostituito.

**Tool launches but shows "Invalid LTI launch".** Il corso è in una struttura tenant/org diversa da quella coperta dalla deployment, oppure la deployment è stata disabilitata dopo la registrazione. Ricontrollare **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > toggle **Enabled** e la lista delle org unit della deployment.

**Names and roles missing inside FastComments.** Brightspace invia i lanci LTI con claims NRPS (Names and Role Provisioning Services). Se un corso è stato aggiornato da un vecchio link LTI 1.1, il lancio può non contenere i claims `name` e `email`. Reinserire l'argomento FastComments tramite **Add Existing** (non migrare il vecchio link) in modo che il lancio usi LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** L'argomento HTML è stato inserito come un semplice `<iframe>` puntato a FastComments invece che tramite **Insert Stuff** > **LTI Advantage**. I frame plain saltano il lancio LTI e mandano gli utenti alla pagina pubblica di FastComments. Eliminare l'iframe e reinserirlo tramite il flusso Insert Stuff.