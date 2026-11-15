I ricarichi (top-up) vengono addebitati sul metodo di pagamento salvato sul tuo account. Se non hai ancora salvato una carta, la pagina Credits mostra `Add a payment method first` con un pulsante `Go to billing`. Salva una carta nella pagina [Billing Info](https://fastcomments.com/auth/my-account/billing-info), poi torna indietro.

Per aggiungere credito:

1. Apri [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info) e clicca `Manage Credits`.
2. Sotto `Add credit`, scegli uno degli importi predefiniti, oppure seleziona `Other amount` e inserisci il tuo importo.
3. Clicca il pulsante `Add`. Mostra l'importo scelto, ad esempio `Add $25.00`.
4. Se la tua banca ti chiede di confermare il pagamento, completa la verifica nella finestra che si apre.

Quando il pagamento va a buon fine, la pagina mostra `Credit added` con il tuo nuovo saldo, e la ricarica appare in `Past top-ups` come `Credited`. Clicca `Add more` per ricaricare di nuovo.

### Importi e Limiti

Gli importi predefiniti sono `10`, `25`, `50` e `100` nella tua valuta di fatturazione. Un importo personalizzato può essere qualsiasi valore da `5` a `10.000` per ricarica, e i limiti esatti per la tua valuta sono mostrati sotto il modulo. Per le valute senza cifre decimali, come lo yen giapponese, i valori predefiniti e i limiti sono cento volte più grandi, in modo da rappresentare all'incirca lo stesso valore.

Le ricariche sono sempre nella valuta di fatturazione del tuo account, la stessa valuta delle tue fatture. La pagina ti indica quale valuta verrà addebitata.

### Conferma Bancaria (3D Secure)

Alcune banche richiedono di confermare un pagamento online, in genere tramite la tua app bancaria o un codice monouso. Se la tua lo richiede, si apre una finestra di conferma dopo aver cliccato `Add`. La tua carta viene addebitata, e il tuo saldo aggiornato, solo dopo l'approvazione della banca.

Se chiudi la pagina prima di completare la conferma, la ricarica viene registrata come `Pending`. La prossima volta che apri la pagina Credits verifichiamo lo stato del pagamento con Stripe e lo contrassegniamo come `Credited` o `Failed` di conseguenza. Se la banca non approva il pagamento, la pagina mostra `Your bank did not approve the payment. Try again, or use a different card.` e non viene addebitato nulla.

### Carte Rifiutate

Se la tua carta viene rifiutata, la pagina mostra `Your card was declined. Check your card on the billing page, or try another card.` Non viene addebitato nulla, il tuo saldo non cambia, e il tentativo è elencato in `Past top-ups` come `Failed`. Aggiorna la carta nella pagina [Billing Info](https://fastcomments.com/auth/my-account/billing-info) e riprova.

Per proteggere la tua carta, un account può effettuare un numero limitato di tentativi di ricarica all'ora. Se vedi `Too many top-up attempts. Wait a while and try again.`, attendi e riprova più tardi.

I pagamenti sono elaborati da Stripe. FastComments non memorizza mai i dettagli della tua carta.