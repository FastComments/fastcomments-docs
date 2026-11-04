Questa pagina copre l'aggiunta di FastComments a un corso Brightspace dopo che un amministratore ha registrato lo strumento e creato una distribuzione. Se lo strumento non è ancora registrato, consulta prima la guida alla registrazione D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments incorporato come argomento di unità in Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments in esecuzione all'interno di un'unità Brightspace, con commenti in thread e un selettore di @-menzioni" />
</div>

Brightspace offre due esperienze di creazione dei contenuti: **Classic Content** e la **New Content Experience** (chiamata anche **Lessons**). Entrambe espongono FastComments, ma i percorsi dei menu differiscono. Ogni sezione seguente copre entrambe dove divergono.

#### Individuare lo strumento FastComments

Lo strumento FastComments appare in due punti all'interno di un editor di contenuti del corso:

1. Il selettore di attività, raggiungibile dal pulsante **Add Existing** di un modulo/unità (etichettato **Add Existing Activities** nelle versioni più vecchie di Brightspace). FastComments appare direttamente nel selettore nelle versioni attuali di Brightspace; nelle versioni più vecchie è annidato sotto un sottomenu **External Learning Tools**. Entrambi i percorsi aggiungono FastComments come argomento autonomo.
2. La finestra di dialogo **Insert Stuff** all'interno dell'editor HTML, sotto **LTI Advantage**. Questo incorpora FastComments in linea in un argomento HTML tramite il flusso di deep linking LTI.

