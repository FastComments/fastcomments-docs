List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Неясен курсор за пагинация, върнат като `nextCursor` от предишна заявка. Свързан със същия `sortBy`. |
| limit | integer | query | No | 1..200, по подразбиране 50 |
| q | string | query | No | По избор, нечувствителен към регистъра филтър за началото на заглавието. |
| sortBy | string | query | No | Сортиране. `updatedAt` (по подразбиране, най-нови първи), `commentCount` (най‑много коментари първо), или `title` (по азбучен ред). |
| hasComments | boolean | query | No | Ако е истина, върнете само страници с поне един коментар. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Example

[inline-code-attrs-start title = 'Пример за get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Дефинирането на хоста е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри на конфигурацията.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Въведете контекст с екземпляр на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте екземпляр на API класа
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Неясен курсор за пагинация, върнат като `nextCursor` от предишна заявка. Свързан със същия `sortBy`. (опционално)
    limit = 56 # int | 1..200, по подразбиране 50 (опционално)
    q = 'q_example' # str | По избор, нечувствителен към регистъра филтър за началото на заглавието. (опционално)
    sort_by = client.PagesSortBy() # PagesSortBy | Сортиране. `updatedAt` (по подразбиране, най-нови първи), `commentCount` (най‑много коментари първо), или `title` (по азбучен ред). (опционално)
    has_comments = True # bool | Ако е истина, върнете само страници с поне един коментар. (опционално)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]

---