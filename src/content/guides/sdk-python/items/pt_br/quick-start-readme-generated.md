### Usando APIs Autenticadas (DefaultApi)

**Importante:** Você deve definir sua chave de API na Configuração antes de fazer solicitações autenticadas. Se não o fizer, as solicitações falharão com erro 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Create and configure the API client
config = Configuration()
config.host = "https://fastcomments.com/api"

# REQUIRED: Set your API key (get this from your FastComments dashboard)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Create the API instance with the configured client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Now you can make authenticated API calls
try:
    # Example: Add an SSO user
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Common errors:
    # - 401: API key is missing or invalid
    # - 400: Request validation failed
```

### Usando APIs Públicas (PublicApi)

Endpoints públicos não requerem autenticação:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com/api"

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
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Count the comments awaiting moderation
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Usando SSO (Single Sign-On)

O SDK inclui utilitários para gerar tokens SSO seguros:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Create user data
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Create SSO instance with your API secret
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generate the SSO token
sso_token = sso.create_token()

# Use this token in your frontend or pass to API calls
print(f"SSO Token: {sso_token}")
```

Para SSO simples (menos seguro, para teste):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Problemas Comuns

1. **Erro 401 "missing-api-key"**: Certifique‑se de definir `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` antes de criar a instância `DefaultApi`.
2. **Classe de API errada**: Use `DefaultApi` para solicitações autenticadas do lado do servidor, `PublicApi` para solicitações client‑side/públicas e `ModerationApi` para solicitações do painel de moderador.
3. **Erros de importação**: Certifique‑se de estar importando do módulo correto:
   - Cliente da API: `from client import ...`
   - Utilitários SSO: `from sso import ...`