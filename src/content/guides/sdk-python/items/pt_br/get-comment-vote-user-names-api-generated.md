## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| dir | integer | query | Yes |  |
| sso | string | query | No |  |

## Resposta

Retorna: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comment_vote_user_names_success_response.py)

## Exemplo

[inline-code-attrs-start title = 'get_comment_vote_user_names Exemplo'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comment_vote_user_names_success_response import GetCommentVoteUserNamesSuccessResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e o padrão é https://fastcomments.com
# Consulte configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    dir = 56 # int | 
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.get_comment_vote_user_names(tenant_id, comment_id, dir, sso=sso)
        print("The response of PublicApi->get_comment_vote_user_names:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comment_vote_user_names: %s\n" % e)
[inline-code-end]