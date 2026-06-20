Visualizadores atualmente online de uma página: pessoas cuja sessão websocket está inscrita na página agora.
Retorna anonCount + totalCount (assinantes em toda a sala, incluindo visualizadores anônimos que não enumeramos).

## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificador da URL da página (limpo no servidor). |
| afterName | string | query | No | Cursor: passe nextAfterName da resposta anterior. |
| afterUserId | string | query | No | Desempate do cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName estiver definido para que empates de nome não descartem entradas. |

## Resposta

Retorna: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e tem como padrão https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe da API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identificador da URL da página (limpo no servidor).
    after_name = 'after_name_example' # str | Cursor: passe nextAfterName da resposta anterior. (opcional)
    after_user_id = 'after_user_id_example' # str | Desempate do cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName estiver definido para que empates de nome não descartem entradas. (opcional)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]