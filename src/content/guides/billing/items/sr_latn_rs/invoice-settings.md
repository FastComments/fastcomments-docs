Stranica [Invoice Settings](https://fastcomments.com/auth/my-account/invoice-settings), do koje se dolazi dugmetom `Invoice Settings` na Billing Info, kontroliše šta se pojavljuje na vašim fakturama i gde se one šalju. Samo `Account Owner` ili `Billing Admin` mogu da je otvore.

- `Company Name`: naziv vašeg naloga kako se prikazuje u kontrolnoj tabli, e-pošti i pozivima tima. Ovo nije naziv za naplatu.
- `Billing Email`: adresa na koju se šalju fakture i obaveštenja o naplati. Postavite ovo na poštansko sanduče za finansije ili računovodstvo ako vlasnik naloga ne treba da ih prima. Ovo ne menja e-poštu za prijavu nijednog korisnika.
- `Invoice Currency`: valuta u kojoj se vaše fakture izdaju. Pogledajte [Currencies](#currencies).
- `Billing Address`: polja `Name`, `Address`, `City`, `State / Province / Region`, `Postal Code` i `Country` koja se štampaju na fakturama. Ista polja se pojavljuju na obrascu Billing Info kada čuvate karticu; oba upisuju na isto mesto.
- `VAT / Tax ID`: opciono. Štampa se na vašim fakturama.

Kliknite na `Save Changes`. Stranica potvrđuje sa `Invoice settings updated successfully.` i promene se primenjuju na sledeću fakturu. Postojeće fakture se ne menjaju.

### VAT And Tax IDs

Izaberite `Country` za naplatu pre unosa ID-a, jer zemlja određuje kakav je to ID: EU VAT broj za bilo koju državu članicu EU, UK VAT broj za Ujedinjeno Kraljevstvo, ABN za Australiju, GST/HST broj za Kanadu, EIN za Sjedinjene Države, i tako dalje za ostale zemlje koje Stripe podržava.

ID se validira prilikom čuvanja. Ako nije validan za izabranu zemlju, stranica to kaže i ID se ne čuva, dok se vaša ostala podešavanja čuvaju. Ako vaša zemlja nije među onima za koje možemo da evidentiramo poreski ID, stranica će vam to takođe reći.

Ako imate adresu za naplatu u EU, Flex fakture se izdaju sa 0% PDV-a kao reverse charge transakcija, i vaš VAT broj se na njima pojavljuje jednom kada se sačuva, tako da vaš računovođa može da obradi PDV na vašoj strani.