FastComments sprovodi Član 17 EU Digital Services Act za tenant-e u EU regiji: **potpuno automatizovane suspenzije korisnika nisu dozvoljene**.

### Šta to praktično znači

Kada je vaš tenant u EU regiji, na formularu za uređivanje agenta:

- Polje za potvrdu **Approvals** za `ban_user` je **zaključano i uključeno** i ne može se isključiti.
- Oznaka glasi: "EU DSA Član 17: suspenzije korisnika zahtijevaju ljudski pregled. 'Ban a user' je zaključano i ne može biti u potpunosti automatizovano u EU regiji."
- Tooltip u koloni za odobravanja glasi: "Zaključano prema Članu 17 EU DSA - potpuno automatizovane zabrane nisu dozvoljene u EU regiji."

Šta god drugo konfigurirali, svaki poziv `ban_user` od bilo kojeg agenta na tenant-u u EU regiji ide u [approvals inbox](#approval-workflow) na ljudski pregled. Zabrana se ne izvršava dok je osoba ne odobri.

### Zašto se ovo sprovodi na nivou platforme, a ne na nivou prompta

Sistemske instrukcije (system prompts) mogu biti ignorisane ili zaobiđene od strane dovoljno loše ponašajućeg modela. Usklađenost sa Članom 17 je previše važna da bi se oslanjala na dobro ponašanje modela; mora postojati čvrsti serverski filter koji sam dispatcher alata nameće. Upravo to i radimo.

### Šta prolazi, a šta ne prolazi kroz odobravanje

- **`ban_user`**: uvijek je ograničen u EU. Uključujući:
  - Vidljive zabrane (`shadowBan: false`).
  - Shadow zabrane (`shadowBan: true`).
  - Zabrane sa `deleteAllUsersComments: true`.
  - Zabrane sa `banIP: true`.
- Sve varijante zabrana dospijevaju u inbox za odobravanja sa razlozima i povjerenjem agenta; osoba odobrava ili odbacuje.

Ostali alati agenta (`mark_comment_spam`, `warn_user`, `lock_comment`, itd.) **nisu** pogođeni Članom 17. I dalje ih možete automatizovati. Član 17 se specifično odnosi na suspenzije korisnika.

### Šta je sa tenant-ima izvan EU

Zaključavanje se ne primjenjuje izvan EU regije. I dalje možete odlučiti da `ban_user` zahtjeva odobrenje — to toplo preporučujemo u prvim nedjeljama rada bilo kojeg moderation agenta — ali to nije izvršno obavezno.

### Shadow zabrane

Shadow zabrane se računaju kao suspenzije u svrhe Člana 17 (korisnik može objavljivati, ali njihov sadržaj je skriven). One su ograničene identično kao i vidljive zabrane.

### Detekcija regije

Regija se određuje na nivou procesa putem `REGION` environment varijable na FastComments deployment-u (čitano od strane `isEURegion()` u `models/constants.ts`). Ne postoji polje regije po tenant-u - zaključavanje se primjenjuje na svaki tenant na instanci koja je deploy-ovana u EU. Ako migrirate vaše podatke sa deployment-a izvan EU na deployment u EU, zaključavanje stupa na snagu za sve tenant-e na toj instanci.

### Šta ako su svi recenzenti nedostupni

Zahtev za odobrenje će ostati u inboxu dok se ne odluči. Automatski ističe 90 dana nakon kreiranja. Ne postoji put "nema dostupnog recenzenta, preći na automatizovanu odluku" - to bi onemogućilo svrhu Člana 17.

Ako je vaša zajednica toliko visokog obima da EU zabrane ne mogu biti pregledane u razumnom roku, razmotrite:

- Dodavanje više recenzenata (pogledajte [Approval Notifications](#approval-notifications)).
- Prebacivanje agenta da agresivnije koristi [`warn_user`](#tool-warn-user), pošto upozorenja nisu predmet Člana 17.
- Smanjivanje sklonosti agenta ka zabrani tako što ćete pooštriti [pravila zajednice](#community-guidelines) ili [početni prompt](#personality-prompt).

### Pogledajte takođe

- [Alat: ban_user](#tool-ban-user) za šta `ban_user` služi i destruktivne opcije iza dodatnih opt-in-a.
- [Proces odobravanja](#approval-workflow) za kompletan životni ciklus odobravanja.