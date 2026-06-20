## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Не |  |
| limit | number | query | Не |  |
| skip | number | query | Не |  |

## Одговор

Враћа: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badge_progress_list_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_user_badge_progress_list'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badge_progress_list_response import APIGetUserBadgeProgressListResponse
from client.rest import ApiException
from pprint import pprint

# Постављање host-а је опционо и подразумевано је https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и овлашћења
# у складу са безбедносном политиком API сервера.
# Примери за сваки метод аутентификације дати су испод, користите пример који
# одговара вашем случају употребе аутентификације.

# Конфигуришите овлашћење помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Ако је потребно, уклоните коментар испод да бисте подесили префикс (нпр. Bearer) за API кључ
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (опционо)
    limit = 3.4 # float |  (опционо)
    skip = 3.4 # float |  (опционо)

    try:
        api_response = api_instance.get_user_badge_progress_list(tenant_id, user_id=user_id, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badge_progress_list:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge_progress_list: %s\n" % e)
[inline-code-end]