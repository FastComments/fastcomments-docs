Każde obciążenie, czy to odnowienie planu, miesiąc Flex, czy zmiana planu, generuje fakturę. Faktury po opłaceniu pełnią również rolę paragonów.

### Znajdowanie faktur

Otwórz [Historię rozliczeń](https://fastcomments.com/auth/my-account/billing-history) lub kliknij `View Invoices` na stronie Informacji rozliczeniowych. Tabela zawiera listę każdej faktury z datą, okresem rozliczeniowym, numerem faktury, opisem, kwotą całkowitą, statusem (`Paid`, `Unpaid`, `Draft` lub `Void`) i terminem płatności, po dziesięć na stronę, od najnowszych.

Dla każdej faktury:

- `View` otwiera fakturę na hostowanej stronie Stripe, gdzie możesz ją również pobrać.
- `PDF` pobiera plik PDF bezpośrednio.
- `Pay Now` pojawia się zamiast `View` na nieopłaconej fakturze i przenosi Cię na stronę płatności Stripe dla tej faktury, gdzie możesz ją opłacić bezpośrednio.

Faktury są również wysyłane e-mailem na Twój adres rozliczeniowy w momencie ich utworzenia. Aby były wysyłane na skrzynkę działu finansów zamiast do właściciela konta, ustaw `Billing Email` w [Ustawieniach faktur](#invoice-settings).

### Co znajduje się na fakturze

Faktura zawiera nazwę i adres rozliczeniowy oraz numer VAT / Tax ID z [Ustawień faktur](#invoice-settings), pozycje faktury, wszelkie zastosowane przedpłacone [kredyty](#credits) oraz kwotę obciążoną na Twojej karcie. Faktury Flex zawierają jedną pozycję dla każdego rodzaju użycia, zobacz [Jak mierzone jest użycie Flex](#how-flex-usage-is-measured). Dla adresów rozliczeniowych w Unii Europejskiej faktury Flex są wystawiane z 0% VAT i z stopką `Reverse Charge Transaction: Customer to pay VAT`.

Zmiany w ustawieniach faktur dotyczą faktur utworzonych po ich zapisaniu. Faktura, która została już wystawiona, nie może być edytowana, więc ustaw dane swojej firmy oraz numer VAT / Tax ID przed następną datą rozliczeniową.

### Konta rozliczane gdzie indziej

Jeśli Twoje rozliczenia są obsługiwane zewnętrznie, na przykład poprzez fakturę uzgodnioną z pomocą techniczną, faktury są wysyłane do Ciebie do opłacenia, zamiast obciążać kartę, i mogą nie być widoczne w Historii rozliczeń. Konta zainstalowane przez Shopify lub BigCommerce są rozliczane przez tę platformę, a strona Informacji rozliczeniowych zawiera link do strony planu na tej platformie. Konta podrzędne rozliczane przez konto nadrzędne nie mają własnych faktur; zobacz [Kiedy jesteś obciążany](#when-you-are-charged).