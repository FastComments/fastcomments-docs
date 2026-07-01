### Instalacja z GitHub

Instaluj bezpośrednio z tagu wydania (zalecane, w pełni powtarzalne):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Zaciągnij tag zamiast gałęzi, aby budowy były deterministyczne. Ten sam format działa w `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Każde otagowane [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) ma również dołączony zbudowany wheel, jeśli wolisz zainstalować binarny artefakt bezpośrednio.

### Zawartość Biblioteki

Ta biblioteka zawiera dwa moduły: wygenerowanego klienta API oraz rdzeniową bibliotekę Pythona, która zawiera ręcznie napisane narzędzia ułatwiające pracę z API, w tym obsługę SSO.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Publiczne vs Zabezpieczone API

Dla klienta API dostępne są trzy klasy, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` zawiera metody, które wymagają Twojego klucza API, a `PublicApi` zawiera metody, które mogą być wywoływane bezpośrednio z przeglądarki/urządzenia mobilnego/etc bez uwierzytelnienia. `ModerationApi` zapewnia rozbudowany zestaw szybkich i działających w czasie rzeczywistym API moderacji. Każda metoda `ModerationApi` przyjmuje parametr `sso` i może uwierzytelnić się za pomocą SSO lub cookie sesji FastComments.com.