## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Отговор

Връща: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_template_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_email_template'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_template_response import GetEmailTemplateResponse
from client.rest import ApiException
from pprint import pprint

# Задаването на хост е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани параметри на конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за удостоверяване и авторизация
# в съответствие с политиката за сигурност на API сървъра.
# Примерите за всеки метод на удостоверяване са дадени по-долу, използвайте примера, който
# отговаря на вашия случай на използване.

# Конфигурирайте удостоверяване с API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Премахнете коментара по-долу, за да зададете префикс (напр. Bearer) за API ключа, ако е необходим
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с инстанция на API клиент
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
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