Podskup akcija moderiranja može se izvršiti izravno iz same niti komentara, bez potrebe za odlaskom na stranicu Comment Moderation.

Kada ste prijavljeni, kliknite gumb za uređivanje u gornjem desnom kutu komentara. Kao moderator trebali biste imati sljedeće opcije:

- **Prikvačiti** (Pin) taj komentar
- **Izbrisati** taj komentar
- **Izbrisati** taj komentar + **BANIRATI korisnika** (Trajno ili Shadow, više detalja kasnije)
- **Urediti** taj komentar
- **Zaključati** ili **Otključati** taj komentar (više detalja u nastavku)
- Označiti taj komentar kao **Odobren** (prikazati ga) ili **Nije odobren** (sakriti ga)
- Označiti taj komentar kao **Spam** ili **Nije spam**

### Zaključavanje komentara

Zaključavanje pojedinačnog komentara sprječava sve nove odgovore na njega, a također sprječava uređivanje ili brisanje samog komentara dok se ne otključa. To vrijedi za sve, uključujući administratore i moderatore. Ako trebate urediti ili ukloniti zaključani komentar, prvo ga otključajte, napravite promjenu i po potrebi ga ponovno zaključajte.

Ikona bravice pojavljuje se u gornjem desnom kutu zaključanog komentara kako bi čitatelji odmah vidjeli da je nit zatvorena. Stavke izbornika Uredi i Izbriši skrivene su za zaključane komentare i u widgetu komentara i u javnom API-ju (`PATCH` i `DELETE` vraćaju `code: 'locked'` ako se pozovu na zaključani komentar).

Dvije namjerne iznimke zaobilaze zaključavanje, jer bi inače ostavili osirotele podatke: kada korisnik izbriše cijeli svoj račun (njegovi komentari se čiste bez obzira na stanje zaključavanja) i kada moderator banira korisnika s opcijom "izbriši sve komentare ovog korisnika" (čišćenje uklanja zaključavanja).

### Zatvaranje niti komentara

Moderatori i administratori mogu zaključati, odnosno zatvoriti, niti komentara odabirom `Close Thread` u izborniku s tri točke na vrhu područja komentara, ako su prijavljeni. Kasnije, u bilo koje vrijeme, mogu odabrati `Re-Open Thread` kako bi ponovno otvorili komentiranje.

Zatvaranje niti komentara sprječava nove komentare, ali i dalje dopušta glasovanje i brisanje vlastitih komentara ako korisnici to žele.

Zatvaranje i ponovno otvaranje niti komentara odmah utječe na sve korisnike koji gledaju nit.

Također možete označiti nit kao samo za čitanje, što uklanja i opcije glasovanja i brisanja, stvaranjem pravila prilagobe posebno za tu stranicu.

### Ažuriranje u stvarnom vremenu

Sve ove akcije odmah će ažurirati niti komentara drugih korisnika bez potrebe da ponovno učitaju stranicu. Međutim, moderatorske akcije poput skrivanja komentara ili označavanja kao spam ne uklanjaju komentar sa **ekrana moderatora** kako bi, ako je potrebno, mogao brzo poništiti akciju. Kako bi se označilo da je komentar skriven, bit će istaknut u usporedbi s ostalim komentarima (boja isticanja ovisi o razlogu uklanjanja).

Na primjer, uz korisnike `A (commenter)`, `B (Moderator 1)` i `C (Moderator 2)`.

...i sljedeći scenarij:

1. `User B (Moderator 1)` skriva komentar.
2. Za `User A (commenter)` taj komentar je odmah skriven.
3. Za `User C (Moderator 2)` taj komentar je odmah skriven.
4. Za korisnika koji je napravio promjenu, `User B (Moderator 1)`, komentar ostaje na njegovom ekranu, ali je istaknut kao uklonjen. Imaju opciju poništiti svoju akciju, u kojem slučaju će drugi korisnici ponovno vidjeti ažuriranje u stvarnom vremenu.