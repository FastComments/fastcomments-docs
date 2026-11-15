Tablica `Past top-ups` na dnu stranice Credits prikazuje vaša najnovija dopunjavanja, od najnovijeg prema najstarijem, s `Date`, `Amount` i `Status` svakog od njih. Prije prvog dopunjavanja prikazuje se `No top-ups yet.`

Svako dopunjavanje ima jedan od tri statusa:

- `Credited`: plaćanje je uspjelo i iznos je dodan na vaše stanje.
- `Pending`: plaćanje je započelo, ali nije završeno, obično jer bankovna potvrda nije bila dovršena. Dopunjavanja u statusu Pending provjeravaju se putem Stripe sljedeći put kada se otvori stranica Credits i prelaze u `Credited` ili `Failed`.
- `Failed`: kartica je odbijena ili banka nije odobrila plaćanje. Ništa nije naplaćeno i vaše stanje se nije promijenilo.

Tablica bilježi dopunjavanja izvršena s ove stranice. Fakture plaćene iz kredita prikazuju se uz same fakture, pod `View Invoices` na stranici [Billing Info](https://fastcomments.com/auth/my-account/billing-info).