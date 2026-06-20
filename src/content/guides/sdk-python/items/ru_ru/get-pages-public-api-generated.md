Список страниц для тенанта. Используется клиентом FChat для рабочего стола для заполнения его списка комнат.
Требуется, чтобы `enableFChat` был установлен в true в результирующей пользовательской конфигурации каждой страницы.
Страницы, требующие SSO, фильтруются в соответствии с групповым доступом запрашивающего пользователя.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозрачный курсор пагинации, возвращённый как `nextCursor` в предыдущем запросе. Привязан к тому же `sortBy`. |
| limit | integer | query | No | 1..200, по умолчанию 50 |
| q | string | query | No | Необязательный регистронезависимый фильтр по префиксу заголовка. |
| sortBy | string | query | No | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала страницы с наибольшим количеством комментариев) или `title` (в алфавитном порядке). |
| hasComments | boolean | query | No | Если true, возвращать только страницы с хотя бы одним комментарием. |

## Response

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Example

[inline-code-attrs-start title = 'get_pages_public Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Непрозрачный курсор пагинации, возвращённый как `nextCursor` в предыдущем запросе. Привязан к тому же `sortBy`. (необязательно)
    limit = 56 # int | 1..200, по умолчанию 50 (необязательно)
    q = 'q_example' # str | Необязательный регистронезависимый фильтр по префиксу заголовка. (необязательно)
    sort_by = client.PagesSortBy() # PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала страницы с наибольшим количеством комментариев), или `title` (в алфавитном порядке). (необязательно)
    has_comments = True # bool | Если true, возвращать только страницы с хотя бы одним комментарием. (необязательно)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]