### ID di Broadcast

Vedrai che in alcune chiamate API devi passare un `broadcastId`. Quando ricevi gli eventi, ti verrà restituito questo ID, così saprai di ignorare l'evento se prevedi di applicare le modifiche in modo ottimistico sul client
(il che probabilmente vorrai fare, poiché offre la migliore esperienza). Passa qui un UUID. L'ID dovrebbe essere abbastanza univoco da non verificarsi due volte in una sessione del browser.

### SSO (Single Sign-On)

Per gli esempi di SSO, vedi sotto.