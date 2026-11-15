Kwota i termin obciążenia karty zależą od rodzaju posiadanego planu.

### Plany stałe

Po rozpoczęciu subskrypcji `Creators` lub `Pro` pierwszy miesiąc lub rok jest pobierany natychmiast, w przeglądarce, po kliknięciu `Pay Now & Start Subscription`. Jeśli Twój bank poprosi o potwierdzenie płatności, dokończ weryfikację w oknie, które się otworzy. Subskrypcja rozpoczyna się dopiero po zaksięgowaniu płatności.

Następnie subskrypcja odnawia się tego samego dnia każdego miesiąca lub każdego roku, jeśli wybrałeś `Pay Annually`, licząc od dnia subskrypcji. Odnowienie jest automatycznie obciążane na kartę zapisaną na koncie, a faktura jest wysyłana e-mailem na Twój adres rozliczeniowy. Nie wymaga to żadnych działań.

Rozliczenie roczne jest o 15% tańsze niż miesięczne. Flex nie może być rozliczany rocznie.

### Flex

Flex jest rozliczany z dołu: korzystasz z usługi w danym miesiącu kalendarzowym, a faktura za ten miesiąc jest tworzona w ostatnim dniu miesiąca (UTC) i natychmiast obciążana na Twoją kartę. Data rozliczenia to zawsze ostatni dzień miesiąca i nie może zostać zmieniona.

Faktura dotyczy wyłącznie tego miesiąca kalendarzowego. Użycie w miesiącu kalendarzowym, w którym się zarejestrowałeś, nigdy nie jest rozliczane, więc pierwsza faktura pojawia się najwcześniej na koniec miesiąca po rejestracji.

Jeśli Twoje użycie jest niższe niż minimum Flex, pozycja `FastComments Flex. (minimum ... /mo)` uzupełnia fakturę do kwoty minimalnej. Aktualne minimum jest widoczne na stronie [Billing Info](https://fastcomments.com/auth/my-account/billing-info) oraz na [stronie cennika](https://fastcomments.com/traffic-pricing).

W każdej chwili możesz sprawdzić, ile wyniesie kolejna faktura. Kwota `Next Invoice Amount` znajduje się na stronie [Billing Info](https://fastcomments.com/auth/my-account/billing-info), a pełne rozbicie pozycja po pozycji na stronie [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing). Sposób liczenia użycia opisano w sekcji [Jak mierzone jest użycie Flex](#how-flex-usage-is-measured).

### Zapisanie karty nie powoduje jej obciążenia

Po skonfigurowaniu rozliczeń zapisanie lub aktualizacja karty bez zmiany planu nigdy jej nie obciąża, chyba że masz niezapłaconą fakturę. Formularz informuje: `You won't be charged now unless you change plans.`, a karta jest po prostu przechowywana w Stripe na potrzeby kolejnej faktury. Tylko gdy nie masz jeszcze poprawnych danych rozliczeniowych, formularz informuje: `Your card will be charged now.`, a nawet wtedy konto Flex nie jest obciążane do końca miesiąca. Wszelkie przedpłacone [kredyty](#credits) na Twoim koncie są wykorzystywane do pokrycia każdej faktury przed obciążeniem karty.

### Zarządzane subkonta

Jeśli Twoje konto zarządza subkontami (na przykład po jednym dla każdej strony lub środowiska, albo dzierżawcy z białym labelkiem), rozliczane jest tylko konto nadrzędne. Użycie każdego subkonta pojawia się jako osobne pozycje na fakturze Flex konta nadrzędnego, poprzedzone nazwą subkonta, a subkonta korzystają z metody płatności i statusu rozliczeń konta nadrzędnego.