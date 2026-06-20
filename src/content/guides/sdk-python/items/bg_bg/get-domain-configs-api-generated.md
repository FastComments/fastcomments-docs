## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Отговор

Връща: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_domain_configs_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_domain_configs'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_domain_configs_response import GetDomainConfigsResponse
from client.rest import ApiException
from pprint import pprint

# Дефинирането на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани конфигурационни параметри.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за удостоверяване и авторизация
# в съответствие с политиката за сигурност на API сървъра.
# По-долу са дадени примери за всеки метод на удостоверяване, използвайте примера, който
# отговаря на вашия случай на използване.

# Конфигурирайте удостоверяване чрез API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Премахнете коментара отдолу, за да зададете префикс (например Bearer) за API ключа, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 

    try:
        api_response = api_instance.get_domain_configs(tenant_id)
        print("The response of DefaultApi->get_domain_configs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_domain_configs: %s\n" % e)
[inline-code-end]