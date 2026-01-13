## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| broadcastId | string | query | No |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| skipDupCheck | boolean | query | No |  |

## Одговор

Враћа: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_feed_post200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример create_feed_post'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_feed_post200_response import CreateFeedPost200Response
from client.models.create_feed_post_params import CreateFeedPostParams
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Постављање host-а је опционално и подразумевано је https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
# Погледајте configuration.py за списак свих подржаних конфигурационих параметара.
# The client must configure the authentication and authorization parameters
# Клијент мора да подеси параметре за аутентификацију и ауторизацију
# in accordance with the API server security policy.
# у складу са политиком безбедности API сервера.
# Examples for each auth method are provided below, use the example that
# Примери за сваки метод аутентификације су дати у наставку, користите пример који
# satisfies your auth use case.
# одговара вашем случају употребе аутентификације.
# Configure API key authorization: api_key
# Конфигуришите API кључ за ауторизацију: api_key
# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Откоментирајте испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно

# Enter a context with an instance of the API client
# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_feed_post_params = client.CreateFeedPostParams() # CreateFeedPostParams | 
    broadcast_id = 'broadcast_id_example' # str |  (optional)
    is_live = True # bool |  (optional)
    do_spam_check = True # bool |  (optional)
    skip_dup_check = True # bool |  (optional)

    try:
        api_response = api_instance.create_feed_post(tenant_id, create_feed_post_params, broadcast_id=broadcast_id, is_live=is_live, do_spam_check=do_spam_check, skip_dup_check=skip_dup_check)
        print("The response of DefaultApi->create_feed_post:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_feed_post: %s\n" % e)
[inline-code-end]