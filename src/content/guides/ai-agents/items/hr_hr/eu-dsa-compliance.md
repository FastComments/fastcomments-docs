FastComments provodi članak 17. EU Zakona o digitalnim uslugama za zakupce u EU regiji: **potpuno automatizirane suspenzije korisnika nisu dopuštene**.

### Što to znači u praksi

When your tenant is in the EU region, on the agent edit form:

- The **Approvals** checkbox for `ban_user` is **locked on** and cannot be unticked.
- The label reads: "EU DSA Article 17: user suspensions require human review. 'Ban a user' is locked on and cannot be fully automated in the EU region."
- A tooltip on the approval column reads: "Locked on by EU DSA Article 17 - fully-automated bans are not permitted in the EU region."

Što god drugo konfigurirali, svaki poziv `ban_user` od bilo kojeg agenta na zakupcu u EU regiji ide u [pretinac odobrenja](#approval-workflow) na ljudsku reviziju. Suspenzija se ne izvršava dok je osoba ne odobri.

### Zašto se ovo provodi na razini platforme, a ne na razini prompta

Sistemske upute (system prompts) mogu biti ignorirane ili zaobiđene od strane dovoljno nesavjesnog modela. Usklađenost s člankom 17. previše je važna da bi se oslanjala na dobro ponašanje modela; mora postojati čvrsta provjera na strani servera koju sam dispatcher alata provodi. To je ono što radimo.

### Što prolazi kroz odobrenje, a što ne

- **`ban_user`**: uvijek je pod kontrolom u EU. Uključujući:
  - Vidljive zabrane (`shadowBan: false`).
  - Skrivene zabrane (`shadowBan: true`).
  - Zabrane s `deleteAllUsersComments: true`.
  - Zabrane s `banIP: true`.
- Sve varijante zabrana završavaju u pretincu odobrenja s objašnjenjem agenta i stupnjem povjerenja; osoba odobrava ili odbija.

Ostali alati agenta (`mark_comment_spam`, `warn_user`, `lock_comment`, itd.) **nisu** pogođeni člankom 17. I dalje ih možete automatizirati. Članak 17. se odnosi isključivo na suspenzije korisnika.

### A što je s zakupcima izvan EU

Zaključavanje ne vrijedi izvan EU regije. Ipak možete odlučiti staviti `ban_user` iza odobrenja — to snažno preporučujemo tijekom prvih nekoliko tjedana rada bilo kojeg moderacijskog agenta — ali to nije prisilno.

### Skrivene zabrane

Skrivene zabrane se računaju kao suspenzije za potrebe članka 17. (korisnik može objavljivati, ali njihov sadržaj je skriven). One su ograničene na isti način kao i vidljive zabrane.

### Detekcija regije

Regija se određuje na razini procesa pomoću okruženjske varijable `REGION` na FastComments deploymentu (čitano od `isEURegion()` u `models/constants.ts`). Ne postoji polje regije po zakupcu - zaključavanje se primjenjuje na svakog zakupca na instanci raspoređenoj u EU. Ako migrirate svoje podatke s deploymenta izvan EU na deployment u EU, zaključavanje stupa na snagu za sve zakupce na toj instanci.

### Što ako svi pregledatelji nisu dostupni

Odobrenje će ostati u pretincu dok se ne donese odluka. Automatski istječe 90 dana nakon stvaranja. Ne postoji put "nema dostupnog recenzenta, prelazak na automatiziranu odluku" — to bi poništilo smisao članka 17.

Ako je vaša zajednica toliko velike količine da se EU zabrane ne mogu pregledati u razumnom roku, razmislite o:

- Dodavanju više pregledatelja (pogledajte [Obavijesti o odobrenju](#approval-notifications)).
- Prebacivanju agenta da agresivnije koristi [`warn_user`](#tool-warn-user), jer upozorenja nisu podložna članku 17.
- Smanjenju sklonosti agenta prema zabrani pooštravanjem [smjernica zajednice](#community-guidelines) ili [početnog prompta](#personality-prompt).

### Pogledajte također

- [Alat: ban_user](#tool-ban-user) za ono što `ban_user` radi i destruktivne opcije koje zahtijevaju dodatne pristanke.
- [Tijek odobravanja](#approval-workflow) za cijeli životni ciklus odobrenja.