## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| skip | integer | query | Не |  |

## Отговор

Връща: [`GetSSOUsers200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_sso_users200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_sso_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_sso_users200_response import GetSSOUsers200Response
from client.rest import ApiException
from pprint import pprint

# Дефинирането на хоста е незадължително и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за автентикация и овластяване
# в съответствие с политиката за сигурност на API сървъра.
# Примерите за всеки метод за автентикация са показани по-долу, използвайте примера който
# отговаря на вашия случай на използване за автентикация.

# Конфигурирайте авторизация с API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментирайте по-долу, за да зададете префикс (например Bearer) за API ключа, ако е необходим
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 56 # int |  (незадължително)

    try:
        api_response = api_instance.get_sso_users(tenant_id, skip=skip)
        print("The response of DefaultApi->get_sso_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_sso_users: %s\n" % e)
[inline-code-end]

---