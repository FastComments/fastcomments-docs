The **Linee guida della comunità** nel campo del modulo di modifica è un blocco di testo di policy opzionale incluso nel messaggio di contesto per il ruolo utente ad ogni esecuzione per questo agente. È delimitato come testo non attendibile (la stessa delimitazione che la piattaforma applica ai corpi dei commenti e ad altri contenuti forniti dagli utenti), quindi il modello lo tratta come riferimento di policy, non come istruzioni di sistema. È il luogo canonico per annotare "quello che è e non è consentito su questo sito" in modo che l'agente lo applichi in modo coerente.

### In cosa differisce dal prompt iniziale

- **Prompt iniziale** - il ruolo dell'agente e lo stile decisionale dell'agente. "Sei un moderatore. Preferisci l'ammonimento al ban."
- **Linee guida della comunità** - le regole della tua comunità, in linguaggio di policy. "Nessun attacco personale. Nessun link promozionale da account con meno di 24 ore. I commenti off-topic possono essere rimossi se una discussione è accesa."

Entrambi confluiscono nella stessa finestra di contesto, ma entrano a livelli diversi - il prompt iniziale fa parte del ruolo di sistema, il documento delle linee guida è testo delimitato dentro il messaggio di contesto del ruolo utente. La separazione rende più semplice l'editing quando vuoi aggiornare uno senza rileggere l'altro.

### Cos'è un buon documento di linee guida

Un documento breve, specifico, scritto da un umano. Le liste funzionano meglio della prosa:

[inline-code-attrs-start title = 'Esempio di linee guida della comunità'; type='text' inline-code-attrs-end]
[inline-code-start]
Allowed:
- Substantive disagreement, even strongly worded.
- Links to original sources, even from new accounts.
- Off-topic asides if the parent thread permits them.

Not allowed:
- Personal attacks against specific named users.
- Doxxing or sharing of private information.
- Coordinated promotional activity (multiple comments pushing the same external link).
- Comments that exist only to derail discussion.

Borderline:
- Strong language without a target. Allowed if not directed at a person.
- Political topics outside the page subject. Off-topic; warn first, don't remove unless persistent.
[inline-code-end]

L'agente applica questo ad ogni esecuzione. Se cambi le linee guida, la modifica entra in vigore al successivo trigger - le esecuzioni passate non vengono rivalutate retroattivamente.

### Cosa non inserire qui

- **Istruzioni di formattazione dell'output** ("rispondi in HTML", "usa emoji"). Quelle appartengono al [prompt iniziale](#personality-prompt).
- **Testo localizzato.** Il documento delle linee guida, come il prompt, è **solo in inglese** per lo stesso motivo: la traduzione automatica può cambiare il comportamento dell'agente in modo silenzioso. Se hai policy che variano per località, scrivile tutte in inglese in questo unico documento e struttura il documento come "per pagine in lingua tedesca: ..."
- **Lunghe citazioni di policy esterne.** Parafrasa. Un contesto lungo costa token ad ogni esecuzione.
- **PII o segreti.** Questo testo viene inviato al fornitore di LLM ad ogni esecuzione.

### Lunghezza

Il campo è limitato a **4000 caratteri** (applicato sia dal modulo che dalla route di salvataggio). Il costo in token ad ogni esecuzione è proporzionale alla lunghezza, quindi anche entro il limite poche centinaia di parole sono di solito sufficienti. Se le policy della tua comunità occupano molte pagine, riassumi le parti che l'agente necessita in un documento specifico per questo campo.

### Versionamento

Non esiste una cronologia di versione incorporata per il documento delle linee guida - l'ultima versione salvata è quella che l'agente usa. Se vuoi una cronologia, copia il documento nel tuo sistema di tracciamento prima di ogni modifica importante. Il flusso [Refine Prompts](#refining-prompts) può registrare le modifiche al *prompt iniziale* ma non tiene versione del documento delle linee guida.