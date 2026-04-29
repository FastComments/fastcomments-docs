FastComments sprovodi Član 17 Zakona o digitalnim uslugama EU za zakupce u EU regionu: **potpuno automatizovane suspenzije korisnika nisu dozvoljene**.

### Šta to znači u praksi

Kada je vaš zakupac u EU regionu, na formi za izmenu agenta:

- Polje **Approvals** za `ban_user` je **zaključano** i ne može se odznačiti.
- Oznaka glasi: "EU DSA Article 17: suspenzije korisnika zahtevaju ljudsku proveru. 'Zabrani korisnika' je zaključano i ne može biti potpuno automatizovano u EU regionu."
- Tooltip na koloni za odobrenje glasi: "Locked on by EU DSA Article 17 - fully-automated bans are not permitted in the EU region."

Bez obzira na ostale konfiguracije, svaki poziv `ban_user` od bilo kog agenta na zakupcu u EU regionu ide u [approvals inbox](#approval-workflow) na ljudsku proveru. Zabrana se ne izvršava dok je ne odobri čovek.

### Zašto se ovo sprovodi na nivou platforme, a ne na nivou prompta

Sistemski promptovi mogu biti ignorišu ili zaobiđeni od strane modela koji se ne ponaša kako treba. Usklađenost sa Članom 17 je suviše važna da bi se oslanjala na dobro ponašanje modela; mora postojati čvrst server-side pristup koji sam dispatcher alata sprovodi. Upravo to i radimo.

### Šta prolazi, a šta ne prolazi kroz odobrenje

- **`ban_user`**: uvek je podložno odobrenju u EU. Uključujući:
  - Vidljive zabrane (`shadowBan: false`).
  - Shadow banove (`shadowBan: true`).
  - Zabrane sa `deleteAllUsersComments: true`.
  - Zabrane sa `banIP: true`.
- Sve varijante zabrana dospevaju u approvals inbox sa razlozima i stepenom uverenja agenta; čovek odobrava ili odbija.

Ostali alati agenta (`mark_comment_spam`, `warn_user`, `lock_comment`, itd.) **nisu** pogođeni Članom 17. Možete ih i dalje automatizovati. Član 17 se konkretno odnosi na suspenzije korisnika.

### Šta je sa zakupcima van EU

Zaključavanje se ne primenjuje van EU regiona. Ipak možete izabrati da stavite `ban_user` iza procesa odobrenja — to toplo preporučujemo tokom prvih nedelja rada bilo kog moderacijskog agenta — ali to nije prisilno.

### Shadow banovi

Shadow banovi se računaju kao suspenzije za potrebe Člana 17 (korisnik može objavljivati, ali njihov sadržaj je skriven). Oni se odobravaju na isti način kao vidljive zabrane.

### Detekcija regiona

Region se određuje na nivou procesa pomoću environment promenljive `REGION` na FastComments deploymentu (čita je `isEURegion()` u `models/constants.ts`). Ne postoji polje za region po zakupcu - zaključavanje važi za sve zakupce na instanci koja je deployovana u EU. Ako prebacite podatke sa deploymenta van EU na deployment u EU, zaključavanje stupa na snagu za sve zakupce na toj instanci.

### Šta ako su svi pregledavaoci nedostupni

Odobrenje ostaje u inboxu dok se ne donese odluka. Automatski ističe 90 dana nakon kreiranja. Ne postoji opcija "nema dostupnog pregledavaoca, pređi na automatizovanu odluku" — to bi poništilo suštinu Člana 17.

Ako je vaša zajednica toliko obimna da EU zabrane ne mogu biti pregledane u razumnom vremenu, razmotrite:

- Dodavanje više pregledavaoca (vidi [Approval Notifications](#approval-notifications)).
- Prebacivanje agenta da agresivnije koristi [`warn_user`](#tool-warn-user), pošto opomene nisu predmet Člana 17.
- Smanjivanje sklonosti agenta ka zabrani pooštravanjem [community guidelines](#community-guidelines) ili [initial prompt](#personality-prompt).

### Pogledajte i

- [Tool: ban_user](#tool-ban-user) za šta `ban_user` služi i destruktivne opcije koje zahtevaju dodatne potvrde.
- [Approval Workflow](#approval-workflow) za celokupan životni ciklus odobrenja.