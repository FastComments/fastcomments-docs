Doładowania są obciążane na metodę płatności zapisaną na Twoim koncie. Jeśli nie zapisałeś jeszcze karty, strona Credits wyświetla `Add a payment method first` z przyciskiem `Go to billing`. Zapisz kartę na stronie [Billing Info](https://fastcomments.com/auth/my-account/billing-info), a następnie wróć.

Aby dodać środki:

1. Otwórz [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info) i kliknij `Manage Credits`.
2. W sekcji `Add credit` wybierz jedną z z góry określonych kwot lub wybierz `Other amount` i wprowadź własną.
3. Kliknij przycisk `Add`. Wyświetla wybraną kwotę, na przykład `Add $25.00`.
4. Jeśli Twój bank poprosi o potwierdzenie płatności, dokończ weryfikację w oknie, które się otworzy.

Gdy płatność zostanie zrealizowana, strona wyświetli `Credit added` z Twoim nowym saldem, a doładowanie pojawi się w `Past top-ups` jako `Credited`. Kliknij `Add more`, aby ponownie doładować konto.

### Kwoty i limity

Z góry określone kwoty to `10`, `25`, `50` i `100` w walucie rozliczeniowej Twojego konta. Kwota niestandardowa może wynosić od `5` do `10,000` na jedno doładowanie, a dokładne limity dla Twojej waluty są wyświetlane pod formularzem. W przypadku walut bez miejsc dziesiętnych, takich jak japoński jen, kwoty z góry określone i limity są sto razy większe, aby reprezentować mniej więcej tę samą wartość.

Doładowania są zawsze w walucie rozliczeniowej Twojego konta, tej samej co Twoje faktury. Strona informuje, która waluta zostanie obciążona.

### Potwierdzenie bankowe (3D Secure)

Niektóre banki wymagają potwierdzenia płatności online, zazwyczaj za pośrednictwem aplikacji bankowej lub jednorazowego kodu. Jeśli Twój bank tego wymaga, po kliknięciu `Add` otworzy się okno potwierdzenia. Twoja karta zostanie obciążona, a saldo zaktualizowane dopiero po zatwierdzeniu przez bank.

Jeśli zamkniesz stronę przed zakończeniem potwierdzenia, doładowanie zostanie zapisane jako `Pending`. Przy następnym otwarciu strony Credits sprawdzimy status płatności w Stripe i oznaczymy ją jako `Credited` lub `Failed`. Jeśli bank nie zatwierdzi płatności, strona wyświetli `Your bank did not approve the payment. Try again, or use a different card.` i nic nie zostanie obciążone.

### Odrzucone karty

Jeśli Twoja karta zostanie odrzucona, strona wyświetli `Your card was declined. Check your card on the billing page, or try another card.` Nic nie zostanie obciążone, saldo się nie zmieni, a próba zostanie odnotowana w `Past top-ups` jako `Failed`. Zaktualizuj kartę na stronie [Billing Info](https://fastcomments.com/auth/my-account/billing-info) i spróbuj ponownie.

Aby chronić Twoją kartę, konto może wykonać ograniczoną liczbę prób doładowania na godzinę. Jeśli zobaczysz `Too many top-up attempts. Wait a while and try again.`, odczekaj i spróbuj ponownie później.

Płatności są przetwarzane przez Stripe. FastComments nigdy nie przechowuje danych Twojej karty.