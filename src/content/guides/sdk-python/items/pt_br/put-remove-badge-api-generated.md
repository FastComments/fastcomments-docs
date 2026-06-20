---
## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| badgeId | string | query | Sim |  |
| userId | string | query | Não |  |
| commentId | string | query | Não |  |
| broadcastId | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/remove_user_badge_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo put_remove_badge'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.remove_user_badge_response import RemoveUserBadgeResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e, por padrão, é https://fastcomments.com
# Consulte configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe da API
    api_instance = client.ModerationApi(api_client)
    badge_id = 'badge_id_example' # str | 
    user_id = 'user_id_example' # str |  (opcional)
    comment_id = 'comment_id_example' # str |  (opcional)
    broadcast_id = 'broadcast_id_example' # str |  (opcional)
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.put_remove_badge(badge_id, user_id=user_id, comment_id=comment_id, broadcast_id=broadcast_id, sso=sso)
        print("The response of ModerationApi->put_remove_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_remove_badge: %s\n" % e)
[inline-code-end]

---