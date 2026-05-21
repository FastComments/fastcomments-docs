The **FastComments** block podpira enotno prijavo (SSO), tako da lahko vaši Shopify kupci komentirajo kot sami brez ustvarjanja ločenega FastComments računa.

### Kako deluje

Ko obiskovalec, ki je prijavljen v vašo trgovino, odpre stran z blokom **FastComments**:

1. Blok zazna Shopify `customer` objekt.
2. Pošlje ime in e-pošto stranke FastComments prek podpisane zahteve app proxy.
3. FastComments ustvari ali poišče uporabnika s ključem `shopify-{customerId}`, tako da isti kupec vedno ustreza istemu FastComments uporabniku med sejami in ponovnimi namestitvami.
4. Ime obiskovalca se prikaže pri njihovih komentarjih. Niso pozvani, da se ponovno prijavijo.

Če obiskovalec ni prijavljen v trgovino, se blok vrne na anonimno komentiranje (ali na FastComments prijavni postopek, odvisno od konfiguracije vašega vtičnika).

### Onemogočanje SSO

SSO je privzeto vklopljen za vsak blok **FastComments**. Če ga želite izklopiti na določenem bloku:

1. Odprite urejevalnik teme Shopify.
2. Odprite predlogo, ki vsebuje blok, in kliknite blok, da ga izberete.
3. Počistite kljukico pri **SSO**.
4. Kliknite **Shrani**.

SSO izklopite, če želite, da komentatorji izberejo ločeno identiteto za pogovor. Na primer, notranja skupnostna stran, kjer osebje komentira pod drugačnim prikaznim imenom.

### Kaj prejme FastComments

SSO paket podatkov, poslan za vsakega kupca, vsebuje:

- ID uporabnika izpeljan iz Shopify ID-ja kupca (`shopify-{customerId}`).
- E-poštni naslov stranke (uporablja se za identifikacijo uporabnika; ni prikazan javno).
- Prikazno ime stranke (uporablja se kot ime avtorja pri njihovem komentarju).

Ne pošiljajo se podatki o zgodovini naročil, plačilu ali naslovu. Paket je podpisan na strežniku; brskalnik stranke nikoli ne vidi poverilnic.

### Povezave za prijavo in odjavo

Ko je SSO vklopljen, povezavi za prijavo in odjavo v pripomočku za komentarje kažeta na `/account/login` in `/account/logout`, standardni poti računa stranke Shopify. Ni ničesar za konfigurirati. Povezave delujejo za vsako trgovino, kjer so omogočeni računi strank.