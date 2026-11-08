## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | path | हाँ |  |
| banEmail | boolean | query | नहीं |  |
| banEmailDomain | boolean | query | नहीं |  |
| banIP | boolean | query | नहीं |  |
| deleteAllUsersComments | boolean | query | नहीं |  |
| bannedUntil | string | query | नहीं |  |
| isShadowBan | boolean | query | नहीं |  |
| updateId | string | query | नहीं |  |
| banReason | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/ban_user_from_comment_result.py)

## उदाहरण

[inline-code-attrs-start title = 'post_ban_user_from_comment उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostBanUserFromCommentOptions
from client.models.ban_user_from_comment_result import BanUserFromCommentResult
from client.rest import ApiException
from pprint import pprint

# होस्ट परिभाषित करना वैकल्पिक है और यह डिफ़ॉल्ट रूप से https://fastcomments.com होता है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API क्लाइंट के इंस्टेंस के साथ एक कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएं
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    ban_email = True # bool |  (वैकल्पिक)
    ban_email_domain = True # bool |  (वैकल्पिक)
    ban_ip = True # bool |  (वैकल्पिक)
    delete_all_users_comments = True # bool |  (वैकल्पिक)
    banned_until = 'banned_until_example' # str |  (वैकल्पिक)
    is_shadow_ban = True # bool |  (वैकल्पिक)
    update_id = 'update_id_example' # str |  (वैकल्पिक)
    ban_reason = 'ban_reason_example' # str |  (वैकल्पिक)
    sso = 'sso_example' # str |  (वैकल्पिक)

    try:
        api_response = api_instance.post_ban_user_from_comment(tenant_id, comment_id, PostBanUserFromCommentOptions(ban_email=ban_email, ban_email_domain=ban_email_domain, ban_ip=ban_ip, delete_all_users_comments=delete_all_users_comments, banned_until=banned_until, is_shadow_ban=is_shadow_ban, update_id=update_id, ban_reason=ban_reason, sso=sso))
        print("The response of ModerationApi->post_ban_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_from_comment: %s\n" % e)
[inline-code-end]