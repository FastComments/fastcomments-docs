List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | path | Так |  |
| cursor | string | query | Ні | Непрозорий курсор пагінації, повернутий як `nextCursor` у попередньому запиті. Прив'язаний до того ж `sortBy`. |
| limit | integer | query | Ні | 1..200, за замовчуванням 50 |
| q | string | query | Ні | Необов'язковий нечутливий до регістру фільтр префіксу назви. |
| sortBy | string | query | Ні | Порядок сортування. `updatedAt` (за замовчуванням, новіші першими), `commentCount` (найбільше коментарів спочатку), або `title` (за алфавітом). |
| hasComments | boolean | query | Ні | Якщо true, повертаються лише сторінки з принаймні одним коментарем. |

## Відповідь

Повертає: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Приклад

[inline-code-attrs-start title = 'get_pages_public Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Визначення хоста є необов'язковим і за замовчуванням встановлює https://fastcomments.com
# Дивіться configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Непрозорий курсор пагінації, повернутий як `nextCursor` у попередньому запиті. Прив'язаний до того ж `sortBy`. (optional)
    limit = 56 # int | 1..200, за замовчуванням 50 (optional)
    q = 'q_example' # str | Необов'язковий нечутливий до регістру фільтр префіксу назви. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Порядок сортування. `updatedAt` (за замовчуванням, новіші першими), `commentCount` (найбільше коментарів спочатку), або `title` (за алфавітом). (optional)
    has_comments = True # bool | Якщо true, повертаються лише сторінки з принаймні одним коментарем. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]

---