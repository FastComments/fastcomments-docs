## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| direction | string | query | No |  |
| sso | string | query | No |  |

## Одговор

Враћа: [`VoteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_response.py)

## Пример

[inline-code-attrs-start title = 'post_vote Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_response import VoteResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционо и подразумева се https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API client-а
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    direction = 'direction_example' # str |  (опционо)
    sso = 'sso_example' # str |  (опционо)

    try:
        api_response = api_instance.post_vote(comment_id, direction=direction, sso=sso)
        print("The response of ModerationApi->post_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_vote: %s\n" % e)
[inline-code-end]