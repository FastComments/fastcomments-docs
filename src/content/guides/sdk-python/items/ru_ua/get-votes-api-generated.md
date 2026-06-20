## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Да |  |

## Ответ

Возвращает: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_votes_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_votes'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_votes_response import GetVotesResponse
from client.rest import ApiException
from pprint import pprint

# Задание хоста необязательно — по умолчанию используется https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Ниже приведены примеры для каждого метода аутентификации, используйте тот
# который соответствует вашему сценарию аутентификации.

# Настройка авторизации по API-ключу: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы задать префикс (например, Bearer) для API-ключа, если это необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Откройте контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_votes(tenant_id, url_id)
        print("The response of DefaultApi->get_votes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_votes: %s\n" % e)
[inline-code-end]