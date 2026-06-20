## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| isLive | boolean | query | Не |  |
| doSpamCheck | boolean | query | Не |  |
| sendEmails | boolean | query | Не |  |
| populateNotifications | boolean | query | Не |  |

## Одговор

Враћа: [`SaveCommentsBulkResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/save_comments_bulk_response.py)

## Пример

[inline-code-attrs-start title = 'Пример save_comments_bulk'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_comment_params import CreateCommentParams
from client.models.save_comments_bulk_response import SaveCommentsBulkResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и овлашћења
# у складу са политиком безбедности API сервера.
# Испод су примери за сваки метод аутентификације, користите пример који
# одговара вашем случају употребе аутентификације.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = [client.CreateCommentParams()] # List[CreateCommentParams] | 
    is_live = True # bool |  (опционално)
    do_spam_check = True # bool |  (опционално)
    send_emails = True # bool |  (опционално)
    populate_notifications = True # bool |  (опционално)

    try:
        api_response = api_instance.save_comments_bulk(tenant_id, create_comment_params, is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications)
        print("The response of DefaultApi->save_comments_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->save_comments_bulk: %s\n" % e)
[inline-code-end]