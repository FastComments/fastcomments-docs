### Broadcast identifikatori

Videćete da treba da pošaljete `broadcastId` u nekim API pozivima. Kada primite događaje, dobićete ovaj ID nazad, tako da znate da ignorišete događaj ako planirate optimistično primeniti izmene na klijentu (što ćete verovatno želeti da uradite jer to pruža najbolje iskustvo). Pošaljite ovde UUID. ID treba da bude dovoljno jedinstven da se ne ponovi dva puta u jednoj sesiji.

```swift
let broadcastId = UUID().uuidString
```