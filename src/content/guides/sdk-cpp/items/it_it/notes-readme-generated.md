### ID di broadcast

Vedrai che dovrai passare un `broadcastId` in alcune chiamate API. Quando ricevi eventi, riceverai indietro questo ID, così saprai di ignorare l'evento se prevedi di applicare ottimisticamente le modifiche sul client
(il che probabilmente vorrai fare, poiché offre la migliore esperienza). Passa qui un UUID. L'ID dovrebbe essere sufficientemente unico da non comparire due volte in una sessione del browser.

### SSO (Accesso unico)

Per esempi di SSO, vedi sotto.