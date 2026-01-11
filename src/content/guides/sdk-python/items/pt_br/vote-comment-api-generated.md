## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| commentId | string | path | Sim |  |
| urlId | string | query | Sim |  |
| broadcastId | string | query | Sim |  |
| sessionId | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_comment200_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de vote_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_body_params import VoteBodyParams
from client.models.vote_comment200_response import VoteComment200Response
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e o padrão é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe de API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    url_id = 'url_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    vote_body_params = client.VoteBodyParams() # VoteBodyParams | 
    session_id = 'session_id_example' # str |  (opcional)
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.vote_comment(tenant_id, comment_id, url_id, broadcast_id, vote_body_params, session_id=session_id, sso=sso)
        print("The response of PublicApi->vote_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->vote_comment: %s\n" % e)
[inline-code-end]