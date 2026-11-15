Naplata je ograničena na dvije vrste korisnika:

- `Account Owner`, korisnik koji je stvorio račun.
- Svaki korisnik s dozvolom `Billing Admin`.

Oni su jedini korisnici koji mogu promijeniti plan, spremiti ili zamijeniti karticu, otvoriti `Invoice Settings`, postaviti Flex limite potrošnje i dodati kredit. Svatko drugi tko pokuša vidi `Only the account owner or a billing admin can change the plan or payment method.`

E-mailovi za naplatu, poput podsjetnika usred probnog razdoblja, upozorenja o neuspjelom plaćanju i obavijesti o deaktivaciji, šalju se korisnicima s dozvolom `Billing Admin`. Dodijelite dozvolu onome tko ih treba primati, na primjer osobi iz financija, i primat će e-mailove bez potrebe za bilo kojim drugim pristupom upravljačkoj ploči. Podsjetnici poslani nakon što je probno razdoblje završilo bez podataka za naplatu idu svakom korisniku upravljačke ploče na računu. Svaki od ovih e-mailova ima poveznicu za odjavu; korisnik koji se odjavi od obavijesti o računu prestaje ih primati.

Fakture same idu na `Billing Email` u [Invoice Settings](#invoice-settings), koji uopće ne mora pripadati korisniku upravljačke ploče.

Stranica [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) umjesto toga zahtijeva dozvolu `Analytics Admin`, budući da prikazuje korištenje, a ne detalje plaćanja.

Za dodjelu ili uklanjanje dozvole otvorite stranicu `Users` i uredite korisnika. Pogledajte [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions). Na Flexu imajte na umu da je svaki dodatni administrator naplaćeno mjesto (seat), pogledajte [Admin and Moderator Seats](#admin-and-moderator-seats).

Ako ste prijavljeni i uopće ne vidite `Billing Info`, ili vas vraća na početnu stranicu, prijavljeni ste kao račun komentatora, a ne kao korisnik upravljačke ploče. Odjavite se i prijavite s računom koji posjeduje web-stranicu.