Na het aanroepen van `sdk.load()`, abonneert de SDK zich automatisch op WebSocket-gebeurtenissen voor de geconfigureerde `urlId`. De volgende gebeurtenissen worden afgehandeld:

- Nieuwe reacties, bewerkingen en verwijderingen
- Stemmen (nieuw en verwijderd)
- Wijzigingen in vastzetten, vergrendelen, markeren en blokkeren
- Gebruikersaanwezigheid (aankomst/vertrek)
- Discussiedraad openen/sluiten
- Toekenningen van badges
- Updates van serverconfiguratie

### Liveweergave regelen

Standaard verschijnen nieuwe reacties van andere gebruikers direct:

```swift
sdk.showLiveRightAway = true   // Standaard: direct tonen
```

Stel dit in op `false` om nieuwe reacties achter een knop 'N nieuwe reacties' te bufferen, zodat de gebruiker kan kiezen wanneer ze worden weergegeven:

```swift
sdk.showLiveRightAway = false
```

### Gebruikersaanwezigheid

Online-/offline-indicatoren verschijnen automatisch op gebruikersavatars wanneer de server aanwezigheidstracking inschakelt. Geen extra configuratie is nodig aan de clientzijde.

---
---