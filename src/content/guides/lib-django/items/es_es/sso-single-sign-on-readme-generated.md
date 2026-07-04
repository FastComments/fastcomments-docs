Enable SSO y elige un modo en `settings.py`. Secure SSO firma al usuario del lado del servidor con HMAC‑SHA256 usando tu secreto de API y se recomienda.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # tu secreto de API; firma Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Mapea los campos de FastComments a tu modelo de usuario. Los valores pueden ser un atributo
        # nombre, una ruta con puntos ("profile.avatar_url"), una callable(user), o None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, o ruta con puntos
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, o ruta con puntos
    },
}
```

> **Elige el `id` de SSO deliberadamente.** El `id` de FastComments es el identificador permanente del historial de comentarios de un usuario. El `USER_MAP` predeterminado lo asigna a la clave primaria de Django para mayor comodidad sin configuración, pero las PKs enteras secuenciales son enumerables y difíciles de cambiar más tarde (cambiar el `id` de un usuario divide su historial en una nueva cuenta). Para cualquier caso más allá de una demostración, asigna `id` a un valor estable y opaco elegido de antemano (un UUID o un ID público dedicado), y nunca pongas datos privados allí. La aplicación de ejemplo usa un `id` basado en el nombre de usuario por esta razón.

SSO se inyecta automáticamente en `{% fastcomments %}`, `{% fastcomments_live_chat %}`, `{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` y `{% fastcomments_user_activity %}` para el usuario actual.

Las URL de login/logout mostradas a los visitantes sin sesión predeterminadamente son `reverse("login")` / `reverse("logout")`; sobrescríbelas con `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Mapeo personalizado

Dos opciones de mayor precedencia sobrescriben `USER_MAP`:

- **Un método en tu modelo de usuario** (el análogo Pythonic de una interfaz):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Un mapper global**, una ruta con puntos a `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

La precedencia es `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.