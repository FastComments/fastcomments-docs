La fatturazione è limitata a due tipi di utenti:

- Il `Account Owner`, l'utente che ha creato l'account.
- Qualsiasi utente con il permesso `Billing Admin`.

Sono gli unici utenti che possono cambiare il piano, salvare o sostituire una carta, aprire `Invoice Settings`, impostare i limiti di spesa Flex e aggiungere credito. Chiunque altro tenti vede `Only the account owner or a billing admin can change the plan or payment method.`

Le email di fatturazione, come il promemoria a metà prova, gli avvisi di pagamento fallito e le notifiche di disattivazione, vengono inviate agli utenti con il permesso `Billing Admin`. Concedi il permesso a chiunque debba riceverle, ad esempio a qualcuno in finanza, e riceverà le email senza bisogno di alcun altro accesso alla dashboard. I promemoria inviati dopo che una prova è terminata senza informazioni di fatturazione vanno a ogni utente della dashboard dell'account. Ognuna di queste email ha un link per annullare l'iscrizione; un utente che rinuncia alle notifiche dell'account smette di riceverle.

Le fatture stesse vanno al `Billing Email` in [Invoice Settings](#invoice-settings), che non deve necessariamente appartenere a un utente della dashboard.

La pagina [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) richiede invece il permesso `Analytics Admin`, poiché mostra l'utilizzo anziché i dettagli dei pagamenti.

Per concedere o rimuovere il permesso, apri la pagina `Users` e modifica l'utente. Vedi [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions). Su Flex, ricorda che ogni amministratore aggiuntivo è un posto fatturato, vedi [Admin and Moderator Seats](#admin-and-moderator-seats).

Se sei connesso e non riesci a vedere `Billing Info` affatto, o ti rimanda alla home page, sei connesso con un account commentatore anziché un utente della dashboard. Esci e accedi con l'account che possiede il sito.