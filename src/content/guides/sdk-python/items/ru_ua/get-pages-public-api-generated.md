Перечисляет страницы для тенанта. Используется десктопным клиентом FChat для заполнения списка комнат.
Требуется, чтобы `enableFChat` был true в окончательной пользовательской конфигурации для каждой страницы.
Страницы, требующие SSO, фильтруются с учётом группового доступа пользователя, выполняющего запрос.

## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Нет | Непрозрачный курсор постраничной навигации, возвращённый как `nextCursor` в предыдущем запросе. Привязан к тому же `sortBy`. |
| limit | integer | query | Нет | 1..200, по умолчанию 50 |
| q | string | query | Нет | Необязательный регистронезависимый фильтр по префиксу заголовка. |
| sortBy | string | query | Нет | Порядок сортировки. `updatedAt` (по умолчанию, сначала самые новые), `commentCount` (сначала страницы с наибольшим количеством комментариев), или `title` (по алфавиту). |
| hasComments | boolean | query | Нет | Если true, возвращать только страницы с хотя бы одним комментарием. |

## Ответ

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Указание хоста необязательно, по умолчанию используется https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Входим в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создаём экземпляр класса API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Непрозрачный курсор постраничной навигации, возвращённый как `nextCursor` в предыдущем запросе. Привязан к тому же `sortBy`. (необязательно)
    limit = 56 # int | 1..200, по умолчанию 50 (необязательно)
    q = 'q_example' # str | Необязательный регистронезависимый фильтр по префиксу заголовка. (необязательно)
    sort_by = client.PagesSortBy() # PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, сначала самые новые), `commentCount` (сначала страницы с наибольшим количеством комментариев), или `title` (по алфавиту). (необязательно)
    has_comments = True # bool | Если true, возвращать только страницы с хотя бы одним комментарием. (необязательно)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]

---