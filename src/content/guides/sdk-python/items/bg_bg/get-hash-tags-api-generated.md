## Параметри

| Име | Тип | Location | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| page | number | query | Не |  |

## Отговор

Връща: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_hash_tags_response.py)

## Пример

[inline-code-attrs-start title = 'Пример на get_hash_tags'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_hash_tags_response import GetHashTagsResponse
from client.rest import ApiException
from pprint import pprint

# Дефинирането на хост е незадължително и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за удостоверяване и авторизация
# в съответствие с политиката за сигурност на API сървъра.
# Примерите за всеки метод за удостоверяване са показани по-долу, използвайте примера, 
# който отговаря на вашия случай на използване за удостоверяване.

# Конфигурирайте удостоверяване чрез API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Премахнете коментара по-долу, за да настроите префикс (e.g. Bearer) за API ключа, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 3.4 # float |  (незадължително)

    try:
        api_response = api_instance.get_hash_tags(tenant_id, page=page)
        print("The response of DefaultApi->get_hash_tags:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_hash_tags: %s\n" % e)
[inline-code-end]