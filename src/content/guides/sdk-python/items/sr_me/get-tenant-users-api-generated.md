## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| skip | number | query | Не |  |

## Одговор

Враћа: [`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_users200_response.py)

## Пример

[inline-code-attrs-start title = 'get_tenant_users Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_users200_response import GetTenantUsers200Response
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционално и подразумева се https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и овлашћења
# у складу са безбедносном политиком API сервера.
# Испод су наведени примери за сваку методу аутентификације, користите онај пример који
# одговара вашем случају коришћења аутентификације.

# Подешавање ауторизације помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Ако је потребно, уклоните коментар испод да бисте подесили префикс (нпр. Bearer) за API кључ
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_tenant_users(tenant_id, skip=skip)
        print("The response of DefaultApi->get_tenant_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_users: %s\n" % e)
[inline-code-end]

---