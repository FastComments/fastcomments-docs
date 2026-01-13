### ID di Broadcast

Vedrai che dovrai passare un `broadcastId` in alcune chiamate API. Quando ricevi eventi, otterrai questo ID indietro, così saprai di ignorare l'evento se prevedi di applicare in modo ottimistico le modifiche lato client (cosa che probabilmente vorrai fare poiché offre la migliore esperienza). Inserisci qui un `UUID`. L'ID dovrebbe essere sufficientemente unico da non presentarsi due volte in una sessione del browser.

### SSO (Single Sign-On)

Per esempi di SSO, vedi sotto.