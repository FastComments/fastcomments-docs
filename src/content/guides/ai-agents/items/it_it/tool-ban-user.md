Lo strumento di ban è l'azione più incisiva che un agente può eseguire. Bannisce un utente dalla tua community, con una durata fissa e alcune opzioni.

### Cosa fa

L'agente sceglie una delle sei durate:

- Un'ora
- Un giorno
- Una settimana
- Un mese
- Sei mesi
- Un anno

Seleziona inoltre tra un **ban visibile** (l'utente vede un messaggio di ban chiaro e può fare ricorso) e un **shadow ban** (l'utente può continuare a pubblicare ma i suoi contenuti sono nascosti agli altri utenti). Le istruzioni della piattaforma indicano all'agente di preferire i ban visibili per i casi alla prima infrazione o borderline, e gli shadow ban per recidivi chiaramente maligni.

### Le due sotto-opzioni distruttive

Due opzioni aggiuntive sono **nascoste all'agente per impostazione predefinita**. Per abilitarne una, seleziona la casella corrispondente nella sezione **Ban options** del modulo di modifica dell'agente:

- **Allow deleting all of the user's comments.** Quando abilitata, l'agente può scegliere di eliminare anche ogni commento che l'utente bannato abbia mai pubblicato nel tuo tenant. Riservare per spam evidente, doxxing o abuso coordinato in cui i contenuti esistenti non hanno valore. **Distruttivo e irreversibile.**
- **Allow banning by IP.** Quando abilitata, l'agente può scegliere di bannare anche l'indirizzo IP da cui è stato pubblicato il commento. Utile contro l'evasione del ban con account alternativi. **Evitare per IP condivisi** (aziendali, scolastici, provider mobili) - utenti innocenti sulla stessa rete saranno bloccati.

La piattaforma applica inoltre dei vincoli lato server: anche se l'agente dovesse comportarsi in modo scorretto e tentare di attivare l'opzione, la richiesta viene rifiutata a meno che non abbiate esplicitamente attivato l'opzione.

### Politica di escalation

Prima di bannare, la piattaforma istruisce l'agente a:

1. Cercare nella [memoria dell'agente](#agent-memory-system) eventuali avvertimenti o annotazioni sull'utente.
2. Preferire di [avvertire](#tool-warn-user) l'utente invece del ban per le prime infrazioni.
3. Saltare il passaggio dell'avvertimento solo per casi chiaramente gravi (contenuti illegali, doxxing, spam coordinato) - e spiegare il motivo nella sua giustificazione.

Questa politica è nelle istruzioni dell'agente, non una regola vincolante lato server, ed è per questo che si raccomanda vivamente di **porre i ban sotto approvazione**.

### Regione UE: approvazione umana richiesta

Nella regione UE, questo strumento è **bloccato per l'approvazione** dall'articolo 17 del Digital Services Act. Ogni ban da qualsiasi agente su un tenant della regione UE finisce nella [casella di approvazioni](#approval-workflow) per la revisione umana. Vedi [Conformità all'articolo 17 del DSA UE](#eu-dsa-compliance).

### Raccomandazioni

- Richiedere approvazione ovunque per almeno il primo mese.
- Bloccare sempre l'opzione **delete-all-comments** se la abiliti - è irreversibile.
- Valuta di porre sotto approvazione anche l'opzione **IP ban** anche dopo che l'agente si è guadagnato fiducia - il costo di un IP ban su una rete condivisa non compare nella cronologia delle esecuzioni dell'agente.

### Vedi anche

- [Banning Users](/guide-moderation.html#banning-users) e [Banning Users With Wildcards](/guide-moderation.html#banning-users-wildcards) nella guida alla moderazione per come funzionano i ban a livello di piattaforma.
- [Avvertire l'utente](#tool-warn-user) - il passo di escalation più leggero.