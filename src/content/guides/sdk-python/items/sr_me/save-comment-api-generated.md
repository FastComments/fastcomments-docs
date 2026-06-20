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

[inline-code-attrs-start title = 'Пример save_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_save_comment_response import APISaveCommentResponse
from client.models.create_comment_params import CreateCommentParams
from client.rest import ApiException
from pprint import pprint

# Подешавање хоста је опционо и подразумева се на https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и овлашћивања
# у складу са безбедносном политиком API сервера.
# Примери за сваки метод аутентификације су дати у наставку, употребите пример који
# одговара вашем случају употребе за аутификацију.

# Конфигуришите API кључ за ауторизацију: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскоментаришите линију испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = client.CreateCommentParams() # CreateCommentParams | 
    is_live = True # bool |  (optional)
    do_spam_check = True # bool |  (optional)
    send_emails = True # bool |  (optional)
    populate_notifications = True # bool |  (optional)

    try:
        api_response = api_instance.save_comment(tenant_id, create_comment_params, is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications)
        print("The response of DefaultApi->save_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->save_comment: %s\n" % e)
[inline-code-end]