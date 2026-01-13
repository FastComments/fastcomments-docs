## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Отговор

Връща: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Пример

[inline-code-attrs-start title = 'delete_email_template Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# Дефинирането на хоста е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за удостоверяване и упълномощаване
# в съответствие с политиката за сигурност на API сървъра.
# По-долу са предоставени примери за всеки метод на удостоверяване, използвайте примера, който
# отговаря на вашия случай на използване.

# Конфигурирайте удостоверяването с API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Разкоментирайте по-долу, за да зададете префикс (e.g. Bearer) за API ключа, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с екземпляр на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте екземпляр на класа API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.delete_email_template(tenant_id, id)
        print("The response of DefaultApi->delete_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_email_template: %s\n" % e)
[inline-code-end]

---