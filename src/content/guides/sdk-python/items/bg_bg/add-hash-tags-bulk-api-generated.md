## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Не |  |

## Отговор

Връща: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_hash_tags_bulk200_response.py)

## Пример

[inline-code-attrs-start title = 'add_hash_tags_bulk Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_hash_tags_bulk200_response import AddHashTagsBulk200Response
from client.models.bulk_create_hash_tags_body import BulkCreateHashTagsBody
from client.rest import ApiException
from pprint import pprint

# Задаването на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани конфигурационни параметри.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за удостоверяване и авторизация
# в съответствие с политиката за сигурност на API сървъра.
# Примерите за всеки метод на удостоверяване са предоставени по-долу, използвайте примера,
# който отговаря на вашия случай на удостоверяване.
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Разкоментирайте по-долу, за да зададете префикс (например Bearer) за API ключа, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с екземпляр на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте екземпляр на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (optional)
    bulk_create_hash_tags_body = client.BulkCreateHashTagsBody() # BulkCreateHashTagsBody |  (optional)

    try:
        api_response = api_instance.add_hash_tags_bulk(tenant_id=tenant_id, bulk_create_hash_tags_body=bulk_create_hash_tags_body)
        print("The response of DefaultApi->add_hash_tags_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tags_bulk: %s\n" % e)
[inline-code-end]