Questa guida spiega come aggiungere FastComments a un corso Moodle 4.x dopo che un amministratore del sito ha registrato lo strumento e lo ha impostato per essere mostrato nel selettore di attività. Se FastComments non è ancora registrato, consultare prima la guida alla registrazione di Moodle.

#### Aprire il corso in modalità di modifica

1. Accedi a Moodle come Docente con permessi di modifica (o ruolo superiore) per il corso.
2. Apri il corso.
3. Attiva la **Modalità modifica** usando l'interruttore nell'angolo in alto a destra dell'intestazione del corso.

Moodle 4.x ha sostituito il vecchio menu a discesa "Aggiungi un'attività o una risorsa" usato nella 3.x con una finestra di selezione attività a schermo intero. Moodle 4.5 mantiene lo stesso selettore ma aggiunge una riga con elementi preferiti/stellati in alto, quindi appuntare FastComments una volta rende più veloce raggiungerlo nelle sezioni successive.

#### Aggiungere l'attività FastComments

1. Scorri fino alla sezione del corso (argomento o settimana) a cui appartiene la discussione.
2. Clicca **Aggiungi un'attività o una risorsa** nella parte inferiore di quella sezione.
3. Nella finestra di selezione, seleziona **FastComments**. Se non lo vedi, passa alla sezione sulle avvertenze qui sotto.

Si apre il modulo delle impostazioni dell'attività. I campi che contano:

- **Activity name** (obbligatorio). Visualizzato nella pagina del corso e nel registro voti. Esempio: `Week 3 Discussion`.
- **Activity description**. Testo introduttivo facoltativo visualizzato sopra il thread di commenti.
- **Show description on course page**. Seleziona questa opzione se vuoi che la descrizione sia visibile senza entrare nell'attività.
- **Preconfigured tool**. Impostato su `FastComments` (selezionato automaticamente quando lanciato dal selettore). Non modificarlo.
- **Launch container**. Impostare su **Nuova finestra**. Vedi la sezione sulle avvertenze per il motivo per cui "Stessa finestra" può causare problemi in alcune installazioni Moodle.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Lasciare vuoti. La Registrazione Dinamica gestisce questi valori a livello di sito.

