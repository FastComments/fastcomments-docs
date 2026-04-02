Efter at have kaldt `sdk.load()`, abonnerer SDK'en automatisk på WebSocket-begivenheder for den konfigurerede `urlId`. Følgende begivenheder håndteres:

- Nye kommentarer, redigeringer og sletninger
- Stemmer (nye og fjernede)
- Ændringer i pin-, låse-, flag- og blokeringstilstande
- Bruger-tilstedeværelse (tilslutning/forladelse)
- Tråd åben/lukket
- Tildeling af badges
- Opdateringer af serverkonfigurationen

### Styring af livevisning

Som standard vises nye kommentarer fra andre brugere med det samme:

```swift
sdk.showLiveRightAway = true   // Standard: vises med det samme
```

Sæt dette til `false` for at bufre nye kommentarer bag en "N nye kommentarer"-knap, så brugeren kan vælge, hvornår de skal vises:

```swift
sdk.showLiveRightAway = false
```

### Bruger-tilstedeværelse

Online-/offlineindikatorer vises automatisk på brugeravatarer, når serveren aktiverer tilstedeværelsessporing. Der kræves ingen yderligere konfiguration på klienten.