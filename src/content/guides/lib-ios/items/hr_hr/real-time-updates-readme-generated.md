Nakon poziva `sdk.load()`, SDK se automatski pretplaćuje na WebSocket događaje za konfigurirani `urlId`. Sljedeći događaji se obrađuju:

- Novi komentari, izmjene i brisanja
- Glasovi (dodani i uklonjeni)
- Promjene stanja pričvršćivanja (pin), zaključavanja, označavanja (flag) i blokiranja
- Prisustvo korisnika (ulazak/izlazak)
- Otvaranje/zatvaranje teme
- Dodjela znački
- Ažuriranja konfiguracije poslužitelja

### Kontrola prikaza uživo

Prema zadanim postavkama, novi komentari drugih korisnika pojavljuju se odmah:

```swift
sdk.showLiveRightAway = true   // Zadano: prikaži odmah
```

Postavite ovo na `false` da biste nove komentare stavili u međuspremnik iza gumba "N novih komentara", dopuštajući korisniku da odabere kada ih otkriti:

```swift
sdk.showLiveRightAway = false
```

### Prisustvo korisnika

Pokazivači online/offline statusa automatski se pojavljuju na avatarima korisnika kada poslužitelj omogući praćenje prisutnosti. Nije potrebna dodatna konfiguracija na klijentu.

---
---