Scorri fino in fondo e clicca **Salva e torna al corso** (o **Salva e visualizza** per aprire subito l'attività).

L'attività appare come una riga nella sezione con l'icona FastComments. Gli studenti cliccano sulla riga per aprire il thread di commenti.

#### Incorporare FastComments inline nell'editor

Per un thread all'interno di una Pagina, capitolo di Book, Lezione o qualsiasi altra risorsa che utilizzi l'editor Atto o TinyMCE:

1. Apri la risorsa in modalità di modifica.
2. Posiziona il cursore dove deve apparire il thread.
3. Nella barra degli strumenti dell'editor, clicca il pulsante **LTI** / **Strumento esterno**. In Atto è etichettato "Insert LTI Advantage content". In TinyMCE (di default in Moodle 4.3+) si trova nel menu **Altro** come **Strumenti esterni**.
4. Scegli **FastComments** dalla lista degli strumenti.
5. FastComments apre un selettore di deep-linking. Conferma il titolo del thread e clicca **Incorpora**.
6. L'editor inserisce un blocco segnaposto LTI. Salva la risorsa.

Ogni istanza incorporata è un thread distinto indicizzato sull'ID dell'elemento di deep-linking, quindi una Pagina con tre embed di FastComments genera tre thread indipendenti.

#### Restrizioni di accesso e impostazioni dei gruppi

Le impostazioni standard delle attività di Moodle si applicano alle attività FastComments:

- **Common module settings** > **Group mode**. Impostare questo su **Gruppi separati** o **Gruppi visibili** non divide automaticamente FastComments in thread per gruppo. La modalità gruppi di Moodle filtra solo il registro voti e l'elenco dei membri. Per avere un thread separato per ogni gruppo, aggiungi un'attività FastComments per ogni gruppo e usa **Restrict access** per delimitare ciascuna.
- **Restrict access** > **Add restriction**. Supporta le condizioni standard di Moodle: **Date**, **Grade**, **Group**, **Grouping**, **User profile**, e insiemi di restrizioni nidificati. Usa **Group** per bloccare un'attività FastComments a un singolo gruppo.
- **Activity completion**. Imposta su **Gli studenti devono visualizzare questa attività per completarla** se vuoi il tracciamento del completamento. FastComments attualmente non invia un evento di completamento a Moodle oltre al lancio.

#### Mappatura dei ruoli

FastComments legge la claim LTI `roles` che Moodle invia ad ogni lancio e la mappa come segue:

- Moodle **Manager** o **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** o **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> sola lettura

Gli admin possono cancellare qualsiasi commento, bannare utenti e modificare le impostazioni del thread. I moderator possono cancellare e approvare i commenti all'interno del thread in cui sono entrati. I ruoli Moodle personalizzati ereditano la mappatura dall'archetipo da cui sono stati clonati.

#### Cosa vedono gli studenti

Gli studenti cliccano sull'attività FastComments (o scorrono fino al blocco incorporato dentro una Pagina o Book). Moodle invia la loro identità a FastComments tramite il lancio LTI:

- Nessuna schermata di login. FastComments li autentica usando l'account Moodle.
- Il loro nome visualizzato, email e avatar provengono da Moodle.
- Il thread è vincolato a `(Moodle site, course, resource link ID)`, quindi la stessa attività duplicata in un altro corso ottiene un thread nuovo.
- Le risposte annidate, le votazioni e le notifiche funzionano come in un thread FastComments indipendente.

#### Limitare l'accesso pubblico (consigliato)

Per impostazione predefinita, i dati dei commenti FastComments sono leggibili pubblicamente. Chiunque possa indovinare l'URL di un thread o un endpoint API può visualizzarne i commenti, anche al di fuori di Moodle. Per le discussioni del corso quasi certamente vorrai limitare la visualizzazione agli studenti iscritti soltanto.

Apri la tua <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pagina di personalizzazione del widget</a> e crea una regola con **Require SSO To View Comments** abilitato, quindi imposta il livello di sicurezza su **Secure SSO** in modo che i thread possano essere caricati solo tramite il lancio LTI firmato.

Vedi [Proteggere i thread di commento con Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) per la procedura completa, incluso come limitare la regola a un singolo dominio o pagina.

#### Problematiche di Moodle

**FastComments non presente nel selettore di attività.** L'amministratore del sito ha registrato lo strumento ma non ha impostato **Tool configuration usage** su **Show in activity chooser and as a preconfigured tool**. Correggi questo sotto **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > icona a ingranaggio sulla tessera FastComments.

**Il lancio fallisce o mostra un frame vuoto quando impostato su "Stessa finestra".** I cookie di sessione di Moodle usano `SameSite=Lax` per impostazione predefinita e alcuni browser li rimuovono sulla POST cross-site che LTI 1.3 usa per il ritorno da FastComments. Imposta **Launch container** su **Nuova finestra** nell'attività. Questa è una richiesta stringente per i FastComments incorporati all'interno di una Pagina o Book, poiché il percorso di lancio incorporato dall'editor apre sempre una nuova finestra.

**La claim `iss` è l'URL del sito Moodle, non un tenant ID.** FastComments usa l'URL del sito Moodle (il valore di configurazione `wwwroot`) come issuer LTI. Se la tua istanza Moodle viene spostata su un nuovo dominio o cambi `wwwroot`, i thread FastComments esistenti rimangono legati al vecchio issuer e non corrisponderanno ai nuovi lanci. Registrare di nuovo lo strumento contro il nuovo URL e migrare i thread tramite l'admin di FastComments se necessario.

**Backup e ripristino dell'attività.** Effettuare il backup di un corso e ripristinarlo in un nuovo corso crea nuovi resource link ID, quindi le attività FastComments ripristinate iniziano con thread vuoti. Il corso originale mantiene i thread originali. Questo è un comportamento voluto, non un bug.

**TinyMCE di default in Moodle 4.5.** Moodle 4.5 viene fornito con TinyMCE come editor predefinito per le nuove installazioni. La posizione del pulsante Strumento esterno è sotto il menu **Altro** (`...`) invece che nella barra principale. I siti più vecchi che sono stati aggiornati dalla 4.1 mantengono Atto a meno che un amministratore non abbia cambiato il predefinito.