### Broadcast ID-ovi

Primijetićete da je potrebno proslijediti `broadcastId` u nekim API pozivima. Kada primite događaje, dobićete ovaj ID nazad, pa ćete znati da zanemarite događaj ako planirate optimistično primijeniti promjene na klijentu (što ćete vjerovatno htjeti uraditi jer pruža najbolje korisničko iskustvo). Ovdje proslijedite UUID. ID treba biti dovoljno jedinstven da se ne pojavi dva puta u istoj sesiji.

```swift
let broadcastId = UUID().uuidString
```