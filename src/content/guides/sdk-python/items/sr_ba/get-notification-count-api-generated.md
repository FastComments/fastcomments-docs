## Параметри

| Name | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Не |  |
| urlId | string | query | Не |  |
| fromCommentId | string | query | Не |  |
| viewed | boolean | query | Не |  |
| type | string | query | Не |  |

## Одговор

Враћа: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notification_count200_response.py)

## Примјер

[inline-code-attrs-start title = 'get_notification_count Примјер'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_notification_count200_response import GetNotificationCount200Response
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционално и подразумјевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора конфигурисати параметре аутентификације и овлашћења
# у складу са политиком безбједности API сервера.
# Примјери за сваки метод аутентификације су испод, користите примјер који
# одговара вашем случају употребе аутентификације.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментирајте испод да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (опционо)
    url_id = 'url_id_example' # str |  (опционо)
    from_comment_id = 'from_comment_id_example' # str |  (опционо)
    viewed = True # bool |  (опционо)
    type = 'type_example' # str |  (опционо)

    try:
        api_response = api_instance.get_notification_count(tenant_id, user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type)
        print("The response of DefaultApi->get_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notification_count: %s\n" % e)
[inline-code-end]