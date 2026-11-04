Lista páginas para um tenant. Usada pelo cliente desktop FChat para popular sua lista de salas.
Requer que `enableFChat` seja true na configuração customizada resolvida de cada página.
Páginas que exigem SSO são filtradas de acordo com o acesso a grupos do usuário solicitante.

## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| cursor | string | query | Não | Cursor opaco de paginação retornado como `nextCursor` de uma requisição anterior. Vinculado ao mesmo `sortBy`. |
| limit | integer | query | Não | 1..200, padrão 50 |
| q | string | query | Não | Filtro opcional de prefixo de título, sem diferenciar maiúsculas de minúsculas. |
| sortBy | string | query | Não | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentados primeiro) ou `title` (alfabética). |
| hasComments | boolean | query | Não | Se true, retorna apenas páginas com pelo menos um comentário. |

## Resposta

Retorna: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
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
    cursor = 'cursor_example' # str | Cursor opaco de paginação retornado como `nextCursor` de uma requisição anterior. Vinculado ao mesmo `sortBy`. (optional)
    limit = 56 # int | 1..200, padrão 50 (optional)
    q = 'q_example' # str | Filtro opcional de prefixo de título, sem diferenciar maiúsculas de minúsculas. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentados primeiro) ou `title` (alfabética). (optional)
    has_comments = True # bool | Se true, retorna apenas páginas com pelo menos um comentário. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]