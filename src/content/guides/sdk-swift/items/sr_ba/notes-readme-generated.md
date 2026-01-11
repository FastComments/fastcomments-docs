### Broadcast ID-ovi

Primetićete da je potrebno poslati `broadcastId` u nekim API pozivima. Kada primite događaje, dobićete ovaj ID nazad, pa ćete znati da zanemarite događaj ako planirate optimistično primijeniti promjene na klijentu (što ćete vjerovatno htjeti učiniti jer pruža najbolje iskustvo). Pošaljite ovdje UUID. ID bi trebao biti dovoljno jedinstven da se ne pojavi dva puta u istoj sesiji.

```swift
let broadcastId = UUID().uuidString
```