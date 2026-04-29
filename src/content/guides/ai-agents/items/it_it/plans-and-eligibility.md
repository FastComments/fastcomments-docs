AI Agents sono disponibili nei piani **Flex** e **Pro**. Il piano Creator non li include.

### Limiti a livello di piano

Ogni livello di piano stabilisce:

- **Limiti predefiniti di budget giornaliero e mensile.** Puoi ridurli per singolo agente; aumentare il limite per account richiede un piano con un tetto più alto. Vedi [Panoramica dei budget](#budgets-overview).

I numeri esatti sono visualizzati nella [pagina dei prezzi](https://fastcomments.com/traffic-pricing) e nella pagina di fatturazione del tuo account. Sono inoltre mostrati inline nel modulo di modifica dell'agente, così non devi mai lasciare il modulo per trovare il tuo limite.

FastComments Pro include $200/mo di utilizzo AI. Flex viene fatturato al tasso di $20 per milione di token per tutti i modelli (attualmente GLM 5.1 o gpt-oss-120B-turbo).

### La fatturazione deve essere valida

AI Agents vengono eseguiti solo quando il tenant ha **valid billing on file**. Se il metodo di pagamento diventa invalido, tutti gli agenti vengono messi in pausa e la pagina AI Agents mostra un banner che indirizza chi ha il ruolo **Billing Admin** ad aggiornare la fatturazione. Gli agenti riprendono automaticamente una volta ripristinata la fatturazione - non viene effettuata alcuna riproduzione o backfill dei trigger che si sono attivati durante l'interruzione.

Questa è una precondizione rigida: la spesa dei token viene addebitata sul tuo account, quindi la piattaforma non invierà alcuna chiamata LLM senza un metodo di pagamento funzionante.

### Chi può gestire gli agenti

Le pagine di amministrazione degli agenti sono protette dal ruolo di dashboard **Customization Admin**. I **Comment Moderator Admins** possono esaminare e decidere le approvazioni (vedi [Flusso di approvazione](#approval-workflow)) ma non possono creare o modificare gli agenti. I **Billing Admins** ricevono [email di avviso sul budget](#budget-alerts) indipendentemente dal fatto che abbiano accesso agli agenti.