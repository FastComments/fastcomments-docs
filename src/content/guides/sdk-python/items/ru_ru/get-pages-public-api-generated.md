List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозрачный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Связан с тем же `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Необязательный нечувствительный к регистру префиксный фильтр заголовков. |
| sortBy | string | query | No | Порядок сортировки. `updatedAt` (по умолчанию, новые сначала), `commentCount` (сначала страницы с наибольшим количеством комментариев) или `title` (по алфавиту). |
| hasComments | boolean | query | No | Если true, возвращать только страницы, содержащие хотя бы один комментарий. |

## Response

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Определение хоста является необязательным и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Войдите в контекст с экземпляром клиента API
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Непрозрачный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Связан с тем же `sortBy`. (optional)
    limit = 56 # int | 1..200, default 50 (optional)
    q = 'q_example' # str | Необязательный нечувствительный к регистру префиксный фильтр заголовков. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, новые сначала), `commentCount` (сначала страницы с наибольшим количеством комментариев) или `title` (по алфавиту). (optional)
    has_comments = True # bool | Если true, возвращать только страницы, содержащие хотя бы один комментарий. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]