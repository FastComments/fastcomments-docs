---
### ID-ovi emitiranja

Primijetit ćete da trebate proslijediti `broadcastId` u nekim API pozivima. Kada primite događaje, dobit ćete natrag ovaj ID, pa ćete znati ignorirati događaj ako planirate optimistično primijeniti promjene na klijentu (što ćete vjerojatno htjeti učiniti jer to pruža najbolje iskustvo). Ovdje proslijedite UUID. ID bi trebao biti dovoljno jedinstven da se ne pojavi dvaput u jednoj sesiji.

```swift
let broadcastId = UUID().uuidString
```
---