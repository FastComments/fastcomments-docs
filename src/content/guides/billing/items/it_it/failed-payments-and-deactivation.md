Un pagamento può fallire perché una carta è scaduta, è stata sostituita, non ha fondi o la banca ha bloccato l'addebito. FastComments ti dà il tempo di sistemare la cosa prima che qualcosa smetta di funzionare, e avvisa le persone che possono risolvere il problema.

### Cosa Succede

1. **L'addebito fallisce.** La fattura rimane `Unpaid` nella [Cronologia Fatturazione](https://fastcomments.com/auth/my-account/billing-history) con un pulsante `Pay Now`. Su un piano fisso Stripe può riprovare l'addebito. Su Flex, FastComments riprova ogni notte le fatture non pagate più vecchie del periodo di grazia.
2. **Viene inviato un avviso.** Ogni notte FastComments controlla ogni account. Se una fattura Flex è ancora non pagata oltre il periodo di grazia e il nuovo tentativo fallisce, oppure un abbonamento a piano fisso è terminato perché non è stato possibile incassarlo, ogni utente con il permesso `Billing Admin` riceve l'email `Your FastComments Tenant Will Be Deactivated Soon!` con la data, e l'account viene programmato per la disattivazione un periodo di grazia dopo. Nel frattempo il widget continua a funzionare.
3. **Disattivazione.** Se alla data programmata non è cambiato nulla, l'account viene disattivato: i commenti vengono nascosti sul tuo sito e i visitatori vedono `This FastComments account's billing information is more than sixty days out of date. Comments hidden until the issue is resolved.` Gli amministratori di fatturazione ricevono di nuovo un'email. I tuoi dati vengono conservati.

Il periodo di grazia è di una settimana per la maggior parte degli account e di due settimane per gli account più vecchi di un anno. Su Flex si applica due volte, una prima dell'avviso e una tra l'avviso e la disattivazione, quindi passano almeno due settimane tra un addebito fallito e la visualizzazione nascosta dei commenti.

### Come Sistemarlo

Salva una carta funzionante nella pagina [Informazioni Fatturazione](https://fastcomments.com/auth/my-account/billing-info). Questo da solo riattiva immediatamente l'account e annulla qualsiasi disattivazione programmata; la fattura non pagata viene poi incassata con la nuova carta, immediatamente su un piano fisso e durante il controllo notturno su Flex. Puoi anche pagare la fattura subito con `Pay Now` nella Cronologia Fatturazione.

Se la carta registrata va bene e il fallimento era temporaneo, non devi fare nulla: il controllo notturno rileva la fattura pagata o l'abbonamento attivo, annulla la disattivazione e invia un'email agli amministratori di fatturazione per conferma. Un account già disattivato viene riattivato allo stesso modo.

### Diverse Fatture Insieme

Se la tua carta non era valida per alcuni mesi e poi è stata sistemata, ogni mese non pagato viene incassato separatamente, quindi potresti vedere diversi addebiti nello stesso giorno. Controlla il periodo di fatturazione su ogni fattura nella Cronologia Fatturazione: si riferiscono a mesi diversi, non sono duplicati.

### Vedi Ancora Un Avviso Di Fatturazione Dopo Il Pagamento?

L'avviso appartiene a un account specifico. Se hai pagato ma un sito mostra ancora `billing information is out of date`, il widget su quel sito molto probabilmente sta usando un account diverso da quello su cui hai pagato. Questo accade quando un sito è stato configurato due volte, o quando esiste un account su `fastcomments.com` e un altro sul deployment UE a `eu.fastcomments.com`, che vengono fatturati separatamente. Confronta il `tenantId` nel codice della tua pagina con quello sotto `Settings -> API/SSO`, oppure [contatta l'assistenza](https://fastcomments.com/auth/my-account/help) e sistemeremo tutto.