### Używanie Nimble

```bash
nimble install fastcomments
```

### Budowanie ze źródła

```bash
nimble build
```

### Zawartość Biblioteki

Ta biblioteka zawiera wygenerowanego klienta API oraz narzędzia SSO ułatwiające pracę z API.

- [Dokumentacja Biblioteki Klienta API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Publiczne vs Zabezpieczone API

Dla klienta API istnieją trzy moduły API: `api_default`, `api_public` i `api_moderation`. `api_default` zawiera metody wymagające klucza API, a `api_public` zawiera wywołania API, które można wykonać bezpośrednio z przeglądarki/urządzenia mobilnego itp. bez uwierzytelnienia. Moduł `api_moderation` zawiera metody przeznaczone do panelu moderatora.

Moduł `api_moderation` oferuje rozbudowany zestaw szybkich i bieżących API moderacji. Każda metoda `api_moderation` przyjmuje parametr `sso` i może uwierzytelnić się za pomocą SSO lub ciasteczka sesyjnego FastComments.com.