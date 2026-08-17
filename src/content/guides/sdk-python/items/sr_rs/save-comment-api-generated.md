## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| isLive | boolean | query | Не |  |
| doSpamCheck | boolean | query | Не |  |
| sendEmails | boolean | query | Не |  |
| populateNotifications | boolean | query | Не |  |

## Одговор

Враћа: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_save_comment_response.py)

## Пример

[inline-code-attrs-start title = 'save_comment Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import SaveCommentOptions
from client.models.api_save_comment_response import APISaveCommentResponse
from client.models.create_comment_params import CreateCommentParams
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционо и подразумева https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
# Клијент мора да конфигурише параметре аутентификације и ауторизације
# у складу са безбедносном политиком API сервера.
# Примери за сваки метод аутентификације су дати испод, користите пример који
# задовољава ваш случај употребе аутентификације.

# Конфигуришите ауторизацију API кључа: api_key
# Одкоментаришите испод да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = client.CreateCommentParams() # CreateCommentParams | 
    is_live = True # bool |  (optional)
    do_spam_check = True # bool |  (optional)
    send_emails = True # bool |  (optional)
    populate_notifications = True # bool |  (optional)

    try:
        api_response = api_instance.save_comment(tenant_id, create_comment_params, SaveCommentOptions(is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications))
        print("The response of DefaultApi->save_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->save_comment: %s\n" % e)
[inline-code-end]