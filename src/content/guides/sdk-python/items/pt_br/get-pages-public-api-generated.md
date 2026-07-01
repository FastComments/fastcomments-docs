List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | path | Sim |  |
| cursor | string | query | Não | Cursor de paginação opaco retornado como `nextCursor` de uma solicitação anterior. Vinculado ao mesmo `sortBy`. |
| limit | integer | query | Não | 1..200, padrão 50 |
| q | string | query | Não | Filtro opcional de prefixo de título sem distinção entre maiúsculas e minúsculas. |
| sortBy | string | query | Não | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (páginas com mais comentários primeiro) ou `title` (alfabética). |
| hasComments | boolean | query | Não | Se true, retorna apenas páginas com pelo menos um comentário. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e o padrão é https://fastcomments.com
# Consulte configuration.py para obter uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Cursor de paginação opaco retornado como `nextCursor` de uma solicitação anterior. Vinculado ao mesmo `sortBy`. (opcional)
    limit = 56 # int | 1..200, padrão 50 (opcional)
    q = 'q_example' # str | Filtro opcional de prefixo de título sem distinção entre maiúsculas e minúsculas. (opcional)
    sort_by = client.PagesSortBy() # PagesSortBy | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (páginas com mais comentários primeiro) ou `title` (alfabética). (opcional)
    has_comments = True # bool | Se true, retorna apenas páginas com pelo menos um comentário. (opcional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]