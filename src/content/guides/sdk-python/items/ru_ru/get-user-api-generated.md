## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Ответ

Возвращает: [`GetUser200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user200_response import GetUser200Response
from client.rest import ApiException
from pprint import pprint

# Указание хоста необязательно и по умолчанию — https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Примеры для каждого метода аутентификации приведены ниже, используйте пример,
# который соответствует вашему сценарию аутентификации.

# Настройте авторизацию по ключу API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте строку ниже, чтобы задать префикс (например, Bearer) для ключа API, если это необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войти в контекст с экземпляром клиента API
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_user(tenant_id, id)
        print("The response of DefaultApi->get_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user: %s\n" % e)
[inline-code-end]