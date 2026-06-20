---
## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| direction | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`VoteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_response.py)

## Пример

[inline-code-attrs-start title = 'post_vote Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_response import VoteResponse
from client.rest import ApiException
from pprint import pprint

# Задаването на хоста е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с екземпляр на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте екземпляр на API класа
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    direction = 'direction_example' # str |  (по избор)
    sso = 'sso_example' # str |  (по избор)

    try:
        api_response = api_instance.post_vote(comment_id, direction=direction, sso=sso)
        print("The response of ModerationApi->post_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_vote: %s\n" % e)
[inline-code-end]

---