## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Нет |  |
| badgeId | string | query | Нет |  |
| type | number | query | Нет |  |
| displayedOnComments | boolean | query | Нет |  |
| limit | number | query | Нет |  |
| skip | number | query | Нет |  |

## Ответ

Возвращает: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_badges200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_user_badges'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_badges200_response import GetUserBadges200Response
from client.rest import ApiException
from pprint import pprint

# Указание хоста необязательно и по умолчанию — https://fastcomments.com
# Смотрите configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Примеры для каждого метода аутентификации приведены ниже, используйте пример,
# который соответствует вашему сценарию аутентификации.

# Настройка авторизации по API ключу: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы при необходимости задать префикс (например, Bearer) для API ключа
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Входим в контекст с экземпляром API клиента
with client.ApiClient(configuration) as api_client:
    # Создаём экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (необязательно)
    badge_id = 'badge_id_example' # str |  (необязательно)
    type = 3.4 # float |  (необязательно)
    displayed_on_comments = True # bool |  (необязательно)
    limit = 3.4 # float |  (необязательно)
    skip = 3.4 # float |  (необязательно)

    try:
        api_response = api_instance.get_user_badges(tenant_id, user_id=user_id, badge_id=badge_id, type=type, displayed_on_comments=displayed_on_comments, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badges: %s\n" % e)
[inline-code-end]

---