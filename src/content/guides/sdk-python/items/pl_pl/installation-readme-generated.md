### Instalacja z GitHub

Instaluj bezpośrednio z tagu wydania (zalecane, w pełni odtwarzalne):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Zablokuj tag zamiast gałęzi, aby kompilacje były deterministyczne. Ten sam format działa w `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Każde otagowane [wydanie GitHub](https://github.com/fastcomments/fastcomments-python/releases) ma również dołączony zbudowany pakiet wheel, jeśli wolisz zainstalować binarny artefakt bezpośrednio.

### Zawartość biblioteki

Ta biblioteka zawiera dwa moduły: wygenerowanego klienta API oraz podstawową bibliotekę Pythona, która zawiera ręcznie napisane narzędzia ułatwiające pracę z API, w tym obsługę SSO.

- [Dokumentacja biblioteki klienta API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Dokumentacja biblioteki podstawowej, w tym przykłady SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Publiczne vs zabezpieczone API

Dla klienta API dostępne są trzy klasy, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` zawiera metody wymagające klucza API, a `PublicApi` zawiera metody, które mogą być wywoływane bezpośrednio z przeglądarki/urządzenia mobilnego itp. bez uwierzytelnienia. `ModerationApi` oferuje rozbudowany zestaw szybkich i bieżących API moderacji. Każda metoda `ModerationApi` przyjmuje parametr `sso` i może uwierzytelnić się za pomocą SSO lub ciasteczka sesji FastComments.com.