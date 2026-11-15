La tabella `Past top-ups` in fondo alla pagina Credits elenca i tuoi ricariche più recenti, dalla più recente, con `Date`, `Amount` e `Status` di ciascuna. Prima della tua prima ricarica mostra `No top-ups yet.`

Ogni ricarica ha uno di tre stati:

- `Credited`: il pagamento è andato a buon fine e l'importo è stato aggiunto al tuo saldo.
- `Pending`: il pagamento è stato avviato ma non è stato completato, di solito perché la conferma bancaria non è stata completata. Le ricariche in sospeso vengono verificate su Stripe alla successiva apertura della pagina Credits e passano a `Credited` o `Failed`.
- `Failed`: la carta è stata rifiutata o la banca non ha approvato il pagamento. Nessun addebito è stato effettuato e il tuo saldo non è cambiato.

La tabella registra le ricariche effettuate da questa pagina. Le fatture pagate con credito sono mostrate insieme alle fatture stesse, sotto `View Invoices` nella pagina [Billing Info](https://fastcomments.com/auth/my-account/billing-info).