Per impostazione predefinita, un agente viene eseguito su tutto il tenant - ogni pagina, ogni locale. Le sezioni **Ambito** e **Locali** nel modulo di modifica consentono di limitarne l'ambito.

### Restringi a pagine specifiche

Il campo **Restringi a pagine specifiche** accetta un pattern URL per riga, nella url-pattern glob syntax. L'agente viene eseguito solo sui commenti la cui URL della pagina corrisponde ad almeno uno dei pattern. Esempi:

- `/news/*` - qualsiasi pagina sotto `/news`.
- `/forums/*` - qualsiasi pagina sotto `/forums`.
- `/blog/2026/*` - qualsiasi pagina sotto `/blog/2026`.
- (più righe insieme) - l'agente viene eseguito se **qualsiasi** pattern corrisponde.

Massimo: 50 pattern per agente. I pattern devono essere url-pattern globs validi - il modulo rifiuta quelli malformati con un errore specifico.

Quando il campo è **vuoto**, l'agente viene eseguito su ogni pagina del tenant.

Quando il campo è **non vuoto**, l'agente fallisce in modalità chiusa: qualsiasi trigger il cui commento non abbia `urlId` (es. eventi a livello tenant senza contesto di pagina) viene saltato. Questo è voluto - "scoped to /news/*" non dovrebbe silenziosamente ricadere su "tutto".

### Restringi a locali specifici

Il selettore a doppia lista **Restringi a locali specifici** accetta gli ID di locale FastComments (`en_us`, `zh_cn`, `de_de`, ecc.). L'agente viene eseguito solo sui commenti il cui locale rilevato è nella lista selezionata.

Il locale rilevato proviene dal campo `locale` del commento, che viene impostato dal widget dei commenti al momento della pubblicazione in base al locale della pagina.

Quando **nessun locale** è selezionato, l'agente viene eseguito su tutti i locali.

Quando **uno o più locali** sono selezionati, l'agente fallisce in modalità chiusa: i trigger senza commento, o i trigger su commenti senza il campo `locale`, vengono saltati.

### Scoping combinato

I filtri URL e locale si combinano con un AND. Un trigger attiva l'agente solo se **entrambi** i filtri lo permettono.

Esempi utili:
- pattern URL `/news/*` + locale `en_us` - solo la sezione notizie in inglese.
- Nessun filtro URL + più locali - a livello tenant, ma solo per le lingue per cui il prompt dell'agente è stato scritto.

### Perché limitare l'ambito di un agente

- **Costo.** Limitare l'ambito riduce il volume di trigger che l'agente deve processare, e quindi riduce la spesa in token.
- **Correttezza.** Un prompt per riassunti tarato su articoli tecnici può produrre output di scarsa qualità sulle pagine prodotto. Limitare l'ambito è uno strumento più preciso che chiedere al prompt di "saltare le pagine non tecniche" in inglese.
- **Comportamento specifico per locale.** Un messaggio di benvenuto che scrive solo in tedesco dovrebbe essere eseguito soltanto sui commenti con locale tedesco. Combina lo scope `de_de` con un tono in lingua tedesca nell'[initial prompt](#personality-prompt).

### Cosa la limitazione dell'ambito *non* fa

- Non cambia il **numero di slot per agente** (vedi [Plans and Eligibility](#plans-and-eligibility)) - un agente con scope continua a occupare uno slot.
- Non cambia i [Budget caps](#budgets-overview) - i limiti giornalieri e mensili per agente si applicano a tutti i trigger corrispondenti.
- Non retroagisce sulle esecuzioni passate - la cronologia delle esecuzioni mostra tutto ciò che l'agente ha fatto, anche se in seguito ne limiti maggiormente l'ambito.