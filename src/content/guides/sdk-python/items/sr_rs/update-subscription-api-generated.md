## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| userId | string | query | Не |  |

## Одговор

Враћа: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_subscription_api_response.py)

## Пример

[inline-code-attrs-start title = 'Пример update_subscription'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_api_user_subscription_data import UpdateAPIUserSubscriptionData
from client.models.update_subscription_api_response import UpdateSubscriptionAPIResponse
from client.rest import ApiException
from pprint import pprint

# Постављање host-а је опционално и подразумева се https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и ауторизације
# у складу са политиком безбедности API сервера.
# Примери за сваки метод аутентификације су наведени у наставку, користите пример који
# одговара вашем случају употребе аутентификације.

# Конфигуришите API key ауторизацију: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Уклоните коментар испод да бисте подесили префикс (нпр. Bearer) за API key, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_api_user_subscription_data = client.UpdateAPIUserSubscriptionData() # UpdateAPIUserSubscriptionData | 
    user_id = 'user_id_example' # str |  (опционо)

    try:
        api_response = api_instance.update_subscription(tenant_id, id, update_api_user_subscription_data, user_id=user_id)
        print("The response of DefaultApi->update_subscription:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_subscription: %s\n" % e)
[inline-code-end]