## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Одговор

Враћа: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_create_user_badge_response.py)

## Пример

[inline-code-attrs-start title = 'Пример create_user_badge'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_create_user_badge_response import APICreateUserBadgeResponse
from client.models.create_user_badge_params import CreateUserBadgeParams
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционо и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора конфигурисати параметре аутентификације и ауторизације
# у складу са политиком безбедности API сервера.
# Примери за сваки метод аутентификације дати су испод, користите пример који
# одговара вашем случају коришћења аутентификације.

# Конфигуришите овлашћење API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Ако је потребно, уклоните коментар испод да подесите префикс (нпр. Bearer) за API кључ
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_user_badge_params = client.CreateUserBadgeParams() # CreateUserBadgeParams | 

    try:
        api_response = api_instance.create_user_badge(tenant_id, create_user_badge_params)
        print("The response of DefaultApi->create_user_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_user_badge: %s\n" % e)
[inline-code-end]