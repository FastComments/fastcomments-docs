## 參數

| 名稱 | 類型 | 位置 | 必要 | 描述 |
|------|------|----------|----------|-------------|
| sso | string | query | 否 |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 範例

[inline-code-attrs-start title = 'post_ban_user_undo 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.ban_user_undo_params import BanUserUndoParams
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.ModerationApi(api_client)
    ban_user_undo_params = client.BanUserUndoParams() # BanUserUndoParams | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_ban_user_undo(ban_user_undo_params, sso=sso)
        print("The response of ModerationApi->post_ban_user_undo:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_undo: %s\n" % e)
[inline-code-end]