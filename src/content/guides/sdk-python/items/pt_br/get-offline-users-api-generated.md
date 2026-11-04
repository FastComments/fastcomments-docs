Comentadores anteriores na página que NÃO estão atualmente online. Ordenados por displayName.
Use isto após esgotar /users/online para renderizar uma seção "Members".
Paginação por cursor em commenterName: o servidor percorre o índice parcial {tenantId, urlId, commenterName}
a partir de afterName via $gt, sem custo de $skip.

## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim | Identificador da URL da página (limpo no lado do servidor). |
| afterName | string | query | Não | Cursor: passe nextAfterName da resposta anterior. |
| afterUserId | string | query | Não | Desempate do cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName está definido para que nomes iguais não descartem entradas. |

## Resposta

Retorna: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identificador da URL da página (limpo no lado do servidor).
    after_name = 'after_name_example' # str | Cursor: passe nextAfterName da resposta anterior. (optional)
    after_user_id = 'after_user_id_example' # str | Desempate do cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName está definido para que nomes iguais não descartem entradas. (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]