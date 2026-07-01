## Параметри

| Ім'я | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Yes |  |

## Відповідь

Повертає: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/bulk_create_hash_tags_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад add_hash_tags_bulk'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.bulk_create_hash_tags_body import BulkCreateHashTagsBody
from client.models.bulk_create_hash_tags_response import BulkCreateHashTagsResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста є необов'язковим і за замовчуванням https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки API сервера.
# Приклади для кожного методу автентифікації наведені нижче, використайте приклад, який
# відповідає вашому випадку використання автентифікації.

# Налаштування авторизації за допомогою API ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для API ключа, якщо необхідно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    bulk_create_hash_tags_body = client.BulkCreateHashTagsBody() # BulkCreateHashTagsBody |  (optional)

    try:
        api_response = api_instance.add_hash_tags_bulk(tenant_id, bulk_create_hash_tags_body)
        print("The response of DefaultApi->add_hash_tags_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tags_bulk: %s\n" % e)
[inline-code-end]