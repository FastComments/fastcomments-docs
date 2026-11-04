FastComments applica l'Articolo 17 dell'EU Digital Services Act per i tenant nella regione UE: **le sospensioni degli utenti completamente automatizzate non sono consentite**.

### Cosa significa in pratica

Quando il tuo tenant si trova nella regione UE, nel modulo di modifica dell'agente:

- La casella di controllo **Approvals** per `ban_user` è **bloccata su attivo** e non può essere deselezionata.
- L'etichetta recita: "EU DSA Article 17: user suspensions require human review. 'Ban a user' is locked on and cannot be fully automated in the EU region."
- Un tooltip sulla colonna di approvazione recita: "Locked on by EU DSA Article 17 - fully-automated bans are not permitted in the EU region."

Indipendentemente da ciò che configuri, ogni chiamata a `ban_user` da parte di qualsiasi agente su un tenant nella regione UE va alla [cassetta degli approvazioni](#approval-workflow) per la revisione umana. Il ban non avviene finché un essere umano non lo approva.

### Perché questo è applicato a livello di piattaforma, non a livello di prompt

I prompt di sistema possono essere ignorati o aggirati da un modello sufficientemente malfunzionante. La conformità all'Articolo 17 è troppo importante per affidarsi al buon comportamento del modello; deve essere un blocco rigido lato server che il dispatcher degli strumenti stesso applica. Ed è esattamente ciò che facciamo.

### Cosa passa e cosa non passa attraverso l'approvazione

- **`ban_user`**: sempre soggetto a blocco nell'UE. Incluso:
  - Ban visibili (`shadowBan: false`).
  - Shadow ban (`shadowBan: true`).
  - Ban con `deleteAllUsersComments: true`.
  - Ban con `banIP: true`.
- Tutte le varianti di ban finiscono nella cassetta degli approvazioni con il ragionamento e la confidenza dell'agente; un essere umano approva o rifiuta.

Gli altri strumenti dell'agente (`mark_comment_spam`, `warn_user`, `lock_comment`, ecc.) **non** sono interessati dall'Articolo 17. Puoi comunque automatizzarli. L'Articolo 17 riguarda specificamente le sospensioni degli utenti.

### E per i tenant non UE

Il blocco non si applica al di fuori della regione UE. Puoi comunque scegliere di sottoporre `ban_user` ad approvazione - lo raccomandiamo vivamente per le prime settimane di vita di qualsiasi agente di moderazione - ma non è applicato obbligatoriamente.

### Shadow ban

Gli shadow ban contano come sospensioni ai fini dell'Articolo 17 (l'utente può pubblicare ma il suo contenuto è nascosto). Sono soggetti a blocco in modo identico ai ban visibili.

### Rilevamento della regione

La regione è determinata a livello di processo dalla variabile d'ambiente `REGION` sul deployment FastComments (letta da `isEURegion()` in `models/constants.ts`). Non esiste un campo regione per singolo tenant - il blocco si applica a ogni tenant su un'istanza deployata nell'UE. Se migri i tuoi dati da un deployment non UE a un deployment UE, il blocco entra in vigore per tutti i tenant su quell'istanza.

### E se tutti i revisori non sono disponibili

L'approvazione rimarrà nella cassetta finché non viene decisa. Scade automaticamente 90 giorni dopo la creazione. Non esiste un percorso "nessun revisore disponibile, si procede con la decisione automatizzata" - ciò vanificherebbe lo scopo dell'Articolo 17.

Se la tua community ha un volume così elevato che i ban nell'UE non possono essere revisionati in tempi ragionevoli, considera di:

- Aggiungere più revisori (vedi [Notifiche di approvazione](#approval-notifications)).
- Passare l'agente a un uso più aggressivo di [`warn_user`](#tool-warn-user), poiché gli avvertimenti non sono soggetti all'Articolo 17.
- Ridurre la propensione dell'agente al ban restringendo le [linee guida della community](#community-guidelines) o il [prompt iniziale](#personality-prompt).

### Vedi anche

- [Strumento: ban_user](#tool-ban-user) per cosa fa `ban_user` e le opzioni distruttive dietro opt-in aggiuntivi.
- [Flusso di approvazione](#approval-workflow) per l'intero ciclo di vita dell'approvazione.