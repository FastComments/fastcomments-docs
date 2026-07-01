## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | запрос | Да |  |
| userId | string | запрос | Нет |  |
| urlId | string | запрос | Нет |  |
| fromCommentId | string | запрос | Нет |  |
| viewed | boolean | запрос | Нет |  |
| type | string | запрос | Нет |  |
| skip | number | запрос | Нет |  |

## Ответ

Возвращает: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notifications_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_notifications'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetNotificationsOptions
from client.models.get_notifications_response import GetNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# Определение хоста является необязательным и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности API‑сервера.
# Примеры для каждого метода аутентификации приведены ниже, используйте пример,
# который соответствует вашему случаю использования аутентификации.

# Настройка авторизации API‑ключом: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы задать префикс (например, Bearer) для API‑ключа, если необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войдите в контекст с экземпляром API‑клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (опционально)
    url_id = 'url_id_example' # str |  (опционально)
    from_comment_id = 'from_comment_id_example' # str |  (опционально)
    viewed = True # bool |  (опционально)
    type = 'type_example' # str |  (опционально)
    skip = 3.4 # float |  (опционально)

    try:
        api_response = api_instance.get_notifications(tenant_id, GetNotificationsOptions(user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type, skip=skip))
        print("The response of DefaultApi->get_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notifications: %s\n" % e)
[inline-code-end]