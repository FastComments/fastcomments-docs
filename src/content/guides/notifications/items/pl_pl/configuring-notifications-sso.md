Dla SSO należy wziąć pod uwagę następującą konfigurację dotyczącą powiadomień:

- Czy użytkownik wyraził zgodę na powiadomienia.
  - Odbywa się to przez ustawienie flagi `optedInNotifications` na `true` lub `false` w obiekcie `SSOUser`.
  - Można to ustawić za pomocą API.
  - Ponadto, jeśli przekażesz wartość tej flagi w ładunku (payload), zostanie ona automatycznie zaktualizowana, gdy użytkownik załaduje wątek komentarzy.
- Czy użytkownik zapisał się na **subskrypcyjne** powiadomienia.
  - Odbywa się to przez ustawienie flagi `optedInSubscriptionNotifications` na `true` lub `false` w obiekcie `SSOUser`.
  - Można to ustawić za pomocą API.
  - Ponadto, jeśli przekażesz wartość tej flagi w ładunku (payload), zostanie ona automatycznie zaktualizowana, gdy użytkownik załaduje wątek komentarzy.
- Określenie ich adresu e-mail.
  - Jeśli nie jest podany, nie możemy wysyłać powiadomień e-mail.
- Czy wyłączyć linki rezygnacji w e-mailach.
  - Odbywa się to przez ustawienie flagi `disableUnsubscribeLinks` w obiekcie `Tenant`.
  - Można to ustawić za pomocą API.
- Czy użyć niestandardowego linku do rezygnacji.
  - Odbywa się to przez właściwość `footerUnsubscribeURL` w obiekcie `DomainConfig`.
  - Można to ustawić za pomocą API.
  - Możesz także rozważyć ustawienie odpowiednich nagłówków rezygnacji za pomocą `emailHeaders` w tym samym obiekcie.

---