Zaračevanje je omejeno na dve vrsti uporabnikov:

- `Account Owner`, uporabnik, ki je ustvaril račun.
- Vsak uporabnik z dovoljenjem `Billing Admin`.

To so edini uporabniki, ki lahko spremenijo paket, shranijo ali zamenjajo kartico, odprejo `Invoice Settings`, nastavijo omejitve porabe Flex in dodajo kredit. Vsak drug, ki to poskusi, vidi `Only the account owner or a billing admin can change the plan or payment method.`

E-poštna sporočila o zaračevanju, kot so opomnik sredi preizkusnega obdobja, opozorila o neuspešnem plačilu in obvestila o deaktivaciji, se pošiljajo uporabnikom z dovoljenjem `Billing Admin`. Dovoljenje dodelite tistemu, ki naj jih prejema, na primer nekome v finance, in ta bo prejemal e-pošto brez potrebe po kakršnem koli drugem dostopu do nadzorne plošče. Opomniki, poslani po koncu preizkusnega obdobja brez podatkov za zaračevanje, gredo vsem uporabnikom nadzorne plošče na računu. Vsaka od teh e-poštnih sporočil vsebuje povezavo za odjavo; uporabnik, ki se odjavi od obvestil računa, jih preneha prejemati.

Računi sami gredo na `Billing Email` v [Invoice Settings](#invoice-settings), ki sploh ne mora pripadati uporabniku nadzorne plošče.

Stran [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) namesto tega potrebuje dovoljenje `Analytics Admin`, saj prikazuje uporabo in ne podrobnosti plačil.

Za dodelitev ali odstranitev dovoljenja odprite stran `Users` in uredite uporabnika. Glejte [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions). Pri Flex upoštevajte, da je vsak dodatni skrbnik zaračan sedež, glejte [Admin and Moderator Seats](#admin-and-moderator-seats).

Če ste prijavljeni in sploh ne vidite `Billing Info`, ali pa vas pošlje nazaj na domačo stran, ste prijavljeni kot račun komentatorja in ne kot uporabnik nadzorne plošče. Odjavite se in prijavite z računom, ki je lastnik spletnega mesta.