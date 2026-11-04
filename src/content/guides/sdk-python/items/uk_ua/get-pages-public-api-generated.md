Перелік сторінок для орендаря (tenant). Використовується десктопним клієнтом FChat для заповнення списку кімнат.
Потребує `enableFChat` = true у розв'язаній кастомній конфігурації для кожної сторінки.
Сторінки, що вимагають SSO, фільтруються відповідно до групового доступу користувача, який виконує запит.

## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| cursor | string | query | Ні | Непрозорий курсор пагінації, повернутий як `nextCursor` у попередньому запиті. Прив'язаний до того самого `sortBy`. |
| limit | integer | query | Ні | 1..200, за замовчуванням 50 |
| q | string | query | Ні | Необов'язковий фільтр за префіксом назви без урахування регістру. |
| sortBy | string | query | Ні | Порядок сортування. `updatedAt` (за замовчуванням, спочатку новіші), `commentCount` (спочатку з найбільшою кількістю коментарів) або `title` (за алфавітом). |
| hasComments | boolean | query | Ні | Якщо true, повертати лише сторінки, що мають щонайменше один коментар. |

## Відповідь

Повертає: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
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
    cursor = 'cursor_example' # str | Непрозорий курсор пагінації, повернутий як `nextCursor` у попередньому запиті. Прив'язаний до того самого `sortBy`. (optional)
    limit = 56 # int | 1..200, за замовчуванням 50 (optional)
    q = 'q_example' # str | Необов'язковий фільтр за префіксом назви без урахування регістру. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Порядок сортування. `updatedAt` (за замовчуванням, спочатку новіші), `commentCount` (спочатку з найбільшою кількістю коментарів) або `title` (за алфавітом). (optional)
    has_comments = True # bool | Якщо true, повертати лише сторінки, що мають щонайменше один коментар. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]