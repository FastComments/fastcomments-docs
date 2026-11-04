FastComments provodi Članak 17. EU akta o digitalnim uslugama za stanare u EU regiji: **potpuno automatizirane suspenzije korisnika nisu dopuštene**.

### Što to znači u praksi

Kada je vaš stanar u EU regiji, na obrascu za uređivanje agenta:

- Potvrdni okvir **Approvals** za `ban_user` je **zaključan uključen** i ne može se poništiti.
- Natpis glasi: "EU DSA Članak 17: suspenzije korisnika zahtijevaju ljudski pregled. 'Ban a user' je zaključan uključen i ne može se u potpunosti automatizirati u EU regiji."
- Opis alata u stupcu odobrenja glasi: "Zaključano uključeno prema EU DSA Članku 17 - potpuno automatizirane zabrane nisu dopuštene u EU regiji."

Što god drugo konfigurirali, svaki `ban_user` poziv od bilo kojeg agenta na stanaru u EU regiji ide u [ulaznicu odobrenja](#approval-workflow) za ljudski pregled. Zabrana se ne dogodi dok je čovjek ne odobri.

### Zašto se ovo provodi na razini platforme, a ne na razini upita

Sistemske upite model može zanemariti ili zaobići ako dovoljno neispravno radi. Usklađenost s Člankom 17. previše je važna da bi se oslanjali na dobro ponašanje modela; mora biti čvrsta poslužiteljska kontrola koju sam alat za raspoređivanje provodi. Što i radimo.

### Što prolazi kroz odobrenje, a što ne

- **`ban_user`**: uvijek kontrolirano u EU. Uključujući:
  - Vidljive zabrane (`shadowBan: false`).
  - Shadow zabrane (`shadowBan: true`).
  - Zabrane s `deleteAllUsersComments: true`.
  - Zabrane s `banIP: true`.
- Sve varijante zabrane dolaze u ulaznicu odobrenja s agentovim obrazloženjem i pouzdanjem; čovjek odobrava ili odbija.

Ostali alati agenta (`mark_comment_spam`, `warn_user`, `lock_comment` itd.) **nisu** pod utjecajem Članka 17. I dalje ih možete automatizirati. Članak 17. se posebno odnosi na suspenzije korisnika.

### Što je sa stanarima izvan EU

Zaključavanje ne vrijedi izvan EU regije. Možete ipak odabrati da `ban_user` stavite pod odobrenje - snažno to preporučujemo tijekom prvih tjedana života svakog agenta za moderiranje - ali se ne provodi.

### Shadow zabrane

Shadow zabrane se smatraju suspenzijama u smislu Članka 17. (korisnik može objavljivati, ali njegov sadržaj je skriven). Kontroliraju se identično kao vidljive zabrane.

### Otkrivanje regije

Regija se određuje na razini procesa pomoću varijable okruženja `REGION` na FastComments implementaciji (čita `isEURegion()` u `models/constants.ts`). Ne postoji polje regije po stanaru - zaključavanje vrijedi za svaki stanar na EU implementiranoj instanci. Ako migrirate svoje podatke s ne-EU implementacije na EU implementaciju, zaključavanje stupa na snagu za sve stanare na toj instanci.

### Što ako su svi pregledavatelji nedostupni

Odobrenje će ostati u ulaznici dok se ne odluči. Automatski istječe 90 dana nakon stvaranja. Ne postoji put "nema dostupnog pregledavatelja, prelazi na automatsku odluku" - to bi pobijedilo svrhu Članka 17.

Ako je vaša zajednica toliko velikog obujma da se EU zabrane ne mogu pregledati u razumnom vremenu, razmotrite:

- Dodavanje više pregledavatelja (vidi [Obavijesti o odobrenju](#approval-notifications)).
- Prebacivanje agenta na agresivnije korištenje [`warn_user`](#tool-warn-user), budući da upozorenja nisu podložna Članku 17.
- Smanjivanje agentove sklonosti zabranama pooštravanjem [smjernica zajednice](#community-guidelines) ili [početnog upita](#personality-prompt).

### Vidi također

- [Alat: ban_user](#tool-ban-user) za što `ban_user` radi i destruktivne opcije iza dodatnih odobrenja.
- [Tijek odobrenja](#approval-workflow) za cijeli životni ciklus odobrenja.