## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| commentId | string | path | 예 |  |
| banEmail | boolean | query | 아니요 |  |
| banEmailDomain | boolean | query | 아니요 |  |
| banIP | boolean | query | 아니요 |  |
| deleteAllUsersComments | boolean | query | 아니요 |  |
| bannedUntil | string | query | 아니요 |  |
| isShadowBan | boolean | query | 아니요 |  |
| updateId | string | query | 아니요 |  |
| banReason | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/ban_user_from_comment_result.py)

## 예제

[inline-code-attrs-start title = 'post_ban_user_from_comment 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.ban_user_from_comment_result import BanUserFromCommentResult
from client.rest import ApiException
from pprint import pprint

# 호스트 설정은 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 모든 지원되는 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스와 함께 컨텍스트에 들어갑니다
with client.ApiClient(configuration) as api_client:
    # API 클래스 인스턴스를 생성합니다
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    ban_email = True # bool |  (선택 사항)
    ban_email_domain = True # bool |  (선택 사항)
    ban_ip = True # bool |  (선택 사항)
    delete_all_users_comments = True # bool |  (선택 사항)
    banned_until = 'banned_until_example' # str |  (선택 사항)
    is_shadow_ban = True # bool |  (선택 사항)
    update_id = 'update_id_example' # str |  (선택 사항)
    ban_reason = 'ban_reason_example' # str |  (선택 사항)
    sso = 'sso_example' # str |  (선택 사항)

    try:
        api_response = api_instance.post_ban_user_from_comment(comment_id, ban_email=ban_email, ban_email_domain=ban_email_domain, ban_ip=ban_ip, delete_all_users_comments=delete_all_users_comments, banned_until=banned_until, is_shadow_ban=is_shadow_ban, update_id=update_id, ban_reason=ban_reason, sso=sso)
        print("The response of ModerationApi->post_ban_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_from_comment: %s\n" % e)
[inline-code-end]