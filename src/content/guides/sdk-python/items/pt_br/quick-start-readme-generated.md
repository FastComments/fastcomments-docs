### Usando APIs Autenticadas (DefaultApi)

**Importante:** Você deve definir sua chave de API na Configuration antes de fazer requisições autenticadas. Se não definir, as requisições falharão com um erro 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Crie e configure o cliente da API
config = Configuration()
config.host = "https://fastcomments.com/api"

# OBRIGATÓRIO: Defina sua chave de API (obtenha isso no seu painel do FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

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

    response = api.add_sso_user(
        tenant_id="YOUR_TENANT_ID",
        create_apisso_user_data=user_data
    )
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Erros comuns:
    # - 401: chave da API ausente ou inválida
    # - 400: Falha na validação da requisição
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
    response = public_api.get_comments_public(
        tenant_id="YOUR_TENANT_ID",
        url_id="page-url-id"
    )
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Usando SSO (Single Sign-On)

O SDK inclui utilitários para gerar tokens SSO seguros:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Crie os dados do usuário
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Crie a instância SSO com seu segredo da API
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Gere o token SSO
sso_token = sso.create_token()

# Use este token no seu frontend ou passe para chamadas de API
print(f"SSO Token: {sso_token}")
```

Para SSO simples (menos seguro, para testes):

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

1. **401 "missing-api-key" error**: Certifique-se de definir `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` antes de criar a instância DefaultApi.
2. **Classe de API incorreta**: Use `DefaultApi` para requisições autenticadas no lado do servidor, `PublicApi` para requisições do lado do cliente/públicas.
3. **Erros de importação**: Certifique-se de importar do módulo correto:
   - Cliente da API: `from client import ...`
   - Utilitários SSO: `from sso import ...`