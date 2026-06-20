---
## Parametreler

| Name | Tür | Location | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| commentId | string | path | Evet |  |
| banEmail | boolean | query | Hayır |  |
| banEmailDomain | boolean | query | Hayır |  |
| banIP | boolean | query | Hayır |  |
| deleteAllUsersComments | boolean | query | Hayır |  |
| bannedUntil | string | query | Hayır |  |
| isShadowBan | boolean | query | Hayır |  |
| updateId | string | query | Hayır |  |
| banReason | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/ban_user_from_comment_result.py)

## Örnek

[inline-code-attrs-start title = 'post_ban_user_from_comment Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.ban_user_from_comment_result import BanUserFromCommentResult
from client.rest import ApiException
from pprint import pprint

# Sunucu tanımlamak isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisi örneği ile bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    ban_email = True # bool |  (isteğe bağlı)
    ban_email_domain = True # bool |  (isteğe bağlı)
    ban_ip = True # bool |  (isteğe bağlı)
    delete_all_users_comments = True # bool |  (isteğe bağlı)
    banned_until = 'banned_until_example' # str |  (isteğe bağlı)
    is_shadow_ban = True # bool |  (isteğe bağlı)
    update_id = 'update_id_example' # str |  (isteğe bağlı)
    ban_reason = 'ban_reason_example' # str |  (isteğe bağlı)
    sso = 'sso_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.post_ban_user_from_comment(comment_id, ban_email=ban_email, ban_email_domain=ban_email_domain, ban_ip=ban_ip, delete_all_users_comments=delete_all_users_comments, banned_until=banned_until, is_shadow_ban=is_shadow_ban, update_id=update_id, ban_reason=ban_reason, sso=sso)
        print("The response of ModerationApi->post_ban_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_from_comment: %s\n" % e)
[inline-code-end]

---