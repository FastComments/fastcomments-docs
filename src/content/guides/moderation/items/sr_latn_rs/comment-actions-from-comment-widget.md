Podskup mera moderacije može se preduzeti direktno iz samog niza komentara, bez potrebe da odete na stranicu Moderacija komentara.

Kada ste prijavljeni, kliknite dugme za uređivanje u gornjem desnom uglu komentara. Kao
moderator, trebalo bi da imate sledeće opcije:

- **Prikači** taj komentar
- **Obriši** taj komentar
- **Obriši** taj komentar + **Zabrani korisnika** (Trajno ili Shadow, više detalja kasnije)
- **Uredi** taj komentar
- **Zaključaj** ili **Otključaj** taj komentar (više detalja dole)
- Obeleži taj komentar kao **Odobren** (prikaži ga) ili **Nije odobren** (sakrij ga)
- Obeleži taj komentar kao **Spam** ili **Nije spam**

### Zaključavanje komentara

Zaključavanje pojedinačnog komentara sprečava bilo kakve nove odgovore na njega, i takođe sprečava da se sam komentar uređuje ili briše dok nije otključan. Ovo važi za sve, uključujući administratore i moderatore. Ako treba da uredite ili uklonite zaključani komentar, prvo ga otključajte, izvršite promenu i po želji ga ponovo zaključajte.

Ikonica katanca pojavljuje se u gornjem desnom uglu zaključanog komentara kako bi čitaoci na prvi pogled videli da je nit zatvorena. Stavke menija Uredi i Obriši su sakrivene za zaključane komentare u widgetu komentara i u javnom API-ju (`PATCH` i `DELETE` vraćaju `code: 'locked'` ako se pozovu na zaključani komentar).

Postoje dve namerne izuzetke koje zaobilaze zaključavanje, jer bi inače ostavile zaostale podatke: kada korisnik obriše čitav nalog (njihovi komentari se čiste bez obzira na stanje zaključavanja), i kada moderator zabrani korisnika sa opcijom "obriši sve komentare od ovog korisnika" (tada se čišćenje vrši uprkos zaključavanjima).

### Zatvaranje niti komentara

Moderatori i administratori mogu zaključati, odnosno zatvoriti, niti komentara izborom `Close Thread` u meniju sa tri tačke na vrhu
sekcije komentara, ako su prijavljeni. Kasnije mogu izabrati `Re-Open Thread` u bilo kom trenutku da ponovo otvore mogućnost komentarisanja.

Zatvaranje niti komentara sprečava nove komentare, ali i dalje dozvoljava glasanje, kao i korisnicima da po potrebi obrišu svoje komentare.

Zatvaranje i ponovno otvaranje niti komentara odmah utiče na sve korisnike koji gledaju nit.

Takođe možete označiti nit kao samo-za-čitanje što uklanja opcije za glasanje i brisanje, kreiranjem pravila prilagođavanja posebno za tu stranicu.

### Ažuriranje uživo

Sve ove akcije će ažurirati niti komentara kod ostalih korisnika odmah, bez potrebe da oni osveže stranicu. Međutim, moderatorske akcije kao što su skrivanje komentara ili označavanje kao spam,
ne uklanjaju komentar sa ekrana **moderatora** tako da, ako je potrebno, mogu brzo opozvati akciju. Da bi naznačili da je komentar
sakriven, biće istaknut u odnosu na ostale komentare (boja isticanja zavisi od razloga uklanjanja).

Na primer, imajući korisnike `A (commenter)`, `B (Moderator 1)`, i `C (Moderator 2)`.

...i sledeći scenario:

1. `User B (Moderator 1)` sakriva komentar.
2. Za `User A (commenter)` taj komentar je odmah sakriven.
3. Za `User C (Moderator 2)` taj komentar je odmah sakriven.
4. Za korisnika koji je izvršio promenu, `User B (Moderator 1)`, komentar ostaje na njegovom ekranu, ali je istaknut kao uklonjen. Ima opciju da opozove svoju akciju, u kom slučaju će ostali korisnici ponovo videti ažuriranje, uživo.