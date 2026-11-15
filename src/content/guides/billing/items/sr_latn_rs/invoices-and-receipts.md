Svaka naplata, bilo da se radi o obnovi plana, Flex mesecu ili promeni plana, proizvodi fakturu. Fakture istovremeno služe i kao računi kada se plate.

### Pronalaženje faktura

Otvorite [Billing History](https://fastcomments.com/auth/my-account/billing-history) ili kliknite na `View Invoices` na stranici Billing Info. Tabela prikazuje svaku fakturu sa datumom, periodom naplate, brojem fakture, opisom, ukupnim iznosom, statusom (`Paid`, `Unpaid`, `Draft` ili `Void`) i datumom dospeća, po deset po stranici, najnovije prve.

Za svaku fakturu:

- `View` otvara fakturu na Stripe-ovoj hostovanoj stranici, gde je možete i preuzeti.
- `PDF` direktno preuzima PDF.
- `Pay Now` se pojavljuje umesto `View` kod neplaćene fakture i vodi vas na Stripe-ovu stranicu za plaćanje te fakture, gde je možete direktno platiti.

Fakture se takođe šalju emailom na vašu adresu za naplatu kada se kreiraju. Da bi išle na finansijsko sanduče umesto vlasnika naloga, podesite `Billing Email` u [Invoice Settings](#invoice-settings).

### Šta se nalazi na fakturi

Faktura prikazuje ime i adresu za naplatu i VAT / Tax ID iz [Invoice Settings](#invoice-settings), stavke, sav preplaćeni [kredit](#credits) koji je primenjen i iznos naplaćen sa vaše kartice. Flex fakture navode jednu stavku po vrsti korišćenja, vidite [How Flex Usage Is Measured](#how-flex-usage-is-measured). Za adrese za naplatu u Evropskoj uniji, Flex fakture se izdaju sa 0% PDV-a i podnožjem `Reverse Charge Transaction: Customer to pay VAT`.

Izmene podešavanja faktura primenjuju se na fakture kreirane nakon čuvanja. Faktura koja je već izdata ne može se menjati, pa podesite podatke o kompaniji i VAT / Tax ID pre sledećeg datuma naplate.

### Nalozi naplaćeni na drugom mestu

Ako se vaša naplata obavlja eksterno, na primer fakturama dogovorenim sa podrškom, fakture vam se šalju na plaćanje umesto da se naplaćuju sa kartice i možda se ne prikazuju u Billing History. Nalozi instalirani preko Shopify-a ili BigCommerce-a naplaćuju se preko te platforme, a stranica Billing Info vodi do stranice plana tamo. Pod-nalozi naplaćivani kroz roditeljski nalog nemaju sopstvene fakture; vidite [When You Are Charged](#when-you-are-charged).