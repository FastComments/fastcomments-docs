FastComments applica l'Articolo 17 del Digital Services Act dell'UE per i tenant nella regione UE: **le sospensioni di utenti completamente automatizzate non sono consentite**.

### Cosa significa in pratica

Quando il tuo tenant si trova nella regione UE, nel modulo di modifica dell'agente:

- La casella di controllo **Approvals** per `ban_user` è **bloccata su ON** e non può essere deselezionata.
- L'etichetta recita: "Articolo 17 del DSA UE: le sospensioni degli utenti richiedono una revisione umana. 'Banna un utente' è bloccato su ON e non può essere completamente automatizzato nella regione UE."
- Un tooltip nella colonna delle approvazioni recita: "Bloccato dall'Articolo 17 del DSA UE - le sospensioni totalmente automatizzate non sono consentite nella regione UE."

Qualsiasi altra configurazione, ogni chiamata `ban_user` da qualsiasi agente su un tenant nella regione UE viene inviata alla [casella delle approvazioni](#approval-workflow) per la revisione umana. La sospensione non avviene fino a quando un umano non la approva.

### Perché questo viene applicato a livello di piattaforma, non a livello di prompt

I prompt di sistema possono essere ignorati o aggirati da un modello che si comporta male. La conformità all'Articolo 17 è troppo importante per affidarsi al buon comportamento del modello; deve essere un controllo server-side rigoroso che lo stesso dispatcher degli strumenti applica. Ed è quello che facciamo.

### Cosa viene sottoposto ad approvazione e cosa no

- **`ban_user`**: sempre vincolato nella UE. Incluso:
  - Bans visibili (`shadowBan: false`).
  - Shadow ban (`shadowBan: true`).
  - Bans con `deleteAllUsersComments: true`.
  - Bans con `banIP: true`.
- Tutte le varianti di ban finiscono nella casella delle approvazioni con il motivo e il livello di confidenza dell'agente; un umano approva o rifiuta.

Gli altri strumenti dell'agente (`mark_comment_spam`, `warn_user`, `lock_comment`, ecc.) **non** sono interessati dall'Articolo 17. Puoi comunque automatizzarli. L'Articolo 17 riguarda specificamente le sospensioni degli utenti.

### E i tenant non UE

Il blocco non si applica fuori dalla regione UE. Puoi comunque decidere di mettere `ban_user` dietro approvazione — lo raccomandiamo fortemente per le prime settimane di vita di qualsiasi agente di moderazione — ma non è imposto.

### Shadow bans

I shadow ban sono considerati sospensioni ai fini dell'Articolo 17 (l'utente può pubblicare ma i suoi contenuti sono nascosti). Sono vincolati nello stesso modo delle sospensioni visibili.

### Rilevamento della regione

La regione è determinata a livello di processo dalla variabile d'ambiente `REGION` nel deployment di FastComments (letta da `isEURegion()` in `models/constants.ts`). Non esiste un campo regione per singolo tenant - il blocco si applica a ogni tenant su un'istanza distribuita nell'UE. Se migri i tuoi dati da un deployment non UE a un deployment UE, il blocco entra in vigore per tutti i tenant su quell'istanza.

### Cosa succede se tutti i revisori sono indisponibili

L'approvazione resterà nella casella fino a quando non viene decisa. Scade automaticamente 90 giorni dopo la creazione. Non esiste un percorso "nessun revisore disponibile, procedi con una decisione automatica" - ciò vanificherebbe lo scopo dell'Articolo 17.

Se la tua community ha un volume così alto che i ban UE non possono essere revisionati in tempi ragionevoli, considera di:

- Aggiungere più revisori (vedi [Approval Notifications](#approval-notifications)).
- Far usare all'agente [`warn_user`](#tool-warn-user) in modo più aggressivo, dato che gli avvertimenti non sono soggetti all'Articolo 17.
- Ridurre l'appetito dell'agente per i ban stringendo le [community guidelines](#community-guidelines) o il [prompt iniziale](#personality-prompt).

### Vedi anche

- [Tool: ban_user](#tool-ban-user) per cosa fa `ban_user` e le opzioni distruttive dietro opt-in aggiuntivi.
- [Approval Workflow](#approval-workflow) per il ciclo di vita completo delle approvazioni.