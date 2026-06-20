## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Так |  |
| banEmail | boolean | query | Ні |  |
| banEmailDomain | boolean | query | Ні |  |
| banIP | boolean | query | Ні |  |
| deleteAllUsersComments | boolean | query | Ні |  |
| bannedUntil | string | query | Ні |  |
| isShadowBan | boolean | query | Ні |  |
| updateId | string | query | Ні |  |
| banReason | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/ban_user_from_comment_result.py)

## Приклад

[inline-code-attrs-start title = 'Приклад використання post_ban_user_from_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.ban_user_from_comment_result import BanUserFromCommentResult
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове і за замовчуванням використовується https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Увійти в контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створюємо екземпляр класу API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    ban_email = True # bool |  (необов'язково)
    ban_email_domain = True # bool |  (необов'язково)
    ban_ip = True # bool |  (необов'язково)
    delete_all_users_comments = True # bool |  (необов'язково)
    banned_until = 'banned_until_example' # str |  (необов'язково)
    is_shadow_ban = True # bool |  (необов'язково)
    update_id = 'update_id_example' # str |  (необов'язково)
    ban_reason = 'ban_reason_example' # str |  (необов'язково)
    sso = 'sso_example' # str |  (необов'язково)

    try:
        api_response = api_instance.post_ban_user_from_comment(comment_id, ban_email=ban_email, ban_email_domain=ban_email_domain, ban_ip=ban_ip, delete_all_users_comments=delete_all_users_comments, banned_until=banned_until, is_shadow_ban=is_shadow_ban, update_id=update_id, ban_reason=ban_reason, sso=sso)
        print("The response of ModerationApi->post_ban_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_from_comment: %s\n" % e)
[inline-code-end]