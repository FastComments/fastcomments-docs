Tabellen `Past top-ups` i bunden af Credits-siden viser dine seneste top-ups, nyeste først, med `Date`, `Amount` og `Status` for hver enkelt. Før din første top-up viser den `No top-ups yet.`

Hver top-up har en af tre statusser:

- `Credited`: betalingen lykkedes, og beløbet blev tilføjet din saldo.
- `Pending`: betalingen blev påbegyndt, men er ikke afsluttet, normalt fordi en bankbekræftelse ikke blev fuldført. Ventende top-ups kontrolleres mod Stripe, næste gang Credits-siden åbnes, og flyttes derefter til `Credited` eller `Failed`.
- `Failed`: kortet blev afvist, eller banken godkendte ikke betalingen. Der blev ikke trukket noget, og din saldo blev ikke ændret.

Tabellen registrerer top-ups foretaget fra denne side. Fakturaer betalt med kredit vises sammen med fakturaerne selv under `View Invoices` på siden [Billing Info](https://fastcomments.com/auth/my-account/billing-info).