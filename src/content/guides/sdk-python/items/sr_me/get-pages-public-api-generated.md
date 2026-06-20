---
Списак страница за tenant. Користи се од стране FChat десктоп клијента за попуњавање његове листе соба. Захтијева да `enableFChat` буде true у решеној прилагођеној конфигурацији за сваку страницу. Странице које захтијевају SSO су филтриране у складу са приступом групе корисника који прави захтјев.

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Не | Непрозирни курсор пагинације враћен као `nextCursor` из претходног захтјева. Везан за исти `sortBy`. |
| limit | integer | query | Не | 1..200, подразумевано 50 |
| q | string | query | Не | Опционо филтрирање префикса наслова које не разликује велика/мала слова. |
| sortBy | string | query | Не | Редослед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво), или `title` (алфабетски). |
| hasComments | boolean | query | Не | Ако је true, враћају се само странице са најмање једним коментаром. |

## Одговор

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Пример

[inline-code-attrs-start title = 'get_pages_public Example'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционо и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Непрозирни курсор пагинације враћен као `nextCursor` из претходног захтјева. Везан за исти `sortBy`. (опционо)
    limit = 56 # int | 1..200, подразумевано 50 (опционо)
    q = 'q_example' # str | Опционо филтрирање префикса наслова које не разликује велика/мала слова. (опционо)
    sort_by = client.PagesSortBy() # PagesSortBy | Редослед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво), или `title` (алфабетски). (опционо)
    has_comments = True # bool | Ако је true, враћају се само странице са најмање једним коментаром. (опционо)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]

---