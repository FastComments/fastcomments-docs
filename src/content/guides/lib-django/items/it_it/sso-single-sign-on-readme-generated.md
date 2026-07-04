Enable SSO e scegli una modalità in `settings.py`. Secure SSO firma l'utente lato server con HMAC‑SHA256 usando il tuo segreto API ed è raccomandato.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # il tuo segreto API; firma Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Mappa i campi FastComments al tuo modello utente. I valori possono essere un attributo
        # nome, un percorso puntato ("profile.avatar_url"), una callable(user), o None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, o percorso puntato
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, o percorso puntato
    },
}
```

> **Scegli deliberatamente l'`id` SSO.** L'`id` di FastComments è l'identificatore permanente per la cronologia dei commenti di un utente. Il `USER_MAP` predefinito lo mappa alla tua chiave primaria Django per comodità zero‑config, ma le PK intere sequenziali sono enumerabili e difficili da modificare in seguito (cambiare l'`id` di un utente divide la sua cronologia in un nuovo account). Per qualsiasi uso oltre una demo, mappa l'`id` a un valore stabile e opaco scelto in anticipo (un UUID o un ID pubblico dedicato), e non inserire mai dati privati al suo interno. L'app di esempio utilizza un `id` basato sul nome utente per questo motivo.

SSO viene inserito automaticamente in `{% fastcomments %}`, `{% fastcomments_live_chat %}`, `{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` e `{% fastcomments_user_activity %}` per l'utente corrente.

Gli URL di login/logout mostrati ai visitatori non autenticati sono per impostazione predefinita `reverse("login")` / `reverse("logout")`; sovrascrivili con `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Mappatura personalizzata

Due opzioni con precedenza più alta hanno la precedenza su `USER_MAP`:

- **Un metodo sul tuo modello utente** (l'analogo Pythonico di un'interfaccia):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Un mapper globale**, un percorso puntato a `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

La precedenza è `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.