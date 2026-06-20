## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Пример

[inline-code-attrs-start title = 'Пример update_email_template'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_email_template_body import UpdateEmailTemplateBody
from client.rest import ApiException
from pprint import pprint

# Указание хоста не обязательно; по умолчанию используется https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности API-сервера.
# Примеры для каждого метода аутентификации приведены ниже. Используйте пример, который
# соответствует вашему сценарию использования аутентификации.

# Настройте авторизацию по ключу API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для ключа API, если это необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Откройте контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_email_template_body = client.UpdateEmailTemplateBody() # UpdateEmailTemplateBody | 

    try:
        api_response = api_instance.update_email_template(tenant_id, id, update_email_template_body)
        print("The response of DefaultApi->update_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_email_template: %s\n" % e)
[inline-code-end]