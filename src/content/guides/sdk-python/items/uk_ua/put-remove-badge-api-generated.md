## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| badgeId | string | query | Так |  |
| userId | string | query | Ні |  |
| commentId | string | query | Ні |  |
| broadcastId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/remove_user_badge_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад put_remove_badge'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.remove_user_badge_response import RemoveUserBadgeResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове і за замовчуванням використовується https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Увійдіть у контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.ModerationApi(api_client)
    badge_id = 'badge_id_example' # str | 
    user_id = 'user_id_example' # str |  (необов'язково)
    comment_id = 'comment_id_example' # str |  (необов'язково)
    broadcast_id = 'broadcast_id_example' # str |  (необов'язково)
    sso = 'sso_example' # str |  (необов'язково)

    try:
        api_response = api_instance.put_remove_badge(badge_id, user_id=user_id, comment_id=comment_id, broadcast_id=broadcast_id, sso=sso)
        print("The response of ModerationApi->put_remove_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_remove_badge: %s\n" % e)
[inline-code-end]