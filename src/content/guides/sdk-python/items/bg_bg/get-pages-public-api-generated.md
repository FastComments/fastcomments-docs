Изброява страниците за tenant. Използва се от настолния клиент FChat за попълване на списъка с стаи.
Изисква `enableFChat` да е true в резолвнатата персонализирана конфигурация за всяка страница.
Страниците, които изискват SSO, се филтрират спрямо груповия достъп на потребителя, отправил заявката.

## Параметри

| Име | Тип | Location | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Не | Непрозрачен курсор за пагинация, върнат като `nextCursor` от предишна заявка. Свързан е със същия `sortBy`. |
| limit | integer | query | Не | 1..200, по подразбиране 50 |
| q | string | query | Не | Опционален филтър по префикс на заглавие, нечувствителен към регистъра. |
| sortBy | string | query | Не | Режим на сортиране. `updatedAt` (по подразбиране, най-новите първи), `commentCount` (най-много коментари първи) или `title` (азбучно). |
| hasComments | boolean | query | Не | Ако е true, връща само страници с поне един коментар. |

## Отговор

Връща: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Задаването на host е незадължително и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с екземпляр на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте екземпляр на API класа
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Непрозрачен курсор за пагинация, върнат като `nextCursor` от предишна заявка. Свързан е със същия `sortBy`. (по избор)
    limit = 56 # int | 1..200, по подразбиране 50 (по избор)
    q = 'q_example' # str | Опционален филтър по префикс на заглавие, нечувствителен към регистъра. (по избор)
    sort_by = client.PagesSortBy() # PagesSortBy | Режим на сортиране. `updatedAt` (по подразбиране, най-новите първи), `commentCount` (най-много коментари първи) или `title` (азбучно). (по избор)
    has_comments = True # bool | Ако е true, връща само страници с поне един коментар. (по избор)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]