## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| page | integer | query | Нет |  |
| limit | integer | query | Нет |  |
| skip | integer | query | Нет |  |
| asTree | boolean | query | Нет |  |
| skipChildren | integer | query | Нет |  |
| limitChildren | integer | query | Нет |  |
| maxTreeDepth | integer | query | Нет |  |
| urlId | string | query | Нет |  |
| userId | string | query | Нет |  |
| anonUserId | string | query | Нет |  |
| contextUserId | string | query | Нет |  |
| hashTag | string | query | Нет |  |
| parentId | string | query | Нет |  |
| direction | string | query | Нет |  |

## Ответ

Возвращает: [`GetComments200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_comments'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments200_response import GetComments200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Указание host необязательно и по умолчанию https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Примеры для каждого метода аутентификации приведены ниже, используйте пример, который
# соответствует вашему сценарию использования аутентификации.

# Настройка авторизации по API-ключу: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы задать префикс (например, Bearer) для API-ключа, если необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Откройте контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (необязательно)
    limit = 56 # int |  (необязательно)
    skip = 56 # int |  (необязательно)
    as_tree = True # bool |  (необязательно)
    skip_children = 56 # int |  (необязательно)
    limit_children = 56 # int |  (необязательно)
    max_tree_depth = 56 # int |  (необязательно)
    url_id = 'url_id_example' # str |  (необязательно)
    user_id = 'user_id_example' # str |  (необязательно)
    anon_user_id = 'anon_user_id_example' # str |  (необязательно)
    context_user_id = 'context_user_id_example' # str |  (необязательно)
    hash_tag = 'hash_tag_example' # str |  (необязательно)
    parent_id = 'parent_id_example' # str |  (необязательно)
    direction = client.SortDirections() # SortDirections |  (необязательно)

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]