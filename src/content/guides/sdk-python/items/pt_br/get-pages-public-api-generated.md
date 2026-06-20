Lista páginas de um tenant. Usado pelo cliente desktop FChat para preencher sua lista de salas.
Exige que `enableFChat` seja true na configuração personalizada resolvida para cada página.
Páginas que requerem SSO são filtradas com base no acesso de grupo do usuário solicitante.

## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Cursor opaco de paginação retornado como `nextCursor` de uma requisição anterior. Vinculado ao mesmo `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Filtro opcional por prefixo de título insensível a maiúsculas/minúsculas. |
| sortBy | string | query | No | Ordem. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentários primeiro), ou `title` (alfabética). |
| hasComments | boolean | query | No | Se true, retorna apenas páginas com pelo menos um comentário. |

## Resposta

Retorna: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e por padrão é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Cursor opaco de paginação retornado como `nextCursor` de uma requisição anterior. Vinculado ao mesmo `sortBy`. (opcional)
    limit = 56 # int | 1..200, padrão 50 (opcional)
    q = 'q_example' # str | Filtro opcional por prefixo de título insensível a maiúsculas/minúsculas. (opcional)
    sort_by = client.PagesSortBy() # PagesSortBy | Ordem. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentários primeiro), ou `title` (alfabética). (opcional)
    has_comments = True # bool | Se true, retorna apenas páginas com pelo menos um comentário. (opcional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]