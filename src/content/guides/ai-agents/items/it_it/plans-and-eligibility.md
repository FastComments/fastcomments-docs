Gli agenti AI sono disponibili nei piani **Flex** e **Pro**. Il piano Creator non li include.

### Limiti a livello di piano

Ogni livello di piano imposta:

- **Limiti di budget giornalieri e mensili predefiniti.** È possibile ridurli per agente; aumentare il limite per account richiede un piano con un tetto più alto. Vedi [Panoramica dei budget](#budgets-overview).

I numeri esatti sono mostrati nella [pagina dei prezzi](https://fastcomments.com/traffic-pricing) e nella pagina di fatturazione del tuo account. Sono inoltre mostrati in linea nel modulo di modifica dell'agente, così non dovrai mai lasciare il modulo per trovare il tuo limite.

FastComments Pro include $200/mese di utilizzo AI. Flex è fatturato al tasso di $20 per milione di token per tutti i modelli (attualmente GLM 5.2 o gpt-oss-120B).

### La fatturazione deve essere valida

Gli agenti AI funzionano solo quando il tenant ha **una fatturazione valida registrata**. Se il metodo di pagamento diventa non valido, tutti gli agenti vengono messi in pausa e la pagina degli agenti AI mostra un banner che indirizza chiunque abbia il ruolo di **Amministratore della fatturazione** ad aggiornare la fatturazione. Gli agenti riprendono autonomamente una volta che la fatturazione è stata ripristinata – nessuna riproduzione o riempimento retroattivo dei trigger attivati durante l'interruzione.

Questo è un requisito rigido: la spesa di token viene fatturata sul tuo account, quindi la piattaforma non invierà alcuna chiamata LLM senza un metodo di pagamento funzionante.

### Chi può gestire gli agenti

Le pagine di amministrazione degli agenti sono protette dal ruolo **Customization Admin** nella dashboard. **Gli amministratori dei moderatori di commenti** possono revisionare e decidere le approvazioni (vedi [Flusso di approvazione](#approval-workflow)) ma non possono creare o modificare gli agenti. **Gli amministratori della fatturazione** ricevono le [email di avviso sul budget](#budget-alerts) indipendentemente dal fatto che abbiano o meno accesso agli agenti.