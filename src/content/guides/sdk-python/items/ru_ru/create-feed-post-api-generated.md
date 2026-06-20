---
## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| broadcastId | string | query | Нет |  |
| isLive | boolean | query | Нет |  |
| doSpamCheck | boolean | query | Нет |  |
| skipDupCheck | boolean | query | Нет |  |

## Ответ

Возвращает: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_feed_posts_response.py)

## Пример

[inline-code-attrs-start title = 'Пример create_feed_post'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_feed_post_params import CreateFeedPostParams
from client.models.create_feed_posts_response import CreateFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# Указание host необязательно, по умолчанию используется https://fastcomments.com
# Смотрите configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Примеры для каждого метода аутентификации приведены ниже, используйте пример, который
# удовлетворяет вашему сценарию аутентификации.

# Настройка авторизации по API-ключу: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, если это необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_feed_post_params = client.CreateFeedPostParams() # CreateFeedPostParams | 
    broadcast_id = 'broadcast_id_example' # str |  (optional)
    is_live = True # bool |  (optional)
    do_spam_check = True # bool |  (optional)
    skip_dup_check = True # bool |  (optional)

    try:
        api_response = api_instance.create_feed_post(tenant_id, create_feed_post_params, broadcast_id=broadcast_id, is_live=is_live, do_spam_check=do_spam_check, skip_dup_check=skip_dup_check)
        print("The response of DefaultApi->create_feed_post:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_feed_post: %s\n" % e)
[inline-code-end]

---