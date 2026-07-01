## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|--------------|-------------|-----------|
| tenantId | string | path | Yes |  |
| postId | string | path | Yes |  |
| isUndo | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Resposta

Returns: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/react_feed_post_response.py)

## Exemplo

[inline-code-attrs-start title = 'react_feed_post_public Exemplo'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import ReactFeedPostPublicOptions
from client.models.react_body_params import ReactBodyParams
from client.models.react_feed_post_response import ReactFeedPostResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e padrão para https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_id = 'post_id_example' # str | 
    react_body_params = client.ReactBodyParams() # ReactBodyParams | 
    is_undo = True # bool |  (opcional)
    broadcast_id = 'broadcast_id_example' # str |  (opcional)
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.react_feed_post_public(tenant_id, post_id, react_body_params, ReactFeedPostPublicOptions(is_undo=is_undo, broadcast_id=broadcast_id, sso=sso))
        print("A resposta de PublicApi->react_feed_post_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exceção ao chamar PublicApi->react_feed_post_public: %s\n" % e)
[inline-code-end]