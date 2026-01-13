---
### Korzystanie z Nimble

```bash
nimble install fastcomments
```

### Budowanie ze źródeł

```bash
nimble build
```

### Zawartość biblioteki

Ta biblioteka zawiera wygenerowanego klienta API oraz narzędzia SSO, które ułatwiają pracę z API.

- [Dokumentacja biblioteki klienta API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Publiczne vs Zabezpieczone API

Dla klienta API istnieją dwa moduły API, `api_default` i `api_public`. `api_default` zawiera metody, które wymagają Twojego klucza API, a `api_public` zawiera wywołania API, które można wykonywać bezpośrednio z przeglądarki/urządzenia mobilnego itp. bez uwierzytelniania.
---