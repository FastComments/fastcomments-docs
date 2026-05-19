Una volta che FastComments è registrato sulla piattaforma, gli istruttori lo aggiungono ai contenuti del corso usando i flussi standard per strumenti esterni della piattaforma. Questa pagina copre Sakai 23.x e Schoology Enterprise.

#### Limitare l'accesso pubblico (consigliato)

Per impostazione predefinita, i dati dei commenti di FastComments sono leggibili pubblicamente su entrambe le piattaforme. Chiunque riesca a indovinare l'URL di un thread o un endpoint API può visualizzarne i commenti, anche al di fuori di Sakai o Schoology. Per le discussioni di corso quasi certamente vorrete limitare la visualizzazione solo agli studenti iscritti.

Apri la tua <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pagina di personalizzazione del widget</a> e crea una regola con **Richiedi SSO per visualizzare i commenti** abilitata, quindi imposta il livello di sicurezza su **SSO protetto** in modo che i thread possano essere caricati solo tramite il lancio LTI firmato.

Vedi [Proteggere i thread di commento con Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) per la guida completa, inclusa la modalità di limitazione della regola a un singolo dominio o pagina.

#### Sakai

**1. Aggiungere FastComments a un sito**

Il manutentore del sito abilita lo strumento per ciascun sito:

1. Apri il sito e clicca **Informazioni sito** nella navigazione a sinistra.
2. Clicca **Gestisci strumenti**.
3. Scorri fino all'elenco **Strumenti esterni** e attiva **FastComments**.
4. Clicca **Continua**, controlla l'elenco degli strumenti, quindi clicca **Fine**.

FastComments ora appare come voce nella navigazione a sinistra del sito.

**2. Riordinare la voce nella navigazione a sinistra**

Vai a **Informazioni sito** > **Ordine strumenti**. Trascina **FastComments** nella posizione desiderata e clicca **Salva**. Da questa schermata puoi anche rinominare l'etichetta di navigazione e nasconderla agli studenti.

**3. Incorporare inline in una pagina Lessons**

Per posizionare FastComments direttamente all'interno di una pagina Lessons anziché come strumento a sé stante nella navigazione a sinistra:

1. Apri lo strumento **Lessons** nel sito.
2. Clicca **Aggiungi contenuto** > **Aggiungi strumento esterno**.
3. Seleziona **FastComments** dall'elenco.
4. Se FastComments ha pubblicizzato Deep Linking durante la registrazione, Sakai apre il selettore di contenuti dello strumento in modo da poter scegliere o etichettare il thread. Se Deep Linking non è stato pubblicizzato, Sakai inserisce un link di avvio predefinito.
5. Salva l'elemento Lessons.

Ogni istanza incorporata ottiene il proprio thread, limitato a quel link di risorsa.

**4. Regolazioni dei permessi per l'accesso degli studenti**

Sakai regola i lanci degli strumenti esterni tramite i Realms. Per confermare che gli studenti possano avviare FastComments:

1. Accedi come amministratore Sakai e apri **Area di amministrazione** > **Realms**.
2. Apri il realm pertinente (per esempio, `!site.template.course` o il realm specifico del sito).
3. Conferma che il ruolo `access` abbia `lti.launch` abilitato e che i permessi di ruolo nel gruppo **external.tools** siano concessi.
4. Salva il realm.

Per override a livello di sito, il manutentore può regolare la visibilità dello strumento per ruolo da **Informazioni sito** > **Ordine strumenti** nascondendo o mostrando FastComments per ruolo.

**5. Cosa vedono gli studenti**

Gli studenti cliccano la voce FastComments nella navigazione a sinistra (o scorrono fino al blocco Lessons incorporato) e approdano direttamente nella vista a thread dei commenti. L'SSO è automatico: Sakai invia l'identità dell'utente nel lancio LTI e FastComments li autentica con il loro account Sakai.

Mappatura dei ruoli:

- Sakai `Instructor` -> moderatore FastComments
- Sakai `Admin` (admin in Area di amministrazione) -> amministratore FastComments
- Sakai `Student` / `access` -> commentatore FastComments

**6. Problemi comuni in Sakai**

- **Tool non visibile in Gestisci strumenti.** Se FastComments non appare nell'elenco Strumenti esterni, l'amministratore Sakai deve aprire il registro strumenti (**Area di amministrazione** > **Strumenti esterni** > **FastComments**) e impostare **Stealthed** su `false`. Gli strumenti stealthed sono nascosti dal selettore Gestisci strumenti per sito.
- **Lanci che si interrompono in browser con sessioni condivise.** Il token CSRF del portale Sakai è legato alla sessione del browser. Se uno studente è connesso a due siti Sakai in tab diversi o ha una sessione scaduta, il lancio restituisce un 403. Soluzione: chiudere gli altri tab Sakai, disconnettersi, riconnettersi e rilanciare. Gli amministratori possono anche aumentare `sakai.csrf.token.cache.ttl` se questo si verifica a livello di cluster.
- **Incorporamento in frame.** Conferma che `lti.frameheight` in `sakai.properties` sia sufficientemente grande (600 o superiore) in modo che il thread di commenti non venga tagliato all'interno di una pagina Lessons.

