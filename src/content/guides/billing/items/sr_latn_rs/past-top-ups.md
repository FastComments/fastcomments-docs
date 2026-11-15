Tabela `Past top-ups` na dnu stranice Credits prikazuje vaše najnovije dopune, od najnovije ka najstarijoj, sa `Date`, `Amount` i `Status` svake od njih. Pre prve dopune prikazuje se `No top-ups yet.`

Svaka dopuna ima jedan od tri statusa:

- `Credited`: plaćanje je uspelo i iznos je dodat na vaše stanje.
- `Pending`: plaćanje je započeto ali nije završeno, obično zbog toga što bankovna potvrda nije dovršena. Dopune sa statusom Pending se proveravaju kod Stripe-a sledeći put kada se otvori stranica Credits i prelaze u `Credited` ili `Failed`.
- `Failed`: kartica je odbijena ili banka nije odobrila plaćanje. Ništa nije naplaćeno i vaše stanje se nije promenilo.

Tabela beleži dopune napravljene sa ove stranice. Fakture plaćene iz kredita prikazuju se uz same fakture, pod `View Invoices` na stranici [Billing Info](https://fastcomments.com/auth/my-account/billing-info).