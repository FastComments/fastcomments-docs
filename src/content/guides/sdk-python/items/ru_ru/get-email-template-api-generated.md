## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Ответ

Возвращает: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_template_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_email_template'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_template_response import GetEmailTemplateResponse
from client.rest import ApiException
from pprint import pprint

# Определение хоста необязательно и по умолчанию https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Ниже приведены примеры для каждого метода аутентификации, используйте пример, который
# соответствует вашему сценарию использования аутентификации.

# Настройте авторизацию по API-ключу: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_email_template(tenant_id, id)
        print("The response of DefaultApi->get_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_template: %s\n" % e)
[inline-code-end]