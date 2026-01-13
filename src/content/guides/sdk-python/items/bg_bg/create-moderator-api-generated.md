## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Отговор

Връща: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_moderator200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за create_moderator'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_moderator200_response import CreateModerator200Response
from client.models.create_moderator_body import CreateModeratorBody
from client.rest import ApiException
from pprint import pprint

# Задаването на host е незадължително и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани конфигурационни параметри.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за удостоверяване и авторизация
# в съответствие с политиката за сигурност на API сървъра.
# Примерите за всеки метод за удостоверяване са показани по-долу; използвайте примера, който
# отговаря на вашия случай на употреба за удостоверяване.

# Конфигурирайте упълномощаването с API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Разкоментирайте по-долу, за да зададете префикс (например Bearer) за API ключ, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_moderator_body = client.CreateModeratorBody() # CreateModeratorBody | 

    try:
        api_response = api_instance.create_moderator(tenant_id, create_moderator_body)
        print("The response of DefaultApi->create_moderator:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_moderator: %s\n" % e)
[inline-code-end]

---