#### Schoology

Schoology Enterprise prevede due scenari di installazione. Conferma quale si applica prima di aggiungere lo strumento a un corso.

**1. Due scenari di installazione**

- **(a) Installazione a livello aziendale.** L'Amministratore di sistema di Schoology ha installato FastComments a livello organizzazione e lo ha assegnato a tutti i corsi o a specifici template di corso. Gli istruttori saltano l'installazione e vanno direttamente a **Aggiungi materiali**.
- **(b) Installazione autonoma da parte dell'istruttore.** L'istruttore installa lo strumento in un singolo corso da **Opzioni corso** > **Strumenti esterni** > **Installa app LTI**. L'installazione autonoma richiede che l'Amministratore di sistema abbia prima approvato l'app FastComments a livello organizzativo.

**2. Aggiungere FastComments come materiale del corso**

All'interno del corso:

1. Apri il corso e vai a **Materiali**.
2. Clicca **Aggiungi materiali** > **Aggiungi File/Link/Strumento esterno**.
3. Scegli **Strumento esterno**.
4. Seleziona **FastComments** dall'elenco degli strumenti registrati.
5. Imposta un **Nome** (è ciò che gli studenti vedono nell'elenco dei materiali) e un'eventuale **Descrizione**.
6. Lascia **Abilita valutazione** (grade passback) **DISATTIVATO**. FastComments non riporta voti a Schoology, quindi abilitare il grade passback crea una colonna vuota nel registro voti.
7. Clicca **Invia**.

Il materiale ora appare nell'elenco dei materiali del corso e apre il thread FastComments quando viene cliccato.

**3. Incorporamento inline tramite l'editor Rich Text**

Se l'Amministratore di sistema ha abilitato il posizionamento Deep Linking per FastComments durante la registrazione, gli istruttori possono incorporare il thread di commenti all'interno di qualsiasi campo Rich Text (istruzioni dell'assegnazione, corpi delle pagine, prompt di discussione):

1. Apri l'editor Rich Text sulla pagina di destinazione.
2. Clicca l'icona **External Tool** (pezzo di puzzle) nella barra degli strumenti.
3. Scegli **FastComments**.
4. Configura l'incorporamento nella finestra di dialogo deep-linking e clicca **Inserisci**.
5. Salva la pagina.

Se il pulsante External Tool non appare nell'editor Rich Text, Deep Linking è disabilitato per questo strumento in questo tenant. Vedi i problemi noti qui sotto.

**4. Visibilità e assegnazioni per sezione**

Schoology limita la disponibilità dello strumento per sezione tramite le Opzioni corso:

1. Dal corso, clicca **Opzioni corso** > **Strumenti esterni**.
2. Per ciascuna app LTI installata, puoi controllare se è disponibile per tutte le sezioni del corso o per sezioni specifiche.
3. Per limitare FastComments a determinate sezioni, deseleziona le sezioni che non dovrebbero vedere lo strumento.
4. L'accesso a livello di sezione determina anche quali sezioni vedono la voce **Aggiungi materiali** > **Strumento esterno** per FastComments.

**5. Cosa vedono gli studenti**

Gli studenti cliccano il materiale FastComments (o scorrono fino all'incorporamento inline) e accedono alla discussione a thread. L'SSO è automatico tramite il lancio LTI di Schoology con il loro account Schoology.

Mappatura dei ruoli:

- Schoology `Administrator` -> amministratore FastComments
- Schoology `Instructor` -> moderatore FastComments
- Schoology `Student` -> commentatore FastComments

**6. Problemi comuni in Schoology**

- **Solo Enterprise.** Gli account personali e gratuiti di Schoology non possono installare strumenti LTI 1.3. Se il tuo tenant è nel piano gratuito, l'opzione **Strumenti esterni** è assente dalle Opzioni corso. Effettua l'upgrade a Schoology Enterprise per usare FastComments.
- **Deep Linking disabilitato per impostazione predefinita dal tenant.** Alcuni tenant Schoology limitano il posizionamento Deep Linking a livello organizzazione. In questo caso, gli istruttori vedono solo il flusso **Aggiungi materiali** > **Strumento esterno** e non il pulsante External Tool nell'editor Rich Text. Per abilitare l'incorporamento inline, l'Amministratore di sistema va su **Impostazioni di sistema** > **Integrazione** > **LTI 1.3** > **FastComments**, abilita il posizionamento **Content Item / Deep Linking**, quindi salva.
- **Override di assegnazione per sezione.** Se FastComments è assegnato a livello enterprise ma l'istruttore non lo vede in **Aggiungi materiali**, la sezione del corso è esclusa nell'assegnazione a livello organizzazione. Chiedi all'Amministratore di sistema di aggiungere la sezione all'assegnazione dell'app FastComments.
- **Nome del materiale vs. identità del thread.** Rinominare il materiale in Schoology non sposta il thread di commenti. I thread sono indicizzati sull'ID del link di risorsa LTI, quindi una rinomina mantiene lo stesso thread; eliminare e ricreare il materiale crea un nuovo thread vuoto.