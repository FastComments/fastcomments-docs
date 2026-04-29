Si attiva quando il conteggio netto dei voti di un commento raggiunge la soglia configurata. Il numero netto di voti è `votesUp - votesDown`.

### Configurazione richiesta

- **Vote threshold** - intero >= 1. Il trigger si attiva con il voto che porta i voti netti esattamente a questo valore.

Se la soglia è 10 e un commento passa da 9 a 10 voti netti, il trigger si attiva una volta. Se un voto poi lo porta da 10 a 11, il trigger **non** si attiva di nuovo - non si riattiva per ogni voto aggiuntivo oltre la soglia.

### Contesto che l'agente riceve

- Il commento, con i conteggi dei voti correnti.
- La **direzione del voto** (`up` o `down`) del voto che ha provocato il superamento della soglia.
- Eventuale cronologia del thread / dell'utente / contesto della pagina come configurato.

### Note importanti

- Un commento che sale a 10, scende a 9 e risale a 10 attiverà il trigger due volte. Non esiste uno stato per commento "fired once" - se ti servono tali semantiche, fai sì che l'agente registri una [nota di memoria](#tools-overview) alla prima esecuzione e la verifichi nelle esecuzioni successive.
- La soglia è sempre un conteggio di voti **netto**, non i soli upvote grezzi. Un commento con 12 up e 2 down ha net 10 e attiva il trigger; uno con 10 up e 0 down lo attiva anche.
- Sono possibili attraversamenti dovuti solo a down-vote - un commento che passa da 11 a 10 a causa di un down-vote attiva anch'esso il trigger. Il parametro `voteDirection` nel contesto indica all'agente da quale direzione è avvenuto il superamento della soglia.

### Utilizzi comuni

- **Pinning** - il [Top Comment Pinner template](#template-top-comment-pinner) è costruito attorno a questo trigger.
- **Promozione / workflow per commenti in evidenza** - emetti un evento tramite [Webhooks](#webhooks-overview) in modo che un sistema esterno possa promuovere il commento altrove nel tuo sito.
- **Tracciamento dell'engagement** - registra una nota di memoria sull'utente che ha scritto il commento in modo che altri agenti sappiano che ha prodotto contenuti popolari.

### Regolazione

La soglia giusta dipende dalla comunità. Osserva [Cronologia delle esecuzioni](#run-history) per alcuni giorni con una soglia bassa (5) per vedere con quale frequenza si attiva. Aumenta la soglia finché la frequenza di attivazione non corrisponde al ritmo che desideri effettivamente.