Se FastComments non appare in nessuno dei due selettori, la distribuzione non è abilitata per l'unità organizzativa che contiene il corso. Chiedete al vostro amministratore Brightspace di aprire **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > strumento FastComments > **View Deployments**, aprire la distribuzione e aggiungere l'unità organizzativa del corso (o un'unità organizzativa padre) sotto **Org Units**.

#### Aggiungere FastComments come argomento in un modulo

Classic Content:

1. Aprite il corso e fate clic su **Content** nella barra di navigazione.
2. Selezionate il modulo che deve contenere la discussione (o createne uno tramite **Add a module**).
3. Fate clic su **Add Existing** (Brightspace più vecchi: **Add Existing Activities** > **External Learning Tools**).
4. Nel selettore, fate clic su **FastComments**. Brightspace crea un argomento nel modulo e vi riporta alla vista dei contenuti.
5. Fate clic sul nuovo argomento. Rinominatelo con qualcosa di descrittivo come `FastComments Discussion` usando l'editor di titolo in linea.

New Content Experience (Lessons):

1. Aprite il corso e fate clic su **Content**.
2. Aprite l'unità e la lezione che devono contenere la discussione.
3. Fate clic su **Add** > **Existing Activity** e selezionate **FastComments** (Brightspace più vecchi: annidato sotto **External Learning Tools**).
4. L'attività viene aggiunta alla lezione.
5. Fate clic sul titolo dell'attività per rinominarla.

La prima volta che qualsiasi utente (docente o studente) apre l'argomento, FastComments inizializza il thread per quel resource link. Il thread è legato all'ID del resource link, quindi rinominare o spostare l'argomento non cambia quale thread viene caricato.

#### Incorporare FastComments in linea in un argomento HTML

Usate questo flusso quando volete che i commenti appaiano sotto una lettura, un video o altro contenuto nella stessa pagina dell'argomento, anziché come argomento separato.

1. Aprite o create un argomento HTML nel modulo/lezione.
2. Fate clic su **Edit HTML** per aprire l'editor HTML di Brightspace.
3. Posizionate il cursore dove dovrebbe apparire il thread dei commenti.
4. Fate clic sul pulsante **Insert Stuff** (icona a forma di pezzo di puzzle nella barra degli strumenti dell'editor).
5. Nella finestra di dialogo Insert Stuff, scorrete fino a **LTI Advantage** e fate clic su **FastComments**.
6. FastComments apre un selettore di deep linking. Confermate il posizionamento (le opzioni predefinite funzionano per le discussioni di contenuto); fate clic su **Insert** o **Continue**.
7. Brightspace torna all'editor HTML con un blocco segnaposto che rappresenta l'avvio LTI. Fate clic su **Save and Close** sull'argomento.

Quando l'argomento si carica, Brightspace sostituisce il segnaposto con un iframe che avvia automaticamente FastComments tramite LTI. Gli studenti vedono il thread di discussione in linea.

Un singolo argomento HTML può contenere più incorporamenti FastComments con deep link. Ogni incorporamento ha il proprio thread perché ogni deep link produce un ID resource link distinto.

#### Argomento del modulo vs Quicklink in linea

Scegliete l'approccio **argomento del modulo** quando:

- La discussione è l'attività principale per quel passaggio nel modulo.
- Volete che l'argomento appaia nell'indice dei contenuti di Brightspace, nel tracciamento del completamento e in Class Progress.

Scegliete l'approccio **incorporamento in linea** quando:

- I commenti devono trovarsi sotto altro contenuto nella stessa pagina.
- Non volete un elemento separato tracciabile per il completamento nell'indice dei contenuti.

#### Visibilità, Bozza e Condizioni di Rilascio

Un nuovo argomento FastComments è visibile agli studenti per impostazione predefinita. Per nasconderlo durante la configurazione:

1. Nell'editor dei contenuti, fate clic sul titolo dell'argomento (Classic) o sul menu a tre punti sull'attività (New Content Experience).
2. Impostate lo stato su **Draft** (Classic) o disattivate **Visibility** (New Content Experience).

Gli argomenti in bozza sono invisibili agli studenti. Docenti e assistenti li vedono comunque con un badge "Draft".

Per limitare l'argomento a un gruppo o una sezione specifica:

1. Aprite l'argomento.
2. Fate clic sul menu del titolo dell'argomento > **Edit Properties In-place** (Classic) o **Edit** > **Restrictions** (New Content Experience).
3. Sotto **Release Conditions**, fate clic su **Create**.
4. Selezionate **Group enrollment** o **Section enrollment**, scegliete il gruppo/sezione e salvate.

Le condizioni di rilascio si combinano con la mappatura dei ruoli di FastComments. Gli studenti che non possono vedere l'argomento non ottengono un avvio LTI.

#### Cosa vedono gli studenti al primo avvio

Quando uno studente fa clic sull'argomento (o carica un argomento HTML con un incorporamento):

1. Brightspace esegue l'avvio LTI 1.3 in background.
2. FastComments riceve il nome, l'email, l'URL dell'avatar e il ruolo LMS dello studente, e lo autentica automaticamente. Non viene mostrata alcuna schermata di accesso FastComments.
3. Il thread dei commenti per quel resource link viene visualizzato all'interno dell'iframe di Brightspace.

Mappatura dei ruoli all'avvio:

- L'`Administrator` di Brightspace diventa un **admin** FastComments per il thread (accesso completo a moderazione, eliminazione, ban e configurazione).
- L'`Instructor` di Brightspace diventa un **moderator** FastComments (pin, nascondi, elimina, ban).
- Tutti gli altri ruoli (`Learner`, `TeachingAssistant`, ecc.) diventano commentatori standard.

I commenti sono attribuiti all'account Brightspace dello studente. Se lo studente modifica il proprio nome o avatar in Brightspace, il prossimo avvio LTI sincronizza la modifica.

#### Bloccare l'Accesso Pubblico (Consigliato)

Per impostazione predefinita, i dati dei commenti FastComments sono leggibili pubblicamente. Chiunque indovini l'URL di un thread o un endpoint API può visualizzarne i commenti, anche al di fuori di Brightspace. Per le discussioni di corso quasi certamente vorrete limitare la visualizzazione ai soli studenti iscritti.

Aprite la vostra <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pagina di personalizzazione del widget</a> e create una regola con **Require SSO To View Comments** abilitato, poi impostate il livello di sicurezza su **Secure SSO** in modo che i thread possano essere caricati solo tramite l'avvio LTI firmato.

Vedete [Protezione dei thread di commenti con Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) per la procedura completa, incluso come limitare la regola a un singolo dominio o pagina.

#### Altezza dell'Iframe e Ridimensionamento

FastComments emette il postMessage `org.imsglobal.lti.frameResize` a ogni render del thread e a ogni modifica del contenuto (nuovo commento, espansione delle risposte). Brightspace ascolta questo messaggio e regola l'altezza dell'iframe in modo che il thread non venga tagliato e non mostri una barra di scorrimento interna.

Se l'iframe rimane a un'altezza fissa ridotta:

- Confermate che il corso sia caricato tramite HTTPS. Il listener postMessage di Brightspace rifiuta i frame con contenuto misto.
- Confermate che nessuna estensione del browser stia bloccando il canale postMessage.
- Per gli incorporamenti in linea in un argomento HTML, l'HTML circostante non deve racchiudere l'iframe in un contenitore a altezza fissa. Rimuovete qualsiasi `style="height: ..."` in linea dall'elemento padre.

#### Particolarità Specifiche di Brightspace

**Strumento non visibile nel selettore Add Existing.** La distribuzione non è abilitata per l'unità organizzativa di questo corso. Un amministratore deve aggiungere l'unità organizzativa (o un'unità padre) all'elenco **Org Units** della distribuzione. La sola registrazione dello strumento non basta; la distribuzione definisce quali corsi vedono lo strumento.

**Mancata corrispondenza di `deployment_id` all'avvio.** FastComments blocca (TOFU) il primo `deployment_id` che vede per una registrazione. Se un amministratore elimina la distribuzione originale e ne crea una nuova, gli avvii dalla nuova distribuzione vengono rifiutati con un errore di mancata corrispondenza della distribuzione. La soluzione è registrare nuovamente FastComments (generare un nuovo URL di registrazione (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ottenetelo qui</a>) ed eseguire di nuovo Dynamic Registration); il vecchio record di configurazione viene sostituito.

**Lo strumento si avvia ma mostra "Invalid LTI launch".** Il corso si trova in una struttura tenant/org diversa da quella coperta dalla distribuzione, oppure la distribuzione è stata disabilitata dopo la registrazione. Ricontrollate l'interruttore **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** e l'elenco delle unità organizzative della distribuzione.

**Nomi e ruoli mancanti all'interno di FastComments.** Brightspace include negli avvii LTI le attestazioni Names and Role Provisioning Services (NRPS). Se un corso è stato aggiornato da un vecchio link LTI 1.1, l'avvio manca delle attestazioni `name` e `email`. Riaggiungete l'argomento FastComments tramite **Add Existing** (non migrate il vecchio link) in modo che l'avvio usi LTI 1.3.

**L'incorporamento mostra una schermata di accesso invece dell'auto-SSO.** L'argomento HTML è stato inserito come semplice `<iframe>` che punta a FastComments anziché tramite **Insert Stuff** > **LTI Advantage**. Gli iframe semplici saltano l'avvio LTI e portano gli utenti alla pagina pubblica di FastComments. Eliminate l'iframe e reinseritelo tramite il flusso Insert Stuff.