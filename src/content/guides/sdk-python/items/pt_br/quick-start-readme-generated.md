### Usando APIs Autenticadas (DefaultApi)

**Importante:** Você deve definir sua chave de API na Configuração antes de fazer solicitações autenticadas. Caso contrário, as solicitações falharão com um erro 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Crie e configure o cliente da API
config = Configuration()
config.host = "https://fastcomments.com"

# OBRIGATÓRIO: Defina sua chave de API (obtenha-a no painel do FastComments)
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
    # - 401: chave de API ausente ou inválida
    # - 400: falha na validação da requisição
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

O `ModerationApi` alimenta o painel de moderador. Os métodos são chamados em nome de um moderador ao passar um token `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Contar os comentários aguardando moderação
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Usando SSO (Single Sign-On)

O SDK inclui utilitários para gerar tokens SSO seguros:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Crie os dados do usuário (id, email e username são obrigatórios)
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

# Use este token no frontend ou passe para chamadas de API
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

### Assinaturas ao Vivo (PubSub)

O módulo `pubsub` permite que você se inscreva em eventos de comentários em tempo real (novos comentários, votos, edições, notificações, etc.) via WebSocket, espelhando o `LiveEventSubscriber` do FastComments Java SDK. Ele requer o extra `pubsub`, que adiciona um cliente WebSocket sobre o cliente de API gerado:

```bash
pip install "fastcomments[pubsub] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```

```python
from pubsub import LiveEventSubscriber

subscriber = LiveEventSubscriber()

def handle_live_event(event):
    print(f"Live event: {event.type}")
    if event.comment:
        print(f"  comment: {event.comment.comment}")

result = subscriber.subscribe_to_changes(
    tenant_id_ws="YOUR_TENANT_ID",
    url_id="page-url-id",
    url_id_ws="page-url-id",
    user_id_ws="a-unique-presence-id",  # ex.: um UUID para esta sessão
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # defina como "eu" para a região da UE
)

# ...mais tarde, quando não quiser mais atualizações:
result.close()
```

O assinante executa a conexão em uma thread daemon em segundo plano, reconecta de forma transparente com jitter e busca quaisquer eventos perdidos enquanto estava desconectado do endpoint de registro de eventos ao reconectar. Passe um callback opcional `can_see_comments` (`List[str] -> Dict[str, str]`, retornando os IDs que o usuário **não** pode ver) para filtrar eventos de comentários que o usuário não tem permissão para visualizar. Defina `disable_live_commenting=True` para fazer `subscribe_to_changes` um no‑op que retorna `None`.

### Problemas Comuns

1. **Erro 401 "missing-api-key"**: Certifique‑se de definir `config.api_key = {"api_key": "YOUR_KEY"}` antes de criar a instância `DefaultApi`.
2. **Classe de API incorreta**: Use `DefaultApi` para requisições autenticadas no lado do servidor, `PublicApi` para requisições públicas/no cliente e `ModerationApi` para requisições do painel de moderação.
3. **Erros de importação**: Certifique‑se de estar importando do módulo correto:
   - Cliente da API: `from client import ...`
   - Utilitários SSO: `from sso import ...`
   - Assinaturas ao vivo: `from pubsub import ...` (necessita do extra `pubsub`)