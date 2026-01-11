req
tenantId
afterId

## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| afterId | string | query | Нет |  |
| limit | integer | query | Нет |  |
| tags | array | query | Нет |  |

## Ответ

Возвращает: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_feed_posts'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_feed_posts200_response import GetFeedPosts200Response
from client.rest import ApiException
from pprint import pprint

# Указание host необязательно и по умолчанию равно https://fastcomments.com
# Смотрите configuration.py для списка всех поддерживаемых параметров конфигурации.
# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Ниже приведены примеры для каждого метода аутентификации, используйте пример, который
# подходит для вашего варианта использования аутентификации.
# Настроить авторизацию по API-ключу: api_key
# Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, если необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войдите в контекст с экземпляром клиента API
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