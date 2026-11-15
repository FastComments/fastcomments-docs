Stran [Invoice Settings](https://fastcomments.com/auth/my-account/invoice-settings), do katerih pridete z gumbom `Invoice Settings` na Billing Info, nadzira, kaj se prikaže na vaših računih in kam so poslani. Odpreti jo lahko samo `Account Owner` ali `Billing Admin`.

- `Company Name`: ime vašega računa, kot je prikazano na nadzorni plošči, v e-pošti in povabilih v ekipo. To ni obračunsko ime.
- `Billing Email`: naslov, kamor so pošiljani računi in obvestila o obračunavanju. To nastavite na poštni predal finance ali računovodstva, če lastnik računa teh ne sme prejemati. S tem ne spremenite prijavne e-pošte nobenega uporabnika.
- `Invoice Currency`: valuta, v kateri so izdani vaši računi. Glejte [Currencies](#currencies).
- `Billing Address`: polja `Name`, `Address`, `City`, `State / Province / Region`, `Postal Code` in `Country`, natisnjena na računih. Isti polji se pojavita na obrazcu Billing Info, ko shranite kartico; oboje zapisuje na isto mesto.
- `VAT / Tax ID`: neobvezno. Natisnjen na vaših računih.

Kliknite `Save Changes`. Stran potrdi z `Invoice settings updated successfully.` in spremembe veljajo za naslednji račun. Obstoječi računi se ne spremenijo.

### DDV in davčne številke

Izberite svojo obračunsko `Country`, preden vnesete številko, ker država določa, kakšna vrsta številke je: EU DDV številka za katero koli državo članico EU, britanska DDV številka za Združeno kraljestvo, ABN za Avstralijo, GST/HST številka za Kanado, EIN za Združene države in tako naprej za druge države, ki jih podpira Stripe.

Številka se potrdi ob shranjevanju. Če ni veljavna za izbrano državo, to stran sporoči in številka se ne shrani, medtem ko se ostale nastavitve shranijo. Če za vašo državo ne moremo zabeležiti davčne številke, vam to stran tudi pove.

Če imate obračunski naslov v EU, so računi Flex izdani z 0 % DDV kot transakcija obračanja davčne obveznosti (reverse charge), vaša DDV številka pa se na njih prikaže, ko je shranjena, tako da lahko vaš računovodja uredi DDV na vaši strani.