---
### ID di broadcast

Vedrai che dovrai passare un `broadcastId` in alcune chiamate API. Quando riceverai eventi, otterrai questo ID, così saprai di ignorare l'evento se prevedi di applicare le modifiche in modo ottimistico sul client (cosa che probabilmente vorrai fare poiché offre la migliore esperienza). Passa qui un UUID. L'ID dovrebbe essere abbastanza unico da non comparire due volte in una sessione del browser.

### SSO (Single Sign-On)

Per esempi di SSO, vedi sotto.
---