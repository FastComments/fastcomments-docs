## 參數

| 名稱 | 型別 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| includeByUserIdAndEmail | boolean | query | 否 |  |
| includeByIP | boolean | query | 否 |  |
| includeByEmailDomain | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`PreBanSummary`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/pre_ban_summary.py)

## 範例

[inline-code-attrs-start title = 'get_pre_ban_summary 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.pre_ban_summary import PreBanSummary
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    include_by_user_id_and_email = True # bool |  (可選)
    include_by_ip = True # bool |  (可選)
    include_by_email_domain = True # bool |  (可選)
    sso = 'sso_example' # str |  (可選)

    try:
        api_response = api_instance.get_pre_ban_summary(comment_id, include_by_user_id_and_email=include_by_user_id_and_email, include_by_ip=include_by_ip, include_by_email_domain=include_by_email_domain, sso=sso)
        print("The response of ModerationApi->get_pre_ban_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_pre_ban_summary: %s\n" % e)
[inline-code-end]