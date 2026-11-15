Stranica [Invoice Settings](https://fastcomments.com/auth/my-account/invoice-settings), do koje se dolazi gumbom `Invoice Settings` na Billing Info, kontrolira što se pojavljuje na vašim računima i gdje se šalju. Samo `Account Owner` ili `Billing Admin` mogu je otvoriti.

- `Company Name`: naziv vašeg računa kako se prikazuje u nadzornoj ploči, e-pošti i pozivnicama timu. Ovo nije naziv za naplatu.
- `Billing Email`: adresa na koju se šalju računi i obavijesti o naplati. Postavite ovo na poštanski sandučić financija ili računovodstva ako vlasnik računa ne smije primati te poruke. Ovo ne mijenja prijavnu e-adresu nijednog korisnika.
- `Invoice Currency`: valuta u kojoj se izdaju vaši računi. Pogledajte [Currencies](#currencies).
- `Billing Address`: `Name`, `Address`, `City`, `State / Province / Region`, `Postal Code` i `Country` koji se ispisuju na računima. Isti se podaci pojavljuju na obrascu Billing Info kada spremate karticu; oba upisuju na isto mjesto.
- `VAT / Tax ID`: neobavezno. Ispisuje se na vašim računima.

Kliknite `Save Changes`. Stranica potvrđuje s `Invoice settings updated successfully.` i promjene primjenjuju se na sljedeći račun. Postojeći računi se ne mijenjaju.

### VAT i porezni brojevi

Odaberite svoju `Country` za naplatu prije unosa broja, jer zemlja određuje vrstu broja: EU PDV broj za bilo koju državu članicu EU, britanski PDV broj za Ujedinjeno Kraljevstvo, ABN za Australiju, GST/HST broj za Kanadu, EIN za Sjedinjene Države i tako dalje za ostale zemlje koje Stripe podržava.

Broj se provjerava prilikom spremanja. Ako nije valjan za odabranu zemlju, stranica to navodi i broj se ne sprema, dok se vaše ostale postavke spremaju. Ako za vašu zemlju ne možemo evidentirati porezni broj, stranica će vam to također reći.

Ako imate adresu za naplatu u EU, Flex računi izdaju se s 0% PDV-a kao reverse charge transakcija, a vaš PDV broj na njima se pojavljuje nakon spremanja, tako da vaše računovodstvo može obraditi PDV na vašoj strani.