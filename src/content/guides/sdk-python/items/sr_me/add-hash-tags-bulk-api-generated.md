## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Не |  |

## Одговор

Враћа: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_hash_tags_bulk200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример add_hash_tags_bulk'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_hash_tags_bulk200_response import AddHashTagsBulk200Response
from client.models.bulk_create_hash_tags_body import BulkCreateHashTagsBody
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да подеси параметре аутентификације и овлашћења
# у складу са безбедносном политиком API сервера.
# Испод су примери за сваки метод аутентификације; користите пример који
# одговара вашем случају коришћења аутентификације.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (опционо)
    bulk_create_hash_tags_body = client.BulkCreateHashTagsBody() # BulkCreateHashTagsBody |  (опционо)

    try:
        api_response = api_instance.add_hash_tags_bulk(tenant_id=tenant_id, bulk_create_hash_tags_body=bulk_create_hash_tags_body)
        print("The response of DefaultApi->add_hash_tags_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tags_bulk: %s\n" % e)
[inline-code-end]