## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| sendEmail | string | query | Не |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за delete_moderator'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Дефинирането на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани конфигурационни параметри.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за удостоверяване и упълномощаване
# в съответствие с политиката за сигурност на API сървъра.
# Примери за всеки метод за удостоверяване са дадени по-долу, използвайте примера който
# удовлетворява вашия случай на използване за удостоверяване.

# Конфигурирайте удостоверяване чрез API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с екземпляр на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте екземпляр на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    send_email = 'send_email_example' # str |  (optional)

    try:
        api_response = api_instance.delete_moderator(tenant_id, id, send_email=send_email)
        print("The response of DefaultApi->delete_moderator:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_moderator: %s\n" % e)
[inline-code-end]