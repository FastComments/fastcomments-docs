## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| userId | string | query | Нет |  |
| anonUserId | string | query | Нет |  |

## Ответ

Возвращает: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_response.py)

## Пример

[inline-code-attrs-start title = 'Пример flag_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_response import FlagCommentResponse
from client.rest import ApiException
from pprint import pprint

# Определение host необязательно, по умолчанию используется https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Примеры для каждого метода аутентификации приведены ниже, используйте пример,
# который соответствует вашему сценарию аутентификации.

# Настройте авторизацию с помощью API-ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы задать префикс (например, Bearer) для API-ключа, если это необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (необязательно)
    anon_user_id = 'anon_user_id_example' # str |  (необязательно)

    try:
        api_response = api_instance.flag_comment(tenant_id, id, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->flag_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->flag_comment: %s\n" % e)
[inline-code-end]