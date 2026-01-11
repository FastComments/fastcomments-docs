### PyPI

```bash
pip install fastcomments
```

### Library Contents

Ta biblioteka zawiera dwa moduły: wygenerowanego klienta API oraz główną bibliotekę Pythona, która zawiera ręcznie napisane narzędzia ułatwiające pracę z API, w tym obsługę SSO.

- [Dokumentacja klienta API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Dokumentacja biblioteki głównej, w tym przykłady SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Publiczne vs zabezpieczone API

Dla klienta API są dwie klasy, `DefaultApi` i `PublicApi`. `DefaultApi` zawiera metody, które wymagają twojego klucza API, natomiast `PublicApi` zawiera wywołania API, które można wykonywać bezpośrednio z przeglądarki/urządzenia mobilnego itp. bez uwierzytelniania.