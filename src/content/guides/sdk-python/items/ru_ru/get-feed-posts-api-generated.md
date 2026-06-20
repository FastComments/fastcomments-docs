req
tenantId
afterId

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| afterId | string | query | Нет |  |
| limit | integer | query | Нет |  |
| tags | array | query | Нет |  |

## Ответ

Возвращает: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_feed_posts'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_feed_posts_response import GetFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# Определение host не обязательно и по умолчанию https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности API-сервера.
# Ниже приведены примеры для каждого метода аутентификации, используйте пример, который
# соответствует вашему сценарию аутентификации.

# Настройте авторизацию по API-ключу: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы задать префикс (e.g. Bearer) для API-ключа, если требуется
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (необязательно)
    limit = 56 # int |  (необязательно)
    tags = ['tags_example'] # List[str] |  (необязательно)

    try:
        api_response = api_instance.get_feed_posts(tenant_id, after_id=after_id, limit=limit, tags=tags)
        print("The response of DefaultApi->get_feed_posts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_feed_posts: %s\n" % e)
[inline-code-end]