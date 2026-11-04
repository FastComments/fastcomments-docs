Списък на страниците за даден tenant. Използва се от FChat десктоп клиента за попълване на списъка със стаи.
Изисква `enableFChat` да е true в резолвнатата персонализирана конфигурация за всяка страница.
Страниците, изискващи SSO, се филтрират спрямо груповия достъп на заявяващия потребител.

## Параметри

| Име | Тип | Местоположение | Задължителен | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Не | Непрозрачен курсор за странизиране, върнат като `nextCursor` от предишна заявка. Свързан със същия `sortBy`. |
| limit | integer | query | Не | 1..200, по подразбиране 50 |
| q | string | query | Не | Незадължителен филтър по префикс на заглавие, без значение на регистъра. |
| sortBy | string | query | Не | Ред на сортиране. `updatedAt` (по подразбиране, най-новите първо), `commentCount` (най-много коментари първо) или `title` (по азбучен ред). |
| hasComments | boolean | query | Не | Ако е true, връща само страници с поне един коментар. |

## Отговор

Връща: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
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
    cursor = 'cursor_example' # str | Непрозрачен курсор за странизиране, върнат като `nextCursor` от предишна заявка. Свързан със същия `sortBy`. (optional)
    limit = 56 # int | 1..200, по подразбиране 50 (optional)
    q = 'q_example' # str | Незадължителен филтър по префикс на заглавие, без значение на регистъра. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Ред на сортиране. `updatedAt` (по подразбиране, най-новите първо), `commentCount` (най-много коментари първо) или `title` (по азбучен ред). (optional)
    has_comments = True # bool | Ако е true, връща само страници с поне един коментар. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]