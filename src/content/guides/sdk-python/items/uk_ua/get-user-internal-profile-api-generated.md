## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| commentId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_internal_profile_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_user_internal_profile'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_internal_profile_response import GetUserInternalProfileResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове і за замовчуванням https://fastcomments.com
# Дивіться configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Увійдіть у контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str |  (необов'язково)
    sso = 'sso_example' # str |  (необов'язково)

    try:
        api_response = api_instance.get_user_internal_profile(comment_id=comment_id, sso=sso)
        print("The response of ModerationApi->get_user_internal_profile:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_user_internal_profile: %s\n" % e)
[inline-code-end]

---