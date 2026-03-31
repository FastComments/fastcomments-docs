Po klicu `sdk.load()`, se SDK samodejno prijavi na WebSocket dogodke za konfiguriran `urlId`. Obdelani so naslednji dogodki:

- Novi komentarji, urejanja in izbris
- Glasovi (novi in odstranjeni)
- Spremembe stanja pripenjanja, zaklepanja, prijave in blokiranja
- Prisotnost uporabnikov (prihod/odhod)
- Odpiranje/zapiranje niti
- Podelitve značk
- Posodobitve konfiguracije strežnika

### Nadzor prikaza v živo

Privzeto se novi komentarji drugih uporabnikov prikažejo takoj:

```swift
sdk.showLiveRightAway = true   // Privzeto: prikaži takoj
```

Nastavite to na `false`, da se novi komentarji zadržijo za gumbom "N novih komentarjev", kar uporabniku omogoča, da izbere, kdaj jih razkrije:

```swift
sdk.showLiveRightAway = false
```

### Prisotnost uporabnikov

Indikatorji "na spletu"/"brez povezave" se samodejno prikažejo na avatarjih uporabnikov, ko strežnik omogoči sledenje prisotnosti. Na odjemalcu ni potrebna dodatna konfiguracija.

---
---