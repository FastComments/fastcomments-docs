### ID di broadcast

Vedrai che devi passare un `broadcastId` in alcune chiamate API. Quando ricevi eventi, ti verrà restituito questo ID, così saprai di ignorare l'evento se prevedi di applicare le modifiche in modo ottimistico sul client (cosa che probabilmente vorrai fare, poiché offre la migliore esperienza). Passa qui un UUID. L'ID dovrebbe essere sufficientemente unico da non comparire due volte in una sessione.

```swift
let broadcastId = UUID().uuidString
```