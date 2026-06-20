---
## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| banEmail | boolean | query | 否 |  |
| banEmailDomain | boolean | query | 否 |  |
| banIP | boolean | query | 否 |  |
| deleteAllUsersComments | boolean | query | 否 |  |
| bannedUntil | string | query | 否 |  |
| isShadowBan | boolean | query | 否 |  |
| updateId | string | query | 否 |  |
| banReason | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

Returns: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/ban_user_from_comment_result.py)

## 範例

[inline-code-attrs-start title = 'post_ban_user_from_comment 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.ban_user_from_comment_result import BanUserFromCommentResult
from client.rest import ApiException
from pprint import pprint

# 定義主機為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 與 API 用戶端實例一起進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    ban_email = True # bool |  (可選)
    ban_email_domain = True # bool |  (可選)
    ban_ip = True # bool |  (可選)
    delete_all_users_comments = True # bool |  (可選)
    banned_until = 'banned_until_example' # str |  (可選)
    is_shadow_ban = True # bool |  (可選)
    update_id = 'update_id_example' # str |  (可選)
    ban_reason = 'ban_reason_example' # str |  (可選)
    sso = 'sso_example' # str |  (可選)

    try:
        api_response = api_instance.post_ban_user_from_comment(comment_id, ban_email=ban_email, ban_email_domain=ban_email_domain, ban_ip=ban_ip, delete_all_users_comments=delete_all_users_comments, banned_until=banned_until, is_shadow_ban=is_shadow_ban, update_id=update_id, ban_reason=ban_reason, sso=sso)
        print("The response of ModerationApi->post_ban_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_from_comment: %s\n" % e)
[inline-code-end]

---