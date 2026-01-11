## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| postId | string | path | Да |  |
| isUndo | boolean | query | Не |  |
| broadcastId | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/react_feed_post_public200_response.py)

## Пример

[inline-code-attrs-start title = 'react_feed_post_public Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.react_body_params import ReactBodyParams
from client.models.react_feed_post_public200_response import ReactFeedPostPublic200Response
from client.rest import ApiException
from pprint import pprint

# Постављање host-а је опционално и подразумева се https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_id = 'post_id_example' # str | 
    react_body_params = client.ReactBodyParams() # ReactBodyParams | 
    is_undo = True # bool |  (опционо)
    broadcast_id = 'broadcast_id_example' # str |  (опционо)
    sso = 'sso_example' # str |  (опционо)

    try:
        api_response = api_instance.react_feed_post_public(tenant_id, post_id, react_body_params, is_undo=is_undo, broadcast_id=broadcast_id, sso=sso)
        print("The response of PublicApi->react_feed_post_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->react_feed_post_public: %s\n" % e)
[inline-code-end]