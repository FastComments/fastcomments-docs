---
Nakon poziva `sdk.load()`, SDK se automatski pretplaćuje na WebSocket događaje za konfigurisani `urlId`. Sledeći događaji se obrađuju:

- Novi komentari, izmene i brisanja
- Glasovi (novi i uklonjeni)
- Promene stanja pin, lock, flag i block
- Prisutnost korisnika (pridruživanje/napuštanje)
- Otvaranje/zatvaranje thread-a
- Dodeljivanje bedževa
- Ažuriranja konfiguracije servera

### Kontrola prikaza uživo

Podrazumevano, novi komentari drugih korisnika pojavljuju se odmah:

```swift
sdk.showLiveRightAway = true   // Podrazumevano: prikaži odmah
```

Postavite ovo na `false` da biste keširali nove komentare iza dugmeta "N novih komentara", dopuštajući korisniku da izabere kada će ih prikazati:

```swift
sdk.showLiveRightAway = false
```

### Prisutnost korisnika

Indikatori online/offline se automatski pojavljuju na avatarima korisnika kada server omogući praćenje prisustva. Na klijentskoj strani nije potrebna dodatna konfiguracija.

---
---