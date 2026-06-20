## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Отговор

Връща: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_email_template_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за create_email_template'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_email_template_body import CreateEmailTemplateBody
from client.models.create_email_template_response import CreateEmailTemplateResponse
from client.rest import ApiException
from pprint import pprint

# Дефинирането на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани параметри на конфигурацията.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за автентикация и авторизация
# в съответствие с политиката за сигурност на API сървъра.
# Примерите за всеки метод на автентикация са показани по-долу, използвайте примера, който
# отговаря на вашия случай на използване за автентикация.

# Конфигурирайте разрешаването чрез API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Разкоментрайте по-долу, за да зададете префикс (например Bearer) за API ключа, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_email_template_body = client.CreateEmailTemplateBody() # CreateEmailTemplateBody | 

    try:
        api_response = api_instance.create_email_template(tenant_id, create_email_template_body)
        print("The response of DefaultApi->create_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_email_template: %s\n" % e)
[inline-code-end]