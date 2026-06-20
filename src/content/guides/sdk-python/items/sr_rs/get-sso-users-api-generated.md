## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| skip | integer | query | Не |  |

## Одговор

Враћа: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_sso_users_response.py)

## Пример

[inline-code-attrs-start title = 'get_sso_users Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_sso_users_response import GetSSOUsersResponse
from client.rest import ApiException
from pprint import pprint

# Постављање host-а је опционално и подразумева се https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних параметара конфигурације.
# Клијент мора да подеси параметре аутентификације и ауторизације
# у складу са безбедносном политиком API сервера.
# Испод су дати примери за сваки метод аутентификације; користите пример који
# одговара вашем случају коришћења аутентификације.
# Configure API key authorization: api_key
# Откоментирајте испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
# Уђите у контекст са инстанцом API клијента
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 56 # int |  (optional)

    try:
        api_response = api_instance.get_sso_users(tenant_id, skip=skip)
        print("The response of DefaultApi->get_sso_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_sso_users: %s\n" % e)
[inline-code-end]

---