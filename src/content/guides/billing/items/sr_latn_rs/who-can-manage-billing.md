Naplata je ograničena na dve vrste korisnika:

- `Account Owner`, korisnik koji je kreirao nalog.
- Svaki korisnik sa `Billing Admin` dozvolom.

Samo oni mogu da menjaju plan, sačuvaju ili zamenjuju karticu, otvore `Invoice Settings`, postave Flex limite potrošnje i dodaju kredit. Svako drugi ko pokuša vidi poruku `Only the account owner or a billing admin can change the plan or payment method.`

E-mailovi vezani za naplatu, poput podsetnika sredinom probnog perioda, upozorenja o neuspešnoj naplati i obaveštenja o deaktivaciji, šalju se korisnicima sa `Billing Admin` dozvolom. Dodelite dozvolu onome ko bi trebalo da ih prima, na primer nekome iz finansija, i primaće e-mailove bez potrebe za bilo kojim drugim pristupom kontrolnoj tabli. Podsetnici koji se šalju nakon što se probni period završi bez podataka za naplatu idu svakom korisniku kontrolne table na nalogu. Svaki od ovih e-mailova sadrži link za odjavu; korisnik koji se odjavi od obaveštenja o nalogu prestaje da ih prima.

Fakture sami idu na `Billing Email` u [Invoice Settings](#invoice-settings), koji uopšte ne mora pripadati korisniku kontrolne table.

Stranica [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) umesto toga zahteva `Analytics Admin` dozvolu, pošto prikazuje korišćenje, a ne detalje plaćanja.

Da biste dodelili ili uklonili dozvolu, otvorite stranicu `Users` i uredite korisnika. Vidite [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions). Na Flex-u, imajte na umu da je svaki dodatni administrator naplaćeno mesto, vidite [Admin and Moderator Seats](#admin-and-moderator-seats).

Ako ste prijavljeni i uopšte ne vidite `Billing Info`, ili vas vraća na početnu stranicu, prijavljeni ste kao nalog komentatora, a ne kao korisnik kontrolne table. Odjavite se i prijavite nalogom koji je vlasnik sajta.