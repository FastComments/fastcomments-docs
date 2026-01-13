---
## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| email | string | path | Да |  |

## Ответ

Возвращает: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_sso_user_by_email_api_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_sso_user_by_email'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_sso_user_by_email_api_response import GetSSOUserByEmailAPIResponse
from client.rest import ApiException
from pprint import pprint

# Указание host необязательно и по умолчанию равно https://fastcomments.com
# Смотрите configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Ниже приведены примеры для каждого метода аутентификации, используйте пример, который
# соответствует вашему случаю аутентификации.

# Настройте авторизацию по API-ключу: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, если требуется
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    email = 'email_example' # str | 

    try:
        api_response = api_instance.get_sso_user_by_email(tenant_id, email)
        print("The response of DefaultApi->get_sso_user_by_email:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_sso_user_by_email: %s\n" % e)
[inline-code-end]

---