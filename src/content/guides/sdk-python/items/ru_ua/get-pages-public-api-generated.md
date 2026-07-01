List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозрачный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Связан с тем же `sortBy`. |
| limit | integer | query | No | 1..200, по умолчанию 50 |
| q | string | query | No | Необязательный регистронезависимый фильтр префикса заголовка. |
| sortBy | string | query | No | Порядок сортировки. `updatedAt` (по умолчанию, новые сначала), `commentCount` (сначала с наибольшим количеством комментариев) или `title` (в алфавитном порядке). |
| hasComments | boolean | query | No | Если `true`, возвращаются только страницы с как минимум одним комментарием. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Example

[inline-code-attrs-start title = 'get_pages_public Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Определение хоста необязательно, по умолчанию используется https://fastcomments.com
# Смотрите configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Входим в контекст с экземпляром клиента API
with client.ApiClient(configuration) as api_client:
    # Создаём экземпляр класса API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Непрозрачный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Связан с тем же `sortBy`. (optional)
    limit = 56 # int | 1..200, default 50 (optional)
    q = 'q_example' # str | Необязательный регистронезависимый фильтр префикса заголовка. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, новые сначала), `commentCount` (сначала с наибольшим количеством комментариев) или `title` (в алфавитном порядке). (optional)
    has_comments = True # bool | Если `true`, возвращаются только страницы с как минимум одним комментарием. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("Ответ PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Исключение при вызове PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]