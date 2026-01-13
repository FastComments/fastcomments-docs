## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Одговор

Враћа: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_tenant'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant200_response import GetTenant200Response
from client.rest import ApiException
from pprint import pprint

# Подешавање host-а је опционо и подразумево је https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора подесити параметре аутентификације и ауторизације
# у складу са безбедносном политиком API сервера.
# Испод су наведени примери за сваки метод аутентификације, користите примјер који
# одговара вашем случају коришћења аутентификације.

# Конфигуришите овлашћење API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскоментаришите испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_tenant(tenant_id, id)
        print("The response of DefaultApi->get_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant: %s\n" % e)
[inline-code-end]