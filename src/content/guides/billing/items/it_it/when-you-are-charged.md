L'importo addebitato sulla tua carta e il momento dell'addebito dipendono dal tipo di piano a cui sei iscritto.

### Piani Fissi

Quando avvii un abbonamento `Creators` o `Pro`, il primo mese o anno viene addebitato immediatamente, nel browser, quando fai clic su `Pay Now & Start Subscription`. Se la tua banca ti chiede di confermare il pagamento, completa la verifica nella finestra che si apre. L'abbonamento inizia solo dopo che il pagamento è andato a buon fine.

Successivamente, l'abbonamento si rinnova lo stesso giorno di ogni mese, o di ogni anno se hai scelto `Pay Annually`, a partire dal giorno dell'iscrizione. Il rinnovo viene addebitato automaticamente sulla carta registrata e la fattura viene inviata via email al tuo indirizzo di fatturazione. Non è richiesta alcuna azione.

La fatturazione annuale è il 15% più economica di quella mensile. Flex non può essere fatturato annualmente.

### Flex

Flex viene fatturato a posteriori: utilizzi il servizio durante un mese solare e la fattura per quel mese viene creata l'ultimo giorno del mese (UTC) e addebitata subito sulla tua carta. La data di fatturazione è sempre l'ultimo giorno del mese e non può essere modificata.

La fattura copre solo quel mese solare. L'utilizzo nel mese solare in cui ti sei iscritto non viene mai fatturato, quindi la tua prima fattura arriva al più presto alla fine del mese successivo all'iscrizione.

Se il tuo utilizzo è inferiore al minimo Flex, una voce con la dicitura `FastComments Flex. (minimum ... /mo)` porta la fattura fino all'importo minimo. Il minimo attuale è mostrato nella tua pagina [Billing Info](https://fastcomments.com/auth/my-account/billing-info) e nella [pagina dei prezzi](https://fastcomments.com/traffic-pricing).

Puoi vedere in qualsiasi momento quale sarà la prossima fattura. L'importo `Next Invoice Amount` è nella pagina [Billing Info](https://fastcomments.com/auth/my-account/billing-info), e il dettaglio completo voce per voce è nella pagina [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing). Ciò che viene conteggiato è descritto in [How Flex Usage Is Measured](#how-flex-usage-is-measured).

### Il Salvataggio di una Carta Non La Addebita

Una volta configurata la fatturazione, salvare o aggiornare una carta senza cambiare piano non la addebita mai, a meno che tu non abbia una fattura non pagata. Il modulo indica `You won't be charged now unless you change plans.` e la carta viene semplicemente memorizzata con Stripe per la prossima fattura. Solo quando non hai ancora una fatturazione valida il modulo indica `Your card will be charged now.`, e anche in quel caso un account Flex non viene addebitato fino alla fine del mese. Qualsiasi [credito](#credits) prepagato sul tuo account viene applicato a ogni fattura prima dell'addebito sulla carta.

### Sub-Account Gestiti

Se il tuo account gestisce sub-account (ad esempio uno per sito o ambiente, o tenant white-labeled), viene fatturato solo l'account principale. L'utilizzo di ogni sub-account appare come voci proprie nella fattura Flex dell'account principale, precedute dal nome del sub-account, e i sub-account condividono il metodo di pagamento e lo stato di fatturazione dell'account principale.