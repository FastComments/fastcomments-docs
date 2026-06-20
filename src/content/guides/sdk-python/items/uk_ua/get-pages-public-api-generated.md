Список сторінок для орендаря. Використовується десктоп-клієнтом FChat для заповнення списку кімнат. Вимагає, щоб `enableFChat` був true у розв'язній користувацькій конфігурації для кожної сторінки. Сторінки, які вимагають SSO, фільтруються відповідно до групового доступу запитуваного користувача.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| cursor | string | query | Ні | Непрозорий курсор пагінації, повернений як `nextCursor` у попередньому запиті. Пов'язаний з тим самим `sortBy`. |
| limit | integer | query | Ні | 1..200, за замовчуванням 50 |
| q | string | query | Ні | Необов'язковий нечутливий до регістру фільтр префіксу заголовка. |
| sortBy | string | query | Ні | Порядок сортування. `updatedAt` (за замовчуванням, починаючи з найновіших), `commentCount` (спочатку сторінки з найбільшою кількістю коментарів), або `title` (алфавітно). |
| hasComments | boolean | query | Ні | Якщо true, повертаються лише сторінки з принаймні одним коментарем. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Example

[inline-code-attrs-start title = 'Приклад get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Визначення хоста є необов'язковим і за замовчуванням встановлюється на https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Відкрийте контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Непрозорий курсор пагінації, повернений як `nextCursor` у попередньому запиті. Пов'язаний з тим самим `sortBy`. (optional)
    limit = 56 # int | 1..200, за замовчуванням 50 (optional)
    q = 'q_example' # str | Необов'язковий нечутливий до регістру фільтр префіксу заголовка. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Порядок сортування. `updatedAt` (за замовчуванням, починаючи з найновіших), `commentCount` (спочатку сторінки з найбільшою кількістю коментарів), або `title` (алфавітно). (optional)
    has_comments = True # bool | Якщо true, повертаються лише сторінки з принаймні одним коментарем. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]