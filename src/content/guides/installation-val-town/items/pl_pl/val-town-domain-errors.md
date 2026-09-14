---
Gdy wyłączysz najemcę `demo`, widget może odmówić załadowania z błędem autoryzacji. Dzieje się tak, ponieważ FastComments nie wie, że powinien zezwolić na użycie Twojego konta na tej domenie.

[Przejdź tutaj, aby dodać swoją witrynę do konta.](https://fastcomments.com/auth/my-account/configure-domains)

Val Town zasługuje na ponowne spojrzenie, ponieważ val może być dostępny pod więcej niż jedną nazwą hosta:

- Każdy HTTP val ma długi domyślny punkt końcowy, `<org>--<id>.web.val.run`.
- Zarezerwowanie niestandardowej subdomeny dodaje `<name>.val.run`.
- Niestandardowa [domena](https://docs.val.town/vals/http/custom-domains/) dodaje trzecią.
- Gałęzie otrzymują własne adresy URL.

Dodaj wszystkie nazwy hostów, z których rzeczywiście serwujesz widget. Jeśli zarezerwujesz subdomenę po skonfigurowaniu wszystkiego, dodaj ją również, w przeciwnym razie widget będzie działał na starym URL i nie zadziała na nowym.
---