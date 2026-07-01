## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| banEmail | boolean | query | No |  |
| banEmailDomain | boolean | query | No |  |
| banIP | boolean | query | No |  |
| deleteAllUsersComments | boolean | query | No |  |
| bannedUntil | string | query | No |  |
| isShadowBan | boolean | query | No |  |
| updateId | string | query | No |  |
| banReason | string | query | No |  |
| sso | string | query | No |  |

## 응답

반환: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/ban_user_from_comment_result.py)

## 예제

[inline-code-attrs-start title = 'post_ban_user_from_comment 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostBanUserFromCommentOptions
from client.models.ban_user_from_comment_result import BanUserFromCommentResult
from client.rest import ApiException
from pprint import pprint

# 호스트를 정의하는 것은 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하십시오.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    ban_email = True # bool |  (옵션)
    ban_email_domain = True # bool |  (옵션)
    ban_ip = True # bool |  (옵션)
    delete_all_users_comments = True # bool |  (옵션)
    banned_until = 'banned_until_example' # str |  (옵션)
    is_shadow_ban = True # bool |  (옵션)
    update_id = 'update_id_example' # str |  (옵션)
    ban_reason = 'ban_reason_example' # str |  (옵션)
    sso = 'sso_example' # str |  (옵션)

    try:
        api_response = api_instance.post_ban_user_from_comment(tenant_id, comment_id, PostBanUserFromCommentOptions(ban_email=ban_email, ban_email_domain=ban_email_domain, ban_ip=ban_ip, delete_all_users_comments=delete_all_users_comments, banned_until=banned_until, is_shadow_ban=is_shadow_ban, update_id=update_id, ban_reason=ban_reason, sso=sso))
        print("The response of ModerationApi->post_ban_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_from_comment: %s\n" % e)
[inline-code-end]