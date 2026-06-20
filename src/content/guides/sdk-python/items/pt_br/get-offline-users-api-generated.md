Comentadores anteriores na página que NÃO estão atualmente online. Ordenados por displayName.
Use isto após esgotar /users/online para renderizar uma seção "Membros".
Paginação por cursor em commenterName: o servidor percorre o índice parcial {tenantId, urlId, commenterName}
a partir de afterName para frente via $gt, sem custo de $skip.

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim | Identificador da URL da página (limpo no servidor). |
| afterName | string | query | Não | Cursor: passe nextAfterName da resposta anterior. |
| afterUserId | string | query | Não | Critério de desempate do cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName estiver definido para que empates por nome não descartem entradas. |

## Resposta

Retorna: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Exemplo

[inline-code-attrs-start title = 'get_offline_users Example'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e o padrão é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identificador da URL da página (limpo no servidor).
    after_name = 'after_name_example' # str | Cursor: passe nextAfterName da resposta anterior. (opcional)
    after_user_id = 'after_user_id_example' # str | Critério de desempate do cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName estiver definido para que empates por nome não descartem entradas. (opcional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]