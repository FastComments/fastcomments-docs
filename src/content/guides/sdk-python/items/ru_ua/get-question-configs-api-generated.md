## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| skip | number | query | Нет |  |

## Ответ

Возвращает: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_configs200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_question_configs'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_configs200_response import GetQuestionConfigs200Response
from client.rest import ApiException
from pprint import pprint

# Указание host необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Примеры для каждого метода аутентификации приведены ниже, используйте пример, который
# соответствует вашему сценарию аутентификации.

# Настройка авторизации по API-ключу: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы задать префикс (например, Bearer) для API-ключа, если нужно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Откройте контекст с экземпляром клиента API
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (необязательно)

    try:
        api_response = api_instance.get_question_configs(tenant_id, skip=skip)
        print("The response of DefaultApi->get_question_configs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_configs: %s\n" % e)
[inline-code-end]