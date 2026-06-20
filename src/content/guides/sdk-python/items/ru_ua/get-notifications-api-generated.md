## Параметры

| Name | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Нет |  |
| urlId | string | query | Нет |  |
| fromCommentId | string | query | Нет |  |
| viewed | boolean | query | Нет |  |
| type | string | query | Нет |  |
| skip | number | query | Нет |  |

## Ответ

Возвращает: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notifications_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_notifications'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_notifications_response import GetNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# Задание host не обязательно и по умолчанию равно https://fastcomments.com
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

# Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, при необходимости
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Откройте контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (необязательно)
    url_id = 'url_id_example' # str |  (необязательно)
    from_comment_id = 'from_comment_id_example' # str |  (необязательно)
    viewed = True # bool |  (необязательно)
    type = 'type_example' # str |  (необязательно)
    skip = 3.4 # float |  (необязательно)

    try:
        api_response = api_instance.get_notifications(tenant_id, user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type, skip=skip)
        print("The response of DefaultApi->get_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notifications: %s\n" % e)
[inline-code-end]