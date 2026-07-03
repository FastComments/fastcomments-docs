### Usando APIs Autenticadas (DefaultApi)

**Importante:** Você deve definir sua chave de API na Configuração antes de fazer solicitações autenticadas. Se não o fizer, as solicitações falharão com um erro 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Crie e configure o cliente API
config = Configuration()
config.host = "https://fastcomments.com"

# OBRIGATÓRIO: Defina sua chave de API (obtenha isto no painel do FastComments)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Crie a instância da API com o cliente configurado
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Agora você pode fazer chamadas de API autenticadas
try:
    # Exemplo: Adicionar um usuário SSO
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Erros comuns:
    # - 401: chave de API está ausente ou inválida
    # - 400: validação da solicitação falhou
```

### Usando APIs Públicas (PublicApi)

Endpoints públicos não requerem autenticação:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Usando o Painel de Moderação (ModerationApi)

A `ModerationApi` alimenta o painel de moderador. Os métodos são chamados em nome de um moderador passando um token `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Contar os comentários que aguardam moderação
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Usando SSO (Single Sign-On)

O SDK inclui utilitários para gerar tokens SSO seguros:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Crie os dados do usuário (id, email e nome de usuário são obrigatórios)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Assine com seu segredo da API (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Gere o token SSO para passar ao widget ou a uma chamada de API
sso_token = sso.create_token()

# Use este token no seu frontend ou passe para chamadas de API
print(f"SSO Token: {sso_token}")
```

Para SSO simples (menos seguro, para testes):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Problemas Comuns

1. **Erro 401 "missing-api-key"**: Certifique‑se de definir `config.api_key = {"api_key": "YOUR_KEY"}` antes de criar a instância DefaultApi.
2. **Classe de API incorreta**: Use `DefaultApi` para solicitações autenticadas no servidor, `PublicApi` para solicitações do lado cliente/públicas, e `ModerationApi` para solicitações do painel de moderador.
3. **Erros de importação**: Certifique‑se de que está importando do módulo correto:
   - Cliente da API: `from client import ...`
   - Utilitários SSO: `from sso import ...`