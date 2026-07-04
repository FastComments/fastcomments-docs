Enable SSO i wybierz tryb w `settings.py`. Secure SSO podpisuje użytkownika po stronie serwera przy użyciu HMAC‑SHA256 i Twojego sekretu API i jest zalecane.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # Twój sekret API; podpisuje Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Mapuj pola FastComments na swój model użytkownika. Wartości mogą być atrybutem
        # nazwą, ścieżką kropkową ("profile.avatar_url"), wywoływalnym(user) lub None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, lub ścieżka kropkowa
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, lub ścieżka kropkowa
    },
}
```

> **Wybierz identyfikator SSO `id` świadomie.** Identyfikator FastComments `id` jest trwałym odnośnikiem do historii komentarzy użytkownika. Domyślne `USER_MAP` mapuje go na klucz główny Django dla wygody bez dodatkowej konfiguracji, ale kolejno przydzielane całkowite klucze PK są wyliczalne i trudne do późniejszej zmiany (zmiana `id` użytkownika dzieli ich historię na nowe konto). Dla wszystkiego poza demonstracją, mapuj `id` na stabilną, nieprzezroczystą wartość wybraną z góry (UUID lub dedykowany publiczny identyfikator) i nigdy nie umieszczaj w nim prywatnych danych. Przykładowa aplikacja używa identyfikatora opartego na nazwie użytkownika z tego powodu.

SSO jest wstrzykiwane automatycznie do `{% fastcomments %}`, `{% fastcomments_live_chat %}`, `{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` i `{% fastcomments_user_activity %}` dla bieżącego użytkownika.

Login/logout URLs wyświetlane niezalogowanym gościom domyślnie to `reverse("login")` / `reverse("logout")`; można je zastąpić za pomocą `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Niestandardowe mapowanie

Dwie opcje o wyższym priorytecie mają pierwszeństwo przed `USER_MAP`:

- **Metoda w Twoim modelu użytkownika** (pythoniczny odpowiednik interfejsu):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Globalny mapper**, ścieżka kropkowa do `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Priorytet jest następujący: `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.