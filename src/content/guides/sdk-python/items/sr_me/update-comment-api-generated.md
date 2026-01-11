## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| contextUserId | string | query | Не |  |
| doSpamCheck | boolean | query | Не |  |
| isLive | boolean | query | Не |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Примјер

[inline-code-attrs-start title = 'update_comment Примјер'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.models.pick_api_comment_updatable_comment_fields import PickAPICommentUpdatableCommentFields
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора конфигурисати параметре аутентификације и овлашћавања
# у складу са безбједносном политиком API сервера.
# Примјери за сваку методу аутентификације дата су испод, користите онај примјер који
# задовољава ваш случај коришћења аутентификације.

# Конфигуришите овлашћење API кључем: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Окоментишите испод да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    body = client.PickAPICommentUpdatableCommentFields() # PickAPICommentUpdatableCommentFields | 
    context_user_id = 'context_user_id_example' # str |  (опционално)
    do_spam_check = True # bool |  (опционално)
    is_live = True # bool |  (опционално)

    try:
        api_response = api_instance.update_comment(tenant_id, id, body, context_user_id=context_user_id, do_spam_check=do_spam_check, is_live=is_live)
        print("The response of DefaultApi->update_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_comment: %s\n" % e)
[inline-code-end]

---