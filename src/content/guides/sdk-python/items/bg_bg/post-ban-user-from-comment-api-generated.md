## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| banEmail | boolean | query | Не |  |
| banEmailDomain | boolean | query | Не |  |
| banIP | boolean | query | Не |  |
| deleteAllUsersComments | boolean | query | Не |  |
| bannedUntil | string | query | Не |  |
| isShadowBan | boolean | query | Не |  |
| updateId | string | query | Не |  |
| banReason | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/ban_user_from_comment_result.py)

## Пример

[inline-code-attrs-start title = 'Пример post_ban_user_from_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.ban_user_from_comment_result import BanUserFromCommentResult
from client.rest import ApiException
from pprint import pprint

# Дефинирането на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани параметри на конфигурацията.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    ban_email = True # bool |  (по избор)
    ban_email_domain = True # bool |  (по избор)
    ban_ip = True # bool |  (по избор)
    delete_all_users_comments = True # bool |  (по избор)
    banned_until = 'banned_until_example' # str |  (по избор)
    is_shadow_ban = True # bool |  (по избор)
    update_id = 'update_id_example' # str |  (по избор)
    ban_reason = 'ban_reason_example' # str |  (по избор)
    sso = 'sso_example' # str |  (по избор)

    try:
        api_response = api_instance.post_ban_user_from_comment(comment_id, ban_email=ban_email, ban_email_domain=ban_email_domain, ban_ip=ban_ip, delete_all_users_comments=delete_all_users_comments, banned_until=banned_until, is_shadow_ban=is_shadow_ban, update_id=update_id, ban_reason=ban_reason, sso=sso)
        print("The response of ModerationApi->post_ban_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_from_comment: %s\n" % e)
[inline-code-end]