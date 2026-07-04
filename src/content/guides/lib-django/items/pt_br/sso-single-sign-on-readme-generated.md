Enable SSO and choose a mode in `settings.py`. Secure SSO signs the user
server-side with HMAC‑SHA256 using your API secret and is recommended.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # seu segredo da API; assina o Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Map FastComments fields to your user model. Values may be an attribute
        # name, a dotted path ("profile.avatar_url"), a callable(user), or None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, or dotted path
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, or dotted path
    },
}
```

> **Escolha o SSO `id` deliberadamente.** O FastComments `id` é o identificador permanente  
> handle para o histórico de comentários de um usuário.  
> O `USER_MAP` padrão mapeia isso para o seu  
> chave primária do Django para conveniência zero-config, mas PKs inteiros sequenciais são  
> enumeráveis e difíceis de mudar depois (alterar o `id` de um usuário divide seu  
> histórico em uma nova conta). Para tudo além de uma demonstração, mapeie o `id` para um valor estável,  
> valor opaco escolhido previamente (um UUID ou um id público dedicado), e nunca coloque  
> dados privados nele. O aplicativo de exemplo usa um id baseado em nome de usuário por esse motivo.

SSO is injected automatically into `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}`, and
`{% fastcomments_user_activity %}` for the current user.

Login/logout URLs shown to signed-out visitors default to `reverse("login")` /
`reverse("logout")`; override them with `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Custom mapping

Two higher-precedence options beat `USER_MAP`:

- **A method on your user model** (the Pythonic analog of an interface):

```python
class User(AbstractUser):
    def to_fastcomments_user_data(self):
        return {"id": self.pk, "email": self.email, "username": self.get_username()}
```

- **A global mapper**, a dotted path to `callable(user) -> dict`:

```python
FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
```

Precedence is `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.