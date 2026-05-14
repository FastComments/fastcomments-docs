Questa guida copre l'aggiunta di FastComments a un corso Moodle 4.x dopo che un amministratore del sito ha registrato lo strumento e lo ha impostato per essere mostrato nel selettore di attività. Se FastComments non è ancora registrato, consulta prima la guida alla registrazione di Moodle.

#### Aprire il corso in modalità modifica

1. Accedi a Moodle come Docente con permessi di modifica (o con un ruolo superiore) per il corso.
2. Apri il corso.
3. Attiva la **Modalità modifica** usando l'interruttore nell'angolo in alto a destra dell'intestazione del corso.

Moodle 4.x ha sostituito il vecchio menu a discesa "Aggiungi un'attività o una risorsa" usato nella 3.x con una finestra di dialogo full-screen per la selezione delle attività. Moodle 4.5 mantiene lo stesso selettore ma aggiunge una fila di elementi preferiti/stelle in alto, quindi appuntare FastComments una volta lo rende più veloce da raggiungere nelle sezioni successive.

#### Aggiungere l'attività FastComments

1. Scorri fino alla sezione del corso (argomento o settimana) dove deve andare la discussione.
2. Clicca **Aggiungi un'attività o una risorsa** in fondo a quella sezione.
3. Nella finestra di dialogo di selezione, seleziona **FastComments**. Se non la vedi, passa alla sezione Avvertenze qui sotto.

Si apre il modulo delle impostazioni dell'attività. I campi importanti:

- **Activity name** (obbligatorio). Viene mostrato nella pagina del corso e nel registro voti. Esempio: `Week 3 Discussion`.
- **Activity description**. Testo introduttivo opzionale visualizzato sopra il thread dei commenti.
- **Show description on course page**. Selezionalo se vuoi che la descrizione sia visibile senza entrare nell'attività.
- **Preconfigured tool**. Impostato su `FastComments` (selezionato automaticamente quando avviato dal selettore). Non modificarlo.
- **Launch container**. Impostalo su **New window**. Vedi la sezione Avvertenze per spiegazioni sul perché "Same window" si rompe in alcune installazioni di Moodle.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Lascia vuoti. La Registrazione dinamica gestisce questi valori a livello di sito.

Scorri fino in fondo e clicca **Save and return to course** (o **Save and display** per aprire immediatamente l'attività).

L'attività appare come una riga nella sezione con l'icona FastComments. Gli studenti cliccano la riga per aprire il thread dei commenti.

#### Incorporare FastComments in linea con l'editor

Per un thread all'interno di una Pagina, di un capitolo di Book, di una Lezione o di qualsiasi altra risorsa che usa l'editor Atto o TinyMCE:

1. Apri la risorsa in modalità modifica.
2. Posiziona il cursore dove deve apparire il thread.
3. Nella barra degli strumenti dell'editor, clicca il pulsante **LTI** / **External tool**. In Atto è etichettato "Insert LTI Advantage content". In TinyMCE (default in Moodle 4.3+) è nel menu **More** come **External tools**.
4. Scegli **FastComments** dalla lista degli strumenti.
5. FastComments apre un selettore di deep-linking. Conferma il titolo del thread e clicca **Embed**.
6. L'editor inserisce un blocco segnaposto LTI. Salva la risorsa.

Ogni istanza incorporata è un thread distinto indicizzato sull'ID dell'elemento di contenuto deep-linking, quindi una Pagina con tre embed di FastComments avrà tre thread indipendenti.

#### Restrizioni di accesso e impostazioni di gruppo

Le impostazioni standard delle attività di Moodle si applicano alle attività FastComments:

- **Common module settings** > **Group mode**. Impostare questo su **Separate groups** o **Visible groups** non suddivide automaticamente FastComments in thread separati per gruppo. La modalità gruppo di Moodle filtra solo il registro voti e l'elenco dei membri. Per avere un thread separato per gruppo, aggiungi una attività FastComments per ogni gruppo e usa le **Restrizioni di accesso** per limitare ciascuna.
- **Restrict access** > **Add restriction**. Supporta le condizioni standard di Moodle: **Date**, **Grade**, **Group**, **Grouping**, **User profile**, e insiemi di restrizioni annidati. Usa **Group** per bloccare un'attività FastComments a un singolo gruppo.
- **Activity completion**. Imposta su **Students must view this activity to complete it** se vuoi il tracciamento del completamento. FastComments attualmente non segnala eventi di completamento a Moodle oltre al lancio.

#### Mappatura dei ruoli

FastComments legge la dichiarazione `roles` che Moodle invia a ogni lancio LTI e la mappa come segue:

- Moodle **Manager** o **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** o **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> sola lettura

Gli amministratori possono eliminare qualsiasi commento, bannare utenti e modificare le impostazioni del thread. I moderatori possono eliminare e approvare commenti all'interno del thread in cui sono entrati. I ruoli personalizzati di Moodle ereditano la mappatura dall'archetipo da cui sono stati clonati.

#### Cosa vedono gli studenti

Gli studenti cliccano l'attività FastComments (o scorrono fino al blocco incorporato all'interno di una Pagina o di un Book). Moodle invia la loro identità a FastComments tramite il lancio LTI:

- Nessuna schermata di login. FastComments li autentica usando l'account Moodle.
- Il loro nome visualizzato, email e avatar provengono da Moodle.
- Il thread è vincolato a `(Moodle site, course, resource link ID)`, quindi la stessa attività duplicata in un altro corso ottiene un thread nuovo.
- Risposte annidate, votazioni e notifiche funzionano allo stesso modo di un thread FastComments autonomo.

#### Problemi noti di Moodle

**FastComments mancante nel selettore di attività.** L'amministratore del sito ha registrato lo strumento ma non ha impostato **Tool configuration usage** su **Show in activity chooser and as a preconfigured tool**. Risolvi questo sotto **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > icona dell'ingranaggio sulla tessera FastComments.

**Il lancio fallisce o mostra un frame vuoto quando impostato su "Same window".** I cookie di sessione di Moodle usano `SameSite=Lax` di default, e alcuni browser li rimuovono durante il POST cross-site che LTI 1.3 usa per tornare da FastComments. Imposta **Launch container** su **New window** nell'attività. Questo è un requisito rigido per i FastComments incorporati all'interno di una Pagina o di un Book, poiché il percorso di lancio incorporato dall'editor apre sempre una nuova finestra.

**La claim `iss` è l'URL del sito Moodle, non un tenant ID.** FastComments usa l'URL del sito Moodle (il valore di configurazione `wwwroot`) come issuer LTI. Se la tua istanza Moodle si sposta su un nuovo dominio o cambi `wwwroot`, i thread FastComments esistenti rimangono legati al vecchio issuer e non corrisponderanno ai nuovi lanci. Registrare nuovamente lo strumento con il nuovo URL e migrare i thread tramite l'amministrazione di FastComments se necessario.

**Backup e ripristino delle attività.** Effettuare il backup di un corso e ripristinarlo in un nuovo corso crea nuovi resource link ID, quindi le attività FastComments ripristinate iniziano con thread vuoti. Il corso originale mantiene i thread originali. Questo è il comportamento previsto, non un bug.

**Moodle 4.5 TinyMCE come predefinito.** Moodle 4.5 è distribuito con TinyMCE come editor predefinito per nuove installazioni. Il pulsante External tool si trova nel menu **More** (`...`) invece che nella barra degli strumenti principale. I siti più vecchi aggiornati da 4.1 mantengono Atto a meno che un amministratore non abbia cambiato il predefinito.