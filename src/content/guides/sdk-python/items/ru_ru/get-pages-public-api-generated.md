Возвращает список страниц для тенанта. Используется десктоп-клиентом FChat для заполнения списка комнат.
Требует, чтобы `enableFChat` был равен true в разрешённом кастомном конфиге каждой страницы.
Страницы, требующие SSO, фильтруются по групповому доступу запрашивающего пользователя.

## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Нет | Непрозрачный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Привязан к тому же `sortBy`. |
| limit | integer | query | Нет | 1..200, по умолчанию 50 |
| q | string | query | Нет | Необязательный фильтр по префиксу заголовка без учёта регистра. |
| sortBy | string | query | Нет | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала с наибольшим числом комментариев) или `title` (по алфавиту). |
| hasComments | boolean | query | Нет | Если true, возвращать только страницы, у которых есть хотя бы один комментарий. |

## Ответ

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
    cursor = 'cursor_example' # str | Непрозрачный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Привязан к тому же `sortBy`. (optional)
    limit = 56 # int | 1..200, по умолчанию 50 (optional)
    q = 'q_example' # str | Необязательный фильтр по префиксу заголовка без учёта регистра. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала с наибольшим числом комментариев) или `title` (по алфавиту). (optional)
    has_comments = True # bool | Если true, возвращать только страницы, у которых есть хотя бы один комментарий. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]