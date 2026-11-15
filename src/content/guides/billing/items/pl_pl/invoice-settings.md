Strona [Invoice Settings](https://fastcomments.com/auth/my-account/invoice-settings), dostępna przez przycisk `Invoice Settings` w sekcji Billing Info, kontroluje to, co pojawia się na Twoich fakturach i gdzie są one wysyłane. Tylko `Account Owner` lub `Billing Admin` może ją otworzyć.

- `Company Name`: nazwa Twojego konta wyświetlana w panelu, e-mailach i zaproszeniach do zespołu. Nie jest to nazwa rozliczeniowa.
- `Billing Email`: adres, na który wysyłane są faktury i powiadomienia rozliczeniowe. Ustaw go na skrzynkę działu finansów lub księgowości, jeśli właściciel konta nie powinien ich otrzymywać. Nie zmienia to adresu e-mail logowania żadnego użytkownika.
- `Invoice Currency`: waluta, w której wystawiane są Twoje faktury. Zobacz [Currencies](#currencies).
- `Billing Address`: pola `Name`, `Address`, `City`, `State / Province / Region`, `Postal Code` i `Country` drukowane na fakturach. Te same pola pojawiają się na formularzu Billing Info przy zapisywaniu karty; oba zapisują dane w tym samym miejscu.
- `VAT / Tax ID`: opcjonalne. Drukowane na Twoich fakturach.

Kliknij `Save Changes`. Strona potwierdzi komunikatem `Invoice settings updated successfully.` a zmiany zostaną zastosowane do następnej faktury. Istniejące faktury nie są zmieniane.

### Numery VAT i identyfikatory podatkowe

Wybierz `Country` rozliczeniową przed wprowadzeniem numeru, ponieważ kraj decyduje o jego rodzaju: numer VAT UE dla każdego państwa członkowskiego UE, numer VAT UK dla Wielkiej Brytanii, ABN dla Australii, numer GST/HST dla Kanady, EIN dla Stanów Zjednoczonych i tak dalej dla pozostałych krajów obsługiwanych przez Stripe.

Numer jest walidowany podczas zapisu. Jeśli nie jest prawidłowy dla wybranego kraju, strona o tym poinformuje, a numer nie zostanie zapisany — pozostałe ustawienia zostaną zapisane. Jeśli dla Twojego kraju nie możemy zarejestrować numeru podatkowego, strona również Cię o tym poinformuje.

Jeśli masz adres rozliczeniowy w UE, faktury Flex są wystawiane z 0% VAT jako transakcja odwrotnego obciążenia (reverse charge), a Twój numer VAT pojawia się na nich po zapisaniu, dzięki czemu Twój księgowy może rozliczyć VAT po Twojej stronie.