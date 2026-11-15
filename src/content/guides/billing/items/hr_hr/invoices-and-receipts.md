Svaka naplata, bilo da se radi o obnovi plana, Flex mjesecu ili promjeni plana, proizvodi fakturu. Fakture udvostručuju ulogu računa nakon plaćanja.

### Pronalaženje faktura

Otvorite [Billing History](https://fastcomments.com/auth/my-account/billing-history) ili kliknite `View Invoices` na stranici Billing Info. Tablica prikazuje svaku fakturu s datumom, razdobljem naplate, brojem fakture, opisom, ukupnim iznosom, statusom (`Paid`, `Unpaid`, `Draft` ili `Void`) i datumom dospijeća, deset po stranici, najnovije prve.

Za svaku fakturu:

- `View` otvara fakturu na Stripe-ovoj hostiranoj stranici, gdje je možete i preuzeti.
- `PDF` izravno preuzima PDF.
- `Pay Now` pojavljuje se umjesto `View` kod neplaćene fakture i vodi vas na Stripe-ovu stranicu za plaćanje te fakture, gdje je možete izravno platiti.

Fakture se također šalju e-poštom na vašu adresu za naplatu prilikom njihove izrade. Da bi išle u financijski poštanski sandučić umjesto vlasniku računa, postavite `Billing Email` u [Invoice Settings](#invoice-settings).

### Što se nalazi na fakturi

Faktura prikazuje naziv i adresu za naplatu te VAT / Tax ID iz [Invoice Settings](#invoice-settings), stavke, svaki primijenjeni unaprijed plaćeni [kredit](#credits) te iznos naplaćen s vaše kartice. Flex fakture navode jednu stavku po vrsti korištenja, vidi [How Flex Usage Is Measured](#how-flex-usage-is-measured). Za adrese za naplatu u Europskoj uniji, Flex fakture izdaju se s 0% PDV-a i podnožjem `Reverse Charge Transaction: Customer to pay VAT`.

Promjene postavki faktura primjenjuju se na fakture stvorene nakon što ih spremite. Faktura koja je već izdana ne može se uređivati, stoga postavite podatke o svojoj tvrtki i VAT / Tax ID prije sljedećeg datuma naplate.

### Računi naplaćeni drugdje

Ako se vaša naplata obavlja izvana, na primjer fakturom dogovorenom s podrškom, fakture vam se šalju na plaćanje umjesto da se naplaćuju s kartice i možda se ne prikazuju u Billing History. Računi instalirani putem Shopifyja ili BigCommercea naplaćuju se preko te platforme, a stranica Billing Info vodi na stranicu plana tamo. Podračuni naplaćivani kroz nadređeni račun nemaju vlastite fakture; vidi [When You Are Charged](#when-you-are-charged).