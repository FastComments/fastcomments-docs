Наводи странице за закупца (tenant). Користи FChat десктоп клијент за попуњавање своје листе соба.
Захтева да `enableFChat` буде true на резолвованој прилагођеној конфигурацији за сваку страницу.
Странице које захтевају SSO се филтрирају према приступу групи тражеог корисника.

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Не | Непровидни пагинациони курсор враћен као `nextCursor` из претходног захтева. Везан за исти `sortBy`. |
| limit | integer | query | Не | 1..200, подразумевано 50 |
| q | string | query | Не | Опциони филтер префикса наслова, без разликовања великих и малих слова. |
| sortBy | string | query | Не | Редослед сортирања. `updatedAt` (подразумевано, прво најновији), `commentCount` (прво они са највише коментара) или `title` (абецедно). |
| hasComments | boolean | query | Не | Ако је true, враћа само странице са бар једним коментаром. |

## Одговор

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

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
    cursor = 'cursor_example' # str | Непровидни пагинациони курсор враћен као `nextCursor` из претходног захтева. Везан за исти `sortBy`. (optional)
    limit = 56 # int | 1..200, подразумевано 50 (optional)
    q = 'q_example' # str | Опциони филтер префикса наслова, без разликовања великих и малих слова. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Редослед сортирања. `updatedAt` (подразумевано, прво најновији), `commentCount` (прво они са највише коментара) или `title` (абецедно). (optional)
    has_comments = True # bool | Ако је true, враћа само странице са бар једним коментаром. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]