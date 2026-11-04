Ative o SSO e escolha um modo em `settings.py`. O SSO Secure assina o usuário
no lado do servidor com HMAC-SHA256 usando seu API secret e é o recomendado.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # seu API secret; assina o SSO Secure
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Mapeie os campos do FastComments para o seu modelo de usuário. Os valores podem ser
        # um nome de atributo, um caminho com pontos ("profile.avatar_url"), um callable(user) ou None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, ou caminho com pontos
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, ou caminho com pontos
    },
}
```

> **Escolha o `id` do SSO deliberadamente.** O `id` do FastComments é o identificador
> permanente do histórico de comentários de um usuário. O `USER_MAP` padrão o mapeia para a
> sua chave primária do Django por conveniência de configuração zero, mas chaves primárias
> inteiras sequenciais são enumeráveis e difíceis de alterar depois (mudar o `id` de um
> usuário divide o histórico dele em uma nova conta). Para qualquer coisa além de uma demo,
> mapeie `id` para um valor estável e opaco escolhido desde o início (um UUID ou um id
> público dedicado), e nunca coloque dados privados nele. O app de exemplo usa um id
> baseado no nome de usuário por esse motivo.

O SSO é injetado automaticamente em `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` e
`{% fastcomments_user_activity %}` para o usuário atual.

As URLs de login/logout exibidas para visitantes desconectados assumem como padrão
`reverse("login")` / `reverse("logout")`; sobrescreva-as com `SSO["LOGIN_URL"]` /
`SSO["LOGOUT_URL"]`.

### Mapeamento personalizado

Duas opções de precedência mais alta têm prioridade sobre `USER_MAP`:

- **Um método no seu modelo de usuário** (o análogo Pythonico de uma interface):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Um mapper global**, um caminho com pontos para `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

A precedência é `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.