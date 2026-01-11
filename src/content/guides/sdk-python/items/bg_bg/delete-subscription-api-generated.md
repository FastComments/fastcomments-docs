---
## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| userId | string | query | Не |  |

## Отговор

Връща: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_subscription_api_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за delete_subscription'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_subscription_api_response import DeleteSubscriptionAPIResponse
from client.rest import ApiException
from pprint import pprint

# Задаването на host е незадължително и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани конфигурационни параметри.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за автентикация и упълномощаване
# в съответствие с политиката за сигурност на API сървъра.
# По-долу са дадени примери за всеки метод за автентикация; използвайте този пример, който
# отговаря на вашия случай на използване за автентикация.

# Конфигурирайте удостоверяване с API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Премахнете коментара по-долу, за да зададете префикс (напр. Bearer) за API ключа, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (по избор)

    try:
        api_response = api_instance.delete_subscription(tenant_id, id, user_id=user_id)
        print("The response of DefaultApi->delete_subscription:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_subscription: %s\n" % e)
[inline-code-end]

---