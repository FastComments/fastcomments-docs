## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | query | Да |  |
| direction | string | query | Да |  |
| userId | string | query | Не |  |
| anonUserId | string | query | Не |  |

## Одговор

Враћа: [`VoteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_response.py)

## Пример

[inline-code-attrs-start title = 'create_vote Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_response import VoteResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционално и подразумевано је на https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентикације и авторизације
# у складу са политиком безбедности API сервера.
# Испод су наведени примери за сваки метод аутентификације, користите пример који
# одговара вашем случају коришћења аутентификације.

# Конфигуришите авторизацију помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментирајте доле да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    direction = 'direction_example' # str | 
    user_id = 'user_id_example' # str |  (опционално)
    anon_user_id = 'anon_user_id_example' # str |  (опционално)

    try:
        api_response = api_instance.create_vote(tenant_id, comment_id, direction, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->create_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_vote: %s\n" % e)
[inline-code-end]

---