Tabela `Past top-ups` na dole strony Credits zawiera listę Twoich ostatnich doładowań, od najnowszych, z `Date`, `Amount` i `Status` każdego z nich. Przed pierwszym doładowaniem wyświetla się tekst `No top-ups yet.`

Każde doładowanie ma jeden z trzech statusów:

- `Credited`: płatność się powiodła, a kwota została dodana do Twojego salda.
- `Pending`: płatność została rozpoczęta, ale nie została zakończona, zwykle dlatego, że potwierdzenie bankowe nie zostało ukończone. Doładowania w stanie Pending są sprawdzane w Stripe przy następnym otwarciu strony Credits i przechodzą do stanu `Credited` lub `Failed`.
- `Failed`: karta została odrzucona lub bank nie zatwierdził płatności. Nic nie zostało pobrane, a Twoje saldo się nie zmieniło.

Tabela rejestruje doładowania wykonane z tej strony. Faktury opłacane z kredytu są wyświetlane wraz z samymi fakturami, pod `View Invoices` na stronie [Billing Info](https://fastcomments.com/auth/my-account/billing-info).