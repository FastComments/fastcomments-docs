## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| contextUserId | string | query | Ні |  |
| doSpamCheck | boolean | query | Ні |  |
| isLive | boolean | query | Ні |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад update_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import UpdateCommentOptions
from client.models.api_empty_response import APIEmptyResponse
from client.models.updatable_comment_params import UpdatableCommentParams
from client.rest import ApiException
from pprint import pprint

# Визначення хосту є необов'язковим і за замовчуванням https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
# Клієнт повинен налаштувати параметри автентифікації та авторизації згідно з політикою безпеки сервера API.
# Приклади для кожного методу автентифікації наведені нижче, використайте приклад, який
# відповідає вашому випадку використання автентифікації.
# Налаштування авторизації за допомогою API ключа: api_key
# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Відкрити контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створити екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    updatable_comment_params = client.UpdatableCommentParams() # UpdatableCommentParams | 
    context_user_id = 'context_user_id_example' # str |  (необов'язково)
    do_spam_check = True # bool |  (необов'язково)
    is_live = True # bool |  (необов'язково)

    try:
        api_response = api_instance.update_comment(tenant_id, id, updatable_comment_params, UpdateCommentOptions(context_user_id=context_user_id, do_spam_check=do_spam_check, is_live=is_live))
        print("The response of DefaultApi->update_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_comment: %s\n" % e)
[inline-code-end]