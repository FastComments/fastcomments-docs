---
## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Так |  |
| voteId | string | path | Так |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_delete_response.py)

## Приклад

[inline-code-attrs-start title = 'delete_moderation_vote Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_delete_response import VoteDeleteResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хосту необов'язкове та за замовчуванням встановлено на https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Увійдіть у контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    vote_id = 'vote_id_example' # str | 
    sso = 'sso_example' # str |  (необов'язково)

    try:
        api_response = api_instance.delete_moderation_vote(comment_id, vote_id, sso=sso)
        print("The response of ModerationApi->delete_moderation_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->delete_moderation_vote: %s\n" % e)
[inline-code-end